use crate::RTypeTrait;

use super::{EnumTable, TextBlock, codeblocks::MatchBlock};

pub fn generate_testblock(et: &EnumTable) -> TextBlock {
    let mut tb = TextBlock::new();

    let enumname = et.get_name();
    let props = et.get_properties();
    let vars = et.get_variants();
    let enumname_lc = enumname.to_ascii_lowercase();
    let col_has_dup_vec = et.get_col_has_duplicate_vec();
    let num_cols = props.len();

    if num_cols == 1 {
        let single_type = &et.parsed_types[0];
        let is_regex = if single_type.to_typestr_no_ref().contains("Regex") {true} else {false};
        if is_regex {
            // if our only type is a regex we dont return a test block
            return TextBlock::new();
        }
    }

    tb.add_line(String::from(
        "#[cfg(test)]"
    ));
    tb.add_line(String::from(
        "mod tests"
    ));
    tb.open_closure(true);
    tb.add_line_indented(String::from(
        "use super::*;"
    ));    
    tb.add_line(String::new());
    tb.add_line_indented(String::from(
        "#[test]"
    ));
    tb.add_line_indented(format!(
        "fn test_{}_do_a_barrel_roll()", enumname_lc 
    ));
    tb.open_closure(true);
    let variant = &vars[0];
    let var_name = format!("{}::{}", enumname, variant);

    tb.add_line_indented(format!(
        "let {} = {};", enumname_lc, var_name 
    ));

    let mut first_iter = true;
    let mut col: usize = 0;
    for prop in props {
        let prop_lc = prop.to_ascii_lowercase();
        let col_type = et.get_col_of_property(prop).unwrap();
        let col_type = &et.parsed_types[col_type];
        let res_or_enumlc = if first_iter {&enumname_lc} else {"result"};
        let col_can_match_as_key = col_type.can_match_as_key();
        // recreate enumbound functions
        let is_regex = if col_type.to_typestr_no_ref().contains("Regex") {true} else {false};
        if is_regex { 
            // No test implemented for regex
            

        } else {
            if col_can_match_as_key && !col_has_dup_vec[col] {
                first_iter = false;
    
                
                    tb.add_line_indented(
                        format!("let result = {}.as_{}();", res_or_enumlc, prop_lc)
                        );
                    tb.add_line_indented(
                        format!("let result = {}::from_{}(result).unwrap();", enumname, prop_lc)
                    );
    
            } else if col_has_dup_vec[col] && col_can_match_as_key {
                //first_iter = false;
                tb.add_line_indented(
                    format!("let value = {}.as_{}();", res_or_enumlc, prop_lc)
                    );
                tb.add_line_indented(
                    format!("let vresult = {}::from_{}(value);", enumname, prop_lc)
                );
                tb.add_line_indented(
                    format!("let result: Vec<&{}> = vresult.iter().filter_map(|x| {{", enumname)
                );            
                tb.open_closure(false);
                let mut minimatch = MatchBlock::new("x".to_string(), false);
                minimatch.add_arm(format!("{}", var_name), "Some(x)".to_string());
                minimatch.add_arm("_".to_string(), "None".to_string());
    
                tb.append_lines(minimatch.to_lines());
                tb.close_closure(true);
                tb.add_line_indented(String::from(").collect();"));      
                tb.add_line_indented(
                    String::from("let result = result[0].clone();")
                );     
                tb.add_line_indented(
                    format!("assert_eq!(value, result.as_{}());", prop_lc)
                );
    
            }
        }
        col += 1;

    }
    tb.add_line_indented(
        format!("assert_eq!({}, result);", enumname_lc)
    );
    tb.close_closure(true);
    tb.close_closure(true);
    tb
}

