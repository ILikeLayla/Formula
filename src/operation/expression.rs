use super::num_type::Num;
use super::traits::{Val, Prt};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub enum BasicOp {
    Add, // a+b
    Sub, // a-b
    Mul, // a*b
    Div, // a/b
    Exp, // a^b
    Log, // a,b
}

#[derive(Debug, Clone)]
pub enum Op {
    Basic(BasicOp)
}

#[derive(Debug, Clone)]
pub struct Expr<'a> {
    a: RefCell<Num<'a>>,
    b: RefCell<Num<'a>>,
    expr_type: Op
}

impl<'a> Expr<'a> {
    pub fn new(a: Num<'a>, b: Num<'a>, expr_type: Op) -> Self {
        let a = RefCell::new(a);
        let b = RefCell::new(b);
        Self {
            a, b, expr_type
        }
    }

    pub fn add(&self, rhs: Num<'a>) -> Self {
        Expr::new(Num::Expr(Box::new(self.clone())), rhs, Op::Basic(BasicOp::Add))
    }

    pub fn sub(&self, rhs: Num<'a>) -> Self {
        Expr::new(Num::Expr(Box::new(self.clone())), rhs, Op::Basic(BasicOp::Sub))
    }

    pub fn mul(&self, rhs: Num<'a>) -> Self {
        Expr::new(Num::Expr(Box::new(self.clone())), rhs, Op::Basic(BasicOp::Mul))
    }
    
    pub fn div(&self, rhs: Num<'a>) -> Self {
        Expr::new(Num::Expr(Box::new(self.clone())), rhs, Op::Basic(BasicOp::Div))
    }
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

impl<'a> std::ops::Add for &'a Expr<'a> {
    type Output = Expr<'a>;
    fn add(self, rhs: Self) -> Self::Output {
        self.add(Num::Expr(Box::new(rhs.clone())))
    }
}

impl<'a> std::ops::Sub for &'a Expr<'a> {
    type Output = Expr<'a>;
    fn sub(self, rhs: Self) -> Self::Output {
        self.sub(Num::Expr(Box::new(rhs.clone())))
    }
}

impl<'a> std::ops::Mul for &'a Expr<'a> {
    type Output = Expr<'a>;
    fn mul(self, rhs: Self) -> Self::Output {
        self.mul(Num::Expr(Box::new(rhs.clone())))
    }
}

impl<'a> std::ops::Div for &'a Expr<'a> {
    type Output = Expr<'a>;
    fn div(self, rhs: Self) -> Self::Output {
        self.div(Num::Expr(Box::new(rhs.clone())))
    }
}