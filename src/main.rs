use crate::parser::Parse;

pub mod ast;
pub mod lexer;
pub mod parser;
pub mod tokens;

fn main() {
    let tokens = lexer::lex("1 - 8 + 5 * 32 / 4 - 3").unwrap();
    let tree = ast::T0::parse(tokens.as_slice());

    println!("{:?}", tree);
}
