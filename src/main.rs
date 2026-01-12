use crate::parser::Parser;

pub mod ast;
pub mod eval;
pub mod lexer;
pub mod parser;
pub mod tokens;
pub mod util;

fn main() {
    let s = "1-8+5*32/ 0x20 - 3 * (2 + var) + 8";
    let mut parser = Parser::new(s.chars());

    //let r = parse_expr(&mut lexer, None, 0);
    let r = parser.parse_expr(None, 0);
    println!("{:?}", r);

    if let Ok(e) = r {
        e.pprint();
    }

    /* while let Some(t) = lexer.next() {
        println!("{:?}", t);
    } */
}
