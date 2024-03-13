#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Country
{
    /// `AFG` <br>
    /// country_code: usize = `4` <br>
    /// name: &str = `Afghanistan` <br>
    /// alpha_2: &str = `AF` <br>
    AFG,
    
    /// `ALA` <br>
    /// country_code: usize = `248` <br>
    /// name: &str = `Ãland Islands` <br>
    /// alpha_2: &str = `AX` <br>
    ALA,
    
    /// `ALB` <br>
    /// country_code: usize = `8` <br>
    /// name: &str = `Albania` <br>
    /// alpha_2: &str = `AL` <br>
    ALB,
    
    /// `DZA` <br>
    /// country_code: usize = `12` <br>
    /// name: &str = `Algeria` <br>
    /// alpha_2: &str = `DZ` <br>
    DZA,
    
    /// `ASM` <br>
    /// country_code: usize = `16` <br>
    /// name: &str = `American Samoa` <br>
    /// alpha_2: &str = `AS` <br>
    ASM,
    
    /// `AND` <br>
    /// country_code: usize = `20` <br>
    /// name: &str = `Andorra` <br>
    /// alpha_2: &str = `AD` <br>
    AND,
    
    /// `AGO` <br>
    /// country_code: usize = `24` <br>
    /// name: &str = `Angola` <br>
    /// alpha_2: &str = `AO` <br>
    AGO,
    
    /// `AIA` <br>
    /// country_code: usize = `660` <br>
    /// name: &str = `Anguilla` <br>
    /// alpha_2: &str = `AI` <br>
    AIA,
    
    /// `ATA` <br>
    /// country_code: usize = `10` <br>
    /// name: &str = `Antarctica` <br>
    /// alpha_2: &str = `AQ` <br>
    ATA,
    
    /// `ATG` <br>
    /// country_code: usize = `28` <br>
    /// name: &str = `Antigua and Barbuda` <br>
    /// alpha_2: &str = `AG` <br>
    ATG,
    
    /// `ARG` <br>
    /// country_code: usize = `32` <br>
    /// name: &str = `Argentina` <br>
    /// alpha_2: &str = `AR` <br>
    ARG,
    
    /// `ARM` <br>
    /// country_code: usize = `51` <br>
    /// name: &str = `Armenia` <br>
    /// alpha_2: &str = `AM` <br>
    ARM,
    
}
impl Country
{
    pub fn get_all_variants() -> [Self; 12]
    {
        country_get_all_variants()
    }
    pub fn as_variant_str(&self) -> &'static str
    {
        country_as_variant_str(self)
    }
    pub fn from_variant_str<T: AsRef<str>>(variantstr: T) -> Option<Self>
    {
        country_from_variant_str(variantstr)
    }
    /// Function to convert from Country to country_code
    pub fn as_country_code(&self) -> usize
    {
        country_as_country_code(self)
    }
    pub fn from_country_code(country_code: usize) -> Option<Self>
    {
        country_from_country_code(country_code)
    }
    /// Function to convert from Country to name
    pub fn as_name(&self) -> &str
    {
        country_as_name(self)
    }
    pub fn from_name(name: &str) -> Option<Self>
    {
        country_from_name(name)
    }
    /// Function to convert from Country to alpha_2
    pub fn as_alpha_2(&self) -> &str
    {
        country_as_alpha_2(self)
    }
    pub fn from_alpha_2(alpha_2: &str) -> Option<Self>
    {
        country_from_alpha_2(alpha_2)
    }
}
impl std::fmt::Display for Country
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result 
    {
        match self {
            Self::AFG => writeln!(f, "{}, country_code = 4 , name = Afghanistan , alpha_2 = AF ", self.as_variant_str())?,
            Self::ALA => writeln!(f, "{}, country_code = 248 , name = Ãland Islands , alpha_2 = AX ", self.as_variant_str())?,
            Self::ALB => writeln!(f, "{}, country_code = 8 , name = Albania , alpha_2 = AL ", self.as_variant_str())?,
            Self::DZA => writeln!(f, "{}, country_code = 12 , name = Algeria , alpha_2 = DZ ", self.as_variant_str())?,
            Self::ASM => writeln!(f, "{}, country_code = 16 , name = American Samoa , alpha_2 = AS ", self.as_variant_str())?,
            Self::AND => writeln!(f, "{}, country_code = 20 , name = Andorra , alpha_2 = AD ", self.as_variant_str())?,
            Self::AGO => writeln!(f, "{}, country_code = 24 , name = Angola , alpha_2 = AO ", self.as_variant_str())?,
            Self::AIA => writeln!(f, "{}, country_code = 660 , name = Anguilla , alpha_2 = AI ", self.as_variant_str())?,
            Self::ATA => writeln!(f, "{}, country_code = 10 , name = Antarctica , alpha_2 = AQ ", self.as_variant_str())?,
            Self::ATG => writeln!(f, "{}, country_code = 28 , name = Antigua and Barbuda , alpha_2 = AG ", self.as_variant_str())?,
            Self::ARG => writeln!(f, "{}, country_code = 32 , name = Argentina , alpha_2 = AR ", self.as_variant_str())?,
            Self::ARM => writeln!(f, "{}, country_code = 51 , name = Armenia , alpha_2 = AM ", self.as_variant_str())?,
        }
        Ok(())
    }
}

const COUNTRY_ALL_VARIANTS_ARRAY: [Country; 12] = [ 
    Country::AFG,
    Country::ALA,
    Country::ALB,
    Country::DZA,
    Country::ASM,
    Country::AND,
    Country::AGO,
    Country::AIA,
    Country::ATA,
    Country::ATG,
    Country::ARG,
    Country::ARM,
];
pub const fn country_get_all_variants() -> [Country; 12]
{
    COUNTRY_ALL_VARIANTS_ARRAY
}

// Variant string representation.
const COUNTRY_AFG_STR:&'static str = "AFG";
const COUNTRY_ALA_STR:&'static str = "ALA";
const COUNTRY_ALB_STR:&'static str = "ALB";
const COUNTRY_DZA_STR:&'static str = "DZA";
const COUNTRY_ASM_STR:&'static str = "ASM";
const COUNTRY_AND_STR:&'static str = "AND";
const COUNTRY_AGO_STR:&'static str = "AGO";
const COUNTRY_AIA_STR:&'static str = "AIA";
const COUNTRY_ATA_STR:&'static str = "ATA";
const COUNTRY_ATG_STR:&'static str = "ATG";
const COUNTRY_ARG_STR:&'static str = "ARG";
const COUNTRY_ARM_STR:&'static str = "ARM";

/// Returns the variants name as a &str.
pub const fn country_as_variant_str(country: &Country) -> &'static str
{
    match country {
        Country::AFG => COUNTRY_AFG_STR,
        Country::ALA => COUNTRY_ALA_STR,
        Country::ALB => COUNTRY_ALB_STR,
        Country::DZA => COUNTRY_DZA_STR,
        Country::ASM => COUNTRY_ASM_STR,
        Country::AND => COUNTRY_AND_STR,
        Country::AGO => COUNTRY_AGO_STR,
        Country::AIA => COUNTRY_AIA_STR,
        Country::ATA => COUNTRY_ATA_STR,
        Country::ATG => COUNTRY_ATG_STR,
        Country::ARG => COUNTRY_ARG_STR,
        Country::ARM => COUNTRY_ARM_STR,
    }
}

/// Returns the enum given a string that might represent the variant's name.
pub fn country_from_variant_str<T: AsRef<str>>(variantstr: T) -> Option<Country>
{
    let variantstr = variantstr.as_ref();
    match variantstr {
        COUNTRY_AFG_STR => Some(Country::AFG),
        COUNTRY_ALA_STR => Some(Country::ALA),
        COUNTRY_ALB_STR => Some(Country::ALB),
        COUNTRY_DZA_STR => Some(Country::DZA),
        COUNTRY_ASM_STR => Some(Country::ASM),
        COUNTRY_AND_STR => Some(Country::AND),
        COUNTRY_AGO_STR => Some(Country::AGO),
        COUNTRY_AIA_STR => Some(Country::AIA),
        COUNTRY_ATA_STR => Some(Country::ATA),
        COUNTRY_ATG_STR => Some(Country::ATG),
        COUNTRY_ARG_STR => Some(Country::ARG),
        COUNTRY_ARM_STR => Some(Country::ARM),
        _ => Option::None,
    }
}

// Constants for `country_code`
const COUNTRY_CODE_AFG: usize = 4;
const COUNTRY_CODE_ALA: usize = 248;
const COUNTRY_CODE_ALB: usize = 8;
const COUNTRY_CODE_DZA: usize = 12;
const COUNTRY_CODE_ASM: usize = 16;
const COUNTRY_CODE_AND: usize = 20;
const COUNTRY_CODE_AGO: usize = 24;
const COUNTRY_CODE_AIA: usize = 660;
const COUNTRY_CODE_ATA: usize = 10;
const COUNTRY_CODE_ATG: usize = 28;
const COUNTRY_CODE_ARG: usize = 32;
const COUNTRY_CODE_ARM: usize = 51;

/// Function to convert from Country to country_code
pub const fn country_as_country_code(country: &Country) -> usize
{
    match country {
        Country::AFG => COUNTRY_CODE_AFG,
        Country::ALA => COUNTRY_CODE_ALA,
        Country::ALB => COUNTRY_CODE_ALB,
        Country::DZA => COUNTRY_CODE_DZA,
        Country::ASM => COUNTRY_CODE_ASM,
        Country::AND => COUNTRY_CODE_AND,
        Country::AGO => COUNTRY_CODE_AGO,
        Country::AIA => COUNTRY_CODE_AIA,
        Country::ATA => COUNTRY_CODE_ATA,
        Country::ATG => COUNTRY_CODE_ATG,
        Country::ARG => COUNTRY_CODE_ARG,
        Country::ARM => COUNTRY_CODE_ARM,
    }
}

/// Function to convert from country_code to Country
pub fn country_from_country_code(country_code: usize) -> Option<Country>
{
    match country_code {
        COUNTRY_CODE_AFG => Some(Country::AFG),
        COUNTRY_CODE_ALA => Some(Country::ALA),
        COUNTRY_CODE_ALB => Some(Country::ALB),
        COUNTRY_CODE_DZA => Some(Country::DZA),
        COUNTRY_CODE_ASM => Some(Country::ASM),
        COUNTRY_CODE_AND => Some(Country::AND),
        COUNTRY_CODE_AGO => Some(Country::AGO),
        COUNTRY_CODE_AIA => Some(Country::AIA),
        COUNTRY_CODE_ATA => Some(Country::ATA),
        COUNTRY_CODE_ATG => Some(Country::ATG),
        COUNTRY_CODE_ARG => Some(Country::ARG),
        COUNTRY_CODE_ARM => Some(Country::ARM),
        _ => Option::None,
    }
}

// Constants for `name`
const NAME_AFG: &'static str = "Afghanistan";
const NAME_ALA: &'static str = "Ãland Islands";
const NAME_ALB: &'static str = "Albania";
const NAME_DZA: &'static str = "Algeria";
const NAME_ASM: &'static str = "American Samoa";
const NAME_AND: &'static str = "Andorra";
const NAME_AGO: &'static str = "Angola";
const NAME_AIA: &'static str = "Anguilla";
const NAME_ATA: &'static str = "Antarctica";
const NAME_ATG: &'static str = "Antigua and Barbuda";
const NAME_ARG: &'static str = "Argentina";
const NAME_ARM: &'static str = "Armenia";

/// Function to convert from Country to name
pub const fn country_as_name(country: &Country) -> &str
{
    match country {
        Country::AFG => NAME_AFG,
        Country::ALA => NAME_ALA,
        Country::ALB => NAME_ALB,
        Country::DZA => NAME_DZA,
        Country::ASM => NAME_ASM,
        Country::AND => NAME_AND,
        Country::AGO => NAME_AGO,
        Country::AIA => NAME_AIA,
        Country::ATA => NAME_ATA,
        Country::ATG => NAME_ATG,
        Country::ARG => NAME_ARG,
        Country::ARM => NAME_ARM,
    }
}

/// Function to convert from name to Country
pub fn country_from_name(name: &str) -> Option<Country>
{
    match name {
        NAME_AFG => Some(Country::AFG),
        NAME_ALA => Some(Country::ALA),
        NAME_ALB => Some(Country::ALB),
        NAME_DZA => Some(Country::DZA),
        NAME_ASM => Some(Country::ASM),
        NAME_AND => Some(Country::AND),
        NAME_AGO => Some(Country::AGO),
        NAME_AIA => Some(Country::AIA),
        NAME_ATA => Some(Country::ATA),
        NAME_ATG => Some(Country::ATG),
        NAME_ARG => Some(Country::ARG),
        NAME_ARM => Some(Country::ARM),
        _ => Option::None,
    }
}

// Constants for `alpha_2`
const ALPHA_2_AFG: &'static str = "AF";
const ALPHA_2_ALA: &'static str = "AX";
const ALPHA_2_ALB: &'static str = "AL";
const ALPHA_2_DZA: &'static str = "DZ";
const ALPHA_2_ASM: &'static str = "AS";
const ALPHA_2_AND: &'static str = "AD";
const ALPHA_2_AGO: &'static str = "AO";
const ALPHA_2_AIA: &'static str = "AI";
const ALPHA_2_ATA: &'static str = "AQ";
const ALPHA_2_ATG: &'static str = "AG";
const ALPHA_2_ARG: &'static str = "AR";
const ALPHA_2_ARM: &'static str = "AM";

/// Function to convert from Country to alpha_2
pub const fn country_as_alpha_2(country: &Country) -> &str
{
    match country {
        Country::AFG => ALPHA_2_AFG,
        Country::ALA => ALPHA_2_ALA,
        Country::ALB => ALPHA_2_ALB,
        Country::DZA => ALPHA_2_DZA,
        Country::ASM => ALPHA_2_ASM,
        Country::AND => ALPHA_2_AND,
        Country::AGO => ALPHA_2_AGO,
        Country::AIA => ALPHA_2_AIA,
        Country::ATA => ALPHA_2_ATA,
        Country::ATG => ALPHA_2_ATG,
        Country::ARG => ALPHA_2_ARG,
        Country::ARM => ALPHA_2_ARM,
    }
}

/// Function to convert from alpha_2 to Country
pub fn country_from_alpha_2(alpha_2: &str) -> Option<Country>
{
    match alpha_2 {
        ALPHA_2_AFG => Some(Country::AFG),
        ALPHA_2_ALA => Some(Country::ALA),
        ALPHA_2_ALB => Some(Country::ALB),
        ALPHA_2_DZA => Some(Country::DZA),
        ALPHA_2_ASM => Some(Country::ASM),
        ALPHA_2_AND => Some(Country::AND),
        ALPHA_2_AGO => Some(Country::AGO),
        ALPHA_2_AIA => Some(Country::AIA),
        ALPHA_2_ATA => Some(Country::ATA),
        ALPHA_2_ATG => Some(Country::ATG),
        ALPHA_2_ARG => Some(Country::ARG),
        ALPHA_2_ARM => Some(Country::ARM),
        _ => Option::None,
    }
}
#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_country_do_a_barrel_roll()
    {
        let country = Country::AFG;
        let result = country.as_country_code();
        let result = Country::from_country_code(result).unwrap();
        let result = result.as_name();
        let result = Country::from_name(result).unwrap();
        let result = result.as_alpha_2();
        let result = Country::from_alpha_2(result).unwrap();
        assert_eq!(country, result);
    }
}

fn main() {
    
}