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

pub struct Lexer<'a> {
    chars: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(chars: Peekable<Chars<'a>>) -> Self {
        Self { chars }
    }

    fn consume_whitespace(&mut self) {
        while let Some(ch) = self.chars.peek()
            && ch.is_whitespace()
        {
            self.chars.next();
        }
    }

    fn read_num(&mut self) -> Option<u32> {
        let mut result: u32 = 0;
        let mut parsed: usize = 0;
        while let Some(&ch) = self.chars.peek() {
            if let Some(d) = ch.to_digit(10) {
                result *= 10;
                result += d;
                parsed += 1;
                self.chars.next();
            } else {
                break;
            }
        }
        if parsed == 0 { None } else { Some(result) }
    }

    fn next_token(&mut self) -> Option<LexedToken> {
        self.consume_whitespace();
        if let Some(ch) = self.chars.peek() {
            let tk = match ch {
                '+' => Token::PLUS,
                '-' => Token::MINUS,
                '*' => Token::ASTER,
                '/' => Token::SLASH,
                '(' => Token::LPAREN,
                ')' => Token::RPAREN,
                _ => {
                    if ch.is_digit(10) {
                        if let Some(n) = self.read_num() {
                            Token::NUMBER(n)
                        } else {
                            Token::ILLEGAL
                        }
                    } else {
                        Token::ILLEGAL
                    }
                }
            };
            self.chars.next();
            Some(LexedToken {
                info: TokenInfo { position: 0 },
                token: tk,
            })
        } else {
            None
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = LexedToken;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}
