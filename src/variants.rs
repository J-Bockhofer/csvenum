
#[derive(Debug)]
pub struct Variant {
    pub variant: String,
}

impl Variant {
    pub fn new(variant: String) -> Self {
        Variant{
            variant
        }
    }

    pub fn combine_str(&self, enumname: &str) -> String {
        format!("{}::{}", enumname, self.variant)
    }
}