#[derive(Debug)]
pub enum T0 {
    Add(Box<Add>),
    Sub(Box<Sub>),
    T1(Box<T1>),
}

#[derive(Debug)]
pub enum T1 {
    Mul(Box<Mul>),
    Div(Box<Div>),
    Atom(Box<Atom>),
}

#[derive(Debug)]
pub struct Add {
    pub a: T0,
    pub b: T1,
}

#[derive(Debug)]
pub struct Sub {
    pub a: T0,
    pub b: T1,
}

#[derive(Debug)]
pub struct Mul {
    pub a: T1,
    pub b: Atom,
}

#[derive(Debug)]
pub struct Div {
    pub a: T1,
    pub b: Atom,
}

#[derive(Debug)]
pub enum Atom {
    Paren(T0),
    Number(u32),
}
