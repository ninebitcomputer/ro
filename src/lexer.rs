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
    end: bool,
    pos: u32,
}

impl<'a> Lexer<'a> {
    pub fn new(chars: Peekable<Chars<'a>>) -> Self {
        Self {
            chars,
            pos: 0,
            end: false,
        }
    }

    // use instead of self.chars.next() for position reporting
    fn next_char(&mut self) -> Option<char> {
        self.pos += 1;
        self.chars.next()
    }

    fn consume_whitespace(&mut self) {
        while let Some(ch) = self.chars.peek()
            && ch.is_whitespace()
        {
            self.next_char();
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
                self.next_char();
            } else {
                // illegal
                if ch.is_alphabetic() {
                    self.next_char();
                    return None;
                }
                break;
            }
        }
        if parsed == 0 { None } else { Some(result) }
    }

    fn match_single(c: char) -> Option<Token> {
        match c {
            '*' => Some(Token::ASTER),
            ',' => Some(Token::COMMA),
            '/' => Some(Token::SLASH),
            '(' => Some(Token::LPAREN),
            ')' => Some(Token::RPAREN),
            '{' => Some(Token::LCURL),
            '}' => Some(Token::RCURL),
            ';' => Some(Token::SEMICOLON),
            '=' => Some(Token::EQUAL),
            _ => None,
        }
    }

    fn match_minus_second(c: char) -> Option<Token> {
        match c {
            '-' => Some(Token::MINUSMINUS),
            '>' => Some(Token::ARROW),
            _ => None,
        }
    }

    fn match_plus_second(c: char) -> Option<Token> {
        match c {
            '+' => Some(Token::PLUSPLUS),
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

    fn expect_string(&mut self) -> Token {
        let mut s = String::with_capacity(16);
        while let Some(x) = self.chars.peek()
            // 0..9 not first char
            && let 'a'..='z' | 'A'..='Z' | '0'..='9' = x
        {
            s.push(*x);
            self.next_char();
        }
        if s.len() == 0 {
            return Token::ILLEGAL;
        }
        if let Some(kwd) = Self::lookup_keyword(s.as_str()) {
            kwd
        } else {
            Token::IDENT(s)
        }
    }

    fn lookup_keyword(s: &str) -> Option<Token> {
        match s {
            "else" => Some(Token::ELSE),
            "float" => Some(Token::FLOAT),
            "fn" => Some(Token::FN),
            "for" => Some(Token::FOR),
            "if" => Some(Token::IF),
            "int" => Some(Token::INT),
            _ => None,
        }
    }

    fn next_token(&mut self) -> Option<LexedToken> {
        self.consume_whitespace();
        if let Some(ch) = self.chars.peek().cloned() {
            let tk = if let Some(t) = Lexer::match_single(ch) {
                self.next_char();
                t
            } else {
                match ch {
                    '-' => {
                        self.chars.next();
                        if let Some(nch) = self.chars.peek()
                            && let Some(tok) = Self::match_minus_second(*nch)
                        {
                            self.chars.next();
                            tok
                        } else {
                            Token::MINUS
                        }
                    }
                    '+' => {
                        self.chars.next();
                        if let Some(nch) = self.chars.peek()
                            && let Some(tok) = Self::match_plus_second(*nch)
                        {
                            self.chars.next();
                            tok
                        } else {
                            Token::PLUS
                        }
                    }
                    '0' => {
                        self.next_char();
                        if let Some(pfx) = self.chars.peek().cloned() {
                            if pfx.is_numeric() {
                                Token::ILLEGAL
                            } else if pfx.is_alphabetic() {
                                self.next_char();
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
                    'a'..='z' | 'A'..='Z' => self.expect_string(),
                    _ => Token::ILLEGAL,
                }
            };

            Some(LexedToken {
                info: TokenInfo { position: self.pos },
                token: tk,
            })
        } else {
            if self.end {
                None
            } else {
                self.end = true;
                Some(LexedToken {
                    info: TokenInfo { position: self.pos },
                    token: Token::EOF,
                })
            }
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = LexedToken;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}
