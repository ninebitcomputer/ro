use crate::parser::Parser;
use crate::util::TPrint;

pub mod ast;
pub mod eval;
pub mod lexer;
pub mod parser;
pub mod tokens;
pub mod util;

fn main() {
    let e = "1-var2+5*32/ 0x20 - 3 * (2 + var) + 8";
    let s = "{ int var = 3; if (var - 3) { var = var + 5;}; float x = y / 10; };";

    let mut eparser = Parser::new(e.chars());
    let mut sparser = Parser::new(s.chars());

    let r = eparser.parse_expr(None, 0);
    println!("{:?}", r);
    if let Ok(expr) = r {
        expr.tprint();
    }

    let r = sparser.parse_statement();
    println!("{:?}", r);
    if let Ok(stmt) = r {
        stmt.tprint();
    }

    /* while let Some(t) = lexer.next() {
        println!("{:?}", t);
    } */
}
