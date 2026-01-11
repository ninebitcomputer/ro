use crate::lexer::LexedToken;
use crate::parser::parse_expr;

pub mod ast;
pub mod eval;
pub mod lexer;
pub mod parser;
pub mod tokens;
pub mod util;

fn main() {
    let s = "1-8+5*32/ 0x20 - 3 * (2 + var)";

    /* {
        let lexer = lexer::Lexer::new(s.chars().peekable()).peekable();
        let tks: Vec<LexedToken> = lexer.collect();
        println!("{:?}", tks);
    } */

    let mut lexer = lexer::Lexer::new(s.chars().peekable()).peekable();

    let r = parse_expr(&mut lexer, None, 0);
    println!("{:?}", r);

    if let Ok(e) = r {
        e.pprint();
    }

    /* while let Some(t) = lexer.next() {
        println!("{:?}", t);
    } */
}
