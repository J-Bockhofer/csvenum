use super::{RType, RTypeTrait, TypeError, Reference};
use regex::Regex;
use std::sync::OnceLock;

/// Vector regex
pub const VECTOR_REGEX_STR: &'static str = r"^Vec<(.*)>"; // why is this so explicit about the char set? -> r"^Vec<([\w&_:<>\(\)\[\], ]*)>";
pub static VECTOR_REGEX: OnceLock<Regex> = OnceLock::new();

/// Array regex
pub const ARRAY_REGEX_STR: &'static str = r"^\[ ?(.+), ?([\d]+) ?\]";
pub static ARRAY_REGEX: OnceLock<Regex> = OnceLock::new();

/// Tuple regex 
pub const TUPLE_REGEX_STR: &'static str = r"^\( ?(.+) ?\)";
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
                let inner_type = inner_type.trim();
                tuple_types.push(Box::new(RType::from_typestr(inner_type)?))
            }
            return Ok(ContainerType::Tuple(tuple_types));
        }


        Err(TypeError::ContainerTypeUnknown(typestr.to_string()))
    }

    fn to_typestr(&self) -> String {
        match self {
            ContainerType::Vector(x) => {
                format!("Vec<{}>", x.to_typestr())
            },
            ContainerType::Array(x, y) => {
                format!("[{}, {}]", x.to_typestr(), y)
            },
            ContainerType::Tuple(vx) => {
                let mut tuplestr = String::from("(");
                for x in vx {
                    tuplestr = tuplestr + &x.to_typestr() + ", ";
                }
                tuplestr.pop(); 
                tuplestr.pop();
                tuplestr.push(')');
                tuplestr
            }
        }
    }
    fn to_typestr_no_ref(&self) -> String {
        match self {
            ContainerType::Vector(x) => {
                format!("Vec<{}>", x.to_typestr_no_ref())
            },
            ContainerType::Array(x, y) => {
                format!("[{}, {}]", x.to_typestr_no_ref(), y)
            },
            ContainerType::Tuple(vx) => {
                let mut tuplestr = String::from("(");
                for x in vx {
                    tuplestr = tuplestr + &x.to_typestr_no_ref() + ", ";
                }
                tuplestr.pop(); 
                tuplestr.pop();
                tuplestr.push(')');
                tuplestr
            }
        }        
    }
    fn to_typestr_no_life(&self) -> String {
        match self {
            ContainerType::Vector(x) => {
                format!("Vec<{}>", x.to_typestr_no_life())
            },
            ContainerType::Array(x, y) => {
                format!("[{}, {}]", x.to_typestr_no_life(), y)
            },
            ContainerType::Tuple(vx) => {
                let mut tuplestr = String::from("(");
                for x in vx {
                    tuplestr = tuplestr + &x.to_typestr_no_life() + ", ";
                }
                tuplestr.pop(); 
                tuplestr.pop();
                tuplestr.push(')');
                tuplestr
            }
        } 
    }
    fn collect_lifetimes(&self, into: &mut Vec<String>) {
        match self {
            ContainerType::Vector(x) => {
                x.collect_lifetimes(into);
            },
            ContainerType::Array(x, y) => {
                x.collect_lifetimes(into);
            },
            ContainerType::Tuple(vx) => {
                for x in vx {
                    x.collect_lifetimes(into);
                }
            }
        }         
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
        let expected = ContainerType::Vector(Box::new(RType::String(Reference::Naked,StringType::str)));
        assert_eq!(result, expected);

        let input = "[usize, 3]";
        let result = ContainerType::from_typestr(input).unwrap();
        let expected = ContainerType::Array(
            Box::new(RType::Numeric(Reference::None,NumericType::usize)), 3
        );
        assert_eq!(result, expected);

        let input = "(usize,usize,enum:MyEnum)";
        let result = ContainerType::from_typestr(input).unwrap();
        let expected = ContainerType::Tuple( vec![
            Box::new(RType::Numeric(Reference::None,NumericType::usize)), 
            Box::new(RType::Numeric(Reference::None,NumericType::usize)), 
            Box::new(RType::Special(Reference::None,SpecialType::Enum("MyEnum".to_string())))
        ]);
        assert_eq!(result, expected);
            
        let input = "(Vec<&str>, usize, enum:MyEnum)";
        let result = ContainerType::from_typestr(input).unwrap().to_typestr();
        let expected = "(Vec<&str>, usize, MyEnum)";
        assert_eq!(result, expected);

        let input = "(Vec<&'a str>)";
        let result = ContainerType::from_typestr(input).unwrap().to_typestr();
        let expected = "(Vec<&'a str>)";
        assert_eq!(result, expected);

    }


}