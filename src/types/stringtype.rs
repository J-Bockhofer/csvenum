
use super::{TypeError, RTypeTrait};

/// Represents known string types
#[allow(non_camel_case_types)]
#[derive(Debug, Eq, PartialEq)]
pub enum StringType {
    /// str
    /// 
    /// Property `typestr` with value `str` 
    /// 
    str,
    /// String
    /// 
    /// Property `typestr` with value `String` 
    String,
    /// OsString
    /// 
    /// Property `typestr` with value `OsString` 
    /// 
    OsString,
    /// OsStr
    /// 
    /// Property `typestr` with value `OsStr` 
    /// 
    OsStr,
    /// CString
    /// 
    /// Property `typestr` with value `CString` 
    /// 
    CString,
    /// CStr
    /// 
    /// Property `typestr` with value `CStr` 
    /// 
    CStr,
    /// char
    /// 
    /// Property `typestr` with value `char` 
    /// 
    char,
}
impl StringType {
    pub fn get_all_variants() -> Vec<StringType> {
        vec![
            StringType::str,
            StringType::String,
            StringType::OsString,
            StringType::OsStr,
            StringType::CString,
            StringType::CStr,
            StringType::char,
        ]
    }
}
/// Consts for Property `typestr` 
const STR_TYPESTR: &'static str = "str";
const STRING_TYPESTR: &'static str = "String";
const OSSTRING_TYPESTR: &'static str = "OsString";
const OSSTR_TYPESTR: &'static str = "OsStr";
const CSTRING_TYPESTR: &'static str = "CString";
const CSTR_TYPESTR: &'static str = "CStr";
const CHAR_TYPESTR: &'static str = "char";

impl RTypeTrait for StringType {
    fn from_typestr<T: AsRef<str>>(typestr: T) -> Result<Self, super::TypeError> where Self: Sized {
        let typestr = typestr.as_ref();
        match typestr {
            STR_TYPESTR => Ok(StringType::str),
            STRING_TYPESTR => Ok(StringType::String),
            OSSTRING_TYPESTR => Ok(StringType::OsString),
            OSSTR_TYPESTR => Ok(StringType::OsStr),
            CSTRING_TYPESTR => Ok(StringType::CString),
            CSTR_TYPESTR => Ok(StringType::CStr),
            CHAR_TYPESTR | "Char" => Ok(StringType::char),
            _ => Err(TypeError::StringTypeUnknown(typestr.to_string())),
        }        
    }
    fn to_typestr(&self) -> String {
        String::new() + match self 
        {
            StringType::str => STR_TYPESTR,
            StringType::String => STRING_TYPESTR,
            StringType::OsString => OSSTRING_TYPESTR,
            StringType::OsStr => OSSTR_TYPESTR,
            StringType::CString => CSTRING_TYPESTR,
            StringType::CStr => CSTR_TYPESTR,
            StringType::char => CHAR_TYPESTR,
        }
    }
    fn to_typestr_no_ref(&self) -> String {
        self.to_typestr()
    }
    fn to_typestr_no_life(&self) -> String {
        self.to_typestr()
    }
    #[allow(unused_variables)]
    fn collect_lifetimes(&self, into: &mut Vec<String>) {
        // We dont have any life times here
    }
    /// for the enum table we cast every string based type down to a const static str for easy reference and initialize from that.
    /// 
    /// we kinda only need this handle for container types tuple and array that may be const if all contained entries are const
    /// 
    /// str can be declared as const but with a static lifetime
    /// 
    /// 
    fn is_const(&self) -> bool {
        match self 
        {
            StringType::str => true,
            StringType::char => true,
            _ => {false},
        }        
    }
    #[allow(unused_variables)]
    fn value_is_valid(&self, valuestr: &str) -> bool {
        match self {
            Self::char => {valuestr.len() == 1}
            _ => {true}
        }
    }
    fn get_depth(&self, counter: usize) -> usize {
        counter
    }
    fn get_breadth(&self, counter: usize) -> usize {
        counter + 1
    }
    fn wrap_valuestr(&self, valuestr: &str) -> String {
        match self {
            Self::char => {format!("'{}'", valuestr)}
            _ => {format!("\"{}\"", valuestr)}
        }        
        
    }
    fn can_match_as_key(&self) -> bool {
        true
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stringtype_from_typestr() {
        let typestr = "str".to_string();
        let result = StringType::from_typestr(typestr).unwrap();
        let expected = StringType::str;
        assert_eq!(result, expected);
        assert_eq!("\"c\"", result.wrap_valuestr("c"));

        let typestr = "char".to_string();
        let result = StringType::from_typestr(typestr).unwrap();
        let expected = StringType::char;
        assert_eq!(result, expected);

        assert_eq!(true, result.value_is_valid("c"));
        assert_eq!(false, result.value_is_valid("char"));
        assert_eq!("'c'", result.wrap_valuestr("c"));
        
    }

}