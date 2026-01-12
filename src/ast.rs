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

impl Expr {
    pub fn pprint(&self) {
        let mut v: Vec<bool> = Vec::new();
        self._pprint(&mut v, true);
    }

    fn _pprint(&self, stack: &mut Vec<bool>, last: bool) {
        use crate::util::BoolStrMap;
        stack.push(last);

        let (end, pfx) = stack.split_last().unwrap();

        let s = if *end { "└─" } else { "├─" };

        println!("{}{}{}", BoolStrMap::new(pfx, "  ", "│ "), s, self);

        match self {
            Expr::Binop(b) => {
                b.a._pprint(stack, false);
                b.b._pprint(stack, true);
            }
            Expr::Unary(u) => {
                u.x._pprint(stack, true);
            }
            _ => (),
        };

        stack.pop();
    }
}
