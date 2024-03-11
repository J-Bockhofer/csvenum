use std::fmt;
use super::{indent_string, PrintConfig, CodeLines};


#[derive(Debug, Default)]
pub struct EnumBlock
{
    config: PrintConfig,
    doclines: Vec<String>, 
    enumname: String,
    variantnames: Vec<String>,
    properties: Option<Vec<String>>,
    data: Option<Vec<Vec<String>>>,
}

 impl
    EnumBlock
 {
    pub fn new() -> Self {
        EnumBlock::default()
    }

    pub fn with<T: AsRef<str>>(classname: T, variants: Vec<T>) -> Self {
        let mut this = EnumBlock::default();
        this.set_classname(classname);
        this.set_variants(variants);
        this
    }

    fn set_classname<T: AsRef<str>>(&mut self, classname: T) {
        self.enumname = classname.as_ref().to_string();
    }

    fn set_variants<T: AsRef<str>>(&mut self, variants: Vec<T>) {
        self.variantnames = variants.iter().map(|v| v.as_ref().to_string()).collect();
    }

    pub fn set_properties<T: AsRef<str>>(&mut self, properties: Vec<T>) {
        self.properties = Some(properties.iter().map(|v| v.as_ref().to_string()).collect());
    }

    /// Data should have form: 
    /// 
    /// first vector: properties
    /// 
    /// second vector: variants
    pub fn set_data(&mut self, data: Vec<Vec<String>>) {
        self.data = Some(data);
    }

    pub fn data_bounds_ok(&self) -> bool {
        if (self.properties.is_none() &&  self.data.is_some()) || (self.properties.is_some() &&  self.data.is_none()) {return false}
        if self.properties.is_none() && self.data.is_none() {return false}
        let data = self.data.as_ref().unwrap();
        let num_props = self.properties.as_ref().unwrap().len();
        let num_vars = self.variantnames.len();
        let num_cols = data.len();
        let num_rows = data[0].len();
        if num_cols != num_props || num_rows != num_vars {return false}
        true
    }


    pub fn set_config(&mut self, config: PrintConfig) {
        self.config = config;
    }

 }


 
impl fmt::Display for EnumBlock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for line in self.to_lines() {
            writeln!(f, "{}", line)?;
        }
        Ok(())
    }
} 


impl CodeLines for EnumBlock 
{
    fn lines_into(&self, dest: &mut Vec<String>) {
        let lines = dest;

        let indent = self.config.tabwidth;

        let enumname = &self.enumname;
        let variants = &self.variantnames;
        let data_ok = self.data_bounds_ok();
        let data = &self.data;
        let props = &self.properties;
        
        
        // Header
        lines.push(format!("pub enum {}", enumname)); // {{
        lines.push(format!("{{")); 

        for row in 0..variants.len() {
            let cvar= &variants[row];
            // Documentation
            let l = format!("/// {}", cvar);
            lines.push(indent_string(l, indent)); 
            lines.push(indent_string(format!("/// "), indent));  

            if data_ok {
                let props = props.as_ref().unwrap();
                let data = data.as_ref().unwrap();
                let num_props = props.len();

                for col in 0..num_props  {
                    let l = format!(
                        "/// Property `{}` with value `{}` "
                        , &props[col], data[col][row]
                    );
                    lines.push(indent_string(l, indent)); 
                    lines.push(indent_string(format!("/// "), indent)); 
                }
            } 

            // Declaration
            let l = format!("{},", cvar);
            lines.push(
                indent_string(l, indent)
            );
        }
    
        // Footer
        lines.push(
            format!("}}")
        );

        //lines
    }

    fn to_lines(&self) -> Vec<String> {
        let mut lines:Vec<String> = vec![];
        self.lines_into(&mut lines);
        lines
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enumblock() {
        let enumname = "MyEnum".to_string();
        let variantnames = vec!["A".to_string(),"B".to_string(), "C".to_string()];
        let enumblock: EnumBlock = EnumBlock::with(enumname, variantnames);
        let mut lines: Vec<String> = vec![];
        enumblock.lines_into(&mut lines);
        println!("{}", enumblock);
        //assert_eq!(ttype, expected);
    }
}