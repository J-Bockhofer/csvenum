use super::TypeError;

/// Trait to simplify handling types 
pub trait RTypeTrait {
    /// Construct the RType from a string that contains type information i.e. " &str " 
    /// 
    /// -> will return RType::String(Reference::Naked, StringType::str)
    fn from_typestr<T: AsRef<str>>(typestr: T) -> Result<Self, TypeError> where Self: Sized;
    /// Conversions into type representation
    /// 
    /// Example input: `&'a str`
    /// 
    /// get normal representation `&'a str`
    fn to_typestr(&self) -> String;
    /// get representation without references or lifetimes `str`
    fn to_typestr_no_ref(&self) -> String;
    /// get representation without lifetime `&str`
    fn to_typestr_no_life(&self) -> String;
    /// collect lifetimes
    fn collect_lifetimes(&self, into: &mut Vec<String>);
    // to typestr const?
    /// Get nesting depth
    fn get_depth(&self, counter: usize) -> usize;
    /// Get nesting breadth
    fn get_breadth(&self, counter: usize) -> usize;

    // Declarations... TypeWrapper ? as Vector again? vector and tuple declaration in text as [value1, value2, value3] with vec! for vectors.
    /// 1. get if the type can be declared as `const`
    fn is_const(&self) -> bool;
    /// 2. Value is valid - assume an input of (2,2,[2,3,4]) -> Tuple(NumType,NumType,Array|Vec<NumType>) 
    /// 
    /// Array|Vec in this case means that [2,3,4] is a valid declaration in text for array and vec tho vec can also have vec! prefixed
    fn value_is_valid(&self, valuestr: &str) -> bool;

    fn wrap_valuestr(&self, valuestr: &str) -> String;
    // we need:
    
    // without lifetime &str
    // without reference
    // const implementation (enum again?) -> const static oncelock hashmap?!

    // what we want to represent
    // const declaration
    // normal let
    // let when const is declared
    // match self?! -> is self match in regex - is_match(haystack: &str) -> Self match haystack {haystack if enum_re.is_match() => {}, etc..}
    // fn signature, return and argument ... lifetimes...
    /// Whether the type can be used as the key in a match expression, floats can not.
    fn can_match_as_key(&self) -> bool;

}
