use super::num_type::Num;
use super::traits::{Val, Prt};
use super::op::*;
use std::cell::RefCell;
use std::rc::Rc;



#[derive(Debug, Clone)]
pub struct Expr<'a> {
    a: Num<'a>,
    b: Num<'a>,
    expr_type: Op
}

impl<'a> Expr<'a> {
    pub fn new(a: Num<'a>, b: Num<'a>, expr_type: Op) -> Num<'a> {
        Num::Expr(Box::new(Self {
            a, b, expr_type
        }))
    }

    // pub fn add(&self, rhs: Num<'a>) -> Num<'a>{
    //     Expr::new(Num::Expr(Box::new(self.clone())), rhs, Op::Basic(BasicOp::Add))
    // }

    // pub fn sub(&self, rhs: Num<'a>) -> Num<'a> {
    //     Expr::new(Num::Expr(Box::new(self.clone())), rhs, Op::Basic(BasicOp::Sub))
    // }

    // pub fn mul(&self, rhs: Num<'a>) -> Num<'a> {
    //     Expr::new(Num::Expr(Box::new(self.clone())), rhs, Op::Basic(BasicOp::Mul))
    // }
    
    // pub fn div(&self, rhs: Num<'a>) -> Num<'a> {
    //     Expr::new(Num::Expr(Box::new(self.clone())), rhs, Op::Basic(BasicOp::Div))
    // }
}

// impl Val for Expr<'_> {
//     fn val(&self) -> FixedNum {
//         FixedNum::Undefined
//     }
// }

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

// impl<'a> std::ops::Add for &'a Expr<'a> {
//     type Output = Num<'a>;
//     fn add(self, rhs: Self) -> Self::Output {
//         self.add(Num::Expr(Box::new(rhs.clone())))
//     }
// }

// impl<'a> std::ops::Sub for &'a Expr<'a> {
//     type Output = Num<'a>;
//     fn sub(self, rhs: Self) -> Self::Output {
//         self.sub(Num::Expr(Box::new(rhs.clone())))
//     }
// }

// impl<'a> std::ops::Mul for &'a Expr<'a> {
//     type Output = Num<'a>;
//     fn mul(self, rhs: Self) -> Self::Output {
//         self.mul(Num::Expr(Box::new(rhs.clone())))
//     }
// }

// impl<'a> std::ops::Div for &'a Expr<'a> {
//     type Output = Num<'a>;
//     fn div(self, rhs: Self) -> Self::Output {
//         self.div(Num::Expr(Box::new(rhs.clone())))
//     }
// }