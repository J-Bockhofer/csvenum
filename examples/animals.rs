extern crate regex;
use regex::Regex;
use std::sync::OnceLock;
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Animals
{
    /// `Cat` <br>
    /// Id_regex: Regex = `^cat[\d]{1,4}` <br>
    Cat,
    
    /// `Dog` <br>
    /// Id_regex: Regex = `^dog[\d]{1,4}` <br>
    Dog,
    
    /// `Cow` <br>
    /// Id_regex: Regex = `^cow[\d]{1,4}` <br>
    Cow,
    
}
impl Animals
{
    pub fn get_all_variants<'a>() -> &'a [Self; 3]
    {
        animals_get_all_variants()
    }
    pub fn as_variant_str(&self) -> &str
    {
        animals_as_variant_str(self)
    }
    pub fn from_variant_str<T: AsRef<str>>(variantstr: T) -> Option<Self>
    {
        animals_from_variant_str(variantstr)
    }
    /// Function to convert from Animals to Id_regex
    pub fn as_id_regex(&self) -> &Regex
    {
        animals_as_id_regex(self)
    }
    pub fn from_id_regex_is_match(haystack: &str) -> Option<Self>
    {
        animals_from_id_regex_is_match(haystack)
    }
}
impl std::fmt::Display for Animals
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result 
    {
        match self {
            Self::Cat => writeln!(f, "{}, Id_regex = ^cat[\\d]{{1,4}} ", self.as_variant_str())?,
            Self::Dog => writeln!(f, "{}, Id_regex = ^dog[\\d]{{1,4}} ", self.as_variant_str())?,
            Self::Cow => writeln!(f, "{}, Id_regex = ^cow[\\d]{{1,4}} ", self.as_variant_str())?,
        }
        Ok(())
    }
}

const ANIMALS_ALL_VARIANTS_ARRAY: &'static [Animals; 3] = &[ 
    Animals::Cat,
    Animals::Dog,
    Animals::Cow,
];
pub fn animals_get_all_variants<'a>() -> &'a [Animals; 3]
{
    &ANIMALS_ALL_VARIANTS_ARRAY
}

// Variant string representation.
const ANIMALS_CAT_STR:&'static str = "Cat";
const ANIMALS_DOG_STR:&'static str = "Dog";
const ANIMALS_COW_STR:&'static str = "Cow";

/// Returns the variants name as a &str.
pub const fn animals_as_variant_str(animals: &Animals) -> &str
{
    match animals {
        Animals::Cat => ANIMALS_CAT_STR,
        Animals::Dog => ANIMALS_DOG_STR,
        Animals::Cow => ANIMALS_COW_STR,
    }
}

/// Returns the enum given a string that might represent the variant's name.
pub fn animals_from_variant_str<T: AsRef<str>>(variantstr: T) -> Option<Animals>
{
    let variantstr = variantstr.as_ref();
    match variantstr {
        ANIMALS_CAT_STR => Some(Animals::Cat),
        ANIMALS_DOG_STR => Some(Animals::Dog),
        ANIMALS_COW_STR => Some(Animals::Cow),
        _ => Option::None,
    }
}

// Constants for `Id_regex`
const ID_REGEX_CAT_REGEX_STR: &'static str = r"^cat[\d]{1,4}";
static ID_REGEX_CAT_REGEX: OnceLock<Regex> = OnceLock::new();
const ID_REGEX_DOG_REGEX_STR: &'static str = r"^dog[\d]{1,4}";
static ID_REGEX_DOG_REGEX: OnceLock<Regex> = OnceLock::new();
const ID_REGEX_COW_REGEX_STR: &'static str = r"^cow[\d]{1,4}";
static ID_REGEX_COW_REGEX: OnceLock<Regex> = OnceLock::new();

/// Function to convert from Animals to Id_regex
pub fn animals_as_id_regex(animals: &Animals) -> &Regex
{
    match animals {
        Animals::Cat => ID_REGEX_CAT_REGEX.get_or_init(||Regex::new(ID_REGEX_CAT_REGEX_STR).expect("Failed to compile regex for property: Id_regex, variant: Cat")),
        Animals::Dog => ID_REGEX_DOG_REGEX.get_or_init(||Regex::new(ID_REGEX_DOG_REGEX_STR).expect("Failed to compile regex for property: Id_regex, variant: Dog")),
        Animals::Cow => ID_REGEX_COW_REGEX.get_or_init(||Regex::new(ID_REGEX_COW_REGEX_STR).expect("Failed to compile regex for property: Id_regex, variant: Cow")),
    }
}

/// Function to convert from Id_regex to Animals
pub fn animals_from_id_regex_is_match(haystack: &str) -> Option<Animals>
{
    let variants = animals_get_all_variants();
    for variant in variants {
        if animals_as_id_regex(&variant).is_match(haystack) {return Some(variant.clone())}
    }
    None
}


// not generated, just here to provide a usage example 
fn main() {
    let an_animal_with_id = "cat1234";
    let animal = Animals::from_id_regex_is_match(an_animal_with_id).unwrap();
    assert_eq!(Animals::Cat, animal);

}