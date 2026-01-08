use crate::ast::*;

pub trait Eval {
    fn eval(&self) -> u32;
}
