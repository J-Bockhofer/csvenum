use crate::{RTypeTrait};

use super::{EnumTable, TextBlock, codeblocks::MatchBlock};

/// Tuple of property and consts + fns, takes for granted that all passed values are of constant type
pub fn generate_property_fns(et: &EnumTable) -> Vec<(String, TextBlock)> {
    let mut propertyblocks = vec![];

    let enumname = et.get_name();
    let props = et.get_properties();
    let vars = et.get_variants();
    let enumname_lc = enumname.to_ascii_lowercase();

    //let mut const_names: Vec<String> = vec![];


    for col in 0..props.len() {
        let prop_name = &props[col];
        let col_type = &et.parsed_types[col];
        let no_ref_type = col_type.to_typestr_no_ref();
        let typeprefix = if no_ref_type == "str".to_string() {"&'static "} else {""};
        let prop_lc = prop_name.to_ascii_lowercase();

        let mut astb = TextBlock::new();
        let mut fromtb = TextBlock::new();
    
        astb.add_line(format!("/// Function to convert from {} to {}", enumname, prop_name));
        let asfn_hdr = format!(
            "pub fn {}_as_{}({}: &{}) -> {}{}", &enumname_lc, prop_lc, enumname_lc, enumname, typeprefix, no_ref_type
        );
        astb.add_line(asfn_hdr);
        astb.open_closure(true);

        fromtb.add_line(format!("/// Function to convert from {} to {}", prop_name, enumname));
        let mut asmatchb = MatchBlock::new(enumname_lc.to_string(), true);

        let fromfn_hdr = format!(
            "pub fn {}_from_{}({}: {}) -> Option<{}>", enumname_lc, prop_lc, prop_lc, col_type.to_typestr(), enumname
        );
        fromtb.add_line(fromfn_hdr);
        fromtb.open_closure(true);

        let mut frommatchb = MatchBlock::new(prop_lc, true);

        let mut pblock = TextBlock::new();
        pblock.add_line(format!(
            "/// Constants for `{}`", prop_name
        ));

        for row in 0..vars.len() {
            let var_name = &vars[row];
            let const_name = format!("{}_{}", prop_name.to_ascii_uppercase(), var_name.to_ascii_uppercase());
            let valstr = &et.get_value_by_col_row(col, row).unwrap();
            let wr_val = col_type.wrap_valuestr(valstr);
            let variant_name = format!("{}::{}", enumname, var_name);



            let const_decl = format!(
                "const {}: {}{} = {};", const_name, typeprefix, no_ref_type, wr_val
            );
            pblock.add_line(const_decl);
            
            asmatchb.add_arm(variant_name.clone(), const_name.clone());
            frommatchb.add_arm(const_name, format!("Some({})", &variant_name));

            //const_names.push(const_name);
        }
        pblock.add_line(String::from(""));
        frommatchb.add_arm("_".to_string(), "Option::None".to_string());

        frommatchb.lines_into(&mut fromtb.lines);
        asmatchb.lines_into(&mut astb.lines);

        fromtb.close_closure(true);
        astb.close_closure(true);

        astb.collect_lines_into(&mut pblock.lines);
        pblock.add_line(String::new());
        fromtb.collect_lines_into(&mut pblock.lines);

        propertyblocks.push((prop_name.to_owned(), pblock));
    }




    propertyblocks
}