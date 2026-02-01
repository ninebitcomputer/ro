use crate::ast::*;
use crate::bast::env::*;

//AST w/symbols resolved

pub struct BAst {
    pub environment: BAstEnv,
    pub statements: Vec<Statement>,
}

pub enum BStmt {
    If(BIf),
    Assign(BAssign),
    While(BWhile),
    Call(BCall),
    Return(Box<BExpr>),
}

pub struct BIf {
    pub guard: Box<BExpr>,
    pub t: BAst,
    pub f: Option<BAst>,
}

pub struct BAssign {
    pub ident: RelVarID,
    pub value: Box<BExpr>,
}

pub struct BWhile {
    pub cond: Box<BExpr>,
    pub body: BAst,
}

pub struct BCall {
    pub ident: RelFnID,
    pub args: Vec<BExpr>,
}

pub enum BExpr {
    Unary(BUnary),
    IntLit(i64),
    FloatLit(f32),
    Binop(BBinop),
    Var(RelVarID),
}

pub struct BUnary {
    pub op: UOp,
    pub expr: Box<BExpr>,
}

pub struct BBinop {
    pub a: Box<BExpr>,
    pub op: Op,
    pub b: Box<BExpr>,
}

impl BExpr {
    pub fn default_value(typ: LType) -> BExpr {
        match typ {
            LType::Int => BExpr::IntLit(0),
            LType::Float => BExpr::FloatLit(0.0),
        }
    }
}
