use std::fmt;

#[derive(Debug)]
pub enum LType {
    Int,
    Float,
}

#[derive(Debug)]
pub enum Statement {
    If {
        guard: Box<Expr>,
        t: Vec<Statement>,
        f: Option<Vec<Statement>>,
    },
    Declare {
        typ: LType,
        ident: String,
        assign: Option<Box<Expr>>,
    },
    Assign {
        ident: String,
        value: Box<Expr>,
    },
    Block(Vec<Statement>),
}

#[derive(Debug)]
pub enum Op {
    Sub,
    Add,
    Div,
    Mul,
}

#[derive(Debug)]
pub enum UOp {
    Neg,
    Pos,
}

#[derive(Debug)]
pub enum Expr {
    Unary(Unary),
    Intermediate(i64),
    Binop(Binop),
    Ident(String),
}

#[derive(Debug)]
pub struct Unary {
    pub op: UOp,
    pub x: Box<Expr>,
}

#[derive(Debug)]
pub struct Binop {
    pub a: Box<Expr>,
    pub op: Op,
    pub b: Box<Expr>,
}

impl fmt::Display for UOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UOp::Neg => write!(f, "-"),
            UOp::Pos => write!(f, "+"),
        }
    }
}

impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Op::Div => write!(f, "/"),
            Op::Mul => write!(f, "*"),
            Op::Sub => write!(f, "-"),
            Op::Add => write!(f, "+"),
        }
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Unary(u) => write!(f, "{}", u.op),
            Expr::Binop(b) => write!(f, "{}", b.op),
            Expr::Intermediate(i) => write!(f, "{}", i),
            Expr::Ident(s) => write!(f, "{}", s),
        }
    }
}

use crate::util::TPrint;

impl TPrint for Expr {
    fn label(&self) -> String {
        self.to_string()
    }

    fn children<'a>(&'a self) -> Box<dyn Iterator<Item = &'a dyn TPrint> + 'a> {
        match self {
            Expr::Binop(bin) => {
                let a: &'a dyn TPrint = bin.a.as_ref();
                let b: &'a dyn TPrint = bin.b.as_ref();
                Box::new([a, b].into_iter())
            }
            Expr::Unary(u) => {
                let e: &'a dyn TPrint = u.x.as_ref();
                Box::new([e].into_iter())
            }
            _ => Box::new([].into_iter()),
        }
    }
}
