///! Convert Types passed as a string to more useful representations for code gen.
///! Allows for specifying "&str" in a textfile to generate associated representations in function signatures and expressions. 
///!
pub mod parser;

pub mod numeric;
pub use numeric::NumericType;

pub mod stringtype;
pub use stringtype::StringType;

pub mod containers;
pub use containers::ContainerType;

pub mod special;
pub use special::SpecialType;

pub mod type_const;
use type_const::ConstType;

pub mod type_container;
use type_container::ContainerType as CoType;

pub mod rtype;
pub use rtype::{RType, RTypeString};

use thiserror::Error as Error;


use std::sync::OnceLock;
use regex::Regex;

#[derive(Error, Debug, PartialEq)]
pub enum TypeError {

    #[error("A string that should represent a valid type could not get parsed -> {0} ")]
    Unknown(String),
    #[error("A string that should represent a valid String type could not get parsed -> {0} ")]
    StringTypeUnknown(String),
    #[error("A string that should represent a valid Numeric type could not get parsed -> {0} ")]
    NumericTypeUnknown(String),
    #[error("A string that should represent a valid Container type could not get parsed -> {0} ")]
    ContainerTypeUnknown(String),
    #[error("A string that should represent a valid type could not get parsed -> {0} ")]
    RTypeUnknown(String),

    #[error("Parsed table needs to have at least 3 lines.")]
    ConstTError,
    #[error("Number of types:  and number of properties:  do not match!")]
    ContainerTError,
    #[error("Number of types: ")]
    TTypeError,
 

}

/// Only Modifier for parsed type not for types in code. - not used...
pub struct TypeModifier {
    pub is_reference: bool,
    pub lifetime: String,
}

impl TypeModifier {
    pub fn new() -> Self {
        TypeModifier { is_reference: false, lifetime: String::new() }
    }
    pub fn as_tuple(&self) -> (bool, &str) {
        return (self.is_reference, &self.lifetime)
    }
    pub fn is_empty(&self) -> bool {
        if !self.is_reference && self.lifetime.is_empty() {return true}
        false
    }
    pub fn with_reference() -> Self {
        TypeModifier { is_reference: true, lifetime: String::new() }
    }
    pub fn with_lifetime(lifetime: &str) -> Self {
        TypeModifier { is_reference: false, lifetime: lifetime.to_string() }
    }    
    pub fn with(is_reference: bool, lifetime: &str) -> Self {
        TypeModifier { is_reference, lifetime: lifetime.to_string() }
    }
}

/// We take in text like this: 
/// ```
///  use::crate::to_code::{TType, TypeToStr}
///  let text = vec!["&str","ABC","SomeText"];
///  // get the type
/// ``` 
/// Every type (here: &str) will have a representation in:
/// 
/// 1. Declarations (const, let) 
///     - const ABC: `&'static str` = "SomeText";
///     - let ABC: &str = "SomeText";
/// 2. Return Type
///     -> &str
/// 3. Function Argument
///     argument: &str
/// 
pub trait TypeToStr {
    fn to_return_typestr(&self) -> String;
    fn to_const_typestr(&self) -> String;
    fn to_let_typestr(&self) -> String;
    fn to_fn_arg(&self) -> String;
    fn get_value_wrapper(&self) -> TypeWrapper; // questionable..
    fn from_typestr<T: AsRef<str>>(typestr: T) -> Result<Self, TypeError> where Self: Sized;
    //fn declare_value<T: AsRef<str>>(&self, value: T) -> String;

}




/// Helper for wrapping values in the given type
#[derive(Debug, Clone)]
pub struct TypeWrapper {
    pub left_hand: String,
    pub right_hand: String,
    is_empty: bool,
}
impl TypeWrapper {
    pub fn with(left_hand: &str, right_hand: &str) -> Self {
        let is_empty = if left_hand.is_empty() && right_hand.is_empty() {true} else {false};
        TypeWrapper { left_hand: left_hand.to_string(), right_hand: right_hand.to_string(), is_empty}
    }
    /// &str -> "value"
    /// usize -> value - no wrapper, bc the value argument to this function is always a &str
    /// tuple -> (value) 
    pub fn wrap_value<T: AsRef<str>>(&self, value: T) -> String 
    {   
        if self.is_empty {return value.as_ref().to_string()}
        format!("{}{}{}", self.left_hand, value.as_ref(), self.right_hand)
    }
    /// collects into single line string, only useful if the containing type is a vector .. implicit... maybe tie the typewrapper to another enum?!
    pub fn wrap_values_vec<T: AsRef<str>>(&self, values: &Vec<T>, value_type_wrapper: TypeWrapper) -> String 
    {
        let mut formatted_values = String::new();
        for value in values {
            let valstr = format!("{},", value_type_wrapper.wrap_value(value.as_ref()));
            formatted_values.push_str(&valstr);
            
        }
        formatted_values.remove(formatted_values.len()-1); // removes the last ,
        return self.wrap_value(formatted_values)
    }
/*     pub fn wrap_value_tuple<T: AsRef<str>>(&self, tuples: &Vec<T>, value_type_wrappers: Vec<TypeWrapper>) -> String 
    {
        /// the value strings will be separated by commas
        let mut formatted_values = String::new();
        for tuple in tuples {
            let valstr = format!(" {}, ", value_type_wrapper.wrap_value(value.as_ref()));
            formatted_values.push_str(&valstr);
        }
        return self.wrap_value(formatted_values)
    } */


}

#[derive(Debug, PartialEq, Eq)]
pub enum TType {
    ConstType(ConstType),
    ContainerType(CoType),
    ///
    /// Regex: Regex'
    /// 
    /// Fn return: &Regex
    /// 
    /// Fn arg: &Regex
    /// 
    /// Const expr: OnceLock<Regex>
    /// 
    /// Let expr: input
    /// 
    /// Declare with value
    Regex,
}

pub fn sanitize_typestr<T: AsRef<str>>(typestr: T) -> String {
    let typestr = typestr.as_ref();
    if typestr.len() > 128 {panic!("I think your typestr is too long: {}", typestr)}
    let mut parsed_type = String::new();

    for char in typestr.chars() {
        match char {
            ' ' => {continue},
            'V' => parsed_type.push('V'),
            '&' => parsed_type.push('&'),
            'e' => parsed_type.push('e'),
            'c' => parsed_type.push('c'),
            '<' => parsed_type.push('<'),
            '>' => parsed_type.push('>'),
            '(' => parsed_type.push('('),
            ')' => parsed_type.push(')'),
            ',' => parsed_type.push(','),
            '1' => parsed_type.push('1'),  // 16
            '2' => parsed_type.push('2'),  // 32    
            '3' => parsed_type.push('3'),  // 32
            '4' => parsed_type.push('4'),  // 64      
            '5' => parsed_type.push('5'),  // 256
            '6' => parsed_type.push('6'),  // 16, 64
            '7' => parsed_type.push('7'),         
            '8' => parsed_type.push('8'),  // 128
            '9' => parsed_type.push('9'),      
            'i' => parsed_type.push('i'),  
            'f' => parsed_type.push('f'),              
            'u' => parsed_type.push('u'),     
            's' => parsed_type.push('s'),  
            'z' => parsed_type.push('z'),  
            'r' => parsed_type.push('r'),    
            't' => parsed_type.push('t'),                                                                                                                                                                                     
            _ => {continue}
        }
    }
    parsed_type
}

impl TypeToStr for TType {
    fn from_typestr<T: AsRef<str>>(typestr: T) -> Result<Self, TypeError> where Self: Sized {
        let typestr = typestr.as_ref();

        let parsed_type = sanitize_typestr(typestr);

        //println!("TType - typestr sanitized: {}", &parsed_type);

        let _container = CoType::from_typestr(&parsed_type);
        if _container.is_ok() {
            return Ok(Self::ContainerType(_container.unwrap()));
        } 

        let _const = ConstType::from_typestr(parsed_type);
        if _const.is_ok() {
            return Ok(Self::ConstType(_const.unwrap()));
        }
        Err(TypeError::Unknown(typestr.to_string()))     
    }
    fn get_value_wrapper(&self) -> TypeWrapper {
        match self {
            Self::ConstType(x) => {x.get_value_wrapper()},
            Self::ContainerType(x) => {x.get_value_wrapper()},
            Self::Regex => {todo!("Implement Regex")},
        }
    }
    fn to_const_typestr(&self) -> String {
        match self {
            Self::ConstType(x) => {x.to_const_typestr()},
            Self::ContainerType(x) => {x.to_const_typestr()},
            Self::Regex => {todo!("Implement Regex")},
        }
    }
    fn to_let_typestr(&self) -> String {
        match self {
            Self::ConstType(x) => {x.to_let_typestr()},
            Self::ContainerType(x) => {x.to_let_typestr()},
            Self::Regex => {todo!("Implement Regex")},
        }         
    }
    fn to_fn_arg(&self) -> String {
        match self {
            Self::ConstType(x) => {x.to_fn_arg()},
            Self::ContainerType(x) => {x.to_fn_arg()},
            Self::Regex => {todo!("Implement Regex")},
        }        
    }
    fn to_return_typestr(&self) -> String {
        match self {
            Self::ConstType(x) => {x.to_return_typestr()},
            Self::ContainerType(x) => {x.to_return_typestr()},
            Self::Regex => {todo!("Implement Regex")},
        }             
    }


}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_typestr() {
        let typestr = " Vec< &str>".to_string();
        let ttype = TType::from_typestr(typestr).unwrap();
        let expected = TType::ContainerType(CoType::Vector(ConstType::Char));
        assert_eq!(ttype, expected);

        let typestr = " (usize, f32, f64, u8 ) ".to_string();
        let ttype = TType::from_typestr(typestr).unwrap();
        let expected = TType::ContainerType(CoType::Tuple(
            vec![
                ConstType::Numeric(format!("usize")),
                ConstType::Numeric(format!("f32")),
                ConstType::Numeric(format!("f64")),
                ConstType::Numeric(format!("u8")),
            ]
        ));
        assert_eq!(ttype, expected);

        let typestr = " Vec< &str> ".to_string();
        let ttype = TType::from_typestr(typestr).unwrap().to_const_typestr();
        let expected = "OnceLock<Vec<&str>>".to_string();
        assert_eq!(ttype, expected);

    }
}