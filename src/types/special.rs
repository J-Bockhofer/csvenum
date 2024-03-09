use super::RTypeTrait;
/// Special one will not contain anything but will be more difficult to turn into code / have different behaviour
#[derive(Debug, Eq, PartialEq)]
pub enum SpecialType {
    // the enum variant will contain the name.. or to separate it be make a named RType, which will contain building functions, expressions and declarations 
    Enum(String), // no just use the trait and store name here
    Regex,
}

impl RTypeTrait for SpecialType {
    fn from_typestr<T: AsRef<str>>(typestr: T) -> Result<Self, super::TypeError> where Self: Sized {
        use super::TypeError;
        Err(TypeError::SPECIALTYPEPARSE)
    }
    fn to_typestr(&self) -> String {
        match self {
            Self::Enum(x) => {x.to_owned()},
            Self::Regex => {todo!("Regex not impl yet")},
        }
    }
    fn to_typestr_no_ref(&self) -> String {
        self.to_typestr()
    }
    fn to_typestr_no_life(&self) -> String {
        self.to_typestr()
    }
    fn collect_lifetimes(&self, into: &mut Vec<String>) {
        // We dont have any life times here
    }
}