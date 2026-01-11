use crate::ast::*;
use crate::lexer::*;
use crate::tokens::*;

use std::iter::Peekable;
pub trait Parse {
    fn parse(tokens: &[LexedToken]) -> Option<Self>
    where
        Self: Sized;
}

#[derive(Debug)]
pub struct ParseError {
    pub token: Option<LexedToken>,
    pub reason: ParseErrorReason,
}

impl ParseError {
    pub fn new(token: Option<LexedToken>, reason: ParseErrorReason) -> Self {
        Self { token, reason }
    }
}

#[derive(Debug)]
pub enum ParseErrorReason {
    BadBinOp,
    BadUnary,
    UnmatchedParens,
    NonAtomicExpression,
    StreamEnded,
}

pub fn parse_statement(lexer: &mut Peekable<Lexer>) -> Option<Statement> {
    None
}

pub fn parse_expr(
    lexer: &mut Peekable<Lexer>,
    l: Option<Expr>,
    min_prec: u32,
) -> Result<Expr, ParseError> {
    let mut lhs = if let Some(x) = l {
        x
    } else {
        expect_atomic(lexer)?
    };

    let mut lookahead = lexer.peek().cloned();

    while let Some(op) = lookahead
        && let Some(p) = op.token.to_prec()
        && p >= min_prec
    {
        lexer.next();
        let mut rhs = expect_atomic(lexer)?;
        lookahead = lexer.peek().cloned();

        while let Some(n_op) = lookahead.as_ref()
            && let Some(n_p) = n_op.token.to_prec()
            && n_p > p
        {
            rhs = parse_expr(lexer, Some(rhs), p + 1)?;
            lookahead = lexer.peek().cloned();
        }

        let binop = match op.token {
            Token::PLUS => Op::Add,
            Token::MINUS => Op::Sub,
            Token::ASTER => Op::Mul,
            Token::SLASH => Op::Div,
            _ => {
                return Err(ParseError::new(Some(op), ParseErrorReason::BadBinOp));
            }
        };

        lhs = Expr::Binop(Binop {
            a: Box::new(lhs),
            op: binop,
            b: Box::new(rhs),
        })
    }
    Ok(lhs)
}

pub fn parse_unary(t: &Token) -> Option<UOp> {
    match t {
        Token::PLUS => Some(UOp::Pos),
        Token::MINUS => Some(UOp::Neg),
        _ => None,
    }
}

pub fn expect_peek(lexer: &mut Peekable<Lexer>) -> Result<LexedToken, ParseError> {
    if let Some(tk) = lexer.peek() {
        Ok(tk.clone())
    } else {
        Err(ParseError::new(None, ParseErrorReason::StreamEnded))
    }
}

pub fn expect_atomic(lexer: &mut Peekable<Lexer>) -> Result<Expr, ParseError> {
    let tk = expect_peek(lexer)?;

    if let Token::LPAREN = tk.token {
        lexer.next();
        let expr = parse_expr(lexer, None, 0)?;
        let rhs = expect_peek(lexer)?;
        if let Token::RPAREN = rhs.token {
            lexer.next();
            Ok(expr)
        } else {
            Err(ParseError::new(
                Some(rhs),
                ParseErrorReason::UnmatchedParens,
            ))
        }
    } else if let Token::NUMBER(n) = tk.token {
        lexer.next();
        Ok(Expr::Intermediate(n.into()))
    } else if let Some(uop) = parse_unary(&tk.token) {
        lexer.next();
        let u = Unary {
            op: uop,
            x: Box::new(expect_atomic(lexer)?),
        };
        Ok(Expr::Unary(u))
    } else if let Token::IDENT(s) = tk.token {
        Ok(Expr::Ident(s.clone()))
    } else {
        Err(ParseError::new(
            Some(tk),
            ParseErrorReason::NonAtomicExpression,
        ))
    }
}
