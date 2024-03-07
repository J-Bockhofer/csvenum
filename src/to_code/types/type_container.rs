
use super::{TypeToStr, TypeWrapper, type_const::ConstType, TypeError};
/// `ContainerType` - represents a type that contains values 
/// 
/// Limitations:
/// 1. Does not implement containers of containers
/// 2. Does not implement containers of regex
/// 
/// ## Example
/// ```
/// use std::sync::OnceLock;
/// // const abc: String = String::from("");
/// // ^ returns a compile error bc a String cant be allocated at compile-time, i.e. not a true constant
/// static ABC: OnceLock<Vec<String>> = OnceLock::new();
/// 
/// fn main() {
///     let abc = ABC.get_or_init(||{vec![String::from("this works!")]});
///     println!("{}", abc);
/// } 
/// 
/// ```
#[derive(Debug, Eq, PartialEq)]
pub enum ContainerType {
    Vector(ConstType),
    Tuple(Vec<ConstType>),
    //Option(ConstType),
}

impl ContainerType {
    pub fn declare_values_in_container<T: AsRef<str>>(&self, values: &Vec<T>) -> String {
        
        let type_wrapper = self.get_value_wrapper();
        match self {
            Self::Vector(x) => {
                let value_wrapper = x.get_value_wrapper();
                //let mut declare_str = String::new();
                type_wrapper.wrap_values_vec(&values, value_wrapper)
                
            },
            Self::Tuple(tuple) => {
                if tuple.len() != values.len() {panic!("Cant not format tuple. Num values and num types are unequal")}
                let mut value_str = String::new();
                let mut idx: usize = 0;
                for typ in tuple {
                    let value_wrapper = typ.get_value_wrapper();
                    value_str += &format!(
                        "{},", value_wrapper.wrap_value(values[idx].as_ref())
                    ); 
                    idx += 1;
                }
                value_str.remove(value_str.len()-1); // removes the last ,
                type_wrapper.wrap_value(value_str)
            },
        }
    }
}


impl TypeToStr for ContainerType {
    fn get_value_wrapper(&self) -> TypeWrapper {
        match self {
            Self::Vector(_) => {TypeWrapper::with("vec![", "]")},
            Self::Tuple(_) => {TypeWrapper::with("(", ")")},
        }
    }
    
    fn to_return_typestr(&self) -> String {
        match self {
            Self::Vector(x) => {
                format!("Vec<{}>", x.to_return_typestr())
            },
            Self::Tuple(types) => {
                let mut typestr = String::new();
                // concat types
                for t in types {
                    typestr += &format!(" {}", t.to_return_typestr());
                }
                typestr.remove(typestr.len()-1); // removes the last ,
                format!("({})", typestr)
            },
        }
    }
    fn to_fn_arg(&self) -> String {
        match self {
            Self::Vector(x) => {
                format!("Vec<{}>", x.to_fn_arg())
            },
            Self::Tuple(types) => {
                let mut typestr = String::new();
                // concat types
                for t in types {
                    typestr += &format!("{},", t.to_fn_arg());
                }
                typestr.remove(typestr.len()-1); // removes the last ,
                format!("({})", typestr)
            },
        }        
    }
    fn to_let_typestr(&self) -> String {
        match self {
            Self::Vector(x) => {
                format!("Vec<{}>", x.to_let_typestr())
            },
            Self::Tuple(types) => {
                let mut typestr = String::new();
                // concat types
                for t in types {
                    typestr += &format!("{},", t.to_let_typestr());
                }
                typestr.remove(typestr.len()-1); // removes the last ,
                format!("({})", typestr)
            },
        }
    }
    fn to_const_typestr(&self) -> String {
        match self {
            Self::Vector(x) => {
                format!("OnceLock<Vec<{}>>", x.to_let_typestr())
            },
            Self::Tuple(types) => {
                let mut typestr = String::new();
                // concat types
                for t in types {
                    typestr += &format!("{},", t.to_let_typestr());
                }
                typestr.remove(typestr.len()-1); // removes the last ,
                format!("({})", typestr)
            },
        }        
    }

    fn from_typestr<T: AsRef<str>>(typestr: T) -> Result<Self, TypeError>  {
        let typestr = typestr.as_ref();
        if typestr.contains("Vec") {
            // we have ourselves a Vec... boiii
            let typestr = typestr.replace("&Vec","Vec").replace("Vec<","").replace(">", "");
            // parsed type now only contains the inner typestr
            let inner_type = ConstType::from_typestr(typestr)?;
            //if inner_type.is_none() {return None}
            return Ok(Self::Vector(inner_type));
        } else if typestr.contains("(") {
            // we have ourselves a Tuple
            let typestr = typestr.replace("&(","(").replace("(","").replace(")", "");
            let inner_types = typestr.split(',');
            let mut vec_types: Vec<ConstType> = vec![];
            for inner in inner_types {
                //println!("ContainerType::Tuple - typestr into const: {}", inner);
                let inner = ConstType::from_typestr(inner)?;
                vec_types.push(inner);
            
            }
            Ok(Self::Tuple(vec_types))

        } else {
            Err(TypeError::Unknown(typestr.to_string()))  
        }
    }



}




#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::OnceLock;
    static ABC: OnceLock<Vec<String>> = OnceLock::new();
    #[test]
    fn tets_it_works() {
        let abc = ABC.get_or_init(||{vec![String::from("this works!")]});
        println!("{:?}", abc);
    }

    #[test]
    fn test_typestr_to_container() {
        let input = "Vec<&str>";
        let this_type = ContainerType::from_typestr(input).unwrap();
        let expected = ContainerType::Vector(ConstType::Char);
        assert_eq!(this_type, expected);

        let input = "Vec<usize>";
        let this_type = ContainerType::from_typestr(input).unwrap();
        let expected = ContainerType::Vector(ConstType::Numeric("usize".to_string()));
        assert_eq!(this_type, expected);

        let input = "Vec<f64>";
        let this_type = ContainerType::from_typestr(input).unwrap();
        let expected = ContainerType::Vector(ConstType::Numeric("f64".to_string()));
        assert_eq!(this_type, expected);   

        let input = "(f64,usize,&str)";
        let this_type = ContainerType::from_typestr(input).unwrap();
        let expected = ContainerType::Tuple(vec![
            ConstType::Numeric("f64".to_string()),
            ConstType::Numeric("usize".to_string()),
            ConstType::Char,
            ]);
        assert_eq!(this_type, expected);             
    }
    #[test]
    fn test_evalute_container_type() {
        let input = ContainerType::Tuple(vec![
            ConstType::Numeric("f64".to_string()),
            ConstType::Numeric("usize".to_string()),
            ConstType::Char,
            ]);
        let values = vec!["0","0","betamax"];
        let result = input.declare_values_in_container(&values);
        let expected = "(0,0,\"betamax\")".to_string();
        assert_eq!(result, expected);  

        let input = ContainerType::Vector(
            ConstType::Char,
            );
        let values = vec!["0","0","betamax"];
        let result = input.declare_values_in_container(&values);
        let expected = "vec![\"0\",\"0\",\"betamax\"]".to_string();
        assert_eq!(result, expected);  

    }


}