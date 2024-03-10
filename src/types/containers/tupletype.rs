use super::{RType, RTypeTrait};

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
        for x in rtypes {
            tuplestr = tuplestr + func(x.as_ref()).as_ref() + ", ";
        }
        tuplestr.pop(); 
        tuplestr.pop();
        tuplestr.push(')');
        tuplestr        
    }
}




#[cfg(test)]
mod tests {
    use super::*;
    use crate::{RType, RTypeTrait, ContainerType, NumericType, StringType, SpecialType, Reference};

    #[test]
    fn test_wrap_tuple() {
        let tupletypes = vec![Box::new(RType::Numeric(Reference::None, NumericType::u8)), Box::new(RType::String(Reference::Naked, StringType::str))];


    }



}