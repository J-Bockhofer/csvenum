use std::io::Write;

//pub mod properties;

extern crate regex;

//use et_error::ETError;

//use properties::ConstProperty;
pub mod parser;
//use parser::TableParser;

pub mod enumtable;

pub mod code_gen;

pub mod types;
pub use types::{RType, RTypeTrait, SpecialType, StringType, NumericType, ContainerType};


//pub mod variants;

//use variants::Variant;
/// Property will be converted to: 
/// - Two functions:
/// 
///     as_(name) -> Enum
/// 
///     from_(name) -> (type_str)
/// 
/// 
/// - One Constant per Variant
/// 
///     const (VARIANT)_(NAME): (type_str)
pub mod table;
pub use table::EnumTable;

pub mod utils;
pub use utils::*;

pub mod reader;
use reader::{read_file_lines, write_lines_to_file};

pub mod to_code;
pub use to_code::{TextBlock, format_enum};
use to_code::{funcblock::{make_fn_get_all_variants, generate_fn_blocks}, format_consts};
#[derive(Debug)]
pub struct Document {
    //pub imports: TextBlock,
    code: Vec<TextBlock>,
}
impl Document {
    pub fn new() -> Self {
        Document { 
            code: vec![], 
        }
    }

    pub fn add_block(&mut self, textblock: TextBlock) {
        self.code.push(textblock);
    }

    pub fn write_to_file(&self, path: &str) {
        let mut file = std::fs::File::create(path).expect("Unable to create file");
        for block in &self.code {
            for line in &block.lines {
                writeln!(file, "{}", line).expect("Unable to write line to file");
            }
        }
    }

    pub fn print_to_lines(self) -> Vec<String> {
        let mut lines = vec![];
        for block in self.code {
            for line in block.lines {
                lines.push(line);
            }
        }
        lines
    }

}



/// Main struct that will be called with the csv file
pub struct Enumdotrs {

}



pub fn generate_from_csv_to_file() {

    let lines = read_file_lines("tests/pisse.csv").unwrap();

    let et = EnumTable::from_csv_lines(lines).unwrap();

    let variants = et.get_variants();
    //let properties = et.get_properties();
    //let types = et.get_types();
    let enumname = et.get_enumname();

    //let vars = make_vars(enumname, variants, properties, types).unwrap();

    let mut code_doc = Document::new();
    code_doc.add_block(format_enum(&et));
    code_doc.add_block(make_fn_get_all_variants(enumname, variants));

    code_doc.add_block(format_consts(&et));
    let fnblocks = generate_fn_blocks(&et);
    for fun in fnblocks {
        code_doc.add_block(fun);
    }

    let lines = code_doc.print_to_lines();
    write_lines_to_file("tests/pisse.rs", lines).unwrap();    
}


/* #[derive(Debug)]
pub struct Vars {
    pub enumname: String,
    pub variants: Vec<Variant>,
    pub properties: Vec<ConstProperty>,
}

impl Vars {
    pub fn new(enumname: &str, variants: Vec<Variant>, properties: Vec<ConstProperty>) -> Self {
        Vars { enumname: enumname.to_string(), variants, properties }
    }
} */


#[derive(Debug)]
pub struct Closure {
    pub header: String,
    pub content: String,

}

/// Function to generate const variable names for the passed variantnames and properties
/// 
/// Table of
/// TYPES    | &'static str | usize
/// EnumName | property1    | property2
/// Variant1 | Value1       | Value2
/* pub fn make_vars(enumname: &str, variantnames: Vec<&str>, properties: Vec<&str>, types: Vec<&str>) -> Option<Vars> {
    if properties.len() != types.len() {todo!("Log error: Types and Properties must have the same length")}
    //let num_variants = variantnames.len();
    let num_props = properties.len();
    // enforce first letter caps for variant

    // 1. Names for PropertyConstants - of properties.len() - FULL_CAPS
    let mut property_constants: Vec<ConstProperty> = vec![];
    for i in 0..num_props {
        let property = properties[i];
        let type_str = types[i];
        if !property.is_ascii() {todo!("Log error: Property Name not ASCII")}
        property_constants.push(
            ConstProperty::from(&enumname, property, type_str)
        );
    }

    // 2. Names for Variants - of variantnames.len() - unchanged
    let mut variant_constants: Vec<Variant> = vec![];
    for name in variantnames {
        variant_constants.push(Variant::new(name.to_string()));
    }

    //println!("{:?}", &property_constants);
    //println!("{:?}", &variant_constants);

    Some(Vars::new(&enumname, variant_constants, property_constants))

} */



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        generate_from_csv_to_file();
    }
}

