use std::fmt;

use super::indent_string;

const TABWIDTH: usize = 4;

/// Struct that contains text... well, what did you expect?
/// 
/// Handles indentation and closures.
/// 
/// ```
///     use csvenum::code_gen::{TextBlock, MatchBlock};    
///     
///     let mut tb = TextBlock::new();
///     tb.add_line("use csvenum::RType;");
///     tb.add_line("pub fn match_rtype(rtype: RType) {");
///     tb.open_closure(false);
///     let mut mb = MatchBlock::new("rtype", false);
///     mb.add_arm("RType::String(_)", "panic!(\"It's a string!\")");
///     mb.add_arm("_", "println!(\" Just a normal type \")");
///     tb.append_lines(mb.to_lines());
///     tb.close_closure(true);
///   
///     println!("{}", tb);
/// 
/// ```
#[derive(Debug)]
pub struct TextBlock {
    pub lines: Vec<String>,
    closure_depth: usize,
}

impl TextBlock {
    pub fn new() -> Self {
        TextBlock { lines: vec![] , closure_depth: 0}
    }
    pub fn from_lines(lines: Vec<String>) -> Self {
        TextBlock { lines , closure_depth: 0}
    }
    pub fn add_line<T: AsRef<str>>(&mut self, line: T) {
        let line = line.as_ref();
        self.lines.push(line.to_string())
    }
    pub fn add_line_indented<T: AsRef<str>>(&mut self, line: T) {

        self.lines.push(indent_string(line.as_ref(), TABWIDTH * self.closure_depth) )
    }

    pub fn collect_lines_into(&self, dest: &mut Vec<String>) {
        for line in self.lines.clone() {
            dest.push(line);
        }
    }
    pub fn to_lines(&self) -> Vec<String> {
        let mut lines = vec![];
        self.collect_lines_into(&mut lines);
        lines
    }

    pub fn open_closure(&mut self, push_bracket: bool) {
        if push_bracket {
            if self.closure_depth > 0 {
                self.add_line_indented(String::from("{"))
            } else {self.add_line(String::from("{"))}               
        }
        self.closure_depth = self.closure_depth.saturating_add(1);
    }
    pub fn close_closure(&mut self, push_bracket: bool) {
        self.closure_depth = self.closure_depth.saturating_sub(1);
        if push_bracket {
            if self.closure_depth > 0 {
                self.add_line_indented(String::from("}"))
            } else {self.add_line(String::from("}"))}          
        }
    }
    pub fn get_current_depth(&self) -> usize {
        self.closure_depth
    }
    pub fn append_lines(&mut self, lines: Vec<String>) {
        for line in lines {
            self.add_line_indented(line);
        }
    }
}

impl fmt::Display for TextBlock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for line in &self.lines {
            writeln!(f, "{}", line)?;
        }
        Ok(())
    }
} 
