use super::{EnumTable, TextBlock};

pub fn generate_testblock(et: &EnumTable) -> TextBlock {
    let mut tb = TextBlock::new();

    let enumname = et.get_name();
    let props = et.get_properties();
    let vars = et.get_variants();
    let enumname_lc = enumname.to_ascii_lowercase();


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

    for prop in props {
        let prop_lc = prop.to_ascii_lowercase();
        // recreate enumbound function
        tb.add_line_indented(
        format!("let result = {}.as_{}();", enumname_lc, prop_lc)
        );
        tb.add_line_indented(
            format!("let result = {}::from_{}(result).unwrap();", enumname, prop_lc)
        );
    }
    tb.add_line_indented(
        format!("assert_eq!({}, result);", enumname_lc)
    );
    tb.close_closure(true);
    tb.close_closure(true);
    tb
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_testblock() {

    }
}