#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    ARROW,
    ASTER,
    COMMA,
    ELSE,
    EOF,
    EQUAL,
    EQUALEQUAL,
    FLOAT,
    FN,
    GT,
    IDENT(String),
    IF,
    ILLEGAL,
    INT,
    LCURL,
    LPAREN,
    LT,
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
    WHILE,
}

pub struct OpInfo {
    pub prec: u32,
    pub l_assoc: bool,
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

    pub fn get_op_info(&self) -> Option<OpInfo> {
        Some(match self {
            Token::PLUS => OpInfo {
                prec: 2,
                l_assoc: true,
            },
            Token::MINUS => OpInfo {
                prec: 2,
                l_assoc: true,
            },
            Token::ASTER => OpInfo {
                prec: 3,
                l_assoc: true,
            },
            Token::SLASH => OpInfo {
                prec: 3,
                l_assoc: true,
            },
            Token::LT => OpInfo {
                prec: 1,
                l_assoc: true,
            },
            Token::GT => OpInfo {
                prec: 1,
                l_assoc: true,
            },
            Token::EQUALEQUAL => OpInfo {
                prec: 0,
                l_assoc: false,
            },
            _ => return None,
        })
    }
}
