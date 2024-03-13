use std::fmt;
use super::indent_string;

/// Struct for making it less repetitive to write match blocks
#[derive(Debug, Clone, Default)]
pub struct MatchBlock {
    tabwidth: usize,
    indent_self: bool,
    pub match_this: String,
    num_arms: usize,
    arms_lh: Vec<String>,
    arms_rh: Vec<String>,
    indent_depth: usize,
}

impl MatchBlock {
    pub fn new<T: AsRef<str>>(match_key: T, indent_self: bool) -> Self {
        MatchBlock {
            tabwidth: 4,
            indent_self,
            match_this: match_key.as_ref().to_string(), 
            num_arms: 0,
            arms_lh: vec![], 
            arms_rh: vec![],
            indent_depth: 0 }
    }
    #[allow(dead_code)]
    pub fn indent_self(&mut self) {
        self.indent_self = true;
        self.indent_depth = self.indent_depth.saturating_add(1);
    }
    #[allow(dead_code)]
    pub fn set_indent_depth(&mut self, depth: usize) {
        self.indent_depth = depth;
    }
    pub fn add_arm<T: AsRef<str>>(&mut self, left: T, right: T) {
        self.arms_lh.push(left.as_ref().to_string());
        self.arms_rh.push(right.as_ref().to_string());
        self.num_arms = self.num_arms + 1;
    }


    pub fn from_keys(match_key: String, left_vals: Vec<String>, right_vals: Vec<String>, indent_self:bool) -> Self {
        let num_arms = left_vals.len();
        if right_vals.len() != num_arms {
            panic!("MatchBlock Error: Passed Left Values and Right Values do not have the same length, {} vs. {} for property, {}", match_key, left_vals.len(), right_vals.len())
        } 
        MatchBlock { tabwidth:4, indent_self, match_this: match_key, num_arms, arms_lh: left_vals, arms_rh: right_vals , indent_depth:0}  
    }

    pub fn lines_into(&self, lines: &mut Vec<String>){
        let self_indent = if self.indent_self {self.tabwidth * self.indent_depth} else {0};
        lines.push(indent_string(
            format!("match {} {{", self.match_this)
        , self_indent));
        for i in 0..self.num_arms {
            lines.push(indent_string(format!(
                "{} => {},", self.arms_lh[i], self.arms_rh[i]
            ), self.tabwidth  + self_indent ));
        }
        lines.push(indent_string(format!("}}"), self_indent));     
    }
    pub fn to_lines(&self) -> Vec<String> {
        let mut lines: Vec<String> = vec![];
        self.lines_into(&mut lines);
        lines          
    }

}


impl fmt::Display for MatchBlock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for line in self.to_lines() {
            writeln!(f, "{}", line)?;
        }
        Ok(())
    }
} 
