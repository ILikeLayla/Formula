use crate::num_type::fixed_num::{Float, FixedNum};
use super::num_type::Num;
use super::traits::{Val, Prt};
use super::op::{Op, BasicOp, Tri, Expo};


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
}

impl Expr<'_> {
    pub fn cal(&self) -> FixedNum {
        match self.expr_type.clone() {
            Op::Basic(basic) => self.basic_cal(basic),
            _ => FixedNum::Undefined,
        }
    }

    fn basic_cal(&self, op: BasicOp) -> FixedNum {
        match op {
            BasicOp::Add => self.add(),
            BasicOp::Sub => self.sub(),
            BasicOp::Mul => self.mul(),
            BasicOp::Div => self.div(),
        }
    }

    fn add(&self) -> FixedNum {
        match self.per_cal() {
            (FixedNum::Undefined, _) => FixedNum::Undefined,
            (_, FixedNum::Undefined) => FixedNum::Undefined,

            (FixedNum::Sign(a), FixedNum::Sign(b)) => FixedNum::Sign(a.add(b)),
            (FixedNum::UnSign(a), FixedNum::UnSign(b)) => FixedNum::UnSign(a.add(b)),
            (FixedNum::Float(a), FixedNum::Float(b)) => FixedNum::Float(a.add(b)),
            
            (FixedNum::Sign(a), FixedNum::UnSign(b)) => FixedNum::Sign(a.add(b.to_i())),
            (FixedNum::Sign(a), FixedNum::Float(b)) => FixedNum::Float(match b {
                Float::F32(b) => a.to_f32().add(Float::F32(b)),
                Float::F64(b) => a.to_f64().add(Float::F64(b))
            }),

            (FixedNum::UnSign(a), FixedNum::Sign(b)) => FixedNum::Sign(a.to_i().add(b)),
            (FixedNum::UnSign(a), FixedNum::Float(b)) => FixedNum::Float(match b {
                Float::F32(b) => a.to_f32().add(Float::F32(b)),
                Float::F64(b) => a.to_f64().add(Float::F64(b))
            }),

            (FixedNum::Float(b), FixedNum::UnSign(a)) => FixedNum::Float(match b {
                Float::F32(b) => a.to_f32().add(Float::F32(b)),
                Float::F64(b) => a.to_f64().add(Float::F64(b))
            }),
            (FixedNum::Float(b), FixedNum::Sign(a)) => FixedNum::Float(match b {
                Float::F32(b) => a.to_f32().add(Float::F32(b)),
                Float::F64(b) => a.to_f64().add(Float::F64(b))
            }),
        }
    }

    fn sub(&self) -> FixedNum {
        match self.per_cal() {
            (FixedNum::Undefined, _) => FixedNum::Undefined,
            (_, FixedNum::Undefined) => FixedNum::Undefined,

            (FixedNum::Sign(a), FixedNum::Sign(b)) => FixedNum::Sign(a.sub(b)),
            (FixedNum::UnSign(a), FixedNum::UnSign(b)) => FixedNum::UnSign(a.sub(b)),
            (FixedNum::Float(a), FixedNum::Float(b)) => FixedNum::Float(a.sub(b)),
            
            (FixedNum::Sign(a), FixedNum::UnSign(b)) => FixedNum::Sign(a.sub(b.to_i())),
            (FixedNum::Sign(a), FixedNum::Float(b)) => FixedNum::Float(match b {
                Float::F32(b) => a.to_f32().sub(Float::F32(b)),
                Float::F64(b) => a.to_f64().sub(Float::F64(b))
            }),

            (FixedNum::UnSign(a), FixedNum::Sign(b)) => FixedNum::Sign(a.to_i().sub(b)),
            (FixedNum::UnSign(a), FixedNum::Float(b)) => FixedNum::Float(match b {
                Float::F32(b) => a.to_f32().sub(Float::F32(b)),
                Float::F64(b) => a.to_f64().sub(Float::F64(b))
            }),

            (FixedNum::Float(b), FixedNum::UnSign(a)) => FixedNum::Float(match b {
                Float::F32(b) => a.to_f32().sub(Float::F32(b)),
                Float::F64(b) => a.to_f64().sub(Float::F64(b))
            }),
            (FixedNum::Float(b), FixedNum::Sign(a)) => FixedNum::Float(match b {
                Float::F32(b) => a.to_f32().sub(Float::F32(b)),
                Float::F64(b) => a.to_f64().sub(Float::F64(b))
            }),
        }
    }

    fn mul(&self) -> FixedNum {
        match self.per_cal() {
            (FixedNum::Undefined, _) => FixedNum::Undefined,
            (_, FixedNum::Undefined) => FixedNum::Undefined,

            (FixedNum::Sign(a), FixedNum::Sign(b)) => FixedNum::Sign(a.mul(b)),
            (FixedNum::UnSign(a), FixedNum::UnSign(b)) => FixedNum::UnSign(a.mul(b)),
            (FixedNum::Float(a), FixedNum::Float(b)) => FixedNum::Float(a.mul(b)),
            
            (FixedNum::Sign(a), FixedNum::UnSign(b)) => FixedNum::Sign(a.mul(b.to_i())),
            (FixedNum::Sign(a), FixedNum::Float(b)) => FixedNum::Float(match b {
                Float::F32(b) => a.to_f32().mul(Float::F32(b)),
                Float::F64(b) => a.to_f64().mul(Float::F64(b))
            }),

            (FixedNum::UnSign(a), FixedNum::Sign(b)) => FixedNum::Sign(a.to_i().mul(b)),
            (FixedNum::UnSign(a), FixedNum::Float(b)) => FixedNum::Float(match b {
                Float::F32(b) => a.to_f32().mul(Float::F32(b)),
                Float::F64(b) => a.to_f64().mul(Float::F64(b))
            }),

            (FixedNum::Float(b), FixedNum::UnSign(a)) => FixedNum::Float(match b {
                Float::F32(b) => a.to_f32().mul(Float::F32(b)),
                Float::F64(b) => a.to_f64().mul(Float::F64(b))
            }),
            (FixedNum::Float(b), FixedNum::Sign(a)) => FixedNum::Float(match b {
                Float::F32(b) => a.to_f32().mul(Float::F32(b)),
                Float::F64(b) => a.to_f64().mul(Float::F64(b))
            }),
        }
    }

    fn div(&self) -> FixedNum {
        match self.per_cal() {
            (FixedNum::Undefined, _) => FixedNum::Undefined,
            (_, FixedNum::Undefined) => FixedNum::Undefined,

            (FixedNum::Sign(a), FixedNum::Sign(b)) => FixedNum::Sign(a.div(b)),
            (FixedNum::UnSign(a), FixedNum::UnSign(b)) => FixedNum::UnSign(a.div(b)),
            (FixedNum::Float(a), FixedNum::Float(b)) => FixedNum::Float(a.div(b)),
            
            (FixedNum::Sign(a), FixedNum::UnSign(b)) => FixedNum::Sign(a.div(b.to_i())),
            (FixedNum::Sign(a), FixedNum::Float(b)) => FixedNum::Float(match b {
                Float::F32(b) => a.to_f32().div(Float::F32(b)),
                Float::F64(b) => a.to_f64().div(Float::F64(b))
            }),

            (FixedNum::UnSign(a), FixedNum::Sign(b)) => FixedNum::Sign(a.to_i().div(b)),
            (FixedNum::UnSign(a), FixedNum::Float(b)) => FixedNum::Float(match b {
                Float::F32(b) => a.to_f32().div(Float::F32(b)),
                Float::F64(b) => a.to_f64().div(Float::F64(b))
            }),

            (FixedNum::Float(b), FixedNum::UnSign(a)) => FixedNum::Float(match b {
                Float::F32(b) => a.to_f32().div(Float::F32(b)),
                Float::F64(b) => a.to_f64().div(Float::F64(b))
            }),
            (FixedNum::Float(b), FixedNum::Sign(a)) => FixedNum::Float(match b {
                Float::F32(b) => a.to_f32().div(Float::F32(b)),
                Float::F64(b) => a.to_f64().div(Float::F64(b))
            }),
        }
    }

    fn per_cal(&self) -> (FixedNum, FixedNum) {
        (self.a.cal(), self.b.cal())
    }
}

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