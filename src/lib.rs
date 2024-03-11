

//pub mod properties;

extern crate regex;
extern crate clap;
//use et_error::ETError;

//use properties::ConstProperty;
pub mod parser;
pub use parser::TableParser;

pub mod enumtable;
pub use enumtable::EnumTable;

pub mod code_gen;
pub use code_gen::EnumModule;

pub mod types;
use parser::ToEnumTable;
pub use types::{RType, RTypeTrait, SpecialType, StringType, NumericType, ContainerType, Reference};



pub mod reader;
use reader::{read_file_lines, write_lines_to_file};





pub fn generate_from_csv_to_file() {

    let lines = read_file_lines("tests_a/pisse.csv").unwrap();

    let parser =  TableParser::from_csv_lines(lines, "$").unwrap();

    let et = parser.to_enumtable().unwrap();

    let em = EnumModule::new(&et, "tests_a/pisse.csv".to_string());

    let lines = em.to_lines();


    write_lines_to_file("tests_a/pisse.rs", lines).unwrap();    
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

