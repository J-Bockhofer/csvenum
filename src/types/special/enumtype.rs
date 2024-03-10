use std::sync::OnceLock;
use regex::Regex;

use crate::{SpecialType};

const VARIANT_REGEX_STR: &'static str = r" ?([a-zA-z]+[0-9a-zA-z_]*)"; 
static VARIANT_REGEX: OnceLock<Regex> = OnceLock::new();

const ENUMNAME_REGEX_STR: &'static str = r"^ ?([a-zA-z]+[0-9a-zA-z_]*)"; 
static ENUMNAME_REGEX: OnceLock<Regex> = OnceLock::new();

pub struct EnumType {

}
impl EnumType {
    pub fn enum_strip_name_from_variant_valuestr(enumname: &str, valuestr: &str) -> String {
        if valuestr.contains(enumname) {
            let mut _str = valuestr.replace(enumname, "");
            if _str.len() > 2 {
                _str.remove(0);
                _str.remove(0);
            } else {
                return String::new();
            }
            return _str;
        } else {
            return valuestr.to_string();
        }
    }

    pub fn value_is_valid_enum(enumname: &str, valuestr: &str) -> bool {
        let var_re = VARIANT_REGEX.get_or_init(|| Regex::new(VARIANT_REGEX_STR).unwrap());
        let valuestr = EnumType::enum_strip_name_from_variant_valuestr(enumname, valuestr);
        // if the valuestr contains the enumname, cut it out can check if the first two chars left are "::" then take the actual variant
        if !valuestr.is_empty() {
            if var_re.is_match(&valuestr) {return true}
        }
        false
    }

    pub fn from_value(valuestr: &str) -> Option<SpecialType> {
        let enum_re = ENUMNAME_REGEX.get_or_init(|| Regex::new(ENUMNAME_REGEX_STR).unwrap());

         if let Some(caps) = enum_re.captures(&valuestr) {
            let name  = caps.get(1).unwrap().as_str();
            return Some(SpecialType::Enum(name.to_string()))
         }
    
        None
    }

}

