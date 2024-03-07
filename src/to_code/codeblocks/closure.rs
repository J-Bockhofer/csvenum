//use crate::indent_string;

/// What do I want?
/// - 
/// 
/// What do I have? 
/// - The types from a string representation
/// 
/// 
/// need AST to CST - internal representation to code
/// 
/// Leaves is the data / types, so the constants in this case
/// 
/// Nodes are what is done to the data / types
/// 
/// Nodes can be:
/// 
/// 1. Expression
/// 2. Function
/// 3. Declaration
/// 
#[derive(Debug)]
pub struct Closure {
    //config: PrintConfig,
    content: Vec<String>,
}

use std::borrow::Cow;

// This is just Rust-Lang... its already difficult enough
// core actions:
// let
// const
// fn
// struct
// enum
// type declaration handling

// Programmable way to create a Program.
// Line::new(format!(), LineType::Doc)


/// Lines provides a simple interface to create a new Line
/// A Line represents a single statement, or opens / closes a closure.
/// 
pub struct Line<'a> {
    /// The Content
    content: Cow<'a, str>,
    /// The location inside
    pub line_idx: usize,

}




