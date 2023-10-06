use super::{Variable, Constant, Expr, fixed_num::FixedNum, Op, BasicOp, EXPR_LIST};

#[derive(Debug, Clone)]
pub enum Num<'a> {
    Var(&'a Variable<'a>),
    Cons(&'a Constant<'a>),
    Expr(&'a Expr<'a>),
    Fixed(FixedNum),
}

impl<'a:'b, 'b> std::ops::Add for Num<'a> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let expr: Expr<'a> = Expr::new(self, rhs, Op::Basic(BasicOp::Add));
        unsafe {
            EXPR_LIST.push(expr);
            Num::Expr(EXPR_LIST.last().unwrap())
        }
        
    }
}