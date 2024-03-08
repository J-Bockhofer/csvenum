///! Convert Types passed as a string to more useful representations for code gen.
///! Allows for specifying "&str" in a textfile to generate associated representations in function signatures and expressions. 
///!
pub mod numeric;
pub use numeric::NumericType;

pub mod stringtype;
pub use stringtype::StringType;

pub mod containers;
pub use containers::ContainerType;

pub mod special;
pub use special::SpecialType;

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



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_typestr() {


    }
}