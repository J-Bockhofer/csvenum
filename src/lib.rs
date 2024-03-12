

//pub mod properties;

extern crate regex;
extern crate clap;
extern crate thiserror;
//use et_error::ETError;

//use properties::ConstProperty;
pub mod parser;
use std::path::PathBuf;

pub use parser::TableParser;

pub mod enumtable;
pub use enumtable::EnumTable;

pub mod code_gen;
pub use code_gen::EnumModule;

pub mod types;
use parser::ToEnumTable;
pub use types::{RType, RTypeTrait, SpecialType, StringType, NumericType, ContainerType, Reference};

use std::env::current_dir;

pub mod reader;
use reader::read_file_lines;


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

    let lines = read_file_lines(&path_to_csv)?;

    let parser =  TableParser::from_csv_lines(lines, &options.multival_split_symbol)?;

    let et = parser.to_enumtable()?;

    let mut em = EnumModule::new(&et, &options);

    em.print_configured_to_file()?;

    Ok(())
}


