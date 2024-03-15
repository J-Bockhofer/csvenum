
use std::collections::{HashMap,HashSet};
use thiserror::Error;

#[allow(dead_code)]
mod structs;
use structs::Property;

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
    #[error("Invalid value: {0} for property: {1} with type:{2} detected in row: {3}.")]
    InvalidValueForPropertyWithTypeAtRow(String, String, String, usize),
    #[error("Failed to get valid data for a field of the EnumTable struct.")]
    DataEmpty,
    #[error("Missing feature: {0}")]
    MissingFeature(String),
    #[error("Duplicate cariant detected: {0} in row: {1}.")]
    DuplicateVariant(String, usize),

}

use crate::{RType, RTypeTrait};

/// Gets created by the table parser
/// 
/// ## Example:
/// 
/// ```
///     use csvenum::TableParser;
/// 
///     let rows: Vec<&str> = vec![
///         "TYPES,         &str,       (usize,f64),    &str",
///         "MyEnumName,    Property1,  Property2,      Property3",
///         "Variant1,      standard,   (0,3.14),       cheap",
///         "Variant2,      medium,     (0,9.82),       pricey",
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
    pub tprops: Vec<Property>,
    types: Vec<String>,
    properties: Vec<String>,
    variants: Vec<String>,
    data: Vec<Vec<String>>,
    pub parsed_types: Vec<RType>,
    // column, unique value (value, group)
    col_has_duplicates: Vec<bool>,
    col_unique_value_maps: Vec<Vec<(String, Vec<String>)>>,
}

impl EnumTable {
    pub fn new() -> Self {
        EnumTable { 
            enumname: String::new(),
            tprops: Vec::new(),
            types: Vec::new(), 
            properties: Vec::new(), 
            variants: Vec::new(), 
            data: Vec::new(),
            parsed_types: Vec::new(),
            col_unique_value_maps: Vec::new(),
            col_has_duplicates: Vec::new(),
         }
    }
    pub fn get_name(&self) -> &str {
        &self.enumname
    }
    pub fn set_name(&mut self, enumname: String) {
        self.enumname = enumname;
    }
    /// Will parse the types and throw an error if any were unparseable
    pub fn set_types(&mut self, types: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
        self.types = types;
        self.parse_types()?;
        Ok(())
    }
    /// Properties will have the length of columns
    pub fn set_properties(&mut self, properties: Vec<String>) {
        self.properties = properties;
    }
    pub fn get_properties(&self) -> &Vec<String> {
        &self.properties
    }
    pub fn get_variants(&self) -> &Vec<String> {
        &self.variants
    }
    /// Variants will have the length of rows
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
            let ttype = RType::from_typestr(type_str)?;
            parsed_types.push(ttype);
        }
        self.parsed_types = parsed_types;
        Ok(())
    }
    // Checks each column for duplicate values, should not error but provide a Vec<Vec<bool>> for col,row = value is_duplicate or not
    // For properties with duplicates we should iterate over the duplicate values and have the associated variants prelinked
    // Make HashMap of K:value V: Vec<variantname>
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
    pub fn check_column_duplicates(&mut self) {
        let mut has_duplicates = Vec::new();

        for column in self.data.iter() {
            let mut seen = HashSet::new();
            let mut has_duplicate = false;
    
            for value in column {
                if !seen.insert(value) {
                    has_duplicate = true;
                    break;
                }
            }
    
            has_duplicates.push(has_duplicate);
        }
        self.col_has_duplicates = has_duplicates;
    }
    pub fn get_col_has_duplicate_vec(&self) -> &Vec<bool> {
        &self.col_has_duplicates
    }

    pub fn check_duplicate_variants(&self) -> Result<(), TableError> {
        let mut unique_checker = HashSet::new();
        let mut row_cnt = 0;
        for variant in &self.variants {
            if !unique_checker.insert(variant) {
                return Err(TableError::DuplicateVariant(variant.to_owned(), row_cnt));
            }  
            row_cnt += 1;
        }
        Ok(())
    }

    pub fn make_duplicate_map(&mut self) {
        let mut col_val_grp: Vec<Vec<(String, Vec<String>)>> = vec![];
        let num_cols = self.properties.len();
        for i in 0..num_cols {
            if self.col_has_duplicates[i] {
                let unique_groupings = group_variants_by_value(self.variants.clone(), self.data[i].clone());
                col_val_grp.push(unique_groupings);
            } else {
                col_val_grp.push(vec![]);
            }
        }
        self.col_unique_value_maps = col_val_grp;
    }

    pub fn get_duplicate_value_map(&self) -> &Vec<Vec<(String, Vec<String>)>> {
        &self.col_unique_value_maps
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
    // Ok if all good, else TableError::InvalidValueForPropertyWithTypeAtRow(String, String, String, usize)
    pub fn check_valid_values(&self) -> Result<(), TableError> {
        if self.parsed_types.is_empty() {return Err(TableError::DataEmpty)}
        let rtypes = &self.parsed_types;

        // loop over properties
        for i in 0..self.data.len() {

            let col_data = &self.data[i];
            let col_type = &rtypes[i];

            // loop over values
            for j in 0..col_data.len() {
                
                let val = &col_data[j];
                if !col_type.value_is_valid(val) {
                    return Err(TableError::InvalidValueForPropertyWithTypeAtRow(val.to_string(), self.properties[i].clone(), col_type.to_typestr(), j))
                }
            }
        } 

        Ok(())
    }

    pub fn check_all_types_const(&self) -> Result<(), TableError> {
        for rtype in &self.parsed_types {
            if !rtype.is_const() {return Err(TableError::MissingFeature(format!("Type: {} is non-const. Static types not implemented yet.", rtype.to_typestr())))}
        }
        Ok(())
    }
    pub fn all_types_depth_smaller_than(&self, rhs: usize) -> bool {
        for rtype in &self.parsed_types {
            if rtype.get_depth(0) > rhs as usize {return false;}
        }
        true     
    }
    pub fn all_types_breadth_smaller_than(&self, rhs: usize) -> bool {
        for rtype in &self.parsed_types {
            if rtype.get_breadth(0) > rhs as usize {return false;}
        }
        true     
    }

    /// In Lieu of a fully working code_gen for all types and a config we do all here for now
    pub fn check_valid_types_for_code(&self) -> Result<(), TableError> {
        self.check_all_types_const()?;
        let max_depth = 2;
        let max_breadth = 5;
        if !self.all_types_depth_smaller_than(max_depth) || !self.all_types_breadth_smaller_than(max_breadth) {
            return Err(TableError::MissingFeature(String::from("Table contains to deeply nested values.")))
        }
        Ok(())
    }

}

pub fn group_variants_by_value(variants: Vec<String>, values: Vec<String>) -> Vec<(String, Vec<String>)> {
    let mut value_map: HashMap<&String, Vec<String>> = HashMap::new();
    let mut unique_values: HashSet<&String> = HashSet::new();
    let mut result: Vec<(String, Vec<String>)> = Vec::new();

    // Iterate over the indices and group variants by their corresponding values
    for (index, value) in values.iter().enumerate() {
        // Check if the value has been encountered before
        if !unique_values.contains(value) {
            unique_values.insert(value);
            value_map.insert(value, Vec::new());
        }
        // Push the variant to the corresponding value group
        if let Some(group) = value_map.get_mut(value) {
            group.push(variants[index].clone());
        }
    }
    //println!("{:?}", unique_values);

    for value in unique_values.iter() {
        if let Some(group) = value_map.get(value) {
            result.push((value.to_string(), group.clone()));
        }
    }
    result    
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_group_variants_by_value() {
        let variants = vec!["Variant1".to_string(), "Variant2".to_string(), "Variant3".to_string(), "Variant4".to_string()];
        let values = vec!["Value1".to_string(), "Value2".to_string(), "Value1".to_string(), "Value3".to_string()];
    
        let result = group_variants_by_value(variants, values);
        println!("{:?}", result);

    }

}