use std::io::{Write, stdout};

use crate::{eval::Eval, parser::Parse};

pub mod ast;
pub mod eval;
pub mod lexer;
pub mod parser;
pub mod tokens;

fn main() {
    //let tokens = lexer::lex("1 - 8 + 5 * 32 / 4 - 3").unwrap();
    //let tree = ast::T0::parse(tokens.as_slice());

    let s = "1 - 8 + 5 * 32 / 4 - 3 * (2 + 8)";
    let mut lexer = lexer::Lexer::new(s.chars().peekable());

    while let Some(t) = lexer.next() {
        println!("{:?}", t);
    }

    //println!("{:?}", tree);
    /* loop {
        let mut buffer = String::new();
        let stdin = std::io::stdin();
        print!("> ");
        stdout().flush().unwrap();
        stdin.read_line(&mut buffer).unwrap();

        let lex_result = lexer::lex(buffer.as_str());
        match lex_result {
            Ok(tokens) => {
                if let Some(tree) = ast::T0::parse(tokens.as_slice()) {
                    println!("{}", tree.eval());
                } else {
                    println!("Syntax error");
                }
            }
            Err(_e) => {
                println!("Error processing tokens")
            }
        }
    } */
}
