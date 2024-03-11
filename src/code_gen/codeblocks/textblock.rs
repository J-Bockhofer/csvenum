use std::fmt;

use super::indent_string;

const TABWIDTH: usize = 4;

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

        self.lines.push(indent_string(line.as_ref().to_string(), TABWIDTH * self.closure_depth) )
    }

    pub fn collect_lines_into(&self, dest: &mut Vec<String>) {
        for line in self.lines.clone() {
            dest.push(line);
        }
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
