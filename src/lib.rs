/// Property will be converted to: 
/// - Two functions:
/// 
///     as_(name) -> Enum
/// 
///     from_(name) -> (type_str)
/// 
/// 
/// - One Constant per Variant
/// 
///     const (VARIANT)_(NAME): (type_str)
pub struct Property {
    pub name: &'static str,
    pub type_str: &'static str, // for return value
}

#[derive(Debug)]
pub struct ConstProperty {
    pub name: String,
    pub var_str: String,
    pub type_str: String, // for return value    
}

impl ConstProperty {
    pub fn from(enumname: &str, variant: &str, property: &str, type_str: &str) -> Self{
        let var_str = format!("{}_{}_{}", enumname.to_uppercase(), variant.to_uppercase(), property.to_uppercase());
        ConstProperty { 
            name: variant.to_string(), 
            var_str, 
            type_str: type_str.to_string() }
    }
}

/// Function to generate const variable names for the passed variantnames and properties
/// 
/// Table of
/// TYPES    | &'static str | usize
/// EnumName | property1    | property2
/// Variant1 | Value1       | Value2
pub fn make_varnames(enumname: &str, variantnames: Vec<&str>, properties: Vec<&str>, types: Vec<&str>) {
    if properties.len() != types.len() {todo!("Log error: Types and Properties must have the same length")}
    //let num_variants = variantnames.len();
    let num_props = properties.len();
    // enforce first letter caps for variant

    // 1. Names for PropertyConstants - of variantnames.len() * properties.len() - FULL_CAPS
    let mut property_constants: Vec<ConstProperty> = vec![];

    for name in variantnames {
        if !name.is_ascii() {todo!("Log error: Variant Name not ASCII")}
        for i in 0..num_props {
            let property = properties[i];
            let type_str = types[i];
            if !property.is_ascii() {todo!("Log error: Property Name not ASCII")}
            property_constants.push(
                ConstProperty::from(enumname, name, property, type_str)
            );
        }
    }

    println!("{:?}", &property_constants);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let _ = make_varnames("Country", vec!["Australia"], vec!["Alpha3"], vec!["&'static str"]);
    }
}

