mod vectortype;
pub use vectortype::VectorType;

mod arraytype;
pub use arraytype::ArrayType;

mod tupletype;
pub use tupletype::TupleType;

mod utils;
pub use utils::NestedValueParser;

use super::{RType, RTypeTrait, TypeError};
use regex::Regex;
use std::sync::OnceLock;

/// Vector regex
pub const VECTOR_REGEX_STR: &'static str = r"^Vec<(.*)>"; // why is this so explicit about the char set? -> r"^Vec<([\w&_:<>\(\)\[\], ]*)>";
pub static VECTOR_REGEX: OnceLock<Regex> = OnceLock::new();

/// Array regex
pub const ARRAY_REGEX_STR: &'static str = r"^\[ ?(.+)(?:,|;) ?([\d]+) ?\]";
pub static ARRAY_REGEX: OnceLock<Regex> = OnceLock::new();

/// Tuple regex 
pub const TUPLE_REGEX_STR: &'static str = r"^\( ?(.+) ?\)";
pub static TUPLE_REGEX: OnceLock<Regex> = OnceLock::new();

/// Represents a container, like Vec<T>, Array \[T; N\] and Tuple(T,R)
/// 
/// Contains more [RType]s
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
            //println!("Vector inner typestr: {}", inner_type);
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
            let types_in_str: Vec<String> = NestedValueParser::parse_nested_str(inner_str, '(', false); //.split(',').collect();
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
                VectorType::wrap_typestr(&x.to_typestr())
            },
            ContainerType::Array(x, y) => {
                ArrayType::wrap_typestr(&x.to_typestr(), y)
            },
            ContainerType::Tuple(vx) => {
                TupleType::wrap_types_with_typefn(vx, |item: &RType| item.to_typestr())
            }
        }
    }
    fn to_typestr_no_ref(&self) -> String {
        match self {
            ContainerType::Vector(x) => {
                VectorType::wrap_typestr(&x.to_typestr_no_ref())
            },
            ContainerType::Array(x, y) => {
                ArrayType::wrap_typestr(&x.to_typestr_no_ref(), y)
            },
            ContainerType::Tuple(vx) => {
                TupleType::wrap_types_with_typefn(vx, |item: &RType| item.to_typestr_no_ref())
            }
        }        
    }
    fn to_typestr_no_life(&self) -> String {
        match self {
            ContainerType::Vector(x) => {
                VectorType::wrap_typestr(&x.to_typestr_no_life())
            },
            ContainerType::Array(x, y) => {
                ArrayType::wrap_typestr(&x.to_typestr_no_life(), y)
            },
            ContainerType::Tuple(vx) => {
                TupleType::wrap_types_with_typefn(vx, |item: &RType| item.to_typestr_no_life())
            }
        } 
    }
    fn collect_lifetimes(&self, into: &mut Vec<String>) {
        match self {
            ContainerType::Vector(x) => {
                x.collect_lifetimes(into);
            },
            ContainerType::Array(x, _) => {
                x.collect_lifetimes(into);
            },
            ContainerType::Tuple(vx) => {
                for x in vx {
                    x.collect_lifetimes(into);
                }
            }
        }         
    }
    fn is_const(&self) -> bool {
        match self {
            ContainerType::Vector(_) => {
                false
            },
            ContainerType::Array(x, _) => {
                x.is_const()
            },
            ContainerType::Tuple(vx) => {
                for x in vx {
                    if !x.is_const() {
                        return false;
                    }
                }
                true
            }
        }         
    }

    fn value_is_valid(&self, valuestr: &str) -> bool {
        // we have to split the valuestr bc it may contain multiple values. let each type handle that independantly
        match self {
            ContainerType::Array(x, y) => {ArrayType::value_is_valid(valuestr, x, y)},
            ContainerType::Vector(x) => {VectorType::value_is_valid(valuestr, x)},
            ContainerType::Tuple(vx) => {TupleType::value_is_valid(valuestr, vx)},
        }


    }
    fn get_depth(&self, counter: usize) -> usize {
        let counter = counter.saturating_add(1);
        match self {
            Self::Vector(x) => {x.get_depth(counter)},
            Self::Array(x, _) => {x.get_depth(counter)},
            Self::Tuple(vx) => {
                let mut inner_cnt = 0; 
                for x in vx {
                    // get the max depth
                    let cnt = x.get_depth(0);
                    if cnt > inner_cnt {inner_cnt = cnt}
                }
                counter.saturating_add(inner_cnt)
            }
        }

    }
    fn get_breadth(&self, counter: usize) -> usize {
        match self {
            Self::Array(x, _) => {x.get_breadth(counter)},
            Self::Vector(x) => {x.get_breadth(counter)},
            Self::Tuple(vx) => {
                let mut breadth: usize = counter ;//+ vx.len();
                for x in vx {
                    // get the full breadth
                    breadth = breadth.saturating_add(x.get_breadth(0));
                }                 
                breadth
            }
        }
    }

    fn wrap_valuestr(&self, valuestr: &str) -> String {
        match self {
            ContainerType::Array(x, _) => {ArrayType::wrap_valuestr(valuestr, x)},
            ContainerType::Vector(x) => {VectorType::wrap_valuestr(valuestr, x)},
            ContainerType::Tuple(vx) => {TupleType::wrap_valuestr(valuestr, vx)},
            //_ => {todo!("Implement value wrapping for Option")}
        }
    }

    fn can_match_as_key(&self) -> bool {
        match self {
            ContainerType::Array(x, _) => {x.can_match_as_key()},
            ContainerType::Vector(x) => {x.can_match_as_key()},
            ContainerType::Tuple(vx) => {
                for x in vx {
                    // get the full breadth
                    if !x.can_match_as_key() {
                        return false
                    }
                } 
                true
            },
            //_ => {todo!("Implement value wrapping for Option")}
        }        
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{NumericType, StringType, SpecialType, Reference};

    #[test]
    fn test_parse_container_type() { 
        let input = "Vec< &str>";
        let result = ContainerType::from_typestr(input).unwrap();
        let expected = ContainerType::Vector(Box::new(RType::String(Reference::Naked,StringType::str)));
        assert_eq!(expected, result);

        let input = "[usize, 3]";
        let result = ContainerType::from_typestr(input).unwrap();
        let expected = ContainerType::Array(
            Box::new(RType::Numeric(Reference::None,NumericType::usize)), 3
        );
        assert_eq!(expected, result);

        let input = "(usize,usize,enum:MyEnum)";
        let result = ContainerType::from_typestr(input).unwrap();
        let expected = ContainerType::Tuple( vec![
            Box::new(RType::Numeric(Reference::None,NumericType::usize)), 
            Box::new(RType::Numeric(Reference::None,NumericType::usize)), 
            Box::new(RType::Special(Reference::None,SpecialType::Enum("MyEnum".to_string())))
        ]);
        assert_eq!(expected, result);
        let expected = false;
        assert_eq!(expected, result.is_const());

        let input = "(Vec<&str>, usize, enum:MyEnum)";
        let result = ContainerType::from_typestr(input).unwrap().to_typestr();
        let expected = "(Vec<&str>, usize, MyEnum)";
        assert_eq!(expected, result);


        let input = "(Vec<&'a str>)";
        let result = ContainerType::from_typestr(input).unwrap().to_typestr();
        let expected = "(Vec<&'a str>)";
        assert_eq!(expected, result);

        let result = ContainerType::from_typestr(input).unwrap().to_typestr_no_ref();
        let expected = "(Vec<str>)";
        assert_eq!(expected, result);

        let input = "[usize, 3]";
        let result = ContainerType::from_typestr(input).unwrap();
        let values = "[3,3,3]";
        assert!(result.value_is_valid(values));
        let values = "[art,3.52,3]";
        assert_eq!(false, result.value_is_valid(values));

    }


}