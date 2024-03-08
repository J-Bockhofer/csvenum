use super::RType;
// types that are parsed from some text content and will be turned into code, will include helper functions
/// Containers with single type T
/// 
/// Hash Map would need a value, but it would be possible to make a default hashmap constructor function from property1 K to property2 V
pub enum ContainerType {
    Vector(Box<RType>),
    Tuple(Vec<Box<RType>>),
    Array(Box<RType>, usize),
    //HashSet(Box<RType>),
    //HashMap(Box<RType>, Box<RType>),

}