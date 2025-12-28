use std::{iter::Peekable, str::Chars};

use crate::tokens::*;

#[derive(Debug, Clone)]
pub struct TokenInfo {
    pub position: u32,
}

#[derive(Debug, Clone)]
pub struct LexedToken {
    pub info: TokenInfo,
    pub token: Token,
}

#[derive(Debug, Clone)]
pub enum LexerErrorReason {
    UnknownToken(char),
}

#[derive(Debug, Clone)]
pub struct LexerError {
    pub info: TokenInfo,
    pub reason: LexerErrorReason,
}

pub struct Lexer {
    stream: String,
}

pub struct LexerIterator<'a> {
    chars: Peekable<Chars<'a>>,
}

impl<'a> Iterator for LexerIterator<'a> {
    type Item = Result<LexedToken, LexerError>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(cur) = self.chars.peek() {
            match *cur {
                _ => Some(Err(LexerError {
                    info: TokenInfo { position: idx },
                    reason: LexerErrorReason::UnknownToken(ch),
                })),
            };
            None
        } else {
            None
        }
    }
}

impl Lexer {
    pub fn new(stream: String) -> Self {
        Self { stream }
    }
    pub fn iter(&'_ self) -> LexerIterator<'_> {
        LexerIterator {
            chars: self.stream.chars().peekable(),
        }
    }
}

pub fn lex(stream: &str) -> Result<Vec<LexedToken>, LexerError> {
    let mut tokens = Vec::new();
    let mut iter = stream.chars().peekable();
    let mut idx = 0u32;

    while let Some(ch) = iter.next() {
        idx += 1;
        if ch.is_whitespace() {
            continue;
        }

        let t = match ch {
            '+' => Token::PLUS,
            '-' => Token::MINUS,
            '*' => Token::ASTER,
            '/' => Token::SLASH,
            '(' => Token::LPAREN,
            ')' => Token::RPAREN,
            '0' => Token::DIGIT(Digit::ZERO),
            '1' => Token::DIGIT(Digit::ONE),
            '2' => Token::DIGIT(Digit::TWO),
            '3' => Token::DIGIT(Digit::THREE),
            '4' => Token::DIGIT(Digit::FOUR),
            '5' => Token::DIGIT(Digit::FIVE),
            '6' => Token::DIGIT(Digit::SIX),
            '7' => Token::DIGIT(Digit::SEVEN),
            '8' => Token::DIGIT(Digit::EIGHT),
            '9' => Token::DIGIT(Digit::NINE),
            _ => {
                return Err(LexerError {
                    info: TokenInfo { position: idx },
                    reason: LexerErrorReason::UnknownToken(ch),
                });
            }
        };
        let lexed = LexedToken {
            info: TokenInfo { position: idx },
            token: t,
        };
        tokens.push(lexed);
    }
    Ok(tokens)
}
