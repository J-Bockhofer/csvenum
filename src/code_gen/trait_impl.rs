
use crate::RTypeTrait;

use super::{EnumTable, TextBlock, codeblocks::MatchBlock};

/// will assume all other blocks have been generated for now
pub fn generate_impl_block(et: &EnumTable, with_var_fns: bool) -> TextBlock {
    let mut tb = TextBlock::new();

    let enumname = et.get_name();
    let props = et.get_properties();
    let variants = et.get_variants();
    let enumname_lc = enumname.to_ascii_lowercase();

    tb.add_line(format!("impl {}", enumname));
    tb.open_closure(true);
    // fn for get all,
    let fn_get_all = format!("pub fn get_all_variants() -> [Self; {}]", variants.len());
    tb.add_line_indented(fn_get_all);
    tb.open_closure(true);
    let linker = format!("{}_get_all_variants()", enumname_lc);
    tb.add_line_indented(linker);
    tb.close_closure(true);
    // fns for as+from_var
    if with_var_fns {
        let fn_as_var = String::from("pub fn as_variant_str(&self) -> &'static str");
        tb.add_line_indented(fn_as_var);
        tb.open_closure(true);
        let linker = format!("{}_as_variant_str(self)", enumname_lc);
        tb.add_line_indented(linker);
        tb.close_closure(true);    
        let fn_from_var = String::from("pub fn from_variant_str<T: AsRef<str>>(variantstr: T) -> Option<Self>");
        tb.add_line_indented(fn_from_var);
        tb.open_closure(true);
        let linker = format!("{}_from_variant_str(variantstr)", enumname_lc);
        tb.add_line_indented(linker);
        tb.close_closure(true); 
    }
    // fns for properties, will always be generated
    for col in 0..props.len() {
        let prop_name = &props[col];
        let col_type = &et.parsed_types[col];
        let no_ref_type = col_type.to_typestr_no_ref();
        let typeprefix = if no_ref_type == "str".to_string() {"&'static "} else {""};
        let prop_lc = prop_name.to_ascii_lowercase();
        tb.add_line_indented(format!("/// Function to convert from {} to {}", enumname, prop_name));
        let asfn_hdr = format!(
            "pub fn as_{}(&self) -> {}{}", prop_lc, typeprefix, no_ref_type
        );
        tb.add_line_indented(asfn_hdr);
        tb.open_closure(true);
        let linker = format!("{}_as_{}(self)", &enumname_lc, prop_lc);
        tb.add_line_indented(linker);
        tb.close_closure(true); 
        let fromfn_hdr = format!(
            "pub fn from_{}({}: {}) -> Option<Self>", prop_lc, prop_lc, col_type.to_typestr()
        );
        tb.add_line_indented(fromfn_hdr);
        tb.open_closure(true);
        let linker = format!("{}_from_{}({})", &enumname_lc, prop_lc, prop_lc);
        tb.add_line_indented(linker);
        tb.close_closure(true); 

    }    
    tb.close_closure(true);
    tb
}

pub fn generate_impl_fmt_display(et: &EnumTable) -> TextBlock {
    let mut tb = TextBlock::new();

    let enumname = et.get_name();
    let props = et.get_properties();
    let variants = et.get_variants();
    //let enumname_lc = enumname.to_ascii_lowercase();

    tb.add_line(format!("impl std::fmt::Display for {}", enumname));
    tb.open_closure(true);  
    tb.add_line_indented(format!("fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result ")); 

    tb.open_closure(true); 

    // create a matcher to print to associated values for self::variant

    let mut matchblk = MatchBlock::new("self".to_string(), true);
    //matchblk.set_indent_depth(tb.get_current_depth());

    for row in 0..variants.len() {
        let variant = &variants[row];
        let var_name = format!("Self::{}", variant);
        let mut val_str = String::new();
        for col in 0..props.len() {
            let prop = &props[col];
            let valstr = et.get_value_by_col_row(col, row).unwrap();
            val_str = format!("{}, {} = {} ", val_str, prop, valstr)
            //val_str += &valstr;
        } // writeln!(f, "{}", line)?;
        matchblk.add_arm(var_name, format!("writeln!(f, \"{{}}{}\", self.as_variant_str())?", val_str,))

    }
    tb.append_lines(matchblk.to_lines());
    tb.add_line_indented(String::from("Ok(())"));

    tb.close_closure(true); 
    tb.close_closure(true); 
    tb
}