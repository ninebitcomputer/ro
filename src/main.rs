use std::io::{Write, stdout};

use crate::{
    eval::Eval,
    parser::{Parse, parse_expr},
};

pub mod ast;
pub mod eval;
pub mod lexer;
pub mod parser;
pub mod tokens;
pub mod util;

fn main() {
    //let s = "1-8+5*32/ 0x20 - 3 * (2 + 8)";
    let s = "6 - + + + - + - + + + 7";
    let mut lexer = lexer::Lexer::new(s.chars().peekable()).peekable();
    let r = parse_expr(&mut lexer, None, 0);
    println!("{:?}", r);

    if let Some(e) = r {
        e.pprint();
    }

    /* while let Some(t) = lexer.next() {
        println!("{:?}", t);
    } */
}
