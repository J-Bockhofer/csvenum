
/// Represents a reference `&` and lifetime `<'a>`
#[derive(Debug, Eq, PartialEq)]
pub enum Reference {
    WithLifetime(String),
    Naked,
    None,
}

impl std::fmt::Display for Reference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let refstr = match self {
            Reference::None => {"".to_string()},
            Reference::Naked => {"&".to_string()},
            Reference::WithLifetime(x) => {format!("&{}",x)},
        };
        write!(f, "{}", refstr)
    }
}

impl Reference {
    pub fn to_refstr(&self) -> String {
        self.to_string()
    }
    pub fn get_lifetime(&self) -> String {
        match self {
            Self::WithLifetime(x) => {x.to_owned()}
            _ => {String::new()},
        }
    }
    pub fn has_lifetime(&self) -> bool {
        match self {
            Self::WithLifetime(_) => {true}
            _ => {false},
        }        
    }
    pub fn to_refstr_no_life(&self) -> String {
        match self {
            Self::None => {String::new()},
            _ => {String::from("&")},

        }
    }

}

