use crate::ast::*;
use crate::lexer::*;
use crate::tokens::*;

pub trait Parse {
    fn parse(tokens: &[LexedToken]) -> Option<Self>
    where
        Self: Sized;
}
