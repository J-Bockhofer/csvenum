
use super::TypeToStr;

/// each type will have two things in their tuple (last two entries) first: bool isReference, second: String specified lifetime... i want to leave that for later aarrgh
#[allow(non_camel_case_types)]
pub enum StringType {
    /// str
    /// 
    /// Property `typestr` with value `str` 
    /// 
    str,
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
}
impl StringType {
    pub fn get_all_variants() -> Vec<StringType> {
        vec![
            StringType::str,
            StringType::OsString,
            StringType::OsStr,
            StringType::CString,
            StringType::CStr,
        ]
    }
}
/// Consts for Property `typestr` 
const STR_TYPESTR: &'static str = "str";
const OSSTRING_TYPESTR: &'static str = "OsString";
const OSSTR_TYPESTR: &'static str = "OsStr";
const CSTRING_TYPESTR: &'static str = "CString";
const CSTR_TYPESTR: &'static str = "CStr";
 
pub fn stringtype_as_typestr(stringtype: &StringType) -> &'static str {
    match stringtype {
        StringType::str => STR_TYPESTR,
        StringType::OsString => OSSTRING_TYPESTR,
        StringType::OsStr => OSSTR_TYPESTR,
        StringType::CString => CSTRING_TYPESTR,
        StringType::CStr => CSTR_TYPESTR,
    }
}
pub fn stringtype_from_typestr(typestr: &'static str) -> Option<StringType> {
    match typestr {
        STR_TYPESTR => Some(StringType::str),
        OSSTRING_TYPESTR => Some(StringType::OsString),
        OSSTR_TYPESTR => Some(StringType::OsStr),
        CSTRING_TYPESTR => Some(StringType::CString),
        CSTR_TYPESTR => Some(StringType::CStr),
        _ => Option::None,
    }
}

