use super::{TextBlock, EnumTable, VarType};

pub fn format_consts(enumtable: &EnumTable) -> TextBlock {
    // const VARIANT_PROPERTY: type = [variant][property] value

    let mut lines: Vec<String> = vec![];
    //let enumname = enumtable.get_enumname();
    let variants = enumtable.get_variants();
    let properties = enumtable.get_properties();
    let types = enumtable.get_types();

    //num_rows = variants.len();
    let num_cols = properties.len();

    for col in 1..num_cols + 1 {
        let type_str = types[col - 1];
        let prop_str = properties[col - 1];
        lines.push(format!("/// Consts for Property `{}` ", prop_str));
        let variant_values = &enumtable.data[col];
        // generate const declarations
        let property_lines = format_const(variants.clone(), prop_str, type_str, variant_values.to_vec());
        for line in property_lines {
            lines.push(line);
        }
        lines.push(String::from(" "));
    }


    TextBlock::with_lines(lines, super::BlockType::CONST)

}








pub fn format_const( variants: Vec<&str>, property: &str, type_str: &str, variant_values: Vec<String>) -> Vec<String> {
    // const VARIANT_PROPERTY: type = [variant][property] value
    let prop_str = property.to_ascii_uppercase();
    let var_type = VarType::from_type_str(type_str);

    let mut lines: Vec<String> = vec![];
    for i in 0..variants.len() {
        let var_str = variants[i].to_ascii_uppercase();
        let value = &variant_values[i];

        let typed_value = var_type.apply_wrapper(value.as_str());

        let line = format!
        (
            "const {}_{}: {} = {};", var_str , prop_str , var_type.const_type , typed_value 
        );
        lines.push(line);
    }

    lines
}