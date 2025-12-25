use crate::ast::*;

pub trait Eval {
    fn eval(&self) -> u32;
}

impl Eval for Atom {
    fn eval(&self) -> u32 {
        match self {
            Atom::Paren(t0) => t0.eval(),
            Atom::Number(d) => *d,
        }
    }
}

impl Eval for T0 {
    fn eval(&self) -> u32 {
        match self {
            T0::T1(t1) => t1.eval(),
            T0::Add(add) => add.a.eval() + add.b.eval(),
            T0::Sub(sub) => sub.a.eval() - sub.b.eval(),
        }
    }
}

impl Eval for T1 {
    fn eval(&self) -> u32 {
        match self {
            T1::Atom(t1) => t1.eval(),
            T1::Mul(mul) => mul.a.eval() * mul.b.eval(),
            T1::Div(div) => div.a.eval() / div.b.eval(),
        }
    }
}
