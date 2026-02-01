use crate::util::TPrint;
use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum LType {
    Int,
    Float,
}

#[derive(Debug)]
pub enum Statement {
    If(SIf),
    Declare(SDeclare),
    Assign(SAssign),
    While(SWhile),
    Call(SCall),
    Function(SFunction),
    Block(Vec<Statement>),
    Return(Box<Expr>),
}

#[derive(Debug)]
pub struct SIf {
    pub guard: Box<Expr>,
    pub t: Vec<Statement>,
    pub f: Option<Vec<Statement>>,
}

#[derive(Debug)]
pub struct SDeclare {
    pub typ: LType,
    pub ident: String,
    pub assign: Option<Box<Expr>>,
}

#[derive(Debug)]
pub struct SFunction {
    pub ret: LType,
    pub ident: String,
    pub params: Vec<(LType, String)>,
    pub body: Vec<Statement>,
}

#[derive(Debug)]
pub struct SAssign {
    pub ident: String,
    pub value: Box<Expr>,
}

#[derive(Debug)]
pub struct SWhile {
    pub cond: Box<Expr>,
    pub body: Vec<Statement>,
}

#[derive(Debug)]
pub struct SCall {
    pub ident: String,
    pub params: Vec<Expr>,
}

impl fmt::Display for LType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LType::Float => write!(f, "float"),
            LType::Int => write!(f, "int"),
        }
    }
}

impl fmt::Display for Statement {
    fn fmt(&self, ft: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Statement::If(sif) => {
                if let Some(_) = sif.f {
                    write!(ft, "if/else")
                } else {
                    write!(ft, "if")
                }
            }
            Statement::Assign(s) => write!(ft, "assign<{}>", s.ident),
            Statement::Block(_) => write!(ft, ""),
            Statement::Declare(declare) => {
                write!(ft, "declare<{} {}>", declare.typ, declare.ident)
            }
            Statement::Call(c) => {
                write!(ft, "call<{}()>", c.ident)
            }
            Statement::Function(f) => {
                write!(ft, "fn <{}(todo) -> todo>", f.ident)
            }
            Statement::Return(_) => {
                write!(ft, "return")
            }
            Statement::While(_) => write!(ft, "while"),
        }
    }
}

impl TPrint for Statement {
    fn label(&self) -> String {
        self.to_string()
    }

    fn children<'a>(&'a self) -> Box<dyn Iterator<Item = &'a dyn TPrint> + 'a> {
        match self {
            Statement::Declare(declare) => {
                if let Some(expr) = &declare.assign {
                    let ptr: &'a dyn TPrint = expr.as_ref();
                    Box::new([ptr].into_iter())
                } else {
                    Box::new(std::iter::empty())
                }
            }
            Statement::Block(stmts) => {
                let itr = stmts.iter().map(|s| s as &dyn TPrint);
                Box::new(itr)
            }
            Statement::Assign(assign) => {
                let ptr: &'a dyn TPrint = assign.value.as_ref();
                Box::new([ptr].into_iter())
            }
            Statement::If(sif) => {
                let pg: &'a dyn TPrint = sif.guard.as_ref();
                let pt = sif.t.iter().map(|s| s as &dyn TPrint);
                let imm = [pg].into_iter().chain(pt);
                if let Some(ff) = &sif.f {
                    let pf = ff.iter().map(|s| s as &dyn TPrint);
                    Box::new(imm.chain(pf))
                } else {
                    Box::new(imm)
                }
            }
            Statement::Call(c) => {
                let itr = c.params.iter().map(|s| s as &dyn TPrint);
                Box::new(itr)
            }
            Statement::Function(f) => {
                let itr = f.body.iter().map(|s| s as &dyn TPrint);
                Box::new(itr)
            }
            Statement::While(w) => {
                let cond: &'a dyn TPrint = w.cond.as_ref();
                let stmts = w.body.iter().map(|s| s as &dyn TPrint);

                Box::new([cond].into_iter().chain(stmts))
            }
            Statement::Return(expr) => {
                let expr: &'a dyn TPrint = expr.as_ref();
                Box::new([expr].into_iter())
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Op {
    Sub,
    Add,
    Div,
    Mul,
    Lt,
    Gt,
    Eq,
}

#[derive(Debug, Clone, Copy)]
pub enum UOp {
    Neg,
    Pos,
}

//TODO: Function calls
#[derive(Debug)]
pub enum Expr {
    Unary(Unary),
    IntLit(i64),
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
            Op::Lt => write!(f, "<"),
            Op::Gt => write!(f, ">"),
            Op::Eq => write!(f, "=="),
        }
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Unary(u) => write!(f, "{}", u.op),
            Expr::Binop(b) => write!(f, "{}", b.op),
            Expr::IntLit(i) => write!(f, "{}", i),
            Expr::Ident(s) => write!(f, "{}", s),
        }
    }
}

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
