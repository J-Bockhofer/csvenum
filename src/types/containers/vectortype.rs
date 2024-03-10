use super::NestedValueParser;

pub struct VectorType {

}
impl VectorType {
    pub fn wrap_str(vstr: &str) -> String {
        format!("Vec<{}>", vstr)
    }


    pub fn value_is_valid(valuestr: &str) -> bool {

            false

    }

}