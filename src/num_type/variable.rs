use crate::traits::Prt;
use super::{fixed_num::*, Expr, traits::Val, Op, BasicOp, Num, Constant};
use std::cell::RefCell;

#[derive(Debug, Clone, Copy)]
pub enum ChangeNum<'a> {
    Var(&'a Variable<'a>),
    Expr(&'a Expr<'a>),
    Undefined,
}

#[derive(Debug, Clone)]
pub struct Variable<'a> {
    name: &'a str,
    expr: RefCell<ChangeNum<'a>>
}

impl<'a> Variable<'a> {
    pub fn new(name:&'a str, expr: ChangeNum<'a>) -> Self {
        Self {
            expr: RefCell::new(expr),
            name
        }
    }
}

// impl Val for Variable<'_> {
//     fn val(&self) -> FixedNum {
//         match self.expr {
//             ChangeNum::Var(num) => num.val().clone(),
//             ChangeNum::Expr(expr) => expr.val().clone(),
//             ChangeNum::Undefined => FixedNum::Undefined,
//         }
//     }
// }

impl Prt for Variable<'_> {
    fn print(&self) -> String {
        // format!("Variable< name:{}, expr:{:?} >", self.name, self.val())
        format!("Variable< name:{} >", self.name )
    }
}

impl<'a> std::fmt::Display for Variable<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.print())
    }
}

// impl<'a> std::ops::Add for &Variable<'a> {
//     type Output = Num<'a>;
//     fn add(self, rhs: Self) -> Self::Output {
//         Num::Expr(())
//     }
// }