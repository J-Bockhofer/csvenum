use crate::{RTypeTrait};

use super::{EnumTable, TextBlock, codeblocks::MatchBlock};


/// Tuple of property and consts + fns, takes for granted that all passed values are of constant type
pub fn generate_property_fns(et: &EnumTable) -> Vec<(String, TextBlock)> {
    let mut propertyblocks = vec![];

    let enumname = et.get_name();
    let props = et.get_properties();
    let vars = et.get_variants();
    let enumname_lc = enumname.to_ascii_lowercase();
    let col_has_dup_vec = et.get_col_has_duplicate_vec();
    let dup_map = et.get_duplicate_value_map();

    //let mut const_names: Vec<String> = vec![];


    for col in 0..props.len() {
        let prop_name = &props[col];
        let col_type = &et.parsed_types[col];
        let no_ref_type = col_type.to_typestr_no_ref();
        let col_is_regex = &no_ref_type == "Regex";
        let typeprefix = if &no_ref_type == "str" || col_is_regex {"&"} else {""};
        let prop_lc = prop_name.to_ascii_lowercase();

        let col_typestr = format!("{}{}", typeprefix, no_ref_type);

        let col_has_dup = col_has_dup_vec[col];
        let col_dup_map = &dup_map[col];

        let mut astb = TextBlock::new();
        let mut fromtb = TextBlock::new();
    
        let fn_const = if col_is_regex {""} else {"const "};

        astb.add_line(format!("/// Function to convert from {} to {}", enumname, prop_name));
        let asfn_hdr = format!(
            "pub {}fn {}_as_{}({}: &{}) -> {}", fn_const, &enumname_lc, &prop_lc, enumname_lc, enumname, col_typestr
        );
        astb.add_line(asfn_hdr);
        astb.open_closure(true);

        fromtb.add_line(format!("/// Function to convert from {} to {}", prop_name, enumname));
        let mut asmatchb = MatchBlock::new(&enumname_lc, true);



            // if col is regex the from function should not take a regex, that makes no sense, it should take a string and return the type that matches the associated regex
            // i do want a third function that checks if a string is valid for a given variant, this is a short one and goes: fn(&self, &str) -> true self.as_regex.is_match(&str)
        let fromfn_hdr = if col_is_regex {
            format!(
            "pub fn {}_from_{}_is_match(haystack: &str) -> Option<{}>", enumname_lc, &prop_lc, enumname)          
        } else {
            if col_has_dup {
                format!(
                    "pub fn {}_from_{}<'a>({}: {}) -> &'a [{}]", enumname_lc, &prop_lc, &prop_lc, col_typestr, enumname)  
            } else {
                format!(
                    "pub fn {}_from_{}({}: {}) -> Option<{}>", enumname_lc, &prop_lc, &prop_lc, col_typestr, enumname)  
            }
 
        } ;
        fromtb.add_line(fromfn_hdr);
        fromtb.open_closure(true);

        let mut frommatchb = MatchBlock::new(&prop_lc, true);

        let mut pblock = TextBlock::new();
        pblock.add_line(String::new());
        pblock.add_line(format!(
            "// Constants for `{}`", prop_name
        ));
        if !col_has_dup {
            for row in 0..vars.len() {
                let var_name = &vars[row];
                let const_name = format!("{}_{}", prop_name.to_ascii_uppercase(), var_name.to_ascii_uppercase());
                let valstr = &et.get_value_by_col_row(col, row).unwrap();
                let wr_val = col_type.wrap_valuestr(valstr);
                let variant_name = format!("{}::{}", enumname, var_name);
                let typeprefix = if &no_ref_type == "str" {"&'static "} else {""};
    
    
                // Constant declaration
                if col_is_regex {
                    pblock.add_line(format!(
                        "const {}: &'static str = {};", const_name.clone()+"_REGEX_STR", wr_val));                     
                    pblock.add_line(format!(
                        "static {}: OnceLock<{}> = OnceLock::new();", const_name.clone()+"_REGEX",no_ref_type ));                   
                } else {
                    pblock.add_line(format!(
                        "const {}: {}{} = {};", const_name, typeprefix, no_ref_type, wr_val));
                };
                if !col_is_regex {
                    asmatchb.add_arm(variant_name.clone(), const_name.clone());
                    frommatchb.add_arm(const_name, format!("Some({})", &variant_name));
                } else {
                    let re_getter = format!(
                        "{}.get_or_init(||Regex::new({}).expect(\"Failed to compile regex for property: {}, variant: {}\"))", 
                        const_name.clone()+"_REGEX", 
                        const_name.clone()+"_REGEX_STR", 
                        prop_name, 
                        var_name
                    );
                    asmatchb.add_arm(variant_name.clone(), re_getter.clone());

/*                     fromtb.add_line(format!("let variants = {}_get_all_variants();",enumname_lc));
                    let re_matcher = re_getter + "is" */

                }

    
                //const_names.push(const_name);
            }
            if no_ref_type != "bool" {
                frommatchb.add_arm("_", "Option::None");
            }
        } else {
            // we need to iterate over duplicate values
            let mut grp_cnt = 0;
            for val_grp in col_dup_map {
                let num_in_group = val_grp.1.len();
                let const_doc = format!("/// Group of {} variants with value: `{}`", num_in_group, val_grp.0);
                let const_name = format!("{}_VALUE_GRP_{}", prop_name.to_ascii_uppercase(), grp_cnt);
                let const_type = format!("[{}; {}]", enumname, num_in_group);
                let mut collected_variants = String::from("[");
                for variant in &val_grp.1 {
                    let variant_name = format!("{}::{}, ", enumname, variant);
                    collected_variants.push_str(&variant_name);
                }
                collected_variants.push_str("];");
                let const_decl = format!("const {}: &'static {} = &{}", const_name, const_type, collected_variants);
                pblock.add_line(const_doc);
                pblock.add_line(const_decl);
                frommatchb.add_arm(col_type.wrap_valuestr(&val_grp.0), &const_name) ;


                grp_cnt += 1;

            }
            if col_type.to_typestr_no_ref() != "bool" {
                frommatchb.add_arm("_", "&[]");
            }
            for row in 0..vars.len() {
                let var_name = &vars[row];
                let valstr = &et.get_value_by_col_row(col, row).unwrap();
                let wr_val = col_type.wrap_valuestr(valstr);
                let variant_name = format!("{}::{}", enumname, var_name);
                asmatchb.add_arm(variant_name, wr_val);
            }

        }




        let as_lines = asmatchb.to_lines();
        astb.append_lines(as_lines);

        if !col_is_regex {
            fromtb.append_lines(frommatchb.to_lines());
        } else {
            fromtb.add_line_indented(format!("let variants = {}_get_all_variants();",enumname_lc));
            fromtb.add_line_indented("for variant in variants {");
            fromtb.open_closure(false);
            fromtb.add_line_indented(format!(
                "if {}_as_{}(&variant).is_match(haystack) {{return Some(variant.clone())}}", &enumname_lc, prop_lc
            ));
            fromtb.close_closure(true);
            fromtb.add_line_indented("None");
        }
        

        

        fromtb.close_closure(true);
        astb.close_closure(true);
        
        pblock.add_line(String::from(""));
        astb.collect_lines_into(&mut pblock.lines);
        pblock.add_line(String::new());
        
        if col_type.can_match_as_key() {
            fromtb.collect_lines_into(&mut pblock.lines);
        }

        propertyblocks.push((prop_name.to_owned(), pblock));
    }




    propertyblocks
}