use std::fmt;
use super::{indent_string, CodeLines};

#[derive(Debug, Clone, Default)]
pub struct MatchBlock {
    tabwidth: usize,
    indent_self: bool,
    pub match_this: String,
    num_arms: usize,
    arms_lh: Vec<String>,
    arms_rh: Vec<String>,
}

impl MatchBlock {
    pub fn new(match_key: String, indent_self: bool) -> Self {
        MatchBlock {
            tabwidth: 4,
            indent_self,
            match_this: match_key, 
            num_arms: 0,
            arms_lh: vec![], 
            arms_rh: vec![] }
    }

    pub fn indent_self(&mut self) {
        self.indent_self = true;
    }

    pub fn add_arm(&mut self, left: String, right: String) {
        self.arms_lh.push(left);
        self.arms_rh.push(right);
        self.num_arms = self.num_arms + 1;
    }


    pub fn from_keys(match_key: String, left_vals: Vec<String>, right_vals: Vec<String>, indent_self:bool) -> Self {
        let num_arms = left_vals.len();
        if right_vals.len() != num_arms {
            panic!("MatchBlock Error: Passed Left Values and Right Values do not have the same length, {} vs. {} for property, {}", match_key, left_vals.len(), right_vals.len())
        } 
        MatchBlock { tabwidth:4, indent_self, match_this: match_key, num_arms, arms_lh: left_vals, arms_rh: right_vals }  
    }

}

impl CodeLines for MatchBlock {
    fn lines_into(&self, lines: &mut Vec<String>){
        let self_indent = if self.indent_self {self.tabwidth} else {0};
        lines.push(indent_string(
            format!("match {} {{", self.match_this)
        , self_indent));
        for i in 0..self.num_arms {
            lines.push(indent_string(format!(
                "{} => {},", self.arms_lh[i], self.arms_rh[i]
            ), self.tabwidth + self_indent));
        }
        lines.push(indent_string(format!("}}"), self_indent));     
    }
    fn to_lines(&self) -> Vec<String> {
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
