
use thiserror::Error as Error;

use super::enumtable::EnumTable;

#[derive(Error, Debug, PartialEq)]
pub enum ParserError {
    #[error("Parsed table needs to have at least 3 lines.")]
    TableEmptyOrShort,
    #[error("Number of types: {0} and number of properties: {1} do not match!")]
    TableHeaderMismatch(usize, usize),
    #[error("Number of types: {0} and number of properties: {1} do not match in data row {2}. Missing Value?")]
    TableDataMismatch(usize, usize, usize),
}

/// Parses a Vec<&str> with CSV to an intermediate representation
/// 
/// ## Example:
/// 
/// ```
///     use table2enum::parser::{TableParser, ToEnumTable};
/// 
///     let rows: Vec<&str> = vec![
///         "TYPES,         &str,       (usize$f64),    &str",
///         "MyEnumName,    Property1,  Property2,      Property3",
///         "Variant1,      standard,   (0$3.14),       cheap",
///         "Variant2,      medium,     (0$9.82),       pricey",
///     ];
/// 
///     let table_parser = TableParser::from_csv_lines(rows, "$");
///     assert!(table_parser.is_ok());
///     let enumtable = table_parser.unwrap().to_enumtable().unwrap();
///     assert_eq!(enumtable.get_name(), "MyEnumName");
///     let val_var1_prop2 = enumtable.get_value_by_col_row(1,0).unwrap();
///     assert_eq!(val_var1_prop2, "(0,3.14)");
///     
/// 
/// ```
/// 
/// TYPES,      &str,       (usize$f64)
/// 
/// EnumName,   Property1,  Property2    
/// 
/// Variant1,   asdfg,      (12$ 3.14)
/// 
/// Variant2,   eghad,      (38$ 9.81)
///  If parsed thing is enum -> Declaration in csv should be "enum: E" should yield in return E and E::A in match
#[derive(Debug)]
pub struct TableParser {

    type_row_idx: usize,

    //data_row_start_idx:usize,
    type_row: Vec<String>,
    property_row: Vec<String>,
    data_cols: Vec<Vec<String>>,

}

impl TableParser {
    pub fn new() -> Self {
        TableParser{
            type_row_idx: 0,
            type_row: vec![],
            property_row: vec![],
            data_cols: vec![],            
        }
    }
    #[allow(dead_code)]
    fn set_type_row_idx(&mut self, idx: usize) {
        self.type_row_idx = idx;
    }
    fn set_type_row(&mut self, line: Vec<String>) {
        self.type_row = line;
    }
    fn set_property_row(&mut self, line: Vec<String>) {
        self.property_row = line;
    }
    fn set_data_rows(&mut self, cols: Vec<Vec<String>>) {
        self.data_cols = cols;
    }
    pub fn from_csv_lines<T: AsRef<str>>(lines: Vec<T>, value_separator: &str)-> Result<Self, ParserError> {
        let mut this = TableParser::new();
        let type_row_idx = this.type_row_idx;
        let prop_row_idx = type_row_idx + 1;
        let dat_row_idx = type_row_idx + 2;

        let num_lines = lines.len();
        if lines.is_empty() || num_lines < 3 {return Err(ParserError::TableEmptyOrShort)}

        let type_row: Vec<String> = lines[type_row_idx].as_ref().split(',').map(|x|{x.replace("\"", " ").replace(value_separator, ",").trim().to_string() }).collect();
        //match type_row[0] {} == "TYPES".to_string()

        let prop_row: Vec<String> = lines[prop_row_idx].as_ref().split(',').map(|x|{x.replace("\"", " ").replace(value_separator, ",").trim().to_string()}).collect();
        let num_cols = type_row.len();

        if num_cols != prop_row.len() {return Err(ParserError::TableHeaderMismatch(num_cols, prop_row.len()))}
        
         // one less column bc variants are not properties/columns

        let mut data: Vec<Vec<String>> = vec![]; // first is column second is row
        for _ in 0..num_cols {
            data.push(vec![]); // init empty colum
        }
        for row in dat_row_idx..num_lines {
            if lines[row].as_ref().is_empty() {continue}
            let line: Vec<String> = lines[row].as_ref().split(',').map(|x|{x.replace("\"", " ").replace(value_separator, ",").trim().to_string()}).collect();
            if line.len() != num_cols {return Err(ParserError::TableDataMismatch(num_cols, line.len(), row))}
            for col in 0..num_cols {
                data[col].push(line[col].clone());
            }
        } 
        this.set_type_row(type_row);
        this.set_property_row(prop_row);
        this.set_data_rows(data);
        
        Ok(this)
    }



}


pub trait ToEnumTable {
    fn to_enumtable(&self) -> Result<EnumTable, Box<dyn std::error::Error>>;
}

impl ToEnumTable for TableParser {
    fn to_enumtable(&self) -> Result<EnumTable, Box<dyn std::error::Error>> {
        let mut enumtable = EnumTable::new();
        // the enumname can be found in the first column of the property row, the rest are actual properties.
        let name = &self.property_row[0];
        enumtable.set_name(name.to_string());
        // properties
        let properties = &self.property_row[1..self.property_row.len()];
        enumtable.set_properties(properties.into());
        // types are same as properties
        let types = &self.type_row[1..self.type_row.len()];
        enumtable.set_types(types.into())?;
        // variants
        let variants = &self.data_cols[0];
        enumtable.set_variants(variants.to_owned());
        // values 
        let values = &self.data_cols[1..self.data_cols.len()];
        enumtable.set_data(values.to_owned());
        enumtable.check_duplicates()?;
        enumtable.check_valid_values()?;
        enumtable.check_valid_types_for_code()?;
        

        Ok(enumtable)
    }
}

