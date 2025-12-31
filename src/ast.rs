#[derive(Debug)]
pub enum Op {
    Sub,
    Add,
    Div,
    Mul,
}

#[derive(Debug)]
pub enum Expr {
    Unary,
    Binop,
}

#[derive(Debug)]
pub enum Unary {
    Intermediate(i32),
}

#[derive(Debug)]
pub struct Binop {
    pub a: Expr,
    pub op: Op,
    pub b: Expr,
}
