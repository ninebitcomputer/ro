use std::fmt;

#[derive(Debug)]
pub enum Op {
    Sub,
    Add,
    Div,
    Mul,
}

#[derive(Debug)]
pub enum Expr {
    Unary(Unary),
    Intermediate(i64),
    Binop(Binop),
}

#[derive(Debug)]
pub enum Unary {}

#[derive(Debug)]
pub struct Binop {
    pub a: Box<Expr>,
    pub op: Op,
    pub b: Box<Expr>,
}

impl Expr {}
