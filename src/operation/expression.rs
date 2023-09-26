use super::num_type::{Constant, Variable, fixed_num::*};
use super::traits::{Val, Prt};

#[derive(Debug)]
pub enum BasicOp {
    Add, // a+b
    Sub, // a-b
    Mul, // a*b
    Div, // a/b
    Exp, // a^b
    Log, // a,b
}

#[derive(Debug)]
pub enum Op {
    Basic(BasicOp)
}

#[derive(Debug)]
pub enum Num<'a> {
    Var(&'a str),
    Cons(&'a str),
    Expr(&'a str),
    Fixed(FixedNum),
}

#[derive(Debug)]
pub struct Expr<'a> {
    a: Num<'a>,
    b: Num<'a>,
    expr_type: Op
}

impl<'a> Expr<'a> {
    pub fn new(a: Num<'a>, b: Num<'a>, expr_type: Op) -> Self {
        Self {
            a, b, expr_type
        }
    }
}

impl Val for Expr<'_> {
    fn val(&self) -> FixedNum {
        FixedNum::Undefined
    }
}

impl Prt for Expr<'_> {
    fn print(&self) -> String {
        format!("Expr< a:{:?}, b:{:?}, operation:{:?} >", self.a, self.b, self.expr_type)
    }
}

impl std::fmt::Display for Expr<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.print())
    }
}