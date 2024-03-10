use crate::{RType, RTypeTrait};

use std::sync::OnceLock;
use regex::Regex;

use super::NestedValueParser;

/// For checking value declarations
const ARRAY_VALUE_REGEX_STR: &'static str = r"^\[ ?(.+), ?([\d]+) ?\]"; 
static ARRAY_VALUE_REGEX: OnceLock<Regex> = OnceLock::new();



pub struct ArrayType {

}

impl ArrayType {
    pub fn wrap_str(vstr: &str, size: &usize) -> String {
        format!("[{}, {}]", vstr, size)
    }

    /// let aaa: [[usize; 3]; 3] = [[3,3,3], [3,3,3], [3,3,3]];
    /// 
    /// After step 1 = [3,3,3], [3,3,3], [3,3,3]
    /// 
    /// 
    /// rtype = type T of the array [T; N]
    pub fn value_is_valid(valuestr: &str, rtype: &Box<RType>, size: &usize) -> bool {
        //let aaa: [Vec<String>; 3] = [vec![], vec![], vec![]];
        let values = NestedValueParser::parse_nested_str(valuestr);
        println!("RType: {:?}", rtype);
        println!("ValStr: {:?}", valuestr);
        println!("Size: {:?}", size);

/*         // Example valuestr = "[[3,3,3], [3,3,3], [3,3,3]]"
        let mut valuestr = valuestr.trim().to_string();
        let firstchar = valuestr.remove(0);
        if firstchar != '[' {return false}
        let lastchar = valuestr.remove(valuestr.len() -1);
        if lastchar != ']' {return false}
        // Now is "[3,3,3], [3,3,3], [3,3,3]"
        // Need to split into the three groupings
        // "3,3,3"
        //let seg_len = (valuestr.len() as f32 / *size as f32).round() as usize ;

        match rtype.as_ref() {
            RType::Container(_, _) => {todo!("nested pain")}
            _ => {
                let valuestrs: Vec<&str> = valuestr.trim().split(',').collect();
                if valuestrs.len() != *size {return false} // not as many values as the size implies
                for val in valuestrs {
                    if !rtype.value_is_valid(val) {return false}
                }
                true

            } 
        } */
        let mut is_valid = true;
        for value in values {
            if !rtype.value_is_valid(&value) {is_valid = false}
        }
        is_valid



    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ RType, Reference, ContainerType}; //NumericType, StringType, SpecialType,

    #[test]
    fn test_arraytype_valid_values() { 
        let input = "[usize; 3]";
        let result = ContainerType::from_typestr(input).unwrap();
        let rtype = RType::from_typestr(input).unwrap();
        let values = "[3,3,3]";
        assert!(result.value_is_valid(values));
        let values = "[art,5.43,3]";
        //assert_eq!(false, ArrayType::value_is_valid(values, &Box::new(RType::Container(Reference::None, result)), &3));

        assert_eq!(false, rtype.value_is_valid(values));
    }

    #[test]
    fn test_arraytype_valid_values_complex() {
        let typestr = "[[usize; 3]; 3]";
        let values = "[[3,3,3], [3,3,3], [3,3,3]]";
        let rtype = RType::from_typestr(typestr).unwrap();
        assert_eq!(true, rtype.value_is_valid(values));
    }




}