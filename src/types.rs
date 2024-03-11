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

mod rtypetrait;
pub use rtypetrait::RTypeTrait;


use thiserror::Error as Error;


use std::{sync::OnceLock, fmt::Debug};
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
    #[error("Found reference but no type -> {0} ")]
    EmptyTypeWithReference(String),
    #[error("CALLED SPECIAL.from_typestr with is invalid. Should not parse itself.")]
    SPECIALTYPEPARSE,
    #[error("Invalid variant name -> {0} ")]
    EnumValueError(String),


}


/// Here come some regex:
/// 
/// https://regex101.com/
/// 
/// 
/// An enum type needs to be specified as:
/// 
/// `enum: MyEnum`
/// 
/// This regex will capture the name `MyEnum`.
const ENUM_REGEX_STR: &'static str = r"^enum: ?([a-zA-z]+[0-9a-zA-z_]*)"; 
static ENUM_REGEX: OnceLock<Regex> = OnceLock::new();


/// Any string type will convert to &str .. deal with conversion yourself if you need it... but i could convert in the as_property method... and also in the from_property method. it would just declare the const as &'static str
const STDSTR_REGEX_STR: &'static str = r"(^&?^[OsC]{0,2}[sS]tri?n?g?)"; 
static STDSTR_REGEX: OnceLock<Regex> = OnceLock::new();

/// May evaluate to fsize, will throw error of NumericTypeUnknown
const NUMERIC_REGEX_STR: &'static str = r"(\b^[uif](?:size|[0-9]{0,3})+\b)";
static NUMERIC_REGEX: OnceLock<Regex> = OnceLock::new();

/// Matches if something has a reference an will capture the lifetime, which may or may not be there in group 1 and the contained type in group 2
const REFLIFETIME_REGEX_STR: &'static str = r"^ ?& ?([\w']{0,} )?([\w\d_:<>\[\], ]*)";
static REFLIFETIME_REGEX: OnceLock<Regex> = OnceLock::new();

/// Rust Type primitive (no value or name attached, except for enum), has functions for:
/// 
/// - constructing itself from a given string
/// 
/// - format to different representations type in fn arg, return value, const/static, 
/// 
/// Excludes:
/// Result
/// Option
/// Error
#[derive(Debug, Eq, PartialEq)]
pub enum RType {
    /// Any Numeric Type
    Numeric(Reference, NumericType),
    /// Any String Type
    String(Reference, StringType),
    /// Any Container Type, except for Option and Result
    Container(Reference, ContainerType), //Box<RType> Type of Container with Box<RType>
    /// Special Types that need special treatment, very special incl:
    /// 
    /// Regex
    /// 
    /// Enum 
    /// 
    /// Bool (not that special actually but easy to find)
    /// 
    Special(Reference, SpecialType),
}

impl RType {
    pub fn get_refstr(&self) -> String {
        match self {
            RType::Special(x, _) => {x.to_refstr()},
            RType::String(x, _) => {x.to_refstr()},
            RType::Numeric(x, _) => {x.to_refstr()},
            RType::Container(x, _) => {x.to_refstr()},
        }
    }
    pub fn get_refstr_no_life(&self) -> String {
        match self {
            RType::Special(x, _) => {x.to_refstr_no_life()},
            RType::String(x, _) => {x.to_refstr_no_life()},
            RType::Numeric(x, _) => {x.to_refstr_no_life()},
            RType::Container(x, _) => {x.to_refstr_no_life()},
        }        
    }
    pub fn get_lifetime(&self) -> String {
        match self {
            RType::Special(x, _) => {x.get_lifetime()},
            RType::String(x, _) => {x.get_lifetime()},
            RType::Numeric(x, _) => {x.get_lifetime()},
            RType::Container(x, _) => {x.get_lifetime()},            
        }
    }
    pub fn has_lifetime(&self) -> bool {
        match self {
            RType::Special(x, _) => {x.has_lifetime()},
            RType::String(x, _) => {x.has_lifetime()},
            RType::Numeric(x, _) => {x.has_lifetime()},
            RType::Container(x, _) => {x.has_lifetime()},            
        }        
    }
}

impl RTypeTrait for RType {
    fn from_typestr<T: AsRef<str>>(typestr: T) -> Result<Self, TypeError> where Self: Sized {
        let mut typestr = typestr.as_ref(); // may be changed when a lifetime is obtained.
        // first we need to look if one of our regex matches

        // strip any references - check if there is one at position 1, + a lifetime
        // let typestr = typestr.replace("&", "").replace(" ", "");

        // 1. construct regex
        let reflt_re = REFLIFETIME_REGEX.get_or_init(|| Regex::new(REFLIFETIME_REGEX_STR).unwrap());
        let string_re = STDSTR_REGEX.get_or_init(|| Regex::new(STDSTR_REGEX_STR).unwrap());
        let enum_re = ENUM_REGEX.get_or_init(|| Regex::new(ENUM_REGEX_STR).unwrap());
        let numeric_re = NUMERIC_REGEX.get_or_init(|| Regex::new(NUMERIC_REGEX_STR).unwrap());

        let mut reference: Reference = Reference::None;//Reference::empty(); 

        if let Some(captures) = reflt_re.captures(&typestr) { 
            //type_modifier.is_reference = true;
            reference = Reference::Naked;
            if let Some(lifetime) = captures.get(1) {
                reference = Reference::WithLifetime(lifetime.as_str().trim().to_string());
            }
            if let Some(inner_type) = captures.get(2) {
                typestr = inner_type.as_str();
            } else {
                return Err(TypeError::EmptyTypeWithReference(typestr.to_string()));
            }
        }

        // enum 
        if let Some(captures) = enum_re.captures(&typestr) {
            let name = captures.get(1).unwrap().as_str().to_string();
            return Ok(RType::Special(reference, SpecialType::Enum(name)));
        } 
        // then regex not impl yet

        // then string - could just be is match
        if let Some(captures) = string_re.captures(&typestr) { 
            let typestr = captures.get(1).unwrap().as_str();
            return Ok(RType::String(reference, StringType::from_typestr(typestr)?));
        }
        // then numeric - could just be is match
        if let Some(captures) = numeric_re.captures(&typestr) { 
            let typestr = captures.get(1).unwrap().as_str();
            return Ok(RType::Numeric(reference, NumericType::from_typestr(typestr)?));
        }
        // then container
        if let Ok(container) = ContainerType::from_typestr(&typestr) {
            return Ok(RType::Container(reference, container));
        }
        // find if reference or lifetime before doing anything else.. no each type will have to decide for itself... unsolved f it
        Err(TypeError::RTypeUnknown(typestr.to_string()))
    }

/*     fn from_valuestr<T: AsRef<str>>(valuestr: T) -> Result<Self, TypeError> where Self: Sized {
        let mut valuestr = valuestr.as_ref();
        
        if let Some(specialtype) = SpecialType::from_valuestr(valuestr) {return Self::Special(Reference::None, specialtype)}
    } */

    fn to_typestr(&self) -> String{
        let lifetime_pd = if self.has_lifetime() {" "} else {""}; 

        self.get_refstr() + lifetime_pd + &match self {
            RType::Special(_, x) => {x.to_typestr()},
            RType::Numeric(_, x) => {x.to_typestr()},
            RType::String(_, x) => {x.to_typestr()},
            RType::Container(_, x) => {x.to_typestr()},
        }
    }
    fn to_typestr_no_ref(&self) -> String {
        match self {
            RType::Special(_, x) => {x.to_typestr_no_ref()},
            RType::Numeric(_, x) => {x.to_typestr_no_ref()},
            RType::String(_, x) => {x.to_typestr_no_ref()},
            RType::Container(_, x) => {x.to_typestr_no_ref()},
        }        
    }
    fn to_typestr_no_life(&self) -> String {
        self.get_refstr_no_life() + &match self {
            RType::Special(_, x) => {x.to_typestr_no_life()},
            RType::Numeric(_, x) => {x.to_typestr_no_life()},
            RType::String(_, x) => {x.to_typestr_no_life()},
            RType::Container(_, x) => {x.to_typestr_no_life()},
        }
    }
    fn collect_lifetimes(&self, into: &mut Vec<String>) {
        into.push(self.get_lifetime());
        match self {
            RType::Special(_, x) => {x.collect_lifetimes(into)},
            RType::Numeric(_, x) => {x.collect_lifetimes(into)},
            RType::String(_, x) => {x.collect_lifetimes(into)},
            RType::Container(_, x) => {x.collect_lifetimes(into)},
        }
    }
    /// Container of type tuple will not be const when it contains non-const types
    fn is_const(&self) -> bool {
        match self {
            RType::Special(_, x) => {x.is_const()},
            RType::Numeric(_, x) => {x.is_const()},
            RType::String(_, x) => {x.is_const()},
            RType::Container(_, x) => {x.is_const()},
        }        
    }
    fn value_is_valid(&self, valuestr: &str) -> bool {
        match self {
            RType::Special(_, x) => {x.value_is_valid(valuestr)},
            RType::Numeric(_, x) => {x.value_is_valid(valuestr)},
            RType::String(_, x) => {x.value_is_valid(valuestr)},
            RType::Container(_, x) => {x.value_is_valid(valuestr)},
        }        
    }
    fn get_depth(&self, counter: usize) -> usize {
        match self {
            RType::Special(_, x) => {x.get_depth(counter)},
            RType::Numeric(_, x) => {x.get_depth(counter)},
            RType::String(_, x) => {x.get_depth(counter)},
            RType::Container(_, x) => {x.get_depth(counter)},
        }           
    }
    fn get_breadth(&self, counter: usize) -> usize {
        match self {
            RType::Special(_, x) => {x.get_breadth(counter)},
            RType::Numeric(_, x) => {x.get_breadth(counter)},
            RType::String(_, x) => {x.get_breadth(counter)},
            RType::Container(_, x) => {x.get_breadth(counter)},
        }          
    }

    fn wrap_valuestr(&self, valuestr: &str) -> String {
        match self {
            Self::String(_, x) => {x.wrap_valuestr(valuestr)},
            Self::Special(_, x) => {x.wrap_valuestr(valuestr)},
            Self::Numeric(_, x) => {x.wrap_valuestr(valuestr)},
            Self::Container(_, x) => {x.wrap_valuestr(valuestr)},
        }
    }

}


impl std::fmt::Display for RType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_typestr())?;
        Ok(())
    }    
}


/// Represents a reference and lifetime
/// Will be wrapped in an option (reference is optional in types), no!
#[derive(Debug, Eq, PartialEq)]
pub enum Reference {
    WithLifetime(String),
    Naked,
    None,
}

impl std::fmt::Display for Reference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let refstr = match self {
            Reference::None => {"".to_string()},
            Reference::Naked => {"&".to_string()},
            Reference::WithLifetime(x) => {format!("&{}",x)},
        };
        write!(f, "{}", refstr)
    }
}

impl Reference {
    pub fn to_refstr(&self) -> String {
        self.to_string()
    }
    pub fn get_lifetime(&self) -> String {
        match self {
            Self::WithLifetime(x) => {x.to_owned()}
            _ => {String::new()},
        }
    }
    pub fn has_lifetime(&self) -> bool {
        match self {
            Self::WithLifetime(_) => {true}
            _ => {false},
        }        
    }
    pub fn to_refstr_no_life(&self) -> String {
        match self {
            Self::None => {String::new()},
            _ => {String::from("&")},

        }
    }

}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_rtype() {
        let typestr = "&str".to_string();
        let result = RType::from_typestr(&typestr).unwrap();
        let expected = RType::String(Reference::Naked,StringType::str);
        assert_eq!(expected, result);

        let typestr = "enum: MyEnum".to_string();
        let result = RType::from_typestr(&typestr).unwrap();
        let expected = RType::Special(Reference::None, SpecialType::Enum("MyEnum".to_string()));
        assert_eq!(expected, result);

        let typestr = "f32".to_string();
        let result = RType::from_typestr(&typestr).unwrap();
        let expected = RType::Numeric(Reference::None,NumericType::f32);
        assert_eq!(expected, result);

        let typestr = "Vec<u8>".to_string();
        let result = RType::from_typestr(&typestr).unwrap();
        let expected = RType::Container(Reference::None, ContainerType::Vector(Box::new(RType::Numeric(Reference::None, NumericType::u8))));
        assert_eq!(expected, result);

        let typestr = "[usize, 3]".to_string();
        let result = RType::from_typestr(&typestr).unwrap();
        let expected = RType::Container(Reference::None, ContainerType::Array(Box::new(RType::Numeric(Reference::None, NumericType::usize)), 3));
        assert_eq!(expected, result);

        let typestr = "Vec<[usize, 3]>".to_string();
        let result = RType::from_typestr(&typestr).unwrap();
        let expected = RType::Container(Reference::None, ContainerType::Vector(
            Box::new(RType::Container(Reference::None, ContainerType::Array(
                Box::new(RType::Numeric(Reference::None, NumericType::usize)), 3)        )))
            );
        assert_eq!(expected, result);

        let typestr = "(usize,f32, &str)".to_string();
        let result = RType::from_typestr(&typestr).unwrap();
        let expected = RType::Container(Reference::None,ContainerType::Tuple(vec![
            Box::new(RType::Numeric(Reference::None,NumericType::usize)),
            Box::new(RType::Numeric(Reference::None,NumericType::f32)),
            Box::new(RType::String(Reference::Naked,StringType::str)),
        ]));
        assert_eq!(expected, result);

        let typestr = "Vec<Vec<(usize, CStr)>>".to_string();
        let result = RType::from_typestr(&typestr).unwrap();
        let expected = RType::Container(Reference::None,ContainerType::Vector(Box::new(
            RType::Container(Reference::None,ContainerType::Vector(Box::new(
                RType::Container(Reference::None,ContainerType::Tuple(vec![
                    Box::new(RType::Numeric(Reference::None,NumericType::usize)),
                    Box::new(RType::String(Reference::None,StringType::CStr)),
                ]))
            )))
        )));
        assert_eq!(expected, result);

        let typestr = "Vec<&'a str>".to_string();
        let result = RType::from_typestr(&typestr).unwrap();
        let expected = RType::Container(Reference::None, ContainerType::Vector(Box::new(RType::String(Reference::WithLifetime("'a".to_string()), StringType::str))));
        assert_eq!(expected, result);

        let typestr = "Vec<&'a str>".to_string();
        let result = RType::from_typestr(&typestr).unwrap();
        let expected = result.to_typestr();
        assert_eq!(expected, typestr);

        let typestr = "(Vec<&str>, usize)".to_string();
        let result = RType::from_typestr(&typestr).unwrap();
        let expected = result.to_typestr();
        assert_eq!(expected, typestr);        

        let typestr = "(Vec<Vec<&'a str>>, usize)".to_string();
        let result = RType::from_typestr(&typestr).unwrap();
        let expected = result.to_typestr();
        assert_eq!(expected, typestr);
        assert_eq!(3, result.get_depth(0));

        let typestr = "(usize, (usize, &str), f32)".to_string();
        let result = RType::from_typestr(&typestr).unwrap();
        let expected = result.to_typestr();
        assert_eq!(expected, typestr);
        assert_eq!(4, result.get_breadth(0));


/*         let typestr = "(Vec<&'a str>, &'b OsStr, [&'a usize, 3])".to_string();
        let result = RType::from_typestr(&typestr).unwrap();
        let expected = result.to_typestr();
        assert_eq!(expected, typestr);
        let mut into: Vec<String> = vec![];
        result.collect_lifetimes(&mut into);
        let expected = vec!["'a","'b","'a"];
        assert_eq!(into, expected); */

    }

}

