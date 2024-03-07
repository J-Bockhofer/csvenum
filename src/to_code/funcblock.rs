

use super::{TextBlock, EnumTable, indent_string, VarType, MatchBlock};
 fn flatten_into(src: &Vec<String>, dest: &mut Vec<String>) {
    for item in src {
       dest.push(item.to_string());
    } 
}



#[derive(Debug)]
pub enum ConvType {
    AS,
    FROM,
}

#[derive(Debug, Clone, Default)]
pub struct ConvFunc {
    pub signature: String,
    pub matcher: MatchBlock,
}

impl ConvFunc {
    pub fn new() -> Self {
        ConvFunc::default()
    }

    pub fn make_signature(&mut self, conv_type: ConvType) {
        match conv_type {
            ConvType::AS => {

            },
            ConvType::FROM => {

            },
        }
    }
}

fn format_as_fn_sign(enumname: &str, property: &str, type_str: &str) -> String {
    let prop_lc = property.to_ascii_lowercase();
    let enum_lc = enumname.to_ascii_lowercase();
    format!("pub fn {enum_lc}_as_{prop_lc}({enum_lc}: &{enumname}) -> {type_str} {{")
}

fn format_from_fn_sign(enumname: &str, property: &str, type_str: &str) -> String {
    let prop_lc = property.to_ascii_lowercase();
    let enum_lc = enumname.to_ascii_lowercase();
    format!("pub fn {enum_lc}_from_{prop_lc}({prop_lc}: {type_str}) -> Option<{enumname}> {{") 
}

fn format_as_matchblock(enumname: &str, property: &str, variants: Vec<&str>, values: Vec<String>) -> MatchBlock {
    let num_arms = variants.len();
    if values.len() != num_arms {panic!("MatchBlock Error: Passed Variants and Values do not have the same length for property, {}", property)}

    let mut matchblock = MatchBlock::new(enumname.to_string().to_ascii_lowercase());
    for i in 0..num_arms {
        let var_str = variants[i].to_ascii_uppercase();
        let prop_str = property.to_ascii_uppercase();
        //let typed_value = var_type.apply_wrapper(&values[i]); // no we want the const here
        matchblock.add_arm(
            format!("{}::{}", enumname, variants[i]),
            format!("{}_{}", var_str, prop_str),
            )
    } 
    matchblock
}

fn format_from_matchblock(enumname: &str, property: &str, variants: Vec<&str>, values: Vec<String>) -> MatchBlock {
    let num_arms = variants.len();
    if values.len() != num_arms {panic!("MatchBlock Error: Passed Variants and Values do not have the same length for property, {}", property)}
    let prop_lc = property.to_ascii_lowercase();

    let mut matchblock = MatchBlock::new(prop_lc);
    for i in 0..num_arms {
        let var_str = variants[i].to_ascii_uppercase();
        let prop_str = property.to_ascii_uppercase();
        //let typed_value = var_type.apply_wrapper(&values[i]);
        matchblock.add_arm(
            format!("{}_{}", var_str, prop_str),
            format!("Some({}::{})", enumname, variants[i]),
            );
    } 
    matchblock.add_arm(
        "_".to_string(),
        "Option::None".to_string(),
        );  
    matchblock
}

fn format_fn_as_for_property(enumname: &str, property: &str, type_str: &str, variants: Vec<&str>, values:Vec<String>) -> TextBlock {
    let indent: usize = 4;
    // output will be
    // function signature 
    // indented match statement over property values

    let mut lines: Vec<String> = vec![];
    // 1. function signature
    let valid_type = VarType::from_type_str(type_str);

    let signt = format_as_fn_sign(enumname, property, &valid_type.input_type);
    lines.push(signt);

    // 2. indented match block
    let matchblock = format_as_matchblock(enumname, property, variants, values);
    flatten_into(&matchblock.to_code(indent, true), &mut lines);

    lines.push(format!("}}"));

    TextBlock::with_lines(lines, super::BlockType::FUNC) 
}

fn format_fn_from_for_property(enumname: &str, property: &str, type_str: &str, variants: Vec<&str>, values:Vec<String>) -> TextBlock {
    let indent: usize = 4;
    // output will be
    // function signature 
    // indented match statement over property values

    let mut lines: Vec<String> = vec![];
    // 1. function signature
    let valid_type = VarType::from_type_str(type_str);

    let signt = format_from_fn_sign(enumname, property, &valid_type.input_type);
    lines.push(signt);

    // 2. indented match block
    let matchblock = format_from_matchblock(enumname, property, variants, values);
    flatten_into(&matchblock.to_code(indent, true), &mut lines);

    lines.push(format!("}}"));

    TextBlock::with_lines(lines, super::BlockType::FUNC) 
}

fn make_function_block_for_property(enumname: &str, property: &str, type_str: &str, variants: Vec<&str>, values:Vec<String>) -> Vec<TextBlock> {
    let mut blocks: Vec<TextBlock> = vec![];
    blocks.push(format_fn_as_for_property(enumname, property, type_str, variants.clone(), values.clone()));
    blocks.push(format_fn_from_for_property(enumname, property, type_str, variants, values));
    blocks
}

pub fn make_fn_get_all_variants(enumname: &str, variants: Vec<&str>) -> TextBlock {
    let tabwidth: usize = 4;
    let mut lines: Vec<String> = vec![];
    lines.push(format!("impl {} {{", enumname));
    lines.push(indent_string(format!(
        "pub fn get_all_variants() -> Vec<{}> {{", enumname
    ), tabwidth));
    //declare empty vec![]
    lines.push(indent_string(format!("vec!["), tabwidth*2));
    for variant in variants {
        lines.push(indent_string(format!(
            "{}::{},", enumname, variant
        ), tabwidth*3));
    }
    //close empry vec![]
    lines.push(indent_string(format!("]"), tabwidth*2));
    //close function
    lines.push(indent_string(format!("}}"), tabwidth));
    // close impl
    lines.push(format!("}}"));

    TextBlock { block_type: super::BlockType::FUNC, lines }
}

pub fn generate_fn_blocks(enumtable: &EnumTable) -> Vec<TextBlock> {
    //let indent:usize = 4;

    let mut blocks: Vec<TextBlock> = vec![]; 
    let enumname = enumtable.get_enumname();
    let props = enumtable.get_properties();
    let num_cols = props.len();
    let variants = enumtable.get_variants(); 
    let types = enumtable.get_types();   

    for col in 1..num_cols + 1 {
        let values = enumtable.data[col].clone();
        let prop = props[col - 1];
        let type_str = types[col - 1];
        let propfns = make_function_block_for_property(enumname, prop, type_str, variants.clone(), values);
        for fun in propfns {
            blocks.push(fun);
        }
    }
    blocks
}