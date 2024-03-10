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
        //assert_eq!(false, ArrayType::value_is_valid(values, &Box::new(RType::Container(Reference::None, result)), &3));
        assert_eq!(false, rtype.value_is_valid(values));
    }

}