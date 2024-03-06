/// What do I want?
/// - 
/// 
/// What do I have? 
/// - The types from a string representation
/// 
/// 
/// need AST to CST - internal representation to code
/// 
/// Leaves is the data / types, so the constants in this case
/// 
/// Nodes are what is done to the data / types
/// 
/// Nodes can be:
/// 
/// 1. Expression
/// 2. Function
/// 3. Declaration
/// 
pub struct Closure {

}


pub enum CodeBlocks {
    ENUM,
    IMPL,
    FUNC,
    CONST,
}

#[derive(Debug, Default)]
pub struct EnumBlock
<
    STR: AsRef<str>  + 
    std::default::Default +
    std::clone::Clone
>
{
    doclines: Vec<STR>, 
    enumname: STR,
    variantnames: Vec<STR>,
    properties: Option<Vec<STR>>,
    data: Option<Vec<Vec<STR>>>,
}

 impl
 <
    STR: AsRef<str>  + 
    std::default::Default +
    std::clone::Clone
 > 
    EnumBlock<STR> 
 {
    pub fn new() -> Self {
        EnumBlock::default()
    }

    pub fn with(classname: STR, variants: &Vec<STR>) -> Self {
        EnumBlock { 
            doclines: vec![], 
            enumname: classname, 
            variantnames: variants.to_vec(), 
            properties: None, 
            data: None }
    }

    fn set_classname(&mut self, classname: STR) {
        self.enumname = classname;
    }

    fn set_variants(&mut self, variants: &Vec<STR>) {
        self.variantnames = variants.to_vec();
    }

    pub fn set_properties(&mut self, properties: Vec<STR>) {
        self.properties = Some(properties);
    }

    /// Data should have form: 
    /// 
    /// first vector: properties
    /// 
    /// second vector: variants
    pub fn set_data(&mut self, data: &Vec<Vec<STR>>) {
        self.data = Some(data.to_vec());
    }


 }