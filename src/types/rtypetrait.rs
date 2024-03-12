use super::TypeError;

pub trait RTypeTrait {
    /// Construct the RType from a string that contains type information i.e. " &str " 
    /// 
    /// -> will return RType::String(Reference::Naked, StringType::str)
    fn from_typestr<T: AsRef<str>>(typestr: T) -> Result<Self, TypeError> where Self: Sized;
    /// Construct RType from value representation i.e. "[[3,3],[3,3],[3,3]]" and self as the type hint bc this is getting messy
    /// 
    /// -> will return RType::Container(Reference::None, ContainerType::Array( 
    ///             Box::new(RType::Container(Reference::None, ContainerType::Array(
    ///                 Box::new(RType::Numeric(NumericType::usize)), 2)
    ///             ), 3))
    /// fn new_from_valuestr<T: AsRef<str>>(&self, valuestr: T) -> Result<Self, TypeError> where Self: Sized;
    /// Conversions into type representation
    /// 
    /// Example input: `&'a str`
    /// 
    /// 1. get normal representation `&'a str`
    fn to_typestr(&self) -> String;
    /// 2. get representation without references or lifetimes `str`
    fn to_typestr_no_ref(&self) -> String;
    /// 3. get representation without lifetime `&str`
    fn to_typestr_no_life(&self) -> String;
    /// 4. collect lifetimes
    fn collect_lifetimes(&self, into: &mut Vec<String>);
    // to typestr const?
    /// Get nesting depth
    /// String = depth 0
    /// Vec<String> = depth 1
    /// Vec<Vec<String>> = depth 2
    /// [String; 3] = depth 1
    /// [[String; 3]; 3] = depth 2
    /// (usize, usize) = depth 1
    /// (Vec<Vec<String>>, [String; 3]) = depth 3
    fn get_depth(&self, counter: usize) -> usize;
    /// Get nesting breadth - only for tuple breadth
    /// String = breadth 0
    /// Vec<String> = breadth ? 0 String
    /// Vec<Vec<String>> = breadth ? 0 Vec<String>
    /// [String; 3] = breadth 0
    /// [[String; 3]; 3] = breadth 0
    /// (usize, usize) = breadth 2
    /// (Vec<Vec<String>>, [String; 3]) = breadth 2
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
