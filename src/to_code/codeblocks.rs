pub mod closure;

pub mod enumblock;

pub mod matchblock;

use super::indent_string;



pub enum CodeBlocks {
    ENUM,
    IMPL,
    FUNC,
    CONST,
}

 pub trait CodeLines {
    fn lines_into(&self, dest: &mut Vec<String>);
    fn to_lines(&self) -> Vec<String>;
 }

 
/// i want codeblocks to be declared in a function for example create_enum_block
/// This struct is kinda just for the paths to leave it out for now?
#[derive(Debug)]
pub struct PrintConfig {
    tabwidth: usize
}

impl Default for PrintConfig {
    fn default() -> Self {
        PrintConfig { 
            tabwidth: 4 
        }
    }
}
