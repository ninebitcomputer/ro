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
        println!("Atom");
        log_tokens(tokens);

        if let Some(i) = parse_number(tokens) {
            return Some(Self::Number(i));
        }

        match tokens {
            [
                LexedToken {
                    token: Token::LPAREN,
                    ..
                },
                rest @ ..,
                LexedToken {
                    token: Token::RPAREN,
                    ..
                },
            ] => {
                let x = T0::parse(rest)?;
                return Some(Self::Paren(x));
            }
            _ => None,
        }
    }
}

impl Parse for Mul {
    fn parse(tokens: &[LexedToken]) -> Option<Self> {
        println!("Mul");
        log_tokens(tokens);

        let (a, b) = split_left_assoc(tokens, Token::ASTER)?;
        Some(Self {
            a: T1::parse(a)?,
            b: Atom::parse(b)?,
        })
    }
}

impl Parse for Div {
    fn parse(tokens: &[LexedToken]) -> Option<Self> {
        println!("Div");
        log_tokens(tokens);

        let (a, b) = split_left_assoc(tokens, Token::SLASH)?;
        Some(Self {
            a: T1::parse(a)?,
            b: Atom::parse(b)?,
        })
    }
}
impl Parse for Add {
    fn parse(tokens: &[LexedToken]) -> Option<Self> {
        println!("Add");
        log_tokens(tokens);

        let (a, b) = split_left_assoc(tokens, Token::PLUS)?;
        Some(Self {
            a: T0::parse(a)?,
            b: T1::parse(b)?,
        })
    }
}
impl Parse for Sub {
    fn parse(tokens: &[LexedToken]) -> Option<Self> {
        println!("Sub");
        log_tokens(tokens);

        let (a, b) = split_left_assoc(tokens, Token::MINUS)?;
        Some(Self {
            a: T0::parse(a)?,
            b: T1::parse(b)?,
        })
    }
}
impl Parse for T0 {
    fn parse(tokens: &[LexedToken]) -> Option<Self> {
        println!("T0");
        log_tokens(tokens);

        if let Some(x) = Add::parse(tokens) {
            return Some(Self::Add(Box::new(x)));
        }
        if let Some(y) = Sub::parse(tokens) {
            return Some(Self::Sub(Box::new(y)));
        }
        if let Some(z) = T1::parse(tokens) {
            return Some(Self::T1(Box::new(z)));
        }
        Option::None
    }
}
impl Parse for T1 {
    fn parse(tokens: &[LexedToken]) -> Option<Self> {
        println!("T1");
        log_tokens(tokens);

        if let Some(x) = Mul::parse(tokens) {
            return Some(Self::Mul(Box::new(x)));
        }
        if let Some(y) = Div::parse(tokens) {
            return Some(Self::Div(Box::new(y)));
        }
        if let Some(z) = Atom::parse(tokens) {
            return Some(Self::Atom(Box::new(z)));
        }
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

fn split_left_assoc(tokens: &[LexedToken], t: Token) -> Option<(&[LexedToken], &[LexedToken])> {
    let idx = tokens.iter().rposition(|x| x.token == t)?;
    let (a, b) = tokens.split_at(idx);
    Some((a, b.get(1..).unwrap_or(&[])))
}

fn log_tokens(tokens: &[LexedToken]) {
    for t in tokens.iter() {
        print!("{}", t.token.to_char());
    }
    println!("");
}
