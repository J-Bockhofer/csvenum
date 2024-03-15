
use super:: RType;

/// Property + Variant
/// 
/// Property holds name and type info
#[derive(Debug)]
pub struct Property {
    pub name: String,
    pub rtype: RType,
    pub is_const: bool,
    pub contains_duplicates: bool,     //  value, variants
    pub value_to_variant_duplicates: Vec<(String, Vec<String>)>, // empty vec when no duplicates
}

/// Variant holds name, fullname, and values as vec?! then index by property or have ref to perpety (&prop, value)
#[derive(Debug)]
pub struct DummyVariant<'a> {
    pub name: &'a str,
    pub enum_name: &'a str,
    //pub assoc_vals: Vec<(&'a Property, String)>,

}


pub struct EnumTable2 {
    pub enum_name: String,




}

impl EnumTable2 {
/*     pub fn new(enum_name: String) -> Self {

    } */


}