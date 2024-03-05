
use super::{TextBlock, EnumTable, indent_string};

pub fn format_enum(enumtable: &EnumTable) -> TextBlock {

    let indent:usize = 4;

    let mut lines: Vec<String> = vec![]; 
    let enumname = enumtable.get_enumname();
    let props = enumtable.get_properties();
    let num_props = props.len();
    let variants = enumtable.get_variants();

    // Header
    lines.push(format!("pub enum {} {{", enumname));

    for row in 0..variants.len() {
        let cvar= variants[row];
        // Documentation
        let l = format!("/// {}", cvar);
        lines.push(indent_string(l, indent)); 
        lines.push(indent_string(format!("/// "), indent));    
        for col in 1..num_props + 1 {
            let l = format!(
                "/// Property `{}` with value `{}` "
                , &props[col - 1], enumtable.data[col][row]
            );
            lines.push(indent_string(l, indent)); 
            lines.push(indent_string(format!("/// "), indent)); 
        }
        // Declaration
        let l = format!("{},", cvar);
        lines.push(
            indent_string(l, indent)
        );
    }

    // Footer
    lines.push(
        format!("}}")
    );

    TextBlock::with_lines(lines, super::BlockType::ENUM)
}
