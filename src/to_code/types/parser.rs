//use crate::regex::Regex;

//use std::ffi::CStr;
use std::sync::OnceLock;
use regex::Regex;
use super::{RType, TypeError};

//const MY_C_STRING: &'static CStr = CStr::from_bytes_with_nul(b"Hello, world!\0").expect("Failed to create CStr");
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
const ENUM_REGEX_STR: &'static str = r"enum: ?([a-zA-z_]+[0-9a-zA-z_]*)"; 
static ENUM_REGEX: OnceLock<Regex> = OnceLock::new();


/// Any string type will convert to &str .. deal with conversion yourself if you need it... but i could convert in the as_property method... and also in the from_property method. it would just declare the const as &'static str
const STDSTR_REGEX_STR: &'static str = r"(&?[OsC]{0,2}[sS]tri?n?g?)"; 
static STDSTR_REGEX: OnceLock<Regex> = OnceLock::new();

/// May evaluate to fsize
const NUMERIC_REGEX_STR: &'static str = r"(\b[uif](?:size|[0-9]{0,3})+\b)";
static NUMERIC_REGEX: OnceLock<Regex> = OnceLock::new();

/// We need to parse the type, if its a reference, and if it has a lifetime
pub struct TypeParser {

}

pub fn parse_rtype(typestr: &str) -> Result<(), TypeError>{
    // first we need to look if one of our regex matches

    // 1. construct regex
    let string_re = STDSTR_REGEX.get_or_init(|| Regex::new(STDSTR_REGEX_STR).unwrap());
    let enum_re = ENUM_REGEX.get_or_init(|| Regex::new(ENUM_REGEX_STR).unwrap());
    let numeric_re = NUMERIC_REGEX.get_or_init(|| Regex::new(NUMERIC_REGEX_STR).unwrap());

    // 2. start


    // find if reference or lifetime beofre doing anything else no each type will have to decide for itself...




    Ok(())
}
