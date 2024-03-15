use super::RTypeTrait;
mod enumtype;
pub use enumtype::EnumType;



/// Special types have vastly different behaviour than the other types and can not be directly inferred from a string representation (except for bool).
#[derive(Debug, Eq, PartialEq)]
pub enum SpecialType {
    // the enum variant will contain the name.. or to separate it be make a named RType, which will contain building functions, expressions and declarations 
    Enum(String), // no just use the trait and store name here
    // Regex should return a as &Regex function and and is match function that takes self and a string and matches against it.
    Regex,
    /// When a property is of type singular bool then collect all variants with true and false as const array and return as Vec in match function
    Bool,
    // bool
}

impl SpecialType {

}

impl RTypeTrait for SpecialType {
    #[allow(unused_variables)]
    fn from_typestr<T: AsRef<str>>(typestr: T) -> Result<Self, super::TypeError> where Self: Sized {
        use super::TypeError;
        Err(TypeError::SPECIALTYPEPARSE)
    }
    // Call this last in a stack because it will always return SpecialType::Regex ... no real way to detect that other than looking for chaos
/*     fn from_valuestr<T: AsRef<str>>(valuestr: T) -> Result<Self, super::TypeError> where Self: Sized {
        if let Some(enumtype) = EnumType::from_value(valuestr.as_ref()) {return Ok(enumtype)}
        // if is "false" || "true" -> bool


        Ok(Self::Regex)
    } */

    fn to_typestr(&self) -> String {
        match self {
            Self::Enum(x) => {x.to_owned()},
            Self::Bool => {"bool".to_string()},
            Self::Regex => {"Regex".to_string()},
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
    /// Not applicable to enum since enums const be decalred as const or static, but as enum MyEnum {...}
    /// 
    /// Regex require OnceLock so they are definitely not const
    /// 
    /// 
    fn is_const(&self) -> bool {
        match self {
            Self::Bool => {true},
            Self::Regex => {true}, // will for now be true to provide a simple method until static types are fully implemented
            _ => {false},
        }        
    }
    /// For enums, values are variants, so the Rust syntax rules for Enum variants apply, it could also have the name of the enum prefixed
    /// 
    /// For Regex, values are r"" that contain weird characters, so take anything as valid i guess... but when get_or_init'ing the OnceLock<Regex> handle the except with a generated error message
    /// 
    fn value_is_valid(&self, valuestr: &str) -> bool {
        match self {
            Self::Enum(x) => {EnumType::value_is_valid_enum(x, valuestr)},
            Self::Regex => {
                true},
            Self::Bool => {
                let valstr = valuestr.trim();
                match valstr {
                    "true" | "True" | "1" | "false" | "False" | "0" => {true}
                    _ => false
                }
            },
        }
    }

    fn get_depth(&self, counter: usize) -> usize {
        counter + 0
    }
    fn get_breadth(&self, counter: usize) -> usize {
        counter + 1
    }
    fn wrap_valuestr(&self, valuestr: &str) -> String {
        match self {
            Self::Enum(x) => {
                if !valuestr.contains(x) {
                    format!("{}::{}",x , valuestr)
                } else {valuestr.to_string()}
            },
            Self::Regex => {
                format!("r\"{}\"", valuestr)
                //todo!("Impl wrapping for value of type regex with OnceLock get_or_init ...");
            },
            Self::Bool => {
                let valstr = valuestr.trim();
                match valstr {
                    "true" | "True" | "1"  => "true".to_string(),
                     "false" | "False" | "0" => "false".to_string(),
                    _ => "false".to_string()
                }                
            },
        }  
    }
    fn can_match_as_key(&self) -> bool {
        true
    }
}

/* #[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stringtype_from_typestr() {
        let typestr = "str".to_string();
        let result = StringType::from_typestr(typestr).unwrap();
        let expected = StringType::str;
        assert_eq!(result, expected);

    }

} */