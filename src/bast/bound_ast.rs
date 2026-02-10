use crate::ast::*;
use crate::bast::env::*;

//AST w/symbols resolved

#[derive(Debug)]
pub enum TypeError {
    WrongType,
}

#[derive(Debug)]
pub struct BAst {
    pub environment: BAstEnv,
    pub statements: Vec<BStmt>,
}

impl BAst {
    pub fn new() -> Self {
        Self {
            environment: BAstEnv::new(),
            statements: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub enum BStmt {
    If(BIf),
    Assign(BAssign),
    While(BWhile),
    Call(BCall),
    Return(Box<AnnotatedExpr>),
    Block(Box<BAst>),
}

#[derive(Debug)]
pub struct BIf {
    pub guard: Box<AnnotatedExpr>,
    pub t: BAst,
    pub f: Option<BAst>,
}

#[derive(Debug)]
pub struct BAssign {
    pub ident: RelVarID,
    pub value: Box<AnnotatedExpr>,
}

#[derive(Debug)]
pub struct BWhile {
    pub cond: Box<AnnotatedExpr>,
    pub body: BAst,
}

#[derive(Debug)]
pub struct BCall {
    pub ident: RelFnID,
    pub args: Vec<AnnotatedExpr>,
}

#[derive(Debug)]
pub struct AnnotatedExpr {
    pub typ: LType,
    pub body: BExpr,
}

#[derive(Debug)]
pub enum BExpr {
    Unary(BUnary),
    IntLit(i64),
    FloatLit(f32),
    Binop(BBinop),
    Var(RelVarID),
}

#[derive(Debug)]
pub struct BUnary {
    pub op: UOp,
    pub expr: Box<AnnotatedExpr>,
}

#[derive(Debug)]
pub struct BBinop {
    pub a: Box<AnnotatedExpr>,
    pub op: Op,
    pub b: Box<AnnotatedExpr>,
}

impl BExpr {
    pub fn default_value(typ: LType) -> BExpr {
        match typ {
            LType::Int => BExpr::IntLit(0),
            LType::Float => BExpr::FloatLit(0.0),
        }
    }
}

impl AnnotatedExpr {
    pub fn cast_to(&mut self, typ: LType) -> Result<(), TypeError> {
        if self.typ == typ {
            Ok(())
        } else {
            Err(TypeError::WrongType)
        }
    }
}
