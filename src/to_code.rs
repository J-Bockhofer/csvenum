use std::io::Write;
use super::{EnumTable, indent_string};

pub mod enumblock;
pub use enumblock::format_enum;

pub mod constblock;
pub use constblock::format_consts;

pub mod funcblock;
//pub use funcblock::format_fn_as_property;

pub mod types;

pub mod codeblocks;

#[derive(Debug)]
pub struct TextBlock {
    pub block_type: BlockType,
    pub lines: Vec<String>,
}

impl TextBlock {
    pub fn with_lines(lines: Vec<String>, block_type: BlockType) -> Self {
        TextBlock { lines, block_type }
    }

    pub fn write_to_file(&self, path: &str) {
        let mut file = std::fs::File::create(path).expect("Unable to create file");
        for line in &self.lines {
            writeln!(file, "{}", line).expect("Unable to write line to file");
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum BlockType {
    ENUM,
    CONST,
    FUNC,
}


#[derive(Debug, Clone)]
pub struct DeclarationWrapper {
    pub left_hand: String,
    pub right_hand: String,
}
impl DeclarationWrapper {
    pub fn with(left_hand: &str, right_hand: &str) -> Self {
        DeclarationWrapper { left_hand: left_hand.to_string(), right_hand: right_hand.to_string() }
    }
    pub fn wrap_value(&self, value: &str) -> String {
        format!("{}{}{}", self.left_hand, value, self.right_hand)
    }
}

pub struct VarType {
    /// what is read from the table
    pub input_type: String, 
    /// what is written after the variable name -> type declaration
    pub const_type: String, 
    /// how the value is written in code, i.e "0" for &str or 0 for numeric
    pub declaration_wrapper: Option<DeclarationWrapper>, 
}
impl VarType {
    pub fn with(input_type: &str, const_type: &str, declaration_wrapper: Option<DeclarationWrapper>) -> Self {
        VarType { input_type: input_type.to_string(), const_type: const_type.to_string(), declaration_wrapper }
    }
    pub fn apply_wrapper(&self, value: &str) -> String {
        if self.declaration_wrapper.is_none() {return value.to_string();}
        let wrapper = self.declaration_wrapper.clone().unwrap();
        wrapper.wrap_value(value)
    }

    pub fn from_type_str(type_str: &str) -> Self {
        // Types that cant be constants:
        // vec![]
        // String::new()
        // 
        //println!("{}", &type_str);
        // Types that can be constants
        // ""
        // 0
    
        // only three allowed "types" in type_str
        // &'static str
        // any numeric
        // regex... not yet
    
        // strip lifetimes - will seperate the type with a whitespace
        let parts: Vec<&str> = type_str.split(' ').collect();
        let actual_type = match parts.len() {
            1 => {parts[0]},
            2 => {parts[1]},
            _ => {panic!("Weird type specification 1 {}", type_str)}
        };
        //println!("{}", actual_type);
        // strip reference 
        let actual_type: String = actual_type.chars().filter(|&c|
            c != '&' && c != '1' && c != '2' && c != '3' && c != '4' && c != '5' && c != '6' && c != '7' && c != '8' && c != '9' && c != '0' && c != ' ' 
        ).collect();
        //println!("{}", actual_type.as_str());
        match actual_type.as_str() {
            "" => {panic!("Weird type specification 2 {}", type_str)},
            "str" => {VarType::with(type_str, "&'static str", Some(DeclarationWrapper::with("\"", "\"")))}
            "i" | "u" | "f" | "isize" | "usize" => {VarType::with(type_str, type_str, None)}
            _ => {panic!("Weird type specification 3 {}", type_str)}
        }
    }

}

#[derive(Debug, Clone, Default)]
struct MatchBlock {
    pub match_this: String,
    num_arms: usize,
    arms_lh: Vec<String>,
    arms_rh: Vec<String>,
}

impl MatchBlock {
    pub fn new(match_key: String) -> Self {
        MatchBlock { 
            match_this: match_key, 
            num_arms: 0,
            arms_lh: vec![], 
            arms_rh: vec![] }
    }

    pub fn add_arm(&mut self, left: String, right: String) {
        self.arms_lh.push(left);
        self.arms_rh.push(right);
        self.num_arms = self.num_arms + 1;
    }

    pub fn to_code(&self, tabwidth: usize, indent_self: bool) -> Vec<String> {
        let self_indent = if indent_self {tabwidth} else {0};
        let mut lines: Vec<String> = vec![];
        lines.push(indent_string(
            format!("match {} {{", self.match_this)
        , self_indent));
        for i in 0..self.num_arms {
            lines.push(indent_string(format!(
                "{} => {},", self.arms_lh[i], self.arms_rh[i]
            ), tabwidth + self_indent));
        }
        lines.push(indent_string(format!("}}"), self_indent));
        lines
    }

    pub fn from_keys(match_key: String, left_vals: Vec<String>, right_vals: Vec<String>) -> Self {
        let num_arms = left_vals.len();
        if right_vals.len() != num_arms {
            panic!("MatchBlock Error: Passed Left Values and Right Values do not have the same length, {} vs. {} for property, {}", match_key, left_vals.len(), right_vals.len())
        } 
        MatchBlock { match_this: match_key, num_arms, arms_lh: left_vals, arms_rh: right_vals }  

    }

}
