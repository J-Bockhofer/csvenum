

#[allow(non_camel_case_types)]
pub enum NumericType {
    /// i8
    /// 
    /// Property `typestr` with value `i8` 
    /// 
    i8,
    /// i16
    /// 
    /// Property `typestr` with value `i16` 
    /// 
    i16,
    /// i32
    /// 
    /// Property `typestr` with value `i32` 
    /// 
    i32,
    /// i64
    /// 
    /// Property `typestr` with value `i64` 
    /// 
    i64,
    /// u8
    /// 
    /// Property `typestr` with value `u8` 
    /// 
    u8,
    /// u16
    /// 
    /// Property `typestr` with value `u16` 
    /// 
    u16,
    /// u32
    /// 
    /// Property `typestr` with value `u32` 
    /// 
    u32,
    /// u64
    /// 
    /// Property `typestr` with value `u64` 
    /// 
    u64,
    /// u128
    /// 
    /// Property `typestr` with value `u128` 
    /// 
    u128,
    /// f32
    /// 
    /// Property `typestr` with value `f32` 
    /// 
    f32,
    /// f64
    /// 
    /// Property `typestr` with value `f64` 
    /// 
    f64,
    /// isize
    /// 
    /// Property `typestr` with value `isize` 
    /// 
    isize,
    /// usize
    /// 
    /// Property `typestr` with value `usize` 
    /// 
    usize,
}
impl NumericType {
    pub fn get_all_variants() -> Vec<NumericType> {
        vec![
            NumericType::i8,
            NumericType::i16,
            NumericType::i32,
            NumericType::i64,
            NumericType::u8,
            NumericType::u16,
            NumericType::u32,
            NumericType::u64,
            NumericType::u128,
            NumericType::f32,
            NumericType::f64,
            NumericType::isize,
            NumericType::usize,
        ]
    }
}
/// Consts for Property `typestr` 
const I8_TYPESTR: &'static str = "i8";
const I16_TYPESTR: &'static str = "i16";
const I32_TYPESTR: &'static str = "i32";
const I64_TYPESTR: &'static str = "i64";
const U8_TYPESTR: &'static str = "u8";
const U16_TYPESTR: &'static str = "u16";
const U32_TYPESTR: &'static str = "u32";
const U64_TYPESTR: &'static str = "u64";
const U128_TYPESTR: &'static str = "u128";
const F32_TYPESTR: &'static str = "f32";
const F64_TYPESTR: &'static str = "f64";
const ISIZE_TYPESTR: &'static str = "isize";
const USIZE_TYPESTR: &'static str = "usize";
 
pub fn numeric_as_typestr(numeric: &NumericType) -> &str {
    match numeric {
        NumericType::i8 => I8_TYPESTR,
        NumericType::i16 => I16_TYPESTR,
        NumericType::i32 => I32_TYPESTR,
        NumericType::i64 => I64_TYPESTR,
        NumericType::u8 => U8_TYPESTR,
        NumericType::u16 => U16_TYPESTR,
        NumericType::u32 => U32_TYPESTR,
        NumericType::u64 => U64_TYPESTR,
        NumericType::u128 => U128_TYPESTR,
        NumericType::f32 => F32_TYPESTR,
        NumericType::f64 => F64_TYPESTR,
        NumericType::isize => ISIZE_TYPESTR,
        NumericType::usize => USIZE_TYPESTR,
    }
}
pub fn numeric_from_typestr(typestr: &str) -> Option<NumericType> {
    match typestr {
        I8_TYPESTR => Some(NumericType::i8),
        I16_TYPESTR => Some(NumericType::i16),
        I32_TYPESTR => Some(NumericType::i32),
        I64_TYPESTR => Some(NumericType::i64),
        U8_TYPESTR => Some(NumericType::u8),
        U16_TYPESTR => Some(NumericType::u16),
        U32_TYPESTR => Some(NumericType::u32),
        U64_TYPESTR => Some(NumericType::u64),
        U128_TYPESTR => Some(NumericType::u128),
        F32_TYPESTR => Some(NumericType::f32),
        F64_TYPESTR => Some(NumericType::f64),
        ISIZE_TYPESTR => Some(NumericType::isize),
        USIZE_TYPESTR => Some(NumericType::usize),
        _ => Option::None,
    }
}
