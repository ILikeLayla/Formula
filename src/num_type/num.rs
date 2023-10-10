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
            Num::Expr(expr) => expr.cal()
        }
    }
}

impl std::ops::Add for Num<'_> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        match &self {
            &Num::Cons(cons) => count::add_one(cons.name())
        }
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