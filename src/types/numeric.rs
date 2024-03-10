use super::{RTypeTrait, TypeError};

use std::sync::OnceLock;
use regex::Regex;


const NUMERIC_REGEX_STR: &'static str = r"^ ?(\d*)$"; 
static NUMERIC_REGEX: OnceLock<Regex> = OnceLock::new();

const FLOAT_REGEX_STR: &'static str = r"^ ?([\d\.]*)$"; 
static FLOAT_REGEX: OnceLock<Regex> = OnceLock::new();

#[allow(non_camel_case_types)]
#[derive(Debug, Eq, PartialEq)]
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

impl RTypeTrait for NumericType {
    fn from_typestr<T: AsRef<str>>(typestr: T) -> Result<Self, super::TypeError> where Self: Sized {
        let typestr = typestr.as_ref();
        match typestr {
            I8_TYPESTR => Ok(NumericType::i8),
            I16_TYPESTR => Ok(NumericType::i16),
            I32_TYPESTR => Ok(NumericType::i32),
            I64_TYPESTR => Ok(NumericType::i64),
            U8_TYPESTR => Ok(NumericType::u8),
            U16_TYPESTR => Ok(NumericType::u16),
            U32_TYPESTR => Ok(NumericType::u32),
            U64_TYPESTR => Ok(NumericType::u64),
            U128_TYPESTR => Ok(NumericType::u128),
            F32_TYPESTR => Ok(NumericType::f32),
            F64_TYPESTR => Ok(NumericType::f64),
            ISIZE_TYPESTR => Ok(NumericType::isize),
            USIZE_TYPESTR => Ok(NumericType::usize),
            _ => Err(TypeError::NumericTypeUnknown(typestr.to_string())),
        }        
    }
    fn to_typestr(&self) -> String {
        String::new() + match self 
        {
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
    fn to_typestr_no_ref(&self) -> String {
        self.to_typestr()
    }
    fn to_typestr_no_life(&self) -> String {
        self.to_typestr()
    }
    #[allow(unused_variables)]
    fn collect_lifetimes(&self, into: &mut Vec<String>) {
        // We dont have any life times here
    }
    fn is_const(&self) -> bool {
        true        
    }
    fn value_is_valid(&self, valuestr: &str) -> bool {
        let num_re = NUMERIC_REGEX.get_or_init(|| Regex::new(NUMERIC_REGEX_STR).unwrap());
        let float_re = FLOAT_REGEX.get_or_init(|| Regex::new(FLOAT_REGEX_STR).unwrap());
        match self {
            NumericType::f32 | NumericType::f64 => {if float_re.is_match(valuestr) {true} else {false}},
            _ => {if num_re.is_match(valuestr) {true} else {false}}
        }
        
    }
    fn get_depth(&self, counter: usize) -> usize {
        counter + 0
    }
    fn get_breadth(&self, counter: usize) -> usize {
        counter + 1
    }
}