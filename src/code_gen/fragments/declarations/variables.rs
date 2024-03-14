
use std::{borrow::Cow, fmt};
use super::{GenError, Visibility};


#[derive(Eq, PartialEq)]
pub enum VarDeclareAs {
    LET,
    CONST,
    STATIC,
}

impl fmt::Display for VarDeclareAs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LET => write!(f, "let")?,
            Self::CONST => write!(f, "const")?,
            Self::STATIC => write!(f, "static")?,
        }
        Ok(())
    }
} 

// HashMap of vardecl per value in table? key as (col, row), value as VarDecl?

/// Does no validation of types and values, only internal validation, i.e. `mut const` would be invalid
/// 
/// This is to avoid double parsing, because determining if values can be const or static in the first place and whether to wrap them with a OnceLock will have to be handled elsewhere.
pub struct VarDeclaration<'a> {
    pub vis: Visibility,
    pub var_as: VarDeclareAs,
    pub is_mut: bool,
    pub var_name: Cow<'a, str>,
    pub var_type: Cow<'a, str>,
    // eq
    // no coupling with RType, this should sit after any type checks just to easily format the output and retrieve parts of it for reuse
    // should already be wrapped, i dont want to the wrapping / const or static decision to occur this deep down in this chain, this struct should just make it easier to define variable and reuse variable names
    pub var_eq: Cow<'a, str>,
    //;
}

impl<'a> VarDeclaration<'a> {
    /// Will be initialized empty with private visbility and as let statement
    pub fn new() -> Self {
        VarDeclaration { vis: Visibility::PRIV, var_as: VarDeclareAs::LET, is_mut: false, var_name: Cow::default(), var_type: Cow::default(), var_eq: Cow::default() }
    }
    pub fn from_args<
        T: Into<Cow<'a, str>>,
        R: Into<Cow<'a, str>>,
        K: Into<Cow<'a, str>>,
        >(var_name: T, var_type: R, var_eq: K) -> Self {
        VarDeclaration { vis: Visibility::PRIV, var_as: VarDeclareAs::LET, 
            is_mut: false,
            var_name: var_name.into(), 
            var_type: var_type.into(), 
            var_eq: var_eq.into() }
    }
    fn as_const(mut self) -> Self {
        self.var_as = VarDeclareAs::CONST;
        self
    }
    fn as_static(mut self) -> Self {
        self.var_as = VarDeclareAs::STATIC;
        self
    }
    fn as_let(mut self) -> Self {
        self.var_as = VarDeclareAs::LET;
        self
    }
    fn as_pub(mut self) -> Self {
        self.vis = Visibility::PUB;
        self
    }
    fn as_priv(mut self) -> Self {
        self.vis = Visibility::PRIV;
        self        
    }
    fn as_mut(mut self) -> Self {
        self.is_mut = true;
        self        
    }
    fn as_immut(mut self) -> Self {
        self.is_mut = false;
        self        
    }
    fn with_name<T: Into<Cow<'a, str>>>(mut self, var_name: T) -> Self {
        self.var_name = var_name.into();
        self
    }
    fn with_type<T: Into<Cow<'a, str>>>(mut self, var_type: T) -> Self {
        self.var_type = var_type.into();
        self
    }
    fn with_value<T: Into<Cow<'a, str>>>(mut self, var_eq: T) -> Self {
        self.var_eq = var_eq.into();
        self
    }
    /// 
    fn to_line(&self) -> Result<String, GenError> {
        if self.var_name.is_empty() || self.var_eq.is_empty() {
            return Err(GenError::DeclarationMalformed(String::from("Missing variable name or values in declaration.")));
        }
        if self.is_mut && self.var_as == VarDeclareAs::CONST  {
            return Err(GenError::DeclarationMalformed(String::from("A const variable was declared as mutable.")));
        }
        if self.vis == Visibility::PUB && self.var_as == VarDeclareAs::LET {
            return Err(GenError::DeclarationMalformed(String::from("A variable with the keyword `let` was declared as public.")));
        }
        if self.var_type.is_empty() && self.var_as != VarDeclareAs::LET {
            return Err(GenError::DeclarationMalformed(String::from("A const or static variable was not assigned a type.")));
        }
        let type_hint = if self.var_type.is_empty() {String::new()} else {format!(": {}", self.var_type)}; 
        let vis = if self.vis == Visibility::PRIV {String::new()} else {format!("{} ", self.vis)};

        Ok(format!(
            "{}{} {}{} = {}", vis, self.var_as, self.var_name, type_hint, self.var_eq
        ))
    }

}


#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_declarations()
    {
        let declaration = VarDeclaration::from_args("ALLCHUTNIC", "&str", "\"leandro\"").as_const().to_line().unwrap();
        let expected = format!("const ALLCHUTNIC: &str = \"leandro\"");
        assert_eq!(expected, declaration);
    }
}

