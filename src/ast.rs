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
    Binop(Binop),
}

#[derive(Debug)]
pub enum Unary {
    Intermediate(i64),
}

#[derive(Debug)]
pub struct Binop {
    pub a: Box<Expr>,
    pub op: Op,
    pub b: Box<Expr>,
}
