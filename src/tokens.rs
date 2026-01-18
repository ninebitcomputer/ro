use std::rc::Rc;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    ARROW,
    ASTER,
    ELSE,
    EOF,
    EQUAL,
    EQUALEQUAL,
    FLOAT,
    IDENT(String),
    IF,
    ILLEGAL,
    INT,
    LCURL,
    LPAREN,
    MINUS,
    MINUSEQUAL,
    MINUSMINUS,
    NUMBER(u32),
    PLUS,
    PLUSEQUAL,
    PLUSPLUS,
    RCURL,
    RPAREN,
    SEMICOLON,
    SLASH,
}

impl Token {
    pub fn to_prec(&self) -> Option<u32> {
        match self {
            Token::PLUS => Some(0),
            Token::MINUS => Some(0),
            Token::ASTER => Some(1),
            Token::SLASH => Some(1),
            _ => None,
        }
    }

    pub fn is_op(&self) -> bool {
        match self {
            Token::PLUS => true,
            Token::MINUS => true,
            Token::ASTER => true,
            Token::SLASH => true,
            _ => false,
        }
    }
}
