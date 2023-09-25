use crate::traits::Prt;
use super::{fixed_num::*, Expr, traits::Val, Op, BasicOp, Num, Constant};

#[derive(Debug)]
pub enum ChangeNum<'a> {
    Var(&'a Variable<'a>),
    Expr(Expr<'a>),
    Undefined,
}

#[derive(Debug)]
pub struct Variable<'a> {
    name: &'a str,
    expr: Box<ChangeNum<'a>>
}

impl<'a> Variable<'a> {
    pub fn new(name:&'a str, expr: ChangeNum<'a>) -> Self {
        Self {
            expr: Box::new(expr),
            name
        }
    }
}

impl Val for Variable<'_> {
    fn val(&self) -> FixedNum {
        match &(*self.expr) {
            ChangeNum::Var(num) => num.val().clone(),
            ChangeNum::Expr(expr) => expr.val().clone(),
            ChangeNum::Undefined => FixedNum::Undefined,
        }
    }
}

impl Prt for Variable<'_> {
    fn print(&self) -> String {
        format!("Variable< name:{}, expr:{:?} >", self.name, self.val())
    }
}

impl<'a> std::fmt::Display for Variable<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.print())
    }
}