use super::{Variable, Constant, Expr, fixed_num::FixedNum, Op, BasicOp};

#[derive(Debug, Clone)]
pub enum Num<'a> {
    Var(&'a Variable<'a>),
    Cons(&'a Constant<'a>),
    Expr(Box<Expr<'a>>),
    Fixed(FixedNum),
}

impl std::ops::Add for Num<'_> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Num::Expr(Box::new(Expr::new(self, rhs, Op::Basic(BasicOp::Add))))
    }
}