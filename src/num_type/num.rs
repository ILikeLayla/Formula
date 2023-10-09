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

    impl Float {
        pub fn add(&self, rhs: Self) -> Self {
            match (self, rhs) {
                (&Float::F64(a), Float::F64(b)) => Float::F64(a + b),
                (&Float::F32(a), Float::F32(b)) => Float::F32(a + b),
                (&Float::F64(a), Float::F32(b)) => Float::F64(a + b as f64),
                (&Float::F32(a), Float::F64(b)) => Float::F64(b + a as f64),
            }
        }

        pub fn sub(&self, rhs: Self) -> Self {
            match (self, rhs) {
                (&Float::F64(a), Float::F64(b)) => Float::F64(a - b),
                (&Float::F32(a), Float::F32(b)) => Float::F32(a - b),
                (&Float::F64(a), Float::F32(b)) => Float::F64(a - b as f64),
                (&Float::F32(a), Float::F64(b)) => Float::F64(b - a as f64),
            }
        }

        pub fn mul(&self, rhs: Self) -> Self {
            match (self, rhs) {
                (&Float::F64(a), Float::F64(b)) => Float::F64(a * b),
                (&Float::F32(a), Float::F32(b)) => Float::F32(a * b),
                (&Float::F64(a), Float::F32(b)) => Float::F64(a * b as f64),
                (&Float::F32(a), Float::F64(b)) => Float::F64(b * a as f64),
            }
        }

        pub fn div(&self, rhs: Self) -> Self {
            match (self, rhs) {
                (&Float::F64(a), Float::F64(b)) => Float::F64(a / b),
                (&Float::F32(a), Float::F32(b)) => Float::F32(a / b),
                (&Float::F64(a), Float::F32(b)) => Float::F64(a / b as f64),
                (&Float::F32(a), Float::F64(b)) => Float::F64(b / a as f64),
            }
        }
    }

    impl UnSignNum {
        pub fn add(&self, rhs: Self) -> Self {
            match (self, rhs) {
                (&UnSignNum::U8(a), UnSignNum::U8(b)) => UnSignNum::U8               (a + b),
                (&UnSignNum::U16(a), UnSignNum::U16(b)) => UnSignNum::U16          (a + b),
                (&UnSignNum::U32(a), UnSignNum::U32(b)) => UnSignNum::U32          (a + b),
                (&UnSignNum::U64(a), UnSignNum::U64(b)) => UnSignNum::U64          (a + b),
                (&UnSignNum::U128(a), UnSignNum::U128(b)) => UnSignNum::U128     (a + b),
                (&UnSignNum::Usize(a), UnSignNum::Usize(b)) => UnSignNum::Usize(a + b),

                (&UnSignNum::U8(b), UnSignNum::U16(a)) => UnSignNum::U16            (a + b as u16),
                (&UnSignNum::U8(b), UnSignNum::U32(a)) => UnSignNum::U32            (a + b as u32),
                (&UnSignNum::U8(b), UnSignNum::U64(a)) => UnSignNum::U64            (a + b as u64),
                (&UnSignNum::U8(b), UnSignNum::U128(a)) => UnSignNum::U128         (a + b as u128),
                (&UnSignNum::U8(b), UnSignNum::Usize(a)) => UnSignNum::Usize      (a + b as usize),

                (&UnSignNum::U16(b), UnSignNum::U8(a)) => UnSignNum::U16            (b + a as u16),
                (&UnSignNum::U16(b), UnSignNum::U32(a)) => UnSignNum::U32          (a + b as u32),
                (&UnSignNum::U16(b), UnSignNum::U64(a)) => UnSignNum::U64          (a + b as u64),
                (&UnSignNum::U16(b), UnSignNum::U128(a)) => UnSignNum::U128       (a + b as u128),
                (&UnSignNum::U16(b), UnSignNum::Usize(a)) => UnSignNum::Usize    (a + b as usize),

                (&UnSignNum::U32(b), UnSignNum::U8(a)) => UnSignNum::U32            (b + a as u32),
                (&UnSignNum::U32(b), UnSignNum::U16(a)) => UnSignNum::U32          (b + a as u32),
                (&UnSignNum::U32(b), UnSignNum::U64(a)) => UnSignNum::U64          (a + b as u64),
                (&UnSignNum::U32(b), UnSignNum::U128(a)) => UnSignNum::U128       (a + b as u128),
                (&UnSignNum::U32(b), UnSignNum::Usize(a)) => UnSignNum::Usize    (a + b as usize),

                (&UnSignNum::U64(b), UnSignNum::U8(a)) => UnSignNum::U64            (b + a as u64),
                (&UnSignNum::U64(b), UnSignNum::U16(a)) => UnSignNum::U64          (b + a as u64),
                (&UnSignNum::U64(b), UnSignNum::U32(a)) => UnSignNum::U64          (b + a as u64),
                (&UnSignNum::U64(b), UnSignNum::U128(a)) => UnSignNum::U128       (a + b as u128),
                (&UnSignNum::U64(b), UnSignNum::Usize(a)) => UnSignNum::Usize    (a + b as usize),

                (&UnSignNum::U128(b), UnSignNum::U8(a)) => UnSignNum::U128         (b + a as u128),
                (&UnSignNum::U128(b), UnSignNum::U16(a)) => UnSignNum::U128       (b + a as u128),
                (&UnSignNum::U128(b), UnSignNum::U32(a)) => UnSignNum::U128       (b + a as u128),
                (&UnSignNum::U128(b), UnSignNum::U64(a)) => UnSignNum::U128       (b + a as u128),
                (&UnSignNum::U128(b), UnSignNum::Usize(a)) => UnSignNum::U128   (b + a as u128),

                (&UnSignNum::Usize(b), UnSignNum::U8(a)) => UnSignNum::Usize      (b + a as usize),
                (&UnSignNum::Usize(b), UnSignNum::U16(a)) => UnSignNum::Usize    (b + a as usize),
                (&UnSignNum::Usize(b), UnSignNum::U32(a)) => UnSignNum::Usize    (b + a as usize),
                (&UnSignNum::Usize(b), UnSignNum::U64(a)) => UnSignNum::Usize    (b + a as usize),
                (&UnSignNum::Usize(b), UnSignNum::U128(a)) => UnSignNum::U128   (a + b as u128),
            }
        }

        pub fn sub(&self, rhs: Self) -> Self {
            match (self, rhs) {
                (&UnSignNum::U8(a), UnSignNum::U8(b)) => UnSignNum::U8               (a - b),
                (&UnSignNum::U16(a), UnSignNum::U16(b)) => UnSignNum::U16          (a - b),
                (&UnSignNum::U32(a), UnSignNum::U32(b)) => UnSignNum::U32          (a - b),
                (&UnSignNum::U64(a), UnSignNum::U64(b)) => UnSignNum::U64          (a - b),
                (&UnSignNum::U128(a), UnSignNum::U128(b)) => UnSignNum::U128     (a - b),
                (&UnSignNum::Usize(a), UnSignNum::Usize(b)) => UnSignNum::Usize(a - b),

                (&UnSignNum::U8(b), UnSignNum::U16(a)) => UnSignNum::U16            (a - b as u16),
                (&UnSignNum::U8(b), UnSignNum::U32(a)) => UnSignNum::U32            (a - b as u32),
                (&UnSignNum::U8(b), UnSignNum::U64(a)) => UnSignNum::U64            (a - b as u64),
                (&UnSignNum::U8(b), UnSignNum::U128(a)) => UnSignNum::U128         (a - b as u128),
                (&UnSignNum::U8(b), UnSignNum::Usize(a)) => UnSignNum::Usize      (a - b as usize),

                (&UnSignNum::U16(b), UnSignNum::U8(a)) => UnSignNum::U16            (b - a as u16),
                (&UnSignNum::U16(b), UnSignNum::U32(a)) => UnSignNum::U32          (a - b as u32),
                (&UnSignNum::U16(b), UnSignNum::U64(a)) => UnSignNum::U64          (a - b as u64),
                (&UnSignNum::U16(b), UnSignNum::U128(a)) => UnSignNum::U128       (a - b as u128),
                (&UnSignNum::U16(b), UnSignNum::Usize(a)) => UnSignNum::Usize    (a - b as usize),

                (&UnSignNum::U32(b), UnSignNum::U8(a)) => UnSignNum::U32            (b - a as u32),
                (&UnSignNum::U32(b), UnSignNum::U16(a)) => UnSignNum::U32          (b - a as u32),
                (&UnSignNum::U32(b), UnSignNum::U64(a)) => UnSignNum::U64          (a - b as u64),
                (&UnSignNum::U32(b), UnSignNum::U128(a)) => UnSignNum::U128       (a - b as u128),
                (&UnSignNum::U32(b), UnSignNum::Usize(a)) => UnSignNum::Usize    (a - b as usize),

                (&UnSignNum::U64(b), UnSignNum::U8(a)) => UnSignNum::U64            (b - a as u64),
                (&UnSignNum::U64(b), UnSignNum::U16(a)) => UnSignNum::U64          (b - a as u64),
                (&UnSignNum::U64(b), UnSignNum::U32(a)) => UnSignNum::U64          (b - a as u64),
                (&UnSignNum::U64(b), UnSignNum::U128(a)) => UnSignNum::U128       (a - b as u128),
                (&UnSignNum::U64(b), UnSignNum::Usize(a)) => UnSignNum::Usize    (a - b as usize),

                (&UnSignNum::U128(b), UnSignNum::U8(a)) => UnSignNum::U128         (b - a as u128),
                (&UnSignNum::U128(b), UnSignNum::U16(a)) => UnSignNum::U128       (b - a as u128),
                (&UnSignNum::U128(b), UnSignNum::U32(a)) => UnSignNum::U128       (b - a as u128),
                (&UnSignNum::U128(b), UnSignNum::U64(a)) => UnSignNum::U128       (b - a as u128),
                (&UnSignNum::U128(b), UnSignNum::Usize(a)) => UnSignNum::U128   (b - a as u128),

                (&UnSignNum::Usize(b), UnSignNum::U8(a)) => UnSignNum::Usize      (b - a as usize),
                (&UnSignNum::Usize(b), UnSignNum::U16(a)) => UnSignNum::Usize    (b - a as usize),
                (&UnSignNum::Usize(b), UnSignNum::U32(a)) => UnSignNum::Usize    (b - a as usize),
                (&UnSignNum::Usize(b), UnSignNum::U64(a)) => UnSignNum::Usize    (b - a as usize),
                (&UnSignNum::Usize(b), UnSignNum::U128(a)) => UnSignNum::U128   (a - b as u128),
            }
        }
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

    pub fn cal(&self) -> fixed_num::FixedNum {
        match self {
            Num::Cons(cons) => cons.cal(),
            Num::Fixed(fix) => fix.clone(),
            Num::Var(var) => var.cal(),
            Num::Undefined => fixed_num::FixedNum::Undefined,
            Num::Expr(expr) => {

            }
        }
    }
}

impl std::ops::Add for Num<'_> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Expr::new(self, rhs, Op::Basic(BasicOp::Add))
    }
}

impl std::ops::Sub for Num<'_> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Expr::new(self, rhs, Op::Basic(BasicOp::Sub))
    }
}

impl std::ops::Mul for Num<'_> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Expr::new(self, rhs, Op::Basic(BasicOp::Mul))
    }
}

impl std::ops::Div for Num<'_> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Expr::new(self, rhs, Op::Basic(BasicOp::Div))
    }
}