
mod codeblocks;
pub use codeblocks::{TextBlock, MatchBlock};
mod enumdecl;

mod propfns;

mod utils;

mod trait_impl;

mod testblock;

use std::fs;

mod fragments;

mod gen_error;
use gen_error::GenError;

use super::reader::write_lines_to_file;

use self::enumdecl::{generate_enum_decl, generate_get_all_variants_fn, generate_variant_str_fns};

use self::propfns::generate_property_fns;

use self::testblock::generate_testblock;
use self::trait_impl::{generate_impl_block, generate_impl_fmt_display};

use super::{EnumTable, RTypeTrait};

use super::EnumOptions;


pub struct EnumModule<'a> {
    pub options: &'a EnumOptions,
    pub enumname: String,
    pub properties: Vec<String>,
// 0. Imports 
    pub imports: Vec<String>,
// 1. Enum declaration with docs
    pub enumdeclaration: TextBlock,
// 2. static vec for get all
    pub get_all_variants_fn: TextBlock,
// 3. variant string representation
    pub variants_as_str_module: TextBlock,
// 4. const & static declares && as_prop from_prop functions
    pub propfn_blocks: Vec<(String, TextBlock)>,
// 5. impl MyEnum with as_prop link for pure implementations 
    pub impl_block: TextBlock,
// 6. impl std::fmt::Display for MyEnum to display the enum variants with all properties
    pub fmt_block: TextBlock,
// 7. test: MyEnum do a barrel roll... convert between all representations
    pub test_block: TextBlock,
}

impl <'a> EnumModule<'a> {
    pub fn new(et: &EnumTable, options: &'a EnumOptions) -> Self {
        let make_variant_str_fns = options.gen_variant_str_fns;
        let mut imports: Vec<String> = vec![];
        for types in &et.parsed_types {
            if types.to_typestr_no_ref() == "Regex" {
                imports.push("extern crate regex;".to_string());
                imports.push("use regex::Regex;".to_string());
                imports.push("use std::sync::OnceLock;".to_string());

                break;
            }
        }


        EnumModule { 
            options,
            //rootfile, 
            enumname: et.get_name().to_string(),
            properties: et.get_properties().to_vec(),
            imports, 
            enumdeclaration: generate_enum_decl(&et), 
            get_all_variants_fn: generate_get_all_variants_fn(&et), 
            variants_as_str_module: if make_variant_str_fns {generate_variant_str_fns(&et)} else {TextBlock::new()},
            propfn_blocks: generate_property_fns(&et),
            impl_block:  if options.gen_impl_links {generate_impl_block(&et, make_variant_str_fns)} else {TextBlock::new()},
            fmt_block: generate_impl_fmt_display(&et),
            test_block: generate_testblock(&et),  
        }
    }
    /// for printing to a singular file
    pub fn to_lines(&self) -> Vec<String> {
        let mut lines = vec![];
        

        for imp in &self.imports {
           lines.push(imp.to_string());
        }
        self.enumdeclaration.collect_lines_into(&mut lines);
        self.get_all_variants_fn.collect_lines_into(&mut lines);
        self.variants_as_str_module.collect_lines_into(&mut lines);

        for blk in &self.propfn_blocks {
            blk.1.collect_lines_into(&mut lines);
        }
        self.impl_block.collect_lines_into(&mut lines);
        self.fmt_block.collect_lines_into(&mut lines);
        self.test_block.collect_lines_into(&mut lines);     
        lines
    }

    pub fn print_configured_to_file(&mut self) -> Result<(), Box<dyn std::error::Error>> {

        // construct root filename
        let filepath = &self.options.path_to_outfile;
        let enum_lc = self.enumname.to_ascii_lowercase();
        let parentpath = if filepath.is_file() {filepath.parent().unwrap().to_path_buf()} else {filepath.to_owned()};

        let file_name = format!("{}.rs",enum_lc); 
        let printfile = parentpath.join(file_name);
        let mainfile = printfile.to_str().unwrap();
        // will the functions be split into separate files?
        if self.options.split_files {

            // Create a new PathBuf for the nested directory
            let mut nested_path = parentpath.clone();
            nested_path.push(enum_lc);

            // Create the nested directory if it doesn't exist
            if !nested_path.exists() {
                fs::create_dir_all(&nested_path)?;
            }


            for prop_block in &self.propfn_blocks {
                let prop = &prop_block.0;
                let prop_lc = prop.to_ascii_lowercase();
                let prop_file = format!("{}.rs", prop_lc);
                self.imports.push(format!("mod {};", prop_lc));
                self.imports.push(format!("use {}::*;", prop_lc));
                let prop_file = nested_path.join(prop_file);
                let prop_file = prop_file.to_str().unwrap();

                let mut lines = vec![format!("use super::{};", &self.enumname)];
                prop_block.1.collect_lines_into(&mut lines);
                // write to file
                write_lines_to_file(prop_file, lines)?;
            }
            self.propfn_blocks = vec![];

            if self.options.gen_variant_str_fns {
                let var_file = format!("variantstr.rs");
                self.imports.push(format!("mod variantstr;"));
                self.imports.push(format!("use variantstr::*;"));
                let var_file = nested_path.join(var_file);
                let var_file = var_file.to_str().unwrap();
                let mut lines = vec![format!("use super::{};", &self.enumname)];
                self.variants_as_str_module.collect_lines_into(&mut lines);
                write_lines_to_file(var_file, lines)?;
                self.variants_as_str_module = TextBlock::new();
            }
        }
        
        let mut full_lines: Vec<String> = self.imports.clone();
        self.enumdeclaration.collect_lines_into(&mut full_lines);

        if self.options.gen_impl_links {
            self.impl_block.collect_lines_into(&mut full_lines);
        }

        if self.options.gen_variant_str_fns {
            self.fmt_block.collect_lines_into(&mut full_lines);
        }

        self.get_all_variants_fn.collect_lines_into(&mut full_lines);
        self.variants_as_str_module.collect_lines_into(&mut full_lines);
        for blk in &self.propfn_blocks {
            blk.1.collect_lines_into(&mut full_lines);
        }

        if self.options.gen_impl_links {
            self.test_block.collect_lines_into(&mut full_lines);  
        }
        write_lines_to_file(mainfile, full_lines)?;

        println!("Successfully generated @ {}", mainfile);
        Ok(())
    }

}

impl <'a> std::fmt::Display for EnumModule<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        writeln!(f, "Main file: {}", self.enumname)?;
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
        write!(f,"{}", self.test_block)?;
        Ok(())
    }
} 



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_enummodule() {
        use crate::parser::TableParser;
                                                        // if this is enum: MyEnum2 then the as_func will need to return Option<MyEnum2>
        let rows: Vec<&str> = vec![
            "TYPES,         &str,       (usize,f64),    &str",
            "MyEnumName,    Property1,  Property2,      Property3",
            "Variant1,      standard,   (0, 3.14),      cheap",
            "Variant2,      medium,     (0, 9.82),      pricey",
        ];
 
        let table_parser = TableParser::from_csv_lines(rows).unwrap();
        let enumtable = table_parser.to_enumtable().unwrap();
        assert_eq!(enumtable.get_col_of_property("Property1"), Some(0));

        let options = EnumOptions::default();

        let enummodule = EnumModule::new(&enumtable, &options);

        println!("{}", enummodule);

    }

}