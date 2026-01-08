use std::{iter::Peekable, str::Chars};

use crate::tokens::*;

#[derive(Debug, Clone, Copy)]
pub struct TokenInfo {
    pub position: u32,
}

#[derive(Debug, Clone, Copy)]
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

    fn read_num(&mut self, radix: u32) -> Option<u32> {
        let mut result: u32 = 0;
        let mut parsed: usize = 0;
        while let Some(&ch) = self.chars.peek() {
            if let Some(d) = ch.to_digit(radix) {
                result *= radix;
                result += d;
                parsed += 1;
                self.chars.next();
            } else {
                // illegal
                if ch.is_alphabetic() {
                    self.chars.next();
                    return None;
                }
                break;
            }
        }
        if parsed == 0 { None } else { Some(result) }
    }

    fn match_single(c: char) -> Option<Token> {
        match c {
            '+' => Some(Token::PLUS),
            '-' => Some(Token::MINUS),
            '*' => Some(Token::ASTER),
            '/' => Some(Token::SLASH),
            '(' => Some(Token::LPAREN),
            ')' => Some(Token::RPAREN),
            _ => None,
        }
    }

    fn expect_number(&mut self, radix: u32) -> Token {
        if let Some(n) = self.read_num(radix) {
            Token::NUMBER(n)
        } else {
            Token::ILLEGAL
        }
    }

    fn next_token(&mut self) -> Option<LexedToken> {
        self.consume_whitespace();
        if let Some(ch) = self.chars.peek().cloned() {
            let tk = if let Some(t) = Lexer::match_single(ch) {
                self.chars.next();
                t
            } else {
                match ch {
                    '0' => {
                        self.chars.next();
                        if let Some(pfx) = self.chars.peek().cloned() {
                            if pfx.is_numeric() {
                                Token::ILLEGAL
                            } else if pfx.is_alphabetic() {
                                self.chars.next();
                                match pfx {
                                    'x' => self.expect_number(16),
                                    'o' => self.expect_number(8),
                                    'b' => self.expect_number(1),
                                    _ => Token::ILLEGAL,
                                }
                            } else {
                                Token::NUMBER(0)
                            }
                        } else {
                            Token::NUMBER(0)
                        }
                    }
                    '1'..='9' => self.expect_number(10),
                    _ => Token::ILLEGAL,
                }
            };

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
