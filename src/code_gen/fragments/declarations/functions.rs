use std::{borrow::Cow, fmt};
use super::{GenError, Visibility};

pub struct FnSignature<'a> {
    pub vis: Visibility,
    pub is_const: bool,
    pub handle: Cow<'a, str>,
    pub generics: Cow<'a, str>,
    pub arg_str: Cow<'a, str>,
    pub return_type: Cow<'a, str>,

}

impl<'a> FnSignature<'a> { 
    pub fn new() -> Self {
        FnSignature { 
            vis: Visibility::PRIV, 
            // can only be const if everything in it is available at compile time, i.e. const return type and limited to if and loop blocks, again i dont want those checks here
            is_const:false, 
            handle: Cow::default(), 
            generics: Cow::default(), 
            arg_str: Cow::default(), 
            return_type: Cow::default() } 
    }

    pub fn as_pub(mut self) -> Self {
        self.vis = Visibility::PUB;
        self
    }



}