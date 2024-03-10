/// will need to convert enumtable into -> code
/// imports (need to import Regex and, or OnceLock)
/// Enum declaration
/// Enum impl
/// AsEnum trait
/// Variable names
/// 
/// 
/// If we opt to use hashmaps for lookup we need to set them in the as from methods instead of usual match arms
/// // Create a HashMap to map from country codes to enum variants
/// let mut code_to_country = HashMap::new();
/// code_to_country.insert("USA", Country::USA);
/// code_to_country.insert("CAN", Country::Canada);
/// code_to_country.insert("MEX", Country::Mexico);
/// 
/// // Create a HashMap to map from enum variants to country codes
/// let mut country_to_code = HashMap::new();
/// for (code, country) in &code_to_country {
///     country_to_code.insert(country, code);
/// }
/// 
pub struct SHUTUP {
    
}


pub struct EnumModule {
    pub rootdir: String,

    // Used Variants, Properties, Types and Values


    // Imports

    // EnumBlock

    // A number of ConstBlocks (per property)

    // Two functions (as & from) per property

    // A config (indenting, split files, functions as impl for Enum)

    // 

}

