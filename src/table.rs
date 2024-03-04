
#[derive(Debug)]
pub struct EnumTable {
    /// h1 contains types
    pub h1: Vec<String>,
    /// h2 contains property names
    pub h2: Vec<String>,
    /// the data, first vector are columns/ properties
    pub data: Vec<Vec<String>>,
}

impl EnumTable {
    pub fn new(h1: Vec<String>, h2: Vec<String>, data: Vec<Vec<String>>) -> Self {
        EnumTable { h1, h2, data }
    }

    pub fn is_empty(&self) -> bool {
        if self.h1.is_empty() || self.h2.is_empty() || self.data.is_empty() {
            return true 
        }
        false
    }

    pub fn get_properties(&self) -> Vec<&str> {
        let mut props: Vec<&str> = vec![];
        for i in 1..self.h2.len() {
            props.push(&self.h2[i]);
        }
        props
    }

    pub fn get_types(&self) -> Vec<&str> {
        let mut types: Vec<&str> = vec![];
        for i in 1..self.h1.len() {
            types.push(&self.h1[i]);
        }
        types
    }

    pub fn get_variants(&self) -> Vec<&str> {
        let mut variants: Vec<&str> = vec![];
        // iterate through the first column of data
        for i in 0..self.data[0].len() {
            variants.push(&self.data[0][i]);
        }
        variants
    }

    pub fn get_enumname(&self) -> &str {
        &self.h2[0]
    }    

    pub fn from_csv_lines(lines: Vec<&str>) -> Option<EnumTable> {
        let num_lines = lines.len();
        if lines.is_empty() || num_lines < 3 {return Option::None;}

        let h1: Vec<String> = lines[0].split(',').map(|x|{x.trim().to_string()}).collect();
        let h2: Vec<String> = lines[1].split(',').map(|x|{x.trim().to_string()}).collect();

        let num_cols = h1.len(); // one less column bc variants are not properties/columns

        let mut data: Vec<Vec<String>> = vec![]; // first is column second is row
        for _ in 0..num_cols {
            data.push(vec![]); // init empty colum
        }
        for row in 2..num_lines {
            let line: Vec<String> = lines[row].split(',').map(|x|{x.trim().to_string()}).collect();
            for col in 0..num_cols {
                data[col].push(line[col].clone());
            }
        }
        Some(EnumTable::new(h1, h2, data))
    }

}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::{make_vars, format_enum};

    #[test]
    fn enumtable_from_csv_lines() {
        let lines = vec![
            "TYPES,&'static str,usize",
            "Country,Alpha3,Numeric",
            "Australia,AUS,157",
            "Denmark,DEN,32",
        ];
        let et = EnumTable::from_csv_lines(lines).unwrap();

        let variants = et.get_variants();
        let properties = et.get_properties();
        let types = et.get_types();
        let enumname = et.get_enumname();

        let vars = make_vars(enumname, variants, properties, types).unwrap();

        let text = format_enum(&et);

        text.write_to_file("pisse.rs");

        println!("{:?}", &text);
        //println!("{:?}", &vars);
    }
}
