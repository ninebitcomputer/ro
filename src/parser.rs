use crate::ast::*;
use crate::lexer::*;
use crate::tokens::*;

use std::iter::Peekable;
pub trait Parse {
    fn parse(tokens: &[LexedToken]) -> Option<Self>
    where
        Self: Sized;
}

pub fn parse_expr(mut l: Peekable<&mut Lexer>) -> Option<Expr> {
    let mut nstack: Vec<u32> = Vec::new();
    let mut ostack: Vec<Token> = Vec::new();

    while let Some(tk) = l.peek() {
        if let Token::NUMBER(n) = tk.token {
            nstack.push(n);
        }
    }
    None
}
