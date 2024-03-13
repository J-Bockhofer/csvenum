
use super::{EnumTable, TextBlock, codeblocks::MatchBlock};

// sits in root, is root, be root, tree root
pub fn generate_enum_decl(et: &EnumTable) -> TextBlock {
    let mut tb = TextBlock::new();
    let variants = et.get_variants();
    let properties = et.get_properties();

    tb.add_line(String::from(
        "#[derive(Debug, Eq, PartialEq, Clone)]"
    ));
    tb.add_line(format!("pub enum {}", et.get_name())); // {{
    tb.open_closure(true);     
    
    for row in 0..variants.len() {
        let variant = &variants[row];
        // Documentation
        tb.add_line_indented(
            format!("/// `{}` <br>", variant)
        );
        for col in 0..properties.len() {
            let val = et.get_value_by_col_row(col, row).unwrap();
            let line = format!(
                "/// {}: {} = `{}` <br>"
                , &properties[col], &et.parsed_types[col], val
            );            
            tb.add_line_indented(line);
        }
        tb.add_line_indented(
            format!("{},", variant)
        );        
        tb.add_line_indented( String::new());         
    }  
    tb.close_closure(true);
    tb
}

// sits in /enumname_utils.rs by default
pub fn generate_get_all_variants_fn(et: &EnumTable) -> TextBlock {
    // declare all variants as const array
    let mut tb = TextBlock::new();
    tb.add_line(String::new());
    let variants = et.get_variants();
    let enumname = et.get_name();

    let enumname_lc = enumname.to_ascii_lowercase();
    let const_varname = format!("{}_ALL_VARIANTS_ARRAY", enumname.to_ascii_uppercase());

    // declare the const array
    let arr_decl = format!("const {}: [{}; {}] = [ ", const_varname, enumname, variants.len());
    tb.open_closure(false);
    tb.add_line(arr_decl);
    for variant in variants { 
        tb.add_line_indented(format!(
            "{}::{},", enumname, variant
        ));
    }
    tb.add_line(String::from("];"));
    tb.close_closure(false);

    // declare the function
    let fn_decl = format!("pub const fn {}_get_all_variants() -> [{}; {}]", enumname_lc, enumname, variants.len());
    tb.add_line(fn_decl);
    tb.open_closure(true);
    tb.add_line_indented(String::from(const_varname));
    tb.close_closure(true);

    tb
}

// sits in /enumname_util.rs by default
pub fn generate_variant_str_fns(et: &EnumTable) -> TextBlock {
    let mut tb = TextBlock::new();
    let variants = et.get_variants();
    let enumname = et.get_name();
    let enumname_lc = enumname.to_ascii_lowercase();
    tb.add_line(String::new());
    tb.add_line(format!("// Variant string representation."));

    let mut const_names: Vec<String> = vec![];
    let mut var_names: Vec<String> = vec![];
    // make consts
    for variant in variants {
        // format var_name
        let var_name = format!("{}::{}", enumname, variant);
        let const_name = format!("{}_{}_STR", enumname.to_ascii_uppercase(), variant.to_ascii_uppercase());
        let const_decl = format!(
            "const {}:&'static str = \"{}\";"
            ,const_name, variant);
        tb.add_line(const_decl);
        const_names.push(const_name);
        var_names.push(var_name);
    }
    tb.add_line(String::new());

    // make matching functions
    tb.add_line(format!("/// Returns the variants name as a &str."));
    let fn_decl = format!("pub const fn {}_as_variant_str({}: &{}) -> &'static str", enumname_lc, enumname_lc, enumname);
    tb.add_line(fn_decl);
    tb.open_closure(true);
    let matchblock = MatchBlock::from_keys(enumname_lc.clone(), var_names.clone(), const_names.clone(), true);
    tb.append_lines(matchblock.to_lines());
    tb.close_closure(true);   

    tb.add_line(String::new());

    tb.add_line(format!("/// Returns the enum given a string that might represent the variant's name."));
    let fn_decl = format!("pub fn {}_from_variant_str<T: AsRef<str>>(variantstr: T) -> Option<{}>", enumname_lc, enumname);
    tb.add_line(fn_decl);
    tb.open_closure(true);
    tb.add_line_indented(String::from("let variantstr = variantstr.as_ref();"));
    let mut matchblock = MatchBlock::new("variantstr".to_string(), true);
    for i in 0..var_names.len() {
        matchblock.add_arm(const_names[i].clone(), format!("Some({})", var_names[i]));
    }
    matchblock.add_arm("_".to_string(), "Option::None".to_string());
    tb.append_lines(matchblock.to_lines());
    tb.close_closure(true);
    tb
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_enum_decl() {
        use crate::parser::TableParser;
 
        let rows: Vec<&str> = vec![
            "TYPES,         &str,       (usize,f64),    &str",
            "MyEnumName,    Property1,  Property2,      Property3",
            "Variant1,      standard,   (0, 3.14),      cheap",
            "Variant2,      medium,     (0, 9.82),      pricey",
        ];
 
        let table_parser = TableParser::from_csv_lines(rows).unwrap();
        let enumtable = table_parser.to_enumtable().unwrap();
        assert_eq!(enumtable.get_col_of_property("Property1"), Some(0));

        let enumdecl = generate_enum_decl(&enumtable);
        println!("{}", enumdecl);
        let get_all_fn = generate_get_all_variants_fn(&enumtable);
        println!("{}", get_all_fn);
        let varstr_fn = generate_variant_str_fns(&enumtable);
        println!("{}", varstr_fn);

    }

}
