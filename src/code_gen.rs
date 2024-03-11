
mod codeblocks;
use codeblocks::TextBlock;
mod enumdecl;

mod propfns;

mod utils;

mod trait_impl;

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


use self::enumdecl::{generate_enum_decl, generate_get_all_variants_fn, generate_variant_str_fns};

use self::propfns::generate_property_fns;

use self::trait_impl::{generate_impl_block, generate_impl_fmt_display};

use super::enumtable::EnumTable;



pub struct EnumModule {
    pub rootfile: String,

// 0. Imports ?
    /// imports will be whole line imports, this will do no formatting
    /// let importline = "use std::sync::OnceLock;"
    pub imports: Vec<String>,
// 1. Enum declaration with docs
    pub enumdeclaration: TextBlock,
// enumdecl
// 2. static vec for get all
    pub get_all_variants_fn: TextBlock,
    pub variants_as_str_module: TextBlock,
// enumimpl
// 3. const & static declares
// constdecl (movable)
// 4. as_prop from_prop functions
// fndecl (movable)
    pub propfn_blocks: Vec<(String, TextBlock)>,


    pub impl_block: TextBlock,

    pub fmt_block: TextBlock,
// 5. impl asMyEnum trait fn(&self) -> MyEnum
// traitimpl
// 6. impl MyEnum in 2. with as_prop link for pure implementations 
// 7. impl std::fmt::Display for MyEnum to display the enum variants with all properties
// fmtimpl
// 8. test: MyEnum do a barrel roll... convert between all representations
// testimpl
//

}

impl EnumModule {
    pub fn new(et: &EnumTable, rootfile: String) -> Self {
        EnumModule { 
            rootfile, 
            imports: vec![], 
            enumdeclaration: generate_enum_decl(&et), 
            get_all_variants_fn: generate_get_all_variants_fn(&et), 
            variants_as_str_module: generate_variant_str_fns(&et),
            propfn_blocks: generate_property_fns(&et),
            impl_block: generate_impl_block(&et),
            fmt_block: generate_impl_fmt_display(&et),
        
        }
    }
}

impl std::fmt::Display for EnumModule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        writeln!(f, "Main file: {}", self.rootfile)?;
        writeln!(f, "Imports: ")?;
        for imp in &self.imports {
            writeln!(f, "{}", imp)?;
        }
        write!(f,"{}", self.enumdeclaration)?;
        write!(f,"{}", self.get_all_variants_fn)?;
        write!(f,"{}", self.variants_as_str_module)?;
        for blk in &self.propfn_blocks {
            writeln!(f, "{}", blk.1)?;
        }
        write!(f,"{}", self.impl_block)?;
        write!(f,"{}", self.fmt_block)?;

        Ok(())
    }
} 



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_enummodule() {
        use crate::parser::{TableParser, ToEnumTable};
                                                        // if this is enum: MyEnum2 then the as_func will need to return Option<MyEnum2>
        let rows: Vec<&str> = vec![
            "TYPES,         &str,       (usize$f64),    &str",
            "MyEnumName,    Property1,  Property2,      Property3",
            "Variant1,      standard,   (0$ 3.14),      cheap",
            "Variant2,      medium,     (0$ 9.82),      pricey",
        ];
 
        let table_parser = TableParser::from_csv_lines(rows, "$").unwrap();
        let enumtable = table_parser.to_enumtable().unwrap();
        assert_eq!(enumtable.get_col_of_property("Property1"), Some(0));
        let rootfile = format!("{}.rs", enumtable.get_name().to_ascii_lowercase());

        let enummodule = EnumModule::new(&enumtable, rootfile);

        println!("{}", enummodule);

    }

}