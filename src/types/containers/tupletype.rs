use super::{RType, RTypeTrait, NestedValueParser};

pub struct TupleType {

}

impl TupleType {
    pub fn wrap_vals<T: AsRef<str>>(vals: Vec<T>) -> String {
        let mut tuplestr = String::from("(");
        for x in vals {
            tuplestr = tuplestr + x.as_ref() + ", ";
        }
        tuplestr.pop(); 
        tuplestr.pop();
        tuplestr.push(')');
        tuplestr


    }
    /// this actually compiles?!
    pub fn wrap_types_with_typefn<T: RTypeTrait, R>(rtypes: &Vec<Box<T>>, func: fn(&T) -> R) -> String 
    where
        R: AsRef<str>,
    {
        let mut tuplestr = String::from("(");
        for x in rtypes { //  v &RType    v String.as_ref()
            tuplestr = tuplestr + func(x.as_ref()).as_ref() + ", ";
        }
        tuplestr.pop(); 
        tuplestr.pop();
        tuplestr.push(')');
        tuplestr        
    }

    pub fn value_is_valid(valuestr: &str, rtypes: &Vec<Box<RType>>) -> bool {
        let values = NestedValueParser::parse_nested_str(valuestr, '(', true);

        // we expect as many values as there are elements in our vector 
        let tuple_size = rtypes.len();
        if values.len() != tuple_size {return false}
        for i in 0..tuple_size {
            if !rtypes[i].value_is_valid(&values[i]) {return false}
        }
        true
    }

    pub fn wrap_valuestr(valuestr: &str, rtypes: &Vec<Box<RType>>) -> String {
        let values = NestedValueParser::parse_nested_str(valuestr, '(', true);
        let mut wrapped_str = String::from("(");
        let tuple_size = rtypes.len();
        for i in 0..tuple_size {
            let valstr = rtypes[i].wrap_valuestr(&values[i]);
            wrapped_str = format!("{}{}, ", wrapped_str, valstr);
        }
        wrapped_str.pop(); wrapped_str.pop();
        wrapped_str += ")";
        wrapped_str


    }
}




#[cfg(test)]
mod tests {

    use crate::{RType, RTypeTrait};//, NumericType, StringType, Reference};

    #[test]
    fn test_wrap_tuple() {
        //let tupletypes = vec![Box::new(RType::Numeric(Reference::None, NumericType::u8)), Box::new(RType::String(Reference::Naked, StringType::str))];
        //let typestr = tupletypes


    }

    #[test]
    fn test_tupletype_valid_values() { 
        let input = "(usize, usize, usize)";
        let rtype = RType::from_typestr(input).unwrap();
        let values = "(3,3,3)";
        assert_eq!(true, rtype.value_is_valid(values));
        let values = "[3,3,3]";
        assert_eq!(false, rtype.value_is_valid(values));
        let values = "(art,5.43,3)";
        //assert_eq!(false, ArrayType::value_is_valid(values, &Box::new(RType::Container(Reference::None, result)), &3));
        assert_eq!(false, rtype.value_is_valid(values));
    }

    #[test]
    fn test_tupletype_valid_values_complex() { 
        let input = "((usize, usize), usize, [usize; 3])";
        let rtype = RType::from_typestr(input).unwrap();
        let values = "((2,2),3,[5,5,5])";
        assert_eq!(true, rtype.value_is_valid(values));
        let values = "((2,2)3,[5,5,5])";
        assert_eq!(false, rtype.value_is_valid(values));
        let values = "(art,5.43,3)";
        assert_eq!(false, rtype.value_is_valid(values));
    }

    #[test]
    fn test_tupletype_wrap_values() { 
        let input = "((usize, usize), usize, [usize; 3])";
        let rtype = RType::from_typestr(input).unwrap();
        let values = "((2,2),3,[5,5,5])";
        let expected = "((2, 2), 3, [5, 5, 5])".to_string();
        assert_eq!(expected, rtype.wrap_valuestr(values));

        let input = "((usize, usize), &str, [usize; 3])";
        let rtype = RType::from_typestr(input).unwrap();
        let values = "((2,2),3,[5,5,5])";
        let expected = "((2, 2), \"3\", [5, 5, 5])".to_string();
        assert_eq!(expected, rtype.wrap_valuestr(values));
    }

}