/// Special one will not contain anything but will be more difficult to turn into code / have different behaviour
pub enum SpecialType {
    // the enum variant will contain the name.. or to separate it be make a named RType, which will contain building functions, expressions and declarations 
    Enum(String), // no just use the trait and store name here
    Regex,
}