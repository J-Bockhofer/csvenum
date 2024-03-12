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
pub use types::{RType, RTypeTrait, SpecialType, StringType, NumericType, ContainerType, Reference};

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

    pub multival_split_symbol: String,

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
            multival_split_symbol: String::from("$"),
        }
    }
}

impl EnumOptions {
    pub fn from_options(path_to_outfile: String, split_files: bool, gen_variant_str_fns: bool, gen_impl_links:bool, multival_split_symbol: String) -> Self {
        EnumOptions { path_to_outfile: PathBuf::from(path_to_outfile), split_files, gen_variant_str_fns, gen_impl_links, experimental_no_type_restrictions: false , multival_split_symbol}
    }
    pub fn set_path_to_csv(&self, path_to_outfile: String) -> Self {
        EnumOptions { 
            path_to_outfile: PathBuf::from(path_to_outfile),
            split_files: self.split_files,
            gen_variant_str_fns: self.gen_variant_str_fns, 
            gen_impl_links: self.gen_impl_links, 
            experimental_no_type_restrictions: false , 
            multival_split_symbol: self.multival_split_symbol.clone() }
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


    let parser =  TableParser::from_csv_lines(lines, &options.multival_split_symbol);
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


