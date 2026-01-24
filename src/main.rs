use crate::ast::Statement;
use crate::parser::Parser;
use crate::util::TPrint;

pub mod ast;
pub mod ir;
pub mod lexer;
pub mod parser;
pub mod tokens;
pub mod util;

fn main() {
    let source = include_str!("ro/fib.ro");
    let mut parser = Parser::new(source.chars());

    let stmts = parser.parse_top().unwrap_or_else(|e| {
        eprintln!("parse_top failed: {e:?}");
        panic!("fib.ro should parse");
    });

    let blk = Statement::Block(stmts);
    println!("basic.ro AST:");
    blk.tprint();

    /* while let Some(t) = lexer.next() {
        println!("{:?}", t);

    } */
}
