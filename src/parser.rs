use crate::ast::*;
use crate::lexer::*;
use crate::tokens::*;

pub trait Parse {
    fn parse(tokens: &[LexedToken]) -> Option<Self>
    where
        Self: Sized;
}

impl Parse for Atom {
    fn parse(tokens: &[LexedToken]) -> Option<Self> {
        if let Some(i) = parse_number(tokens) {
            return Some(Self::Number(i));
        }
        let mut stack: i32 = 0;
        let mut idx: i32 = 0;

        for t in tokens.iter() {
            if idx == 0 && !matches!(t.token, Token::LPAREN) {
                return None;
            }

            match t.token {
                Token::LPAREN => stack += 1,
                Token::RPAREN => {
                    stack -= 1;
                    if stack == 0 {
                        break;
                    }
                }
                _ => {}
            };

            idx += 1;
        }

        Option::None
    }
}

impl Parse for Mul {
    fn parse(tokens: &[LexedToken]) -> Option<Self> {
        Option::None
    }
}

impl Parse for Div {
    fn parse(tokens: &[LexedToken]) -> Option<Self> {
        Option::None
    }
}
impl Parse for Add {
    fn parse(tokens: &[LexedToken]) -> Option<Self> {
        Option::None
    }
}
impl Parse for Sub {
    fn parse(tokens: &[LexedToken]) -> Option<Self> {
        Option::None
    }
}
impl Parse for T0 {
    fn parse(tokens: &[LexedToken]) -> Option<Self> {
        Option::None
    }
}
impl Parse for T1 {
    fn parse(tokens: &[LexedToken]) -> Option<Self> {
        Option::None
    }
}

fn parse_number(tokens: &[LexedToken]) -> Option<u32> {
    let mut r = 0u32;
    for t in tokens.iter() {
        if let Token::DIGIT(d) = &t.token {
            r *= 10;
            r += d.num() as u32;
        } else {
            return None;
        }
    }
    Some(r)
}
