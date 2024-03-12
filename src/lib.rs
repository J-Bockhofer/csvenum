///! # csvenum
///! 
///! CLI to generate Rust enums with associated constants from a csv-table. 
///! 
///! 
///! ## Why?
///! 
///! If you have ever needed to declare a lot of constants you will know that it may require looking at a dataset and copying the values to code.
///! 
///! Not much fun, and LLMs still tend to have a short memory, making consistent results a challenge.
///! 
///! So here's a code-gen to assist in creating enums with associated constants. Like route or query parameter bindings for an API.
///! 
///! Just declare a table in a .csv, generate and you're done.
///! 
///! Especially for painstakingly hand-coded, or macro-intensive libraries like [celes](https://crates.io/crates/celes), it may be worthwhile to consider including a .csv file in the source code. 
///! 
///! This file could provide users with a convenient means to append their own data.
///! 
///! 
///! # Usage
///! 
///! Since this crate is meant to be a tool for speeding up Rust development, it is available as a `cargo install`.
///! 
///! ```console
///! cargo install csvenum
///! ```
///! 
///! 
///! After installing, create a table like this in a `.csv`:
///! 
///! ```
///! ENUM        , &str      , usize     ,  (f64 $ f64)
///! 
///! Country     , ISO3      , Numeric   ,  Lat_Lon
///! 
///! Sweden      , SWE       , 752       ,  (60.128161 $ 18.643501)
///! 
///! Vietnam     , VDR       , 704       ,  (14.058324 $ 108.277199)
///! 
///! Brazil      , BRA       , 076       ,  (-14.235004 $ -51.92528)
///! 
///! ```
///! 
///! 
///! To generate the code pass the filename to the CLI. Here `countries.csv`.
///! 
///! 
///! ```console
///! cargo csvenum countries.csv
///! ```
///! 
///! 
///! The generated enum will be in a file called `country.rs` in the directory of the passed `.csv`.
///! 
///! 
///! In code you can now access the values like so:
///! 
///! ```rust
///!     let countries = Country::get_all_variants();
///!     for country in countries {
///!         println!("{}", country);
///!         println!("{}", country.as_iso3());
///!     }    
///! ```
///! 
///! 
///! Will output:
///! 
///! 
///! ```console
///! Sweden, ISO3 = SWE , Numeric = 752 , Lat_Lon = (60.128161 , 18.643501) 
///! SWE
///! Vietnam, ISO3 = VDR , Numeric = 704 , Lat_Lon = (14.058324 , 108.277199) 
///! VDR
///! Brazil, ISO3 = BRA , Numeric = 076 , Lat_Lon = (-14.235004 , -51.92528) 
///! BRA
///! ```
///! 
///! 
///! See CLI options and the table format below for details.
///! 
///! 
///! 
///! ## Table format
///! 
///! A table for code-gen with `csvenum` will always have the following shape.
///! 
///! - First line: Specifies the types of the column values, starting with the word `ENUM`.
///! 
///! - Second line: Specifies the name for the enum and the column names, referred to here as properties.
///! 
///! - Third line and after: The data.
///! 
///! The data will be rows, starting with the variant name, followed by the values per property.
///! 
///! Example
///! 
///! ```
///! ENUM,      &str,       usize,      (f64$f64)        <-- Column types
///! 
///! GPIOpin,    Address,    Value,      XY              <-- Enum name followed by the property names
///! 
///! PIN0,       0x00,       42,         (3.57$4.56)     <-- Variant name and associated values
///! PIN1,       0x02,       56,         (8.12$7.64)
///! PIN2,       0x04,       68,         (5.84$2.75)
///! 
///! ```
///! 
///! For now tables are limited to only include constant values, but there are plans to provide OnceLock<> implementations for others.
///! 
///! Also there is an arbitrary but reasonable limit on value nesting depth to avoid headaches.
///! 
///! Note that property names will have to follow valid 
///! 
///! ## CLI options
///! 
///! ```console
///! cargo csvenum --help
///! ```
///! 
///! ```
///! Usage: csvenum [OPTIONS] <FILENAME_CSV>
///! 
///! Arguments:
///!   <FILENAME_CSV>  Filename of the CSV file (required)
///! 
///! Options:
///!   -o, --outfile 
///!           Path to the output file (optional)
///!   -s, --split-properties 
///!           Whether to split property declarations into separate files (optional), defaults to: false [possible values: true, false]
///!   -v, --variant-str-fns 
///!           Generate variant as & from str fns (optional), defaults to: true [possible values: true, false]
///!   -i, --impl-links 
///!           Pure conversion functions only or also impl links to them (optional), defaults to: true [possible values: true, false]
///!   -m, --multival-split-symbol
///!           Multi-value split symbol (optional), defaults to: '$'
///!   -h, --help
///!           Print help
///!   -V, --version
///!           Print version
///! ```
///! 
///! ## Future plans
///! 
///! 0. Need to deal with trailing commas in csv.
///! 
///! 0. Generate FromStr impl to check all associated string constants.
///! 
///! 0. Check u-numeric values for sign.
///! 
///! 1. Provide `OnceLock` wrappers for non-const statics.  
///! 
///! 2. impl custom Ord as special column of usize.
///! 
///! 3. Option on data for missing values.
///! 
///! 4. BTrees and HashMaps for large datasets.
///! 
///! Please report any issue you find or suggestion you have to further improve this tool!



extern crate regex;
extern crate clap;
extern crate thiserror;

//mod tester;

use std::path::PathBuf;
mod parser;
pub use parser::TableParser;
pub use parser::ToEnumTable;


mod enumtable;
pub use enumtable::EnumTable;

pub mod code_gen;
pub use code_gen::EnumModule;

mod types;
pub use types::{RType, RTypeTrait, SpecialType, StringType, NumericType, ContainerType, Reference, containers::NestedValueParser};

use std::env::current_dir;

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


