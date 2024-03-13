//! # csvenum-lib
//! 
//! Documentation for the csvenum-CLI lib
//! 
//! The following is meant for internal reference and the curious minded.
//! 
//! `If you are looking to generate enums from a csv, refer to the Usage section of the [readme](https://github.com/J-Bockhofer/csvenum/blob/main/README.md)`
//! 
//! 
//! ## Structure
//!  
//! 1. A .csv will be read into lines by the [reader] module.
//! 
//! 2. The lines will get separated into values by the [TableParser].
//! 
//! 3. The [TableParser] will then create and validate the [EnumTable].
//!  
//! 4. The [EnumTable] will be passed to the constructor of the [EnumModule] that handles code generation and CLI options.
//! 
//! The thin CLI calls the [generate_configured_enum_from_csv] function which handles the above steps.
//! 
//! To construct an enumtable for testing, the code will look something like this:
//! 
//! ```rust
//!
//!     use csvenum::TableParser;
//! 
//!     let rows: Vec<&str> = vec![
//!         "TYPES,         &str,       (usize,f64),    bool",
//!         "MyEnumName,    Property1,  Property2,      Property3",
//!         "Variant1,      standard,   (0,3.14),       true",
//!         "Variant2,      medium,     (0,9.82),       false",
//!     ];
//! 
//!     let table_parser = TableParser::from_csv_lines(rows);
//!     assert!(table_parser.is_ok());
//!     let enumtable = table_parser.unwrap().to_enumtable().unwrap();
//!     assert_eq!(enumtable.get_name(), "MyEnumName");
//!     // row and column indices have moved to only represent the associated values
//!     let val_var1_prop2 = enumtable.get_value_by_col_row(1,0).unwrap();
//!     assert_eq!(val_var1_prop2, "(0,3.14)");
//! ```
//! 
//! ## Code generation
//! 
//! To make writing the code-gen easier this crate implements a basic type system, consisting of enums and a unify'ing trait.
//! 
//! The base type being the [RType] that all other types are nested in.
//! 
//! The corresponding [RTypeTrait] is implemented for all nested types and provides QoL functions for validation and wrapping values in their respective container. 
//! 
//! The [code_gen] module contains the necessary code for generation. A small example can be found in the [code_gen::TextBlock] documentation.
//! 
//! 


extern crate regex;
extern crate clap;
extern crate thiserror;


use std::path::PathBuf;
use std::env::current_dir;


mod parser;
pub use parser::TableParser;

mod enumtable;
pub use enumtable::EnumTable;

pub mod code_gen;
pub use code_gen::EnumModule;

mod types;
pub use types::{RType, RTypeTrait, SpecialType, StringType, NumericType, ContainerType, Reference, containers::NestedValueParser};

mod reader;
pub use reader::{read_file_lines, write_lines_to_file};

/// Contains the options for code generation
#[derive(Debug)]
pub struct EnumOptions {
    /// The output file
    pub path_to_outfile: PathBuf,
    /// Whether the property declarations should be split into separate files 
    pub split_files: bool,

    /// Generate variant as & from str fns
    pub gen_variant_str_fns: bool,

    /// Pure conversion functions only or also impl links to them
    pub gen_impl_links: bool,

    /// EXPERIMENTAL: Unlock const type restrictions on input table
    pub experimental_no_type_restrictions: bool,


}

impl Default for EnumOptions { 
    /// by default the code will be a single file with all options
    fn default() -> Self {
        EnumOptions { 
            path_to_outfile: current_dir().unwrap(),
            split_files: false, 
            gen_variant_str_fns: true, 
            gen_impl_links: true, 
            experimental_no_type_restrictions: false,
        }
    }
}

impl EnumOptions {
    pub fn from_options(path_to_outfile: String, split_files: bool, gen_variant_str_fns: bool, gen_impl_links:bool) -> Self {
        EnumOptions { path_to_outfile: PathBuf::from(path_to_outfile), split_files, gen_variant_str_fns, gen_impl_links, experimental_no_type_restrictions: false }
    }
    pub fn set_path_to_csv(&self, path_to_outfile: String) -> Self {
        EnumOptions { 
            path_to_outfile: PathBuf::from(path_to_outfile),
            split_files: self.split_files,
            gen_variant_str_fns: self.gen_variant_str_fns, 
            gen_impl_links: self.gen_impl_links, 
            experimental_no_type_restrictions: false , 
         }
    }
}

/// Main function that stages file reading, parsing, validation and code gen
pub fn generate_configured_enum_from_csv(options: Option<EnumOptions>, path_to_csv: String) -> Result<(), Box<dyn std::error::Error>> {
    let options = if options.is_some() {options.unwrap()} else {EnumOptions::default().set_path_to_csv(path_to_csv.clone())};

    let lines =  read_file_lines(&path_to_csv);
    if let Err(err) = lines {
        eprintln!("Error reading file: {}", err);
        std::process::exit(1);
    }
    let lines = lines.unwrap();


    let parser =  TableParser::from_csv_lines(lines);
    if let Err(err) = parser {
        eprintln!("Error parsing file: {}", err);
        std::process::exit(1);
    }
    let parser = parser.unwrap();

    let et = parser.to_enumtable();
    if let Err(err) = et {
        eprintln!("Error validating table: {}", err);
        std::process::exit(1);
    }
    let et = et.unwrap();


    let mut em = EnumModule::new(&et, &options);

    let res = em.print_configured_to_file();
    if let Err(err) = res {
        eprintln!("Error writing table: {}", err);
        std::process::exit(1);
    }

    Ok(())
}


