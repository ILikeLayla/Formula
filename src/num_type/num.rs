use crate::{warn, val::Val};

use super::{Variable, Constant, Expr, Op, BasicOp, count};

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

    impl std::fmt::Display for SignNum {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                SignNum::I8(num) => write!(f, "{}", num),
                SignNum::I16(num) => write!(f, "{}", num),
                SignNum::I32(num) => write!(f, "{}", num),
                SignNum::I64(num) => write!(f, "{}", num),
                SignNum::I128(num) => write!(f, "{}", num),
                SignNum::Isize(num) => write!(f, "{}", num),
            }
        }
    }

    impl std::fmt::Display for UnSignNum {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                UnSignNum::U8(num) => write!(f, "{}", num),
                UnSignNum::U16(num) => write!(f, "{}", num),
                UnSignNum::U32(num) => write!(f, "{}", num),
                UnSignNum::U64(num) => write!(f, "{}", num),
                UnSignNum::U128(num) => write!(f, "{}", num),
                UnSignNum::Usize(num) => write!(f, "{}", num),
            }
        }
    }

    impl std::fmt::Display for Float {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Float::F32(num) => write!(f, "{}", num),
                Float::F64(num) => write!(f, "{}", num),
            }
        }
    }

    impl std::fmt::Display for FixedNum {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                FixedNum::Sign(num) => write!(f, "{}", num),
                FixedNum::UnSign(num) => write!(f, "{}", num),
                FixedNum::Float(num) => write!(f, "{}", num),
                FixedNum::Undefined => write!(f, "UNDEFINED"),
            }
        }
    }

    impl Float {
        pub fn add(&self, rhs: Self) -> Self {
            match (self, rhs) {
                (&Float::F32(a), b) => match b {
                    Float::F32(b) => Float::F32(a + b),
                    Float::F64(b) => Float::F64(b + a as f64),
                }

                (&Float::F64(a), b) => match b {
                    Float::F32(b) => Float::F64(a + b as f64),
                    Float::F64(b) => Float::F64(b + a as f64),
                }
            }
        }

        pub fn sub(&self, rhs: Self) -> Self {
            match (self, rhs) {
                (&Float::F32(a), b) => match b {
                    Float::F32(b) => Float::F32(a - b),
                    Float::F64(b) => Float::F64(b - a as f64),
                }

                (&Float::F64(a), b) => match b {
                    Float::F32(b) => Float::F64(a - b as f64),
                    Float::F64(b) => Float::F64(b - a as f64),
                }
            }
        }

        pub fn mul(&self, rhs: Self) -> Self {
            match (self, rhs) {
                (&Float::F32(a), b) => match b {
                    Float::F32(b) => Float::F32(a * b),
                    Float::F64(b) => Float::F64(b * a as f64),
                }

                (&Float::F64(a), b) => match b {
                    Float::F32(b) => Float::F64(a * b as f64),
                    Float::F64(b) => Float::F64(b * a as f64),
                }
            }
        }

        pub fn div(&self, rhs: Self) -> Self {
            match (self, rhs) {
                (&Float::F32(a), b) => match b {
                    Float::F32(b) => Float::F32(a / b),
                    Float::F64(b) => Float::F64(b / a as f64),
                }

                (&Float::F64(a), b) => match b {
                    Float::F32(b) => Float::F64(a / b as f64),
                    Float::F64(b) => Float::F64(b / a as f64),
                }
            }
        }
    }

    impl UnSignNum {
        pub fn add(&self, rhs: Self) -> Self {
            match (self, rhs) {
                (&UnSignNum::U8(a), b) => match b {
                    UnSignNum::U8(b) => UnSignNum::U8               (b + a),
                    UnSignNum::U16(b) => UnSignNum::U16            (b + a as u16),
                    UnSignNum::U32(b) => UnSignNum::U32            (b + a as u32),
                    UnSignNum::U64(b) => UnSignNum::U64            (b + a as u64),
                    UnSignNum::U128(b) => UnSignNum::U128         (b + a as u128),
                    UnSignNum::Usize(b) => UnSignNum::Usize      (b + a as usize),
                }

                (&UnSignNum::U16(a), b) => match b {
                    UnSignNum::U8(b) => UnSignNum::U16              (a + b as u16),
                    UnSignNum::U16(b) => UnSignNum::U16            (b + a as u16),
                    UnSignNum::U32(b) => UnSignNum::U32            (b + a as u32),
                    UnSignNum::U64(b) => UnSignNum::U64            (b + a as u64),
                    UnSignNum::U128(b) => UnSignNum::U128         (b + a as u128),
                    UnSignNum::Usize(b) => UnSignNum::Usize      (b + a as usize),
                }

                (&UnSignNum::U32(a), b) => match b {
                    UnSignNum::U8(b) => UnSignNum::U32              (a + b as u32),
                    UnSignNum::U16(b) => UnSignNum::U32            (a + b as u32),
                    UnSignNum::U32(b) => UnSignNum::U32            (b + a as u32),
                    UnSignNum::U64(b) => UnSignNum::U64            (b + a as u64),
                    UnSignNum::U128(b) => UnSignNum::U128         (b + a as u128),
                    UnSignNum::Usize(b) => UnSignNum::Usize      (b + a as usize),
                }

                (&UnSignNum::U64(a), b) => match b {
                    UnSignNum::U8(b) => UnSignNum::U64              (a + b as u64),
                    UnSignNum::U16(b) => UnSignNum::U64            (a + b as u64),
                    UnSignNum::U32(b) => UnSignNum::U64            (a + b as u64),
                    UnSignNum::U64(b) => UnSignNum::U64            (b + a as u64),
                    UnSignNum::U128(b) => UnSignNum::U128         (b + a as u128),
                    UnSignNum::Usize(b) => UnSignNum::Usize      (b + a as usize),
                }

                (&UnSignNum::U128(a), b) => match b {
                    UnSignNum::U8(b) => UnSignNum::U128             (a + b as u128),
                    UnSignNum::U16(b) => UnSignNum::U128           (a + b as u128),
                    UnSignNum::U32(b) => UnSignNum::U128           (a + b as u128),
                    UnSignNum::U64(b) => UnSignNum::U128           (a + b as u128),
                    UnSignNum::U128(b) => UnSignNum::U128         (b + a as u128),
                    UnSignNum::Usize(b) => UnSignNum::Usize      (b + a as usize),
                }

                (&UnSignNum::Usize(a), b) => match b {
                    UnSignNum::U8(b) => UnSignNum::Usize            (a + b as usize),
                    UnSignNum::U16(b) => UnSignNum::Usize          (a + b as usize),
                    UnSignNum::U32(b) => UnSignNum::Usize          (a + b as usize),
                    UnSignNum::U64(b) => UnSignNum::Usize          (a + b as usize),
                    UnSignNum::U128(b) => UnSignNum::U128         (b + a as u128),
                    UnSignNum::Usize(b) => UnSignNum::Usize      (b + a as usize),
                }
            }
        }

        pub fn sub(&self, rhs: Self) -> Self {
            match (self, rhs) {
                (&UnSignNum::U8(a), b) => match b {
                    UnSignNum::U8(b) => UnSignNum::U8               (b - a),
                    UnSignNum::U16(b) => UnSignNum::U16            (b - a as u16),
                    UnSignNum::U32(b) => UnSignNum::U32            (b - a as u32),
                    UnSignNum::U64(b) => UnSignNum::U64            (b - a as u64),
                    UnSignNum::U128(b) => UnSignNum::U128         (b - a as u128),
                    UnSignNum::Usize(b) => UnSignNum::Usize      (b - a as usize),
                }

                (&UnSignNum::U16(a), b) => match b {
                    UnSignNum::U8(b) => UnSignNum::U16              (a - b as u16),
                    UnSignNum::U16(b) => UnSignNum::U16            (b - a as u16),
                    UnSignNum::U32(b) => UnSignNum::U32            (b - a as u32),
                    UnSignNum::U64(b) => UnSignNum::U64            (b - a as u64),
                    UnSignNum::U128(b) => UnSignNum::U128         (b - a as u128),
                    UnSignNum::Usize(b) => UnSignNum::Usize      (b - a as usize),
                }

                (&UnSignNum::U32(a), b) => match b {
                    UnSignNum::U8(b) => UnSignNum::U32              (a - b as u32),
                    UnSignNum::U16(b) => UnSignNum::U32            (a - b as u32),
                    UnSignNum::U32(b) => UnSignNum::U32            (b - a as u32),
                    UnSignNum::U64(b) => UnSignNum::U64            (b - a as u64),
                    UnSignNum::U128(b) => UnSignNum::U128         (b - a as u128),
                    UnSignNum::Usize(b) => UnSignNum::Usize      (b - a as usize),
                }

                (&UnSignNum::U64(a), b) => match b {
                    UnSignNum::U8(b) => UnSignNum::U64              (a - b as u64),
                    UnSignNum::U16(b) => UnSignNum::U64            (a - b as u64),
                    UnSignNum::U32(b) => UnSignNum::U64            (a - b as u64),
                    UnSignNum::U64(b) => UnSignNum::U64            (b - a as u64),
                    UnSignNum::U128(b) => UnSignNum::U128         (b - a as u128),
                    UnSignNum::Usize(b) => UnSignNum::Usize      (b - a as usize),
                }

                (&UnSignNum::U128(a), b) => match b {
                    UnSignNum::U8(b) => UnSignNum::U128             (a - b as u128),
                    UnSignNum::U16(b) => UnSignNum::U128           (a - b as u128),
                    UnSignNum::U32(b) => UnSignNum::U128           (a - b as u128),
                    UnSignNum::U64(b) => UnSignNum::U128           (a - b as u128),
                    UnSignNum::U128(b) => UnSignNum::U128         (b - a as u128),
                    UnSignNum::Usize(b) => UnSignNum::Usize      (b - a as usize),
                }

                (&UnSignNum::Usize(a), b) => match b {
                    UnSignNum::U8(b) => UnSignNum::Usize            (a - b as usize),
                    UnSignNum::U16(b) => UnSignNum::Usize          (a - b as usize),
                    UnSignNum::U32(b) => UnSignNum::Usize          (a - b as usize),
                    UnSignNum::U64(b) => UnSignNum::Usize          (a - b as usize),
                    UnSignNum::U128(b) => UnSignNum::U128         (b - a as u128),
                    UnSignNum::Usize(b) => UnSignNum::Usize      (b - a as usize),
                }
            }
        }

        pub fn mul(&self, rhs: Self) -> Self {
            match (self, rhs) {
                (&UnSignNum::U8(a), b) => match b {
                    UnSignNum::U8(b) => UnSignNum::U8               (b * a),
                    UnSignNum::U16(b) => UnSignNum::U16            (b * a as u16),
                    UnSignNum::U32(b) => UnSignNum::U32            (b * a as u32),
                    UnSignNum::U64(b) => UnSignNum::U64            (b * a as u64),
                    UnSignNum::U128(b) => UnSignNum::U128         (b * a as u128),
                    UnSignNum::Usize(b) => UnSignNum::Usize      (b * a as usize),
                }

                (&UnSignNum::U16(a), b) => match b {
                    UnSignNum::U8(b) => UnSignNum::U16              (a * b as u16),
                    UnSignNum::U16(b) => UnSignNum::U16            (b * a as u16),
                    UnSignNum::U32(b) => UnSignNum::U32            (b * a as u32),
                    UnSignNum::U64(b) => UnSignNum::U64            (b * a as u64),
                    UnSignNum::U128(b) => UnSignNum::U128         (b * a as u128),
                    UnSignNum::Usize(b) => UnSignNum::Usize      (b * a as usize),
                }

                (&UnSignNum::U32(a), b) => match b {
                    UnSignNum::U8(b) => UnSignNum::U32              (a * b as u32),
                    UnSignNum::U16(b) => UnSignNum::U32            (a * b as u32),
                    UnSignNum::U32(b) => UnSignNum::U32            (b * a as u32),
                    UnSignNum::U64(b) => UnSignNum::U64            (b * a as u64),
                    UnSignNum::U128(b) => UnSignNum::U128         (b * a as u128),
                    UnSignNum::Usize(b) => UnSignNum::Usize      (b * a as usize),
                }

                (&UnSignNum::U64(a), b) => match b {
                    UnSignNum::U8(b) => UnSignNum::U64              (a * b as u64),
                    UnSignNum::U16(b) => UnSignNum::U64            (a * b as u64),
                    UnSignNum::U32(b) => UnSignNum::U64            (a * b as u64),
                    UnSignNum::U64(b) => UnSignNum::U64            (b * a as u64),
                    UnSignNum::U128(b) => UnSignNum::U128         (b * a as u128),
                    UnSignNum::Usize(b) => UnSignNum::Usize      (b * a as usize),
                }

                (&UnSignNum::U128(a), b) => match b {
                    UnSignNum::U8(b) => UnSignNum::U128             (a * b as u128),
                    UnSignNum::U16(b) => UnSignNum::U128           (a * b as u128),
                    UnSignNum::U32(b) => UnSignNum::U128           (a * b as u128),
                    UnSignNum::U64(b) => UnSignNum::U128           (a * b as u128),
                    UnSignNum::U128(b) => UnSignNum::U128         (b * a as u128),
                    UnSignNum::Usize(b) => UnSignNum::Usize      (b * a as usize),
                }

                (&UnSignNum::Usize(a), b) => match b {
                    UnSignNum::U8(b) => UnSignNum::Usize            (a * b as usize),
                    UnSignNum::U16(b) => UnSignNum::Usize          (a * b as usize),
                    UnSignNum::U32(b) => UnSignNum::Usize          (a * b as usize),
                    UnSignNum::U64(b) => UnSignNum::Usize          (a * b as usize),
                    UnSignNum::U128(b) => UnSignNum::U128         (b * a as u128),
                    UnSignNum::Usize(b) => UnSignNum::Usize      (b * a as usize),
                }
            }
        }

        pub fn div(&self, rhs: Self) -> Self {
            match (self, rhs) {
                (&UnSignNum::U8(a), b) => match b {
                    UnSignNum::U8(b) => UnSignNum::U8               (b / a),
                    UnSignNum::U16(b) => UnSignNum::U16            (b / a as u16),
                    UnSignNum::U32(b) => UnSignNum::U32            (b / a as u32),
                    UnSignNum::U64(b) => UnSignNum::U64            (b / a as u64),
                    UnSignNum::U128(b) => UnSignNum::U128         (b / a as u128),
                    UnSignNum::Usize(b) => UnSignNum::Usize      (b / a as usize),
                }

                (&UnSignNum::U16(a), b) => match b {
                    UnSignNum::U8(b) => UnSignNum::U16              (a / b as u16),
                    UnSignNum::U16(b) => UnSignNum::U16            (b / a as u16),
                    UnSignNum::U32(b) => UnSignNum::U32            (b / a as u32),
                    UnSignNum::U64(b) => UnSignNum::U64            (b / a as u64),
                    UnSignNum::U128(b) => UnSignNum::U128         (b / a as u128),
                    UnSignNum::Usize(b) => UnSignNum::Usize      (b / a as usize),
                }

                (&UnSignNum::U32(a), b) => match b {
                    UnSignNum::U8(b) => UnSignNum::U32              (a / b as u32),
                    UnSignNum::U16(b) => UnSignNum::U32            (a / b as u32),
                    UnSignNum::U32(b) => UnSignNum::U32            (b / a as u32),
                    UnSignNum::U64(b) => UnSignNum::U64            (b / a as u64),
                    UnSignNum::U128(b) => UnSignNum::U128         (b / a as u128),
                    UnSignNum::Usize(b) => UnSignNum::Usize      (b / a as usize),
                }

                (&UnSignNum::U64(a), b) => match b {
                    UnSignNum::U8(b) => UnSignNum::U64              (a / b as u64),
                    UnSignNum::U16(b) => UnSignNum::U64            (a / b as u64),
                    UnSignNum::U32(b) => UnSignNum::U64            (a / b as u64),
                    UnSignNum::U64(b) => UnSignNum::U64            (b / a as u64),
                    UnSignNum::U128(b) => UnSignNum::U128         (b / a as u128),
                    UnSignNum::Usize(b) => UnSignNum::Usize      (b / a as usize),
                }

                (&UnSignNum::U128(a), b) => match b {
                    UnSignNum::U8(b) => UnSignNum::U128             (a / b as u128),
                    UnSignNum::U16(b) => UnSignNum::U128           (a / b as u128),
                    UnSignNum::U32(b) => UnSignNum::U128           (a / b as u128),
                    UnSignNum::U64(b) => UnSignNum::U128           (a / b as u128),
                    UnSignNum::U128(b) => UnSignNum::U128         (b / a as u128),
                    UnSignNum::Usize(b) => UnSignNum::Usize      (b / a as usize),
                }

                (&UnSignNum::Usize(a), b) => match b {
                    UnSignNum::U8(b) => UnSignNum::Usize            (a / b as usize),
                    UnSignNum::U16(b) => UnSignNum::Usize          (a / b as usize),
                    UnSignNum::U32(b) => UnSignNum::Usize          (a / b as usize),
                    UnSignNum::U64(b) => UnSignNum::Usize          (a / b as usize),
                    UnSignNum::U128(b) => UnSignNum::U128         (b / a as u128),
                    UnSignNum::Usize(b) => UnSignNum::Usize      (b / a as usize),
                }
            }
        }

        pub fn to_i(&self) -> SignNum {
            match self {
                &UnSignNum::U8(a) => SignNum::I16(a as i16),
                &UnSignNum::U16(a) => SignNum::I32(a as i32),
                &UnSignNum::U32(a) => SignNum::I64(a as i64),
                &UnSignNum::U64(a) => SignNum::I128(a as i128),
                &UnSignNum::U128(a) => SignNum::I128(a as i128),
                &UnSignNum::Usize(a) => SignNum::Isize(a as isize),
            }
        }

        pub fn to_f32(&self) -> Float {
            match self {
                &UnSignNum::U8(a) => Float::F32(a as f32),
                &UnSignNum::U16(a) => Float::F32(a as f32),
                &UnSignNum::U32(a) => Float::F32(a as f32),
                &UnSignNum::U64(a) => Float::F32(a as f32),
                &UnSignNum::U128(a) => Float::F32(a as f32),
                &UnSignNum::Usize(a) => Float::F32(a as f32),
            }
        }

        pub fn to_f64(&self) -> Float {
            match self {
                &UnSignNum::U8(a) => Float::F64(a as f64),
                &UnSignNum::U16(a) => Float::F64(a as f64),
                &UnSignNum::U32(a) => Float::F64(a as f64),
                &UnSignNum::U64(a) => Float::F64(a as f64),
                &UnSignNum::U128(a) => Float::F64(a as f64),
                &UnSignNum::Usize(a) => Float::F64(a as f64),
            }
        }
    }

    impl SignNum {
        pub fn add(&self, rhs: Self) -> Self {
            match (self, rhs) {
                (&SignNum::I8(a), b) => match b {
                    SignNum::I8(b) => SignNum::I8               (b + a),
                    SignNum::I16(b) => SignNum::I16            (b + a as i16),
                    SignNum::I32(b) => SignNum::I32            (b + a as i32),
                    SignNum::I64(b) => SignNum::I64            (b + a as i64),
                    SignNum::I128(b) => SignNum::I128         (b + a as i128),
                    SignNum::Isize(b) => SignNum::Isize      (b + a as isize),
                }

                (&SignNum::I16(a), b) => match b {
                    SignNum::I8(b) => SignNum::I16              (a + b as i16),
                    SignNum::I16(b) => SignNum::I16            (b + a as i16),
                    SignNum::I32(b) => SignNum::I32            (b + a as i32),
                    SignNum::I64(b) => SignNum::I64            (b + a as i64),
                    SignNum::I128(b) => SignNum::I128         (b + a as i128),
                    SignNum::Isize(b) => SignNum::Isize      (b + a as isize),
                }

                (&SignNum::I32(a), b) => match b {
                    SignNum::I8(b) => SignNum::I32              (a + b as i32),
                    SignNum::I16(b) => SignNum::I32            (a + b as i32),
                    SignNum::I32(b) => SignNum::I32            (b + a as i32),
                    SignNum::I64(b) => SignNum::I64            (b + a as i64),
                    SignNum::I128(b) => SignNum::I128         (b + a as i128),
                    SignNum::Isize(b) => SignNum::Isize      (b + a as isize),
                }

                (&SignNum::I64(a), b) => match b {
                    SignNum::I8(b) => SignNum::I64              (a + b as i64),
                    SignNum::I16(b) => SignNum::I64            (a + b as i64),
                    SignNum::I32(b) => SignNum::I64            (a + b as i64),
                    SignNum::I64(b) => SignNum::I64            (b + a as i64),
                    SignNum::I128(b) => SignNum::I128         (b + a as i128),
                    SignNum::Isize(b) => SignNum::Isize      (b + a as isize),
                }

                (&SignNum::I128(a), b) => match b {
                    SignNum::I8(b) => SignNum::I128             (a + b as i128),
                    SignNum::I16(b) => SignNum::I128           (a + b as i128),
                    SignNum::I32(b) => SignNum::I128           (a + b as i128),
                    SignNum::I64(b) => SignNum::I128           (a + b as i128),
                    SignNum::I128(b) => SignNum::I128         (b + a as i128),
                    SignNum::Isize(b) => SignNum::Isize      (b + a as isize),
                }

                (&SignNum::Isize(a), b) => match b {
                    SignNum::I8(b) => SignNum::Isize            (a + b as isize),
                    SignNum::I16(b) => SignNum::Isize          (a + b as isize),
                    SignNum::I32(b) => SignNum::Isize          (a + b as isize),
                    SignNum::I64(b) => SignNum::Isize          (a + b as isize),
                    SignNum::I128(b) => SignNum::I128         (b + a as i128),
                    SignNum::Isize(b) => SignNum::Isize      (b + a as isize),
                }
            }
        }

        pub fn sub(&self, rhs: Self) -> Self {
            match (self, rhs) {
                (&SignNum::I8(a), b) => match b {
                    SignNum::I8(b) => SignNum::I8               (b - a),
                    SignNum::I16(b) => SignNum::I16            (b - a as i16),
                    SignNum::I32(b) => SignNum::I32            (b - a as i32),
                    SignNum::I64(b) => SignNum::I64            (b - a as i64),
                    SignNum::I128(b) => SignNum::I128         (b - a as i128),
                    SignNum::Isize(b) => SignNum::Isize      (b - a as isize),
                }

                (&SignNum::I16(a), b) => match b {
                    SignNum::I8(b) => SignNum::I16              (a - b as i16),
                    SignNum::I16(b) => SignNum::I16            (b - a as i16),
                    SignNum::I32(b) => SignNum::I32            (b - a as i32),
                    SignNum::I64(b) => SignNum::I64            (b - a as i64),
                    SignNum::I128(b) => SignNum::I128         (b - a as i128),
                    SignNum::Isize(b) => SignNum::Isize      (b - a as isize),
                }

                (&SignNum::I32(a), b) => match b {
                    SignNum::I8(b) => SignNum::I32              (a - b as i32),
                    SignNum::I16(b) => SignNum::I32            (a - b as i32),
                    SignNum::I32(b) => SignNum::I32            (b - a as i32),
                    SignNum::I64(b) => SignNum::I64            (b - a as i64),
                    SignNum::I128(b) => SignNum::I128         (b - a as i128),
                    SignNum::Isize(b) => SignNum::Isize      (b - a as isize),
                }

                (&SignNum::I64(a), b) => match b {
                    SignNum::I8(b) => SignNum::I64              (a - b as i64),
                    SignNum::I16(b) => SignNum::I64            (a - b as i64),
                    SignNum::I32(b) => SignNum::I64            (a - b as i64),
                    SignNum::I64(b) => SignNum::I64            (b - a as i64),
                    SignNum::I128(b) => SignNum::I128         (b - a as i128),
                    SignNum::Isize(b) => SignNum::Isize      (b - a as isize),
                }

                (&SignNum::I128(a), b) => match b {
                    SignNum::I8(b) => SignNum::I128             (a - b as i128),
                    SignNum::I16(b) => SignNum::I128           (a - b as i128),
                    SignNum::I32(b) => SignNum::I128           (a - b as i128),
                    SignNum::I64(b) => SignNum::I128           (a - b as i128),
                    SignNum::I128(b) => SignNum::I128         (b - a as i128),
                    SignNum::Isize(b) => SignNum::Isize      (b - a as isize),
                }

                (&SignNum::Isize(a), b) => match b {
                    SignNum::I8(b) => SignNum::Isize            (a - b as isize),
                    SignNum::I16(b) => SignNum::Isize          (a - b as isize),
                    SignNum::I32(b) => SignNum::Isize          (a - b as isize),
                    SignNum::I64(b) => SignNum::Isize          (a - b as isize),
                    SignNum::I128(b) => SignNum::I128         (b - a as i128),
                    SignNum::Isize(b) => SignNum::Isize      (b - a as isize),
                }
            }
        }

        pub fn mul(&self, rhs: Self) -> Self {
            match (self, rhs) {
                (&SignNum::I8(a), b) => match b {
                    SignNum::I8(b) => SignNum::I8               (b * a),
                    SignNum::I16(b) => SignNum::I16            (b * a as i16),
                    SignNum::I32(b) => SignNum::I32            (b * a as i32),
                    SignNum::I64(b) => SignNum::I64            (b * a as i64),
                    SignNum::I128(b) => SignNum::I128         (b * a as i128),
                    SignNum::Isize(b) => SignNum::Isize      (b * a as isize),
                }

                (&SignNum::I16(a), b) => match b {
                    SignNum::I8(b) => SignNum::I16              (a * b as i16),
                    SignNum::I16(b) => SignNum::I16            (b * a as i16),
                    SignNum::I32(b) => SignNum::I32            (b * a as i32),
                    SignNum::I64(b) => SignNum::I64            (b * a as i64),
                    SignNum::I128(b) => SignNum::I128         (b * a as i128),
                    SignNum::Isize(b) => SignNum::Isize      (b * a as isize),
                }

                (&SignNum::I32(a), b) => match b {
                    SignNum::I8(b) => SignNum::I32              (a * b as i32),
                    SignNum::I16(b) => SignNum::I32            (a * b as i32),
                    SignNum::I32(b) => SignNum::I32            (b * a as i32),
                    SignNum::I64(b) => SignNum::I64            (b * a as i64),
                    SignNum::I128(b) => SignNum::I128         (b * a as i128),
                    SignNum::Isize(b) => SignNum::Isize      (b * a as isize),
                }

                (&SignNum::I64(a), b) => match b {
                    SignNum::I8(b) => SignNum::I64              (a * b as i64),
                    SignNum::I16(b) => SignNum::I64            (a * b as i64),
                    SignNum::I32(b) => SignNum::I64            (a * b as i64),
                    SignNum::I64(b) => SignNum::I64            (b * a as i64),
                    SignNum::I128(b) => SignNum::I128         (b * a as i128),
                    SignNum::Isize(b) => SignNum::Isize      (b * a as isize),
                }

                (&SignNum::I128(a), b) => match b {
                    SignNum::I8(b) => SignNum::I128             (a * b as i128),
                    SignNum::I16(b) => SignNum::I128           (a * b as i128),
                    SignNum::I32(b) => SignNum::I128           (a * b as i128),
                    SignNum::I64(b) => SignNum::I128           (a * b as i128),
                    SignNum::I128(b) => SignNum::I128         (b * a as i128),
                    SignNum::Isize(b) => SignNum::Isize      (b * a as isize),
                }

                (&SignNum::Isize(a), b) => match b {
                    SignNum::I8(b) => SignNum::Isize            (a * b as isize),
                    SignNum::I16(b) => SignNum::Isize          (a * b as isize),
                    SignNum::I32(b) => SignNum::Isize          (a * b as isize),
                    SignNum::I64(b) => SignNum::Isize          (a * b as isize),
                    SignNum::I128(b) => SignNum::I128         (b * a as i128),
                    SignNum::Isize(b) => SignNum::Isize      (b * a as isize),
                }
            }
        }

        pub fn div(&self, rhs: Self) -> Self {
            match (self, rhs) {
                (&SignNum::I8(a), b) => match b {
                    SignNum::I8(b) => SignNum::I8               (b / a),
                    SignNum::I16(b) => SignNum::I16            (b / a as i16),
                    SignNum::I32(b) => SignNum::I32            (b / a as i32),
                    SignNum::I64(b) => SignNum::I64            (b / a as i64),
                    SignNum::I128(b) => SignNum::I128         (b / a as i128),
                    SignNum::Isize(b) => SignNum::Isize      (b / a as isize),
                }

                (&SignNum::I16(a), b) => match b {
                    SignNum::I8(b) => SignNum::I16              (a / b as i16),
                    SignNum::I16(b) => SignNum::I16            (b / a as i16),
                    SignNum::I32(b) => SignNum::I32            (b / a as i32),
                    SignNum::I64(b) => SignNum::I64            (b / a as i64),
                    SignNum::I128(b) => SignNum::I128         (b / a as i128),
                    SignNum::Isize(b) => SignNum::Isize      (b / a as isize),
                }

                (&SignNum::I32(a), b) => match b {
                    SignNum::I8(b) => SignNum::I32              (a / b as i32),
                    SignNum::I16(b) => SignNum::I32            (a / b as i32),
                    SignNum::I32(b) => SignNum::I32            (b / a as i32),
                    SignNum::I64(b) => SignNum::I64            (b / a as i64),
                    SignNum::I128(b) => SignNum::I128         (b / a as i128),
                    SignNum::Isize(b) => SignNum::Isize      (b / a as isize),
                }
                

                (&SignNum::I64(a), b) => match b {
                    SignNum::I8(b) => SignNum::I64              (a / b as i64),
                    SignNum::I16(b) => SignNum::I64            (a / b as i64),
                    SignNum::I32(b) => SignNum::I64            (a / b as i64),
                    SignNum::I64(b) => SignNum::I64            (b / a as i64),
                    SignNum::I128(b) => SignNum::I128         (b / a as i128),
                    SignNum::Isize(b) => SignNum::Isize      (b / a as isize),
                }

                (&SignNum::I128(a), b) => match b {
                    SignNum::I8(b) => SignNum::I128             (a / b as i128),
                    SignNum::I16(b) => SignNum::I128           (a / b as i128),
                    SignNum::I32(b) => SignNum::I128           (a / b as i128),
                    SignNum::I64(b) => SignNum::I128           (a / b as i128),
                    SignNum::I128(b) => SignNum::I128         (b / a as i128),
                    SignNum::Isize(b) => SignNum::Isize      (b / a as isize),
                }

                (&SignNum::Isize(a), b) => match b {
                    SignNum::I8(b) => SignNum::Isize            (a / b as isize),
                    SignNum::I16(b) => SignNum::Isize          (a / b as isize),
                    SignNum::I32(b) => SignNum::Isize          (a / b as isize),
                    SignNum::I64(b) => SignNum::Isize          (a / b as isize),
                    SignNum::I128(b) => SignNum::I128         (b / a as i128),
                    SignNum::Isize(b) => SignNum::Isize      (b / a as isize),
                }
            }
        }

        pub fn to_u(&self) -> UnSignNum {
            match self {
                &SignNum::I8(a) => UnSignNum::U8(a as u8),
                &SignNum::I16(a) => UnSignNum::U8(a as u8),
                &SignNum::I32(a) => UnSignNum::U16(a as u16),
                &SignNum::I64(a) => UnSignNum::U32(a as u32),
                &SignNum::I128(a) => UnSignNum::U64(a as u64),
                &SignNum::Isize(a) => UnSignNum::Usize(a as usize),
            }
        }

        pub fn to_f32(&self) -> Float {
            match self {
                &SignNum::I8(a) => Float::F32(a as f32),
                &SignNum::I16(a) => Float::F32(a as f32),
                &SignNum::I32(a) => Float::F32(a as f32),
                &SignNum::I64(a) => Float::F32(a as f32),
                &SignNum::I128(a) => Float::F32(a as f32),
                &SignNum::Isize(a) => Float::F32(a as f32),
            }
        }

        pub fn to_f64(&self) -> Float {
            match self {
                &SignNum::I8(a) => Float::F32(a as f32),
                &SignNum::I16(a) => Float::F32(a as f32),
                &SignNum::I32(a) => Float::F32(a as f32),
                &SignNum::I64(a) => Float::F32(a as f32),
                &SignNum::I128(a) => Float::F32(a as f32),
                &SignNum::Isize(a) => Float::F32(a as f32),
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
    pub fn drop(mut self) {
        if self.droppable() {
            self.drop_borrow();
            match self {
                Num::Var(var) => var.drop_name(),
                Num::Cons(cons) => cons.drop_name(),
                _ => ()
            };
            drop(self)
        } else {
            warn::delete_before_no_borrow()
        }
    }

    pub fn symbol(&self) -> String {
        match self {
            Num::Cons(cons) => format!("{}", cons.name()),
            Num::Var(var) => format!("{}", var.name()),
            Num::Fixed(fix) => format!("{}", fix),
            Num::Expr(expr) => format!("{}", *expr),
            Num::Undefined => "UNDEFINED".to_string(),
        }
    }

    pub fn droppable(&self) -> bool {
        return match self {
            Num::Cons(cons) => cons.droppable(),
            Num::Var(var) => var.droppable(),
            _ => true
        }
    }

    pub fn cal(&self) -> fixed_num::FixedNum {
        match self {
            Num::Cons(cons) => cons.cal(),
            Num::Fixed(fix) => fix.clone(),
            Num::Var(var) => var.cal(),
            Num::Undefined => fixed_num::FixedNum::Undefined,
            Num::Expr(expr) => expr.cal()
        }
    }

    pub fn drop_borrow(&mut self) {
        match self.expr() {
            Num::Cons(cons) => count::sub_one(cons.name().to_str()),
            Num::Var(var) => count::sub_one(var.name().to_str()),
            Num::Expr(mut expr) => (*expr).drop_borrow(),
            _ => (),
        }
    }

    pub fn expr(&self) -> Self {
        match self {
            Num::Var(var) => var.expr(),
            Num::Cons(cons) => cons.expr(),
            Num::Expr(expr) => Num::Expr(expr.clone()),
            _ => Num::Undefined
        }
    }

    pub fn replace(&mut self, to: Self) {
        *self = to
    }

    pub fn val_print(&self) -> String {
        match self {
            Num::Cons(cons) => cons.val_print(),
            Num::Var(var) => var.val_print(),
            Num::Expr(expr) => format!("({})", expr),
            Num::Fixed(fix) => format!("{}", fix),
            Num::Undefined => "UNDEFINED".to_string()
        }
    }
}

impl<'a: 'static> Num<'a> {
    pub fn change_val(&mut self, aim: Num<'a>) {
        match self {
            Num::Var(var) => var.change_num(aim),
            _  => warn::type_unchangable(),
        }
    }
}

impl std::fmt::Display for Num<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Num::Cons(cons) => write!(f, "{}", cons),
            Num::Var(var) => write!(f, "{}", var),
            Num::Expr(expr) => write!(f, "{}", expr),
            Num::Fixed(fix) => write!(f, "{}", fix),
            Num::Undefined => write!(f, "UNDEFINED"),
        }
    }
}

impl std::ops::Add for Num<'_> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        match &self {
            &Num::Cons(cons) => count::add_one(cons.name().to_str()),
            &Num::Var(var) => count::add_one(var.name().to_str()),
            _ => (),
        }
        match &rhs {
            &Num::Cons(cons) => count::add_one(cons.name().to_str()),
            &Num::Var(var) => count::add_one(var.name().to_str()),
            _ => (),
        }
        Expr::new(self, rhs, Op::Basic(BasicOp::Add))
    }
}

impl std::ops::Sub for Num<'_> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        match &self {
            &Num::Cons(cons) => count::add_one(cons.name().to_str()),
            &Num::Var(var) => count::add_one(var.name().to_str()),
            _ => (),
        }
        match &rhs {
            &Num::Cons(cons) => count::add_one(cons.name().to_str()),
            &Num::Var(var) => count::add_one(var.name().to_str()),
            _ => (),
        }
        Expr::new(self, rhs, Op::Basic(BasicOp::Sub))
    }
}

impl std::ops::Mul for Num<'_> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        match &self {
            &Num::Cons(cons) => count::add_one(cons.name().to_str()),
            &Num::Var(var) => count::add_one(var.name().to_str()),
            _ => (),
        }
        match &rhs {
            &Num::Cons(cons) => count::add_one(cons.name().to_str()),
            &Num::Var(var) => count::add_one(var.name().to_str()),
            _ => (),
        }
        Expr::new(self, rhs, Op::Basic(BasicOp::Mul))
    }
}

impl std::ops::Div for Num<'_> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        match &self {
            &Num::Cons(cons) => count::add_one(cons.name().to_str()),
            &Num::Var(var) => count::add_one(var.name().to_str()),
            _ => (),
        }
        match &rhs {
            &Num::Cons(cons) => count::add_one(cons.name().to_str()),
            &Num::Var(var) => count::add_one(var.name().to_str()),
            _ => (),
        }
        Expr::new(self, rhs, Op::Basic(BasicOp::Div))
    }
}