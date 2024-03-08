use super::{RType, RTypeTrait, TypeError};
use regex::Regex;
use std::sync::OnceLock;

/// Vector regex
pub const VECTOR_REGEX_STR: &'static str = r"^Vec<([\w&_:<>\(\)\[\], ]*)>";
pub static VECTOR_REGEX: OnceLock<Regex> = OnceLock::new();

/// Array regex
pub const ARRAY_REGEX_STR: &'static str = r"^\[ ?(.+), ?([\d]+) ?\]";
pub static ARRAY_REGEX: OnceLock<Regex> = OnceLock::new();

/// Tuple regex 
pub const TUPLE_REGEX_STR: &'static str = r"^ ?\( ?(.+) ?\)";
pub static TUPLE_REGEX: OnceLock<Regex> = OnceLock::new();

// types that are parsed from some text content and will be turned into code, will include helper functions
/// Containers with single type T
/// 
/// Hash Map would need a value, but it would be possible to make a default hashmap constructor function from property1 K to property2 V
/// 
#[derive(Debug, Eq, PartialEq)]
pub enum ContainerType {
    Vector(Box<RType>),
    Tuple(Vec<Box<RType>>),
    Array(Box<RType>, usize),
    //HashSet(Box<RType>),
    //HashMap(Box<RType>, Box<RType>),

}

impl RTypeTrait for ContainerType {
    fn from_typestr<T: AsRef<str>>(typestr: T) -> Result<Self, super::TypeError> where Self: Sized {
        let typestr = typestr.as_ref();
        let vec_re = VECTOR_REGEX.get_or_init(|| Regex::new(VECTOR_REGEX_STR).unwrap());
        let arr_re = ARRAY_REGEX.get_or_init(|| Regex::new(ARRAY_REGEX_STR).unwrap());
        let tuple_re = TUPLE_REGEX.get_or_init(|| Regex::new(TUPLE_REGEX_STR).unwrap());

        // check what type of container we are:
        // 1. Vector
        if let Some(captures) = vec_re.captures(&typestr) {
            let inner_type = captures.get(1).unwrap().as_str();
            return Ok(ContainerType::Vector(Box::new(RType::from_typestr(inner_type)?)));
        }
        // 2. Array
        if let Some(captures) = arr_re.captures(&typestr) {
            let inner_type = captures.get(1).unwrap().as_str();
            let size = captures.get(2).unwrap().as_str().parse::<usize>().unwrap();
            return Ok(ContainerType::Array(Box::new(RType::from_typestr(inner_type)?), size));
        } 

        // 3. Tuple          
        if let Some(captures) = tuple_re.captures(&typestr) {
            let inner_str = captures.get(1).unwrap().as_str();
            let types_in_str: Vec<&str> = inner_str.split(',').collect();
            let mut tuple_types: Vec<Box<RType>> = vec![];
            for inner_type in types_in_str {
                let inner_type = inner_type.replace(" ", "");
                tuple_types.push(Box::new(RType::from_typestr(inner_type)?))
            }
            return Ok(ContainerType::Tuple(tuple_types));
        }


        Err(TypeError::ContainerTypeUnknown(typestr.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{NumericType, StringType, SpecialType};

    #[test]
    fn test_parse_container_type() { 
        let input = "Vec< &str>";
        let result = ContainerType::from_typestr(input).unwrap();
        let expected = ContainerType::Vector(Box::new(RType::String(StringType::str)));
        assert_eq!(result, expected);

        let input = "[usize, 3]";
        let result = ContainerType::from_typestr(input).unwrap();
        let expected = ContainerType::Array(
            Box::new(RType::Numeric(NumericType::usize)), 3
        );
        assert_eq!(result, expected);

        let input = "(usize,usize,enum:MyEnum)";
        let result = ContainerType::from_typestr(input).unwrap();
        let expected = ContainerType::Tuple( vec![
            Box::new(RType::Numeric(NumericType::usize)), 
            Box::new(RType::Numeric(NumericType::usize)), 
            Box::new(RType::Special(SpecialType::Enum("MyEnum".to_string())))
        ]);
        assert_eq!(result, expected);

    }

}