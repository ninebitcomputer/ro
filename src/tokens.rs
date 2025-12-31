#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    PLUS,
    MINUS,
    ASTER,
    SLASH,
    LPAREN,
    RPAREN,
    ILLEGAL,
    NUMBER(u32),
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
}
