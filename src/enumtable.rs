
/* use thiserror::Error as Error;

#[derive(Error, Debug, PartialEq)]
pub enum ParserError {

    #[error("Parsed table needs to have at least 3 lines.")]
    TableEmptyOrShort,
    #[error("Number of types: {0} and number of properties: {1} do not match!")]
    TableHeaderMismatch(usize, usize),
    #[error("Number of types: {0} and number of properties: {1} do not match in data row {2}!")]
    TableDataMismatch(usize, usize, usize),    

} */

use crate::to_code::types::{TType, TypeToStr};

/// Gets created by the table parser
/// 
/// ## Example:
/// 
/// ```
///     use table2enum::parser::{TableParser, ToEnumTable};
/// 
///     let rows: Vec<&str> = vec![
///         "TYPES,         &str,       (usize$f64),    enum: E",
///         "MyEnumName,    Property1,  Property2,      Property3",
///         "Variant1,      standard,   (0$3.14),       A",
///         "Variant2,      medium,     (0$9.82),       B",
///     ];
/// 
///     let table_parser = TableParser::from_csv_lines(rows).unwrap();
///     let enumtable = table_parser.to_enumtable().unwrap();
///     assert_eq!(enumtable.get_name(), "MyEnumName");
/// 
/// ```
/// 
/// 
#[derive(Debug)]
pub struct EnumTable {
    enumname: String,
    types: Vec<String>,
    properties: Vec<String>,
    variants: Vec<String>,
    data: Vec<Vec<String>>,
    pub parsed_types: Vec<TType>,
}

impl EnumTable {
    pub fn new() -> Self {
        EnumTable { 
            enumname: String::new(), 
            types: Vec::new(), 
            properties: Vec::new(), 
            variants: Vec::new(), 
            data: Vec::new(),
            parsed_types: Vec::new() }
    }
    pub fn get_name(&self) -> &str {
        &self.enumname
    }
    pub fn set_name(&mut self, enumname: String) {
        self.enumname = enumname;
    }
    pub fn set_types(&mut self, types: Vec<String>) {
        self.types = types;
    }
    pub fn set_properties(&mut self, properties: Vec<String>) {
        self.properties = properties;
    }
    pub fn set_variants(&mut self, variants: Vec<String>) {
        self.variants = variants;
    }
    pub fn set_data(&mut self, data: Vec<Vec<String>>) {
        self.data = data;
    }
    pub fn get_col_of_property(&self, prop: &str) -> Option<usize> {
        self.properties.iter().position(|x| x == prop)
    }
    pub fn get_row_of_variant(&self, var: &str) -> Option<usize> {
        self.variants.iter().position(|x| x == var)
    }
    pub fn get_value_by_col_row(&self, col: usize, row: usize) -> Option<String> {
        let num_cols = self.data.len();
        if col >= num_cols {return None}
        let num_rows = self.data[col].len();
        if row >= num_rows {return None}
        return Some(self.data[col][row].to_owned())
    }

    pub fn parse_types(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut parsed_types = vec![];
        for type_str in &self.types {
            let ttype = TType::from_typestr(type_str)?;
            parsed_types.push(ttype);
        }
        self.parsed_types = parsed_types;
        Ok(())
    }
    pub fn get_variant_at(&self, idx: usize) -> &String {
        &self.variants[idx]
    }
    pub fn get_property_at(&self, idx: usize) -> &String {
        &self.properties[idx]
    }    

}
