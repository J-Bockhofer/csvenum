/* 
#[derive(Debug)]
pub struct ConstProperty {
    pub name: String,
    pub prop_str: String,
    pub type_str: String, // for return value
    pub as_fn_hdr: String,
    pub from_fn_hdr: String, 
}

impl ConstProperty {
    pub fn new(name: &str, type_str: &str) -> Self {
        ConstProperty { 
            name: name.to_string(), 
            prop_str: "".to_string(), 
            type_str: type_str.to_string(), 
            as_fn_hdr: "".to_string(), 
            from_fn_hdr: "".to_string(), 
        }
    }

    pub fn from(enumname: &str, property: &str, type_str: &str) -> Self{
        let prop_lc = property.to_ascii_lowercase();
        let enum_lc = enumname.to_ascii_lowercase();

        let prop_str = format!("{}", property.to_ascii_uppercase());
        let as_fn_hdr = format!("pub fn {enum_lc}_as_{prop_lc}({enum_lc}: &{enumname}) -> {type_str}");
        let from_fn_hdr = format!("pub fn {enum_lc}_from_{prop_lc}({prop_lc}: {type_str}) -> Option<{enumname}>");
        ConstProperty {
            name: property.to_string(), 
            prop_str, 
            type_str: type_str.to_string(),
            as_fn_hdr,
            from_fn_hdr,
         }
    }
} */