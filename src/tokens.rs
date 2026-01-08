#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Token {
    PLUS,
    MINUS,
    ASTER,
    SLASH,
    LPAREN,
    RPAREN,
    NUMBER(u32),
    ILLEGAL,
    EOF,
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
