use std::{borrow::Cow, fmt};
use super::{GenError, Visibility};

pub struct FnSignature<'a> {
    pub vis: Visibility,
    pub handle: Cow<'a, str>,
    pub generics: Cow<'a, str>,
    pub arg_str: Cow<'a, str>,
    pub return_type: Cow<'a, str>,

}
