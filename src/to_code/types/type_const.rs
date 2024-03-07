use super::{TypeToStr, TypeWrapper, TypeError};
/// ## ConstType
/// 
/// Can only have a single value and can be declared as const.
/// Bottom type.
/// 
/// Let's say our input string is "&str".
/// 
/// That would be equivalent to ConstType::Char.
///
#[derive(Debug, Eq, PartialEq)]
pub enum ConstType {
    /// Chars: &'static str | &str 
    /// 
    /// Fn return: &str
    /// 
    /// Fn arg: &str
    /// 
    /// Const expr: &'static str
    /// 
    /// Let expr: &str
    /// 
    /// Declare with value: "value" 
    Char,
    /// Numeric: (u | i | f + digits) + usize + isize
    /// 
    /// Fn return: input
    /// 
    /// Fn arg: input
    /// 
    /// Const expr: input
    /// 
    /// Let expr: input
    /// 
    /// Declare with value: value
    Numeric(String),

}

impl ConstType {
    /// does not output a full line, but just a value wrapped in it's type
    /// Examples:
    /// value in text is harald
    /// will print "harald" 
    /// value in text is 0
    /// will print 0 
    pub fn declare_value<T: AsRef<str>>(&self, value: T) -> String {
        let value = value.as_ref();
        match self {
            Self::Char => {format!("\"{}\"", value)},
            Self::Numeric(_) => {format!("{}", value)},
        }
    }
}


impl TypeToStr for ConstType {
    fn to_return_typestr(&self) -> String {
        return match self {
            Self::Char => {String::from("&str")},
            Self::Numeric(x) => {x.to_string()},
        }        
    }
    fn to_const_typestr(&self) -> String {
        return match self {
            Self::Char => {String::from("&'static str")},
            Self::Numeric(x) => {x.to_string()},
        }         
    }
    fn to_fn_arg(&self) -> String {
        return match self {
            Self::Char => {String::from("&str")},
            Self::Numeric(x) => {x.to_string()},
        }         
    }
    fn to_let_typestr(&self) -> String {
        return match self {
            Self::Char => {String::from("&str")},
            Self::Numeric(x) => {x.to_string()},
        }         
    }
    fn get_value_wrapper(&self) -> TypeWrapper {
        match self {
            Self::Char => {TypeWrapper::with("\"", "\"")},
            Self::Numeric(_) => {TypeWrapper::with("","")},
        }
    }
    fn from_typestr<T: AsRef<str>>(typestr: T) -> Result<Self, TypeError> {
        let typestr = typestr.as_ref();

        let parts: Vec<&str> = typestr.split(' ').collect();
        let actual_type = match parts.len() {
            1 => {parts[0]},
            2 => {parts[1]},
            _ => {return Err(TypeError::Unknown(typestr.to_string()));}
        };

        let actual_type: String = actual_type.chars().filter(|&c|
            c != '&' && c != '1' && c != '2' && c != '3' && c != '4' && c != '5' && c != '6' && c != '7' && c != '8' && c != '9' && c != '0' && c != ' ' 
        ).collect();
        //println!("{}", actual_type.as_str());
        match actual_type.as_str() {
            "" => {Err(TypeError::Unknown(typestr.to_string()))},
            "str" => {
                Ok(Self::Char)
            }
            "i" | "u" | "f" | "isize" | "usize" => {
                Ok(Self::Numeric(typestr.to_string()))
            }
            _ => {Err(TypeError::Unknown(typestr.to_string()))}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const STR_STR: &'static str = "str";
    const STR_USIZE: &'static str = "usize"; 
    const STR_F64: &'static str = "f64"; 

    #[test]
    fn test_consttype_soundness() {
        // 
        let const_type = ConstType::Char;
        let result = const_type.to_return_typestr();
        assert_eq!(result, format!("&{}", STR_STR));
        let result = const_type.to_let_typestr();
        assert_eq!(result, format!("&{}", STR_STR));
        let result = const_type.to_fn_arg();
        assert_eq!(result, format!("&{}", STR_STR));
        let result = const_type.to_const_typestr();
        assert_eq!(result, format!("&'static {}", STR_STR));

        let const_type = ConstType::Numeric("usize".to_string());
        let result = const_type.to_return_typestr();
        assert_eq!(result, format!("{}", STR_USIZE));
        let result = const_type.to_let_typestr();
        assert_eq!(result, format!("{}", STR_USIZE));
        let result = const_type.to_fn_arg();
        assert_eq!(result, format!("{}", STR_USIZE));
        let result = const_type.to_const_typestr();
        assert_eq!(result, format!("{}", STR_USIZE));

        let const_type = ConstType::Numeric("f64".to_string());
        let result = const_type.to_return_typestr();
        assert_eq!(result, format!("{}", STR_F64));
        let result = const_type.to_let_typestr();
        assert_eq!(result, format!("{}", STR_F64));
        let result = const_type.to_fn_arg();
        assert_eq!(result, format!("{}", STR_F64));
        let result = const_type.to_const_typestr();
        assert_eq!(result, format!("{}", STR_F64));        
    }
}