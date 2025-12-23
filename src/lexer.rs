use crate::tokens::*;

pub struct TokenInfo {
    position: u32,
}

pub struct LexedToken {
    info: TokenInfo,
    token: Token,
}

pub enum LexerErrorReason {
    UnknownToken(String),
}

pub struct LexerError {
    info: TokenInfo,
    reason: LexerErrorReason,
}

pub fn lex(stream: &str) -> Result<Vec<LexedToken>, LexerError> {
    Ok(vec![])
}
