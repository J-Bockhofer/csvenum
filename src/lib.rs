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
pub mod properties;
use properties::ConstProperty;

pub mod variants;
use variants::Variant;

pub mod table;
use table::EnumTable;

pub mod utils;
use utils::{indent_string};

use std::io::Write;


#[derive(Debug)]
pub struct Vars {
    pub enumname: String,
    pub variants: Vec<Variant>,
    pub properties: Vec<ConstProperty>,
}

impl Vars {
    pub fn new(enumname: &str, variants: Vec<Variant>, properties: Vec<ConstProperty>) -> Self {
        Vars { enumname: enumname.to_string(), variants, properties }
    }
}

#[derive(Debug)]
pub struct TextBlock {
    pub lines: Vec<String>,
}

impl TextBlock {
    pub fn with_lines(lines: Vec<String>) -> Self {
        TextBlock { lines }
    }

    pub fn write_to_file(&self, path: &str) {
        let mut file = std::fs::File::create(path).expect("Unable to create file");
        for line in &self.lines {
            writeln!(file, "{}", line).expect("Unable to write line to file");
        }
    }
}


#[derive(Debug)]
pub struct MatchBlock {
    pub match_this: String,
    pub arms_lh: Vec<String>,
    pub arms_rh: Vec<String>,
}



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
pub fn make_vars(enumname: &str, variantnames: Vec<&str>, properties: Vec<&str>, types: Vec<&str>) -> Option<Vars> {
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

}

pub fn format_enum(enumtable: &EnumTable) -> TextBlock {

    let indent:usize = 4;

    let mut lines: Vec<String> = vec![]; 
    let enumname = enumtable.get_enumname();
    let props = enumtable.get_properties();
    let num_props = props.len();
    let variants = enumtable.get_variants();

    // Header
    lines.push(format!("pub enum {} {{", enumname));

    for row in 0..variants.len() {
        let cvar= variants[row];
        // Documentation
        let l = format!("/// {}", cvar);
        lines.push(indent_string(l, indent));     
        for col in 1..num_props + 1 {
            let l = format!("/// Property `{}` with value `{}` ", &props[col - 1], enumtable.data[col][row]);
            lines.push(
                indent_string(l, indent)
            ); 
        }
        // Declaration
        let l = format!("{},", cvar);
        lines.push(
            indent_string(l, indent)
        );
    }

    // Footer
    lines.push(
        format!("}}")
    );

    TextBlock::with_lines(lines)
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let x = make_vars("Country", vec!["Australia"], vec!["Alpha3"], vec!["&'static str"]).unwrap();

        println!("{:?}", &x);
    }
}

