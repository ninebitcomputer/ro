use crate::ast::*;
use crate::lexer::*;
use crate::tokens::*;

use std::iter::Peekable;
pub trait Parse {
    fn parse(tokens: &[LexedToken]) -> Option<Self>
    where
        Self: Sized;
}

pub fn parse_expr(lexer: &mut Peekable<Lexer>, l: Option<Expr>, min_prec: u32) -> Option<Expr> {
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

        while let Some(n_op) = lookahead
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
                return None;
            }
        };

        lhs = Expr::Binop(Binop {
            a: Box::new(lhs),
            op: binop,
            b: Box::new(rhs),
        })
    }
    Some(lhs)
}

pub fn parse_unary(t: &Token) -> Option<UOp> {
    match t {
        Token::PLUS => Some(UOp::Pos),
        Token::MINUS => Some(UOp::Neg),
        _ => None,
    }
}

pub fn expect_atomic(lexer: &mut Peekable<Lexer>) -> Option<Expr> {
    let tk = lexer.peek()?.clone();

    if let Token::LPAREN = tk.token {
        lexer.next();
        let expr = parse_expr(lexer, None, 0)?;
        let rhs = lexer.peek()?.clone();
        if let Token::RPAREN = rhs.token {
            lexer.next();
            Some(expr)
        } else {
            None
        }
    } else if let Token::NUMBER(n) = tk.token {
        lexer.next();
        Some(Expr::Intermediate(n.into()))
    } else if let Some(uop) = parse_unary(&tk.token) {
        lexer.next();
        let u = Unary {
            op: uop,
            x: Box::new(expect_atomic(lexer)?),
        };
        Some(Expr::Unary(u))
    } else {
        None
    }
}
