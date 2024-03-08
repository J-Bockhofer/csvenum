
use super::*;

use super::containers::{VECTOR_REGEX, VECTOR_REGEX_STR};


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
const ENUM_REGEX_STR: &'static str = r"^enum: ?([a-zA-z_]+[0-9a-zA-z_]*)"; 
static ENUM_REGEX: OnceLock<Regex> = OnceLock::new();


/// Any string type will convert to &str .. deal with conversion yourself if you need it... but i could convert in the as_property method... and also in the from_property method. it would just declare the const as &'static str
const STDSTR_REGEX_STR: &'static str = r"(^&?^[OsC]{0,2}[sS]tri?n?g?)"; 
static STDSTR_REGEX: OnceLock<Regex> = OnceLock::new();

/// May evaluate to fsize
const NUMERIC_REGEX_STR: &'static str = r"(\b^[uif](?:size|[0-9]{0,3})+\b)";
static NUMERIC_REGEX: OnceLock<Regex> = OnceLock::new();


/// Okay this is disgusting but since all of our 

/// Rust Type primitive (no value or name attached), has functions for:
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
    Numeric(NumericType),
    /// Any String Type
    String(StringType),
    /// Any Container Type, except for Option and Result
    Container(ContainerType), //Box<RType> Type of Container with Box<RType>
    /// Special Types that need special treatment, very special incl:
    /// 
    /// Regex
    /// 
    /// Enum ( tuple of enum?? )
    Special(SpecialType),
}

impl RTypeString for RType {
    fn from_typestr<T: AsRef<str>>(typestr: T) -> Result<Self, TypeError> where Self: Sized {
        let typestr = typestr.as_ref();
    // first we need to look if one of our regex matches

    // strip any references
    let typestr = typestr.replace("&", "").replace(" ", "");

    // 1. construct regex
    let string_re = STDSTR_REGEX.get_or_init(|| Regex::new(STDSTR_REGEX_STR).unwrap());
    let enum_re = ENUM_REGEX.get_or_init(|| Regex::new(ENUM_REGEX_STR).unwrap());
    let numeric_re = NUMERIC_REGEX.get_or_init(|| Regex::new(NUMERIC_REGEX_STR).unwrap());

    //let vec_re = VECTOR_REGEX.get_or_init(|| Regex::new(VECTOR_REGEX_STR).unwrap());

    // 2. start matching
    // enum first
    if let Some(captures) = enum_re.captures(&typestr) {
        let name = captures.get(1).unwrap().as_str().to_string();
        return Ok(RType::Special(SpecialType::Enum(name)));
    } 
    // then regex not impl yet

    // then string
    if let Some(captures) = string_re.captures(&typestr) { 
        let typestr = captures.get(1).unwrap().as_str();
        return Ok(RType::String(StringType::from_typestr(typestr)?));
    }

    // then numeric
    if let Some(captures) = numeric_re.captures(&typestr) { 
        let typestr = captures.get(1).unwrap().as_str();
        return Ok(RType::Numeric(NumericType::from_typestr(typestr)?));
    }

    //ContainerType::from_typestr(&typestr)?;
    // then container
    if let Ok(container) = ContainerType::from_typestr(&typestr) {
        return Ok(RType::Container(container));
    }


    // find if reference or lifetime beofre doing anything else no each type will have to decide for itself... unsolved f it


    Err(TypeError::RTypeUnknown(typestr))




    }
}

pub trait RTypeString {
    fn from_typestr<T: AsRef<str>>(typestr: T) -> Result<Self, TypeError> where Self: Sized;    
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_rtype() {
        let typestr = "&str".to_string();
        let result = RType::from_typestr(&typestr).unwrap();
        let expected = RType::String(StringType::str);
        assert_eq!(result, expected);

        let typestr = "enum: MyEnum".to_string();
        let result = RType::from_typestr(&typestr).unwrap();
        let expected = RType::Special(SpecialType::Enum("MyEnum".to_string()));
        assert_eq!(result, expected);

        let typestr = "f32".to_string();
        let result = RType::from_typestr(&typestr).unwrap();
        let expected = RType::Numeric(NumericType::f32);
        assert_eq!(result, expected);

        let typestr = "Vec<u8>".to_string();
        let result = RType::from_typestr(&typestr).unwrap();
        let expected = RType::Container(ContainerType::Vector(Box::new(RType::Numeric(NumericType::u8))));
        assert_eq!(result, expected);

        let typestr = "[usize, 3]".to_string();
        let result = RType::from_typestr(&typestr).unwrap();
        let expected = RType::Container(ContainerType::Array(Box::new(RType::Numeric(NumericType::usize)), 3));
        assert_eq!(result, expected);

        let typestr = "Vec<[usize, 3]>".to_string();
        let result = RType::from_typestr(&typestr).unwrap();
        let expected = RType::Container(ContainerType::Vector(Box::new(RType::Container(
            ContainerType::Array(Box::new(RType::Numeric(NumericType::usize)), 3)        )))
            );
        assert_eq!(result, expected);

        let typestr = "(usize,f32, &str)".to_string();
        let result = RType::from_typestr(&typestr).unwrap();
        let expected = RType::Container(ContainerType::Tuple(vec![
            Box::new(RType::Numeric(NumericType::usize)),
            Box::new(RType::Numeric(NumericType::f32)),
            Box::new(RType::String(StringType::str)),
        ]));
        assert_eq!(result, expected);

    }

}