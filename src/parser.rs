use crate::ast::*;
use crate::lexer::*;
use crate::tokens::*;

use std::iter::Peekable;
use std::str::Chars;
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
    BadStatement,
    ExpectedToken(Token),
    ExpectedIdentifier,
    NonAtomicExpression,
    StreamEnded,
}

pub struct Parser<'a> {
    lexer: Peekable<Lexer<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(chars: Chars<'a>) -> Self {
        Self {
            lexer: Lexer::new(chars.peekable()).peekable(),
        }
    }

    pub fn parse_statement(&mut self) -> Result<Statement, ParseError> {
        let tk = self.expect_peek()?;
        match &tk.token {
            Token::LCURL => {
                dbg!("block");
                let s = self.parse_block()?;
                self.expect_token(Token::SEMICOLON)?;

                Ok(Statement::Block(s))
            }
            Token::IF => {
                self.lexer.next();
                let guard = Box::new(self.parse_expr(None, 0)?);
                let t = self.parse_block()?;
                let f = if self.accept_token(Token::ELSE) {
                    Some(self.parse_block()?)
                } else {
                    None
                };
                self.expect_token(Token::SEMICOLON)?;

                Ok(Statement::If(SIf { guard, t, f }))
            }
            Token::IDENT(ident) => {
                self.lexer.next();
                self.expect_token(Token::EQUAL)?;
                let value = Box::new(self.parse_expr(None, 0)?);
                self.expect_token(Token::SEMICOLON)?;

                Ok(Statement::Assign(SAssign {
                    ident: ident.clone(),
                    value,
                }))
            }
            _ => {
                if let Some(typ) = self.lookup_type(&tk.token) {
                    self.lexer.next();
                    let ident = self.expect_identifier()?;

                    let assign = if self.accept_token(Token::EQUAL) {
                        Some(Box::new(self.parse_expr(None, 0)?))
                    } else {
                        None
                    };
                    self.expect_token(Token::SEMICOLON)?;

                    Ok(Statement::Declare(SDeclare { typ, ident, assign }))
                } else {
                    Err(ParseError::new(Some(tk), ParseErrorReason::BadStatement))
                }
            }
        }
    }

    pub fn lookup_type(&self, t: &Token) -> Option<LType> {
        Some(match t {
            Token::INT => LType::Int,
            Token::FLOAT => LType::Float,
            _ => return None,
        })
    }

    pub fn parse_block(&mut self) -> Result<Vec<Statement>, ParseError> {
        self.expect_token(Token::LCURL)?;
        let mut statements: Vec<Statement> = Vec::new();

        while !self.accept_token(Token::RCURL) {
            statements.push(self.parse_statement()?);
        }

        Ok(statements)
    }

    pub fn parse_top(&mut self) -> Result<Vec<Statement>, ParseError> {
        let mut stmts: Vec<Statement> = Vec::new();
        while !self.accept_token(Token::EOF) {
            stmts.push(self.parse_statement()?);
        }
        Ok(stmts)
    }

    pub fn parse_expr(&mut self, l: Option<Expr>, min_prec: u32) -> Result<Expr, ParseError> {
        let mut lhs = if let Some(x) = l {
            x
        } else {
            self.expect_atomic()?
        };

        let mut lookahead = self.lexer.peek().cloned();

        while let Some(op) = lookahead
            && let Some(p) = op.token.to_prec()
            && p >= min_prec
        {
            self.lexer.next();
            let mut rhs = self.expect_atomic()?;
            lookahead = self.lexer.peek().cloned();

            while let Some(n_op) = lookahead.as_ref()
                && let Some(n_p) = n_op.token.to_prec()
                && n_p > p
            {
                rhs = self.parse_expr(Some(rhs), p + 1)?;
                lookahead = self.lexer.peek().cloned();
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

    pub fn expect_peek(&mut self) -> Result<LexedToken, ParseError> {
        if let Some(tk) = self.lexer.peek() {
            Ok(tk.clone())
        } else {
            Err(ParseError::new(None, ParseErrorReason::StreamEnded))
        }
    }

    // consumes target token
    pub fn expect_token(&mut self, token: Token) -> Result<(), ParseError> {
        let tk = self.expect_peek()?;
        if tk.token == token {
            self.lexer.next();
            Ok(())
        } else {
            Err(ParseError::new(
                Some(tk),
                ParseErrorReason::ExpectedToken(token),
            ))
        }
    }

    pub fn expect_identifier(&mut self) -> Result<String, ParseError> {
        let tk = self.expect_peek()?;
        if let Token::IDENT(s) = tk.token {
            self.lexer.next();
            Ok(s.clone())
        } else {
            Err(ParseError::new(
                Some(tk),
                ParseErrorReason::ExpectedIdentifier,
            ))
        }
    }

    pub fn accept_token(&mut self, token: Token) -> bool {
        if let Some(tk) = self.lexer.peek()
            && tk.token == token
        {
            self.lexer.next();
            true
        } else {
            false
        }
    }

    pub fn expect_atomic(&mut self) -> Result<Expr, ParseError> {
        let tk = self.expect_peek()?;

        if let Token::LPAREN = tk.token {
            self.lexer.next();
            let expr = self.parse_expr(None, 0)?;
            self.expect_token(Token::RPAREN)?;
            Ok(expr)
        } else if let Token::NUMBER(n) = tk.token {
            self.lexer.next();
            Ok(Expr::Intermediate(n.into()))
        } else if let Some(uop) = Self::parse_unary(&tk.token) {
            self.lexer.next();
            let u = Unary {
                op: uop,
                x: Box::new(self.expect_atomic()?),
            };
            Ok(Expr::Unary(u))
        } else if let Token::IDENT(s) = tk.token {
            self.lexer.next();
            Ok(Expr::Ident(s.clone()))
        } else {
            Err(ParseError::new(
                Some(tk),
                ParseErrorReason::NonAtomicExpression,
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Parser;
    use crate::ast::Statement;
    use crate::util::TPrint;

    #[test]
    fn parse_fib_and_print_ast() {
        let source = include_str!("ro/basic.ro");
        let mut parser = Parser::new(source.chars());

        let stmts = parser.parse_top().expect("fib.ro should parse");
        //assert_eq!(stmts.len(), 3);
        let blk = Statement::Block(stmts);
        println!("fib.ro AST:");
        blk.tprint();
    }
}
