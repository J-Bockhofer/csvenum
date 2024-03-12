use crate::{RType, RTypeTrait};


use super::NestedValueParser;




pub struct ArrayType {

}

impl ArrayType {
    pub fn wrap_typestr(vstr: &str, size: &usize) -> String {
        format!("[{}, {}]", vstr, size)
    }

    /// let aaa: [[usize; 3]; 3] = [[3,3,3], [3,3,3], [3,3,3]];
    /// 
    /// After step 1 = [3,3,3], [3,3,3], [3,3,3]
    /// 
    /// 
    /// rtype = type T of the array [T; N]
    pub fn value_is_valid(valuestr: &str, rtype: &Box<RType>, size: &usize) -> bool {
        //let aaa: [Vec<String>; 3] = [vec![], vec![], vec![]];
        let values = NestedValueParser::parse_nested_str(valuestr, '[', true);

/*         println!("RType: {:?}", rtype);
        println!("ValStr: {:?}", valuestr);
        println!("Size: {:?}", size);
        println!("Parsed values: {:?}", values); */
        // We expect N values for an array [T; N]
        if values.len() != *size {
            return false;
        }
        //let mut is_valid = true;
        for value in values {
            if !rtype.value_is_valid(&value) {return false}
        }
        true
    }

    pub fn wrap_valuestr(valuestr: &str, rtype: &Box<RType>) -> String {
        let values = NestedValueParser::parse_nested_str(valuestr, '[', true);

        let mut wrapped_str = String::from("[");
        //let mut is_valid = true;
        for value in values {
            let valstr = rtype.wrap_valuestr(&value);
            wrapped_str = format!("{}{}, ", wrapped_str, valstr);
        }
        wrapped_str.pop(); wrapped_str.pop();
        wrapped_str += "]";
        wrapped_str
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ RType, ContainerType}; //NumericType, StringType, SpecialType,

    #[test]
    fn test_arraytype_valid_values() { 
        let input = "[usize; 3]";
        let result = ContainerType::from_typestr(input).unwrap();
        let rtype = RType::from_typestr(input).unwrap();
        let values = "[3,3,3]";
        assert!(result.value_is_valid(values));
        let values = "[art,5.43,3]";
        //assert_eq!(false, ArrayType::value_is_valid(values, &Box::new(RType::Container(Reference::None, result)), &3));

        assert_eq!(false, rtype.value_is_valid(values));
    }

    #[test]
    fn test_arraytype_valid_values_complex() {
        let typestr = "[[usize; 3]; 3]";
        let values = "[[3,3,3], [3,3,3], [3,3,3]]";
        let rtype = RType::from_typestr(typestr).unwrap();
        assert_eq!(true, rtype.value_is_valid(values));

        let values = "[[3,3,3], [3,3,3], [3,3,3,4]]";
        assert_eq!(false, rtype.value_is_valid(values));
    }

    #[test]
    fn test_arraytype_wrap_values() {
        let typestr = "[[usize; 3]; 3]";
        let values = "[[3,3,3], [3,3,3], [3,3,3]]";
        let rtype = RType::from_typestr(typestr).unwrap();
        assert_eq!(true, rtype.value_is_valid(values));
        let expected = "[[3, 3, 3], [3, 3, 3], [3, 3, 3]]";
        assert_eq!(expected, rtype.wrap_valuestr(values));
    }


}