use super::{Variable, Constant, Expr, Op, BasicOp};

pub mod fixed_num {
    #[derive(Debug, Copy, Clone)]
    pub enum FixedNum {
        Sign(SignNum),
        UnSign(UnSignNum),
        Float(Float),
        Undefined
    }

    #[derive(Debug, Copy, Clone)]
    pub enum SignNum {
        I8(i8), I16(i16), I32(i32), I64(i64), I128(i128), Isize(isize)
    }

    #[derive(Debug, Copy, Clone)]
    pub enum UnSignNum {
        U8(u8), U16(u16), U32(u32), U64(u64), U128(u128), Usize(usize)
    }

    #[derive(Debug, Copy, Clone)]
    pub enum Float {
        F32(f32), F64(f64)
    }
}

#[derive(Debug, Clone)]
pub enum Num<'a> {
    Var(&'a Variable<'a>),
    Cons(&'a Constant<'a>),
    Expr(Box<Expr<'a>>),
    Fixed(fixed_num::FixedNum),
    Undefined
}

impl Num<'_> {
    pub fn drop(self) {
        match self {
            Num::Var(var) => var.drop_name(),
            Num::Cons(cons) => cons.drop_name(),
            _ => ()
        };
        drop(self)
    }
}

impl std::ops::Add for Num<'_> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let expr = Expr::new(self, rhs, Op::Basic(BasicOp::Add));
        Num::Expr(Box::new(expr))
    }
}