
use std::collections::HashSet;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum TableError {
    #[error("Requested variant with name: {0} does not exist.")]
    NoVariantWithName(String),
    #[error("Requested property with name: {0} does not exist.")]
    NoPropertyWithName(String),    
    #[error("Out of bounds. Passed index {0} for data with {1} rows.")]
    OutOfBoundsRow(usize, usize),
    #[error("Out of bounds. Passed index {0} for data with {1} columns.")]
    OutOfBoundsColumn(usize, usize),
    #[error("Duplicate value detected in column: {0}, row: {1} with value: {2}.")]
    DuplicateValue(String, usize, String),
}

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
///     assert_eq!(enumtable.get_col_of_property("Property1"), Some(0));
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
    pub fn get_value_by_col_row(&self, col: usize, row: usize) -> Result<String, TableError> {
        let num_cols = self.data.len();
        if col >= num_cols {return Err(TableError::OutOfBoundsColumn(col, num_cols))}
        let num_rows = self.data[col].len();
        if row >= num_rows {return Err(TableError::OutOfBoundsRow(row, num_rows))}
        return Ok(self.data[col][row].to_owned())
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
    /// Checks each column for duplicate values
    pub fn check_duplicates(&self) -> Result<(), TableError> {
        let mut unique_checker = HashSet::new();
        let num_cols = self.properties.len();
        let num_rows = self.variants.len();
        for i in 0..num_cols {
            let prop = &self.properties[i];
            for j in 0..num_rows {
                let value = &self.data[i][j];
                if !unique_checker.insert(value) {
                    return Err(TableError::DuplicateValue(prop.to_owned(), j, value.to_owned()));
                }                
            }
            unique_checker.clear();
        }
        Ok(())
    }
    /// No extra bounds check other than Rust internal panic
    pub fn get_variant_at(&self, idx: usize) -> &String {
        &self.variants[idx]
    }
    /// No extra bounds check other than Rust internal panic
    pub fn get_property_at(&self, idx: usize) -> &String {
        &self.properties[idx]
    }    
    pub fn get_value_by_prop_var(&self, property: &str, variant: &str) -> Result<String, TableError> {
        let prop_idx = self.get_col_of_property(property);
        let col_idx = if prop_idx.is_none() {return Err(TableError::NoPropertyWithName(property.to_string()))} else {prop_idx.unwrap()};
        let var_idx = self.get_row_of_variant(variant);
        let row_idx = if var_idx.is_none() {return Err(TableError::NoVariantWithName(variant.to_string()))} else {var_idx.unwrap()};
        Ok(self.get_value_by_col_row(col_idx, row_idx)?)
    }
}
