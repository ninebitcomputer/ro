use std::io::{Write, stdout};

use crate::{eval::Eval, parser::Parse};

pub mod ast;
pub mod eval;
pub mod lexer;
pub mod parser;
pub mod tokens;

fn main() {
    let s = "1-8+5*32/ 0x20 - 3 * (2 + 8)";
    let mut lexer = lexer::Lexer::new(s.chars().peekable());

    while let Some(t) = lexer.next() {
        println!("{:?}", t);
    }
}
