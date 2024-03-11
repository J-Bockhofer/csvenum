
//use crate::EnumTable;

fn main() {
    // we have some .csv file

    // need to set flag if output in one file or multiple

    // pass csv to Something::from_csv()

    // need to call Something.to_file()

    // done


}

#[derive(Debug, Eq, PartialEq)]
pub enum StringType
{
    /// `str` <br>
    /// typestr: &'static str = `str` <br>
    str,
    
    /// `OsString` <br>
    /// typestr: &'static str = `OsString` <br>
    OsString,
    
    /// `OsStr` <br>
    /// typestr: &'static str = `OsStr` <br>
    OsStr,
    
    /// `CString` <br>
    /// typestr: &'static str = `CString` <br>
    CString,
    
    /// `CStr` <br>
    /// typestr: &'static str = `CStr` <br>
    CStr,
    
}
const STRINGTYPE_ALL_VARIANTS_ARRAY: [StringType; 5] = [ 
StringType::str,
StringType::OsString,
StringType::OsStr,
StringType::CString,
StringType::CStr,
];
pub fn stringtype_get_all_variants() -> [StringType; 5]
{
    STRINGTYPE_ALL_VARIANTS_ARRAY
}
// Variant string representation.
const STRINGTYPE_STR_STR:&'static str = "str";
const STRINGTYPE_OSSTRING_STR:&'static str = "OsString";
const STRINGTYPE_OSSTR_STR:&'static str = "OsStr";
const STRINGTYPE_CSTRING_STR:&'static str = "CString";
const STRINGTYPE_CSTR_STR:&'static str = "CStr";

/// Returns the variants name as a &str.
pub fn stringtype_as_variant_str(stringtype: &StringType) -> &'static str
{
match stringtype {
    StringType::str => STRINGTYPE_STR_STR,
    StringType::OsString => STRINGTYPE_OSSTRING_STR,
    StringType::OsStr => STRINGTYPE_OSSTR_STR,
    StringType::CString => STRINGTYPE_CSTRING_STR,
    StringType::CStr => STRINGTYPE_CSTR_STR,
}
}

/// Returns the enum given a string that might represent the variant's name.
pub fn stringtype_from_variant_str<T: AsRef<str>>(variantstr: T) -> Option<StringType>
{
    let variantstr = variantstr.as_ref();
match variantstr {
    STRINGTYPE_STR_STR => Some(StringType::str),
    STRINGTYPE_OSSTRING_STR => Some(StringType::OsString),
    STRINGTYPE_OSSTR_STR => Some(StringType::OsStr),
    STRINGTYPE_CSTRING_STR => Some(StringType::CString),
    STRINGTYPE_CSTR_STR => Some(StringType::CStr),
    _ => Option::None,
}
}
/// Constants for `typestr`
const TYPESTR_STR: &'static str = "str";
const TYPESTR_OSSTRING: &'static str = "OsString";
const TYPESTR_OSSTR: &'static str = "OsStr";
const TYPESTR_CSTRING: &'static str = "CString";
const TYPESTR_CSTR: &'static str = "CStr";

/// Function to convert from StringType to typestr
pub fn stringtype_as_typestr(stringtype: &StringType) -> &'static str
{
match stringtype {
    StringType::str => TYPESTR_STR,
    StringType::OsString => TYPESTR_OSSTRING,
    StringType::OsStr => TYPESTR_OSSTR,
    StringType::CString => TYPESTR_CSTRING,
    StringType::CStr => TYPESTR_CSTR,
}
}

/// Function to convert from typestr to StringType
pub fn stringtype_from_typestr(typestr: &'static str) -> Option<StringType>
{
match typestr {
    TYPESTR_STR => Some(StringType::str),
    TYPESTR_OSSTRING => Some(StringType::OsString),
    TYPESTR_OSSTR => Some(StringType::OsStr),
    TYPESTR_CSTRING => Some(StringType::CString),
    TYPESTR_CSTR => Some(StringType::CStr),
    _ => Option::None,
}
}
impl StringType
{
    pub fn get_all_variants() -> [Self; 5]
    {
        stringtype_get_all_variants()
    }
    pub fn as_variant_str(&self) -> &'static str
    {
        stringtype_as_variant_str(self)
    }
    pub fn from_variant_str<T: AsRef<str>>(variantstr: T) -> Option<Self>
    {
        stringtype_from_variant_str(variantstr)
    }
    /// Function to convert from StringType to typestr
    pub fn as_typestr(&self) -> &'static str
    {
        stringtype_as_typestr(self)
    }
    pub fn from_typestr(typestr: &'static str) -> Option<Self>
    {
        stringtype_from_typestr(typestr)
    }
}
impl std::fmt::Display for StringType
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result 
    {
        match self {
            Self::str => writeln!(f, "{}, typestr = str ", self.as_variant_str())?,
            Self::OsString => writeln!(f, "{}, typestr = OsString ", self.as_variant_str())?,
            Self::OsStr => writeln!(f, "{}, typestr = OsStr ", self.as_variant_str())?,
            Self::CString => writeln!(f, "{}, typestr = CString ", self.as_variant_str())?,
            Self::CStr => writeln!(f, "{}, typestr = CStr ", self.as_variant_str())?,
        }
        Ok(())
    }
}
#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_stringtype_do_a_barrel_roll()
    {
        let stringtype = StringType::str;
        let result = stringtype.as_typestr();
        let result = StringType::from_typestr(result).unwrap();
        assert_eq!(stringtype, result);
    }
}
