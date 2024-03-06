///! Convert Types passed as a string to more useful representations for code gen.
///! Allows for specifying "&str" in a textfile to generate associated representations in function signatures and expressions. 
///!

pub mod type_const;
use type_const::ConstType;

pub mod type_container;
use type_container::ContainerType;
/// We take in text like this: 
/// ```
///  let text = vec!["&str","ABC","SomeText"];
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
    fn get_value_wrapper(&self) -> TypeWrapper;
    fn from_typestr<T: AsRef<str>>(typestr: T) -> Option<Self> where Self: Sized;
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

#[derive(Debug, PartialEq, Eq)]
pub enum TType {
    ConstType(ConstType),
    ContainerType(ContainerType),
    ///
    /// Regex: Regex'
    /// 
    /// Fn return: &Regex
    /// 
    /// Fn arg: &Regex
    /// 
    /// Const expr: OnceLock<Regex>
    /// 
    /// Let expr: input
    /// 
    /// Declare with value
    Regex,
}

pub fn sanitize_typestr<T: AsRef<str>>(typestr: T) -> String {
    let typestr = typestr.as_ref();
    if typestr.len() > 128 {panic!("I think your typestr is too long: {}", typestr)}
    let mut parsed_type = String::new();

    for char in typestr.chars() {
        match char {
            ' ' => {continue},
            'V' => parsed_type.push('V'),
            '&' => parsed_type.push('&'),
            'e' => parsed_type.push('e'),
            'c' => parsed_type.push('c'),
            '<' => parsed_type.push('<'),
            '>' => parsed_type.push('>'),
            '(' => parsed_type.push('('),
            ')' => parsed_type.push(')'),
            ',' => parsed_type.push(','),
            '1' => parsed_type.push('1'),  // 16
            '2' => parsed_type.push('2'),  // 32    
            '3' => parsed_type.push('3'),  // 32
            '4' => parsed_type.push('4'),  // 64      
            '5' => parsed_type.push('5'),  // 256
            '6' => parsed_type.push('6'),  // 16, 64
            '7' => parsed_type.push('7'),         
            '8' => parsed_type.push('8'),  // 128
            '9' => parsed_type.push('9'),      
            'i' => parsed_type.push('i'),  
            'f' => parsed_type.push('f'),              
            'u' => parsed_type.push('u'),     
            's' => parsed_type.push('s'),  
            'z' => parsed_type.push('z'),  
            'r' => parsed_type.push('r'),    
            't' => parsed_type.push('t'),                                                                                                                                                                                     
            _ => {continue}
        }
    }
    parsed_type
}

impl TType {
    /// the typestr will have to be sanitized by this point meaning no whitespaces (.trim()) or odd characters.
    pub fn from_typestr<T: AsRef<str>>(typestr: T) -> Option<Self> {
        let typestr = typestr.as_ref();

        let parsed_type = sanitize_typestr(typestr);

        //println!("TType - typestr sanitized: {}", &parsed_type);

        let _container = ContainerType::from_typestr(&parsed_type);
        if _container.is_some() {
            return Some(Self::ContainerType(_container.unwrap()));
        } 

        let _const = ConstType::from_typestr(parsed_type);
        if _const.is_some() {
            return Some(Self::ConstType(_const.unwrap()));
        }
        None

    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_typestr() {
        let typestr = " Vec< &str>".to_string();
        let ttype = TType::from_typestr(typestr).unwrap();
        let expected = TType::ContainerType(ContainerType::Vector(ConstType::Char));
        assert_eq!(ttype, expected);

        let typestr = " (usize, f32, f64, u8 ) ".to_string();
        let ttype = TType::from_typestr(typestr).unwrap();
        let expected = TType::ContainerType(ContainerType::Tuple(
            vec![
                ConstType::Numeric(format!("usize")),
                ConstType::Numeric(format!("f32")),
                ConstType::Numeric(format!("f64")),
                ConstType::Numeric(format!("u8")),
            ]
        ));
        assert_eq!(ttype, expected);
    }
}