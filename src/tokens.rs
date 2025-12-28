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
