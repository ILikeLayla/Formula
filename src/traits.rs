use super::num_type::{Constant, Variable, num::*};
use super::operation::Expr;

pub use prt::Prt;
pub use val::Val;

mod prt {
    use super::*;

    pub trait Prt {
        fn print(&self) -> String;
    }




}

mod val {
    use super::*;

    pub trait Val {
        fn val(&self) -> FixedNum;
    }

    impl Val for u8 {
        fn val(&self) -> FixedNum {
            FixedNum::UnSign(UnSignNum::U8(*self))
        }
    }

    impl Val for u16 {
        fn val(&self) -> FixedNum {
            FixedNum::UnSign(UnSignNum::U16(*self))
        }
    }

    impl Val for u32 {
        fn val(&self) -> FixedNum {
            FixedNum::UnSign(UnSignNum::U32(*self))
        }
    }

    impl Val for u64 {
        fn val(&self) -> FixedNum {
            FixedNum::UnSign(UnSignNum::U64(*self))
        }
    }

    impl Val for u128 {
        fn val(&self) -> FixedNum {
            FixedNum::UnSign(UnSignNum::U128(*self))
        }
    }

    impl Val for usize {
        fn val(&self) -> FixedNum {
            FixedNum::UnSign(UnSignNum::Usize(*self))
        }
    }

    impl Val for i8 {
        fn val(&self) -> FixedNum {
            FixedNum::Sign(SignNum::I8(*self))
        }
    }

    impl Val for i16 {
        fn val(&self) -> FixedNum {
            FixedNum::Sign(SignNum::I16(*self))
        }
    }

    impl Val for i32 {
        fn val(&self) -> FixedNum {
            FixedNum::Sign(SignNum::I32(*self))
        }
    }

    impl Val for i64 {
        fn val(&self) -> FixedNum {
            FixedNum::Sign(SignNum::I64(*self))
        }
    }

    impl Val for i128 {
        fn val(&self) -> FixedNum {
            FixedNum::Sign(SignNum::I128(*self))
        }
    }

    impl Val for isize {
        fn val(&self) -> FixedNum {
            FixedNum::Sign(SignNum::Isize(*self))
        }
    }

    impl Val for f32 {
        fn val(&self) -> FixedNum {
            FixedNum::Float(Float::F32(*self))
        }
    }

    impl Val for f64 {
        fn val(&self) -> FixedNum {
            FixedNum::Float(Float::F64(*self))
        }
    }
}
