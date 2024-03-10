use super::{NestedValueParser, RType, RTypeTrait};


pub struct VectorType {

}
impl VectorType {
    pub fn wrap_typestr(typestr: &str) -> String {
        format!("Vec<{}>", typestr)
    }


    pub fn value_is_valid(valuestr: &str, rtype: &Box<RType>) -> bool {
        // the num of contained elements can be unlimited but they all have to conform to RType
        let values = NestedValueParser::parse_nested_str(valuestr, '[', true);

        for value in values {
            if !rtype.value_is_valid(&value) {
                return false
            }
        }
        true
    }



}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ RType}; //NumericType, StringType, SpecialType,

    #[test]
    fn test_vectortype_valid_values() { 
        let input = "Vec<usize>";
        let rtype = RType::from_typestr(input).unwrap();
        let values = "[3,3,3]";
        assert_eq!(true, rtype.value_is_valid(values));
        let values = "[art,5.43,3]";
        //assert_eq!(false, ArrayType::value_is_valid(values, &Box::new(RType::Container(Reference::None, result)), &3));
        assert_eq!(false, rtype.value_is_valid(values));
    }

    #[test]
    fn test_vectortype_valid_values_complex() { 
        let input = "Vec<Vec<[usize; 3]>>";
        let rtype = RType::from_typestr(input).unwrap();
        let values = "[[[3,3,3]]]";
        assert_eq!(true, rtype.value_is_valid(values));
        let values = "[art,5.43,3]";
        //assert_eq!(false, ArrayType::value_is_valid(values, &Box::new(RType::Container(Reference::None, result)), &3));
        assert_eq!(false, rtype.value_is_valid(values));
    }

}