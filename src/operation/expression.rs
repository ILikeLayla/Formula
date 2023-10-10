use crate::num_type::fixed_num::{Float, FixedNum};
use super::num_type::Num;
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

            (FixedNum::Sign(a), num) => match num {
                FixedNum::Undefined => FixedNum::Undefined,
                FixedNum::Sign(b) => FixedNum::Sign(a.add(b)),
                FixedNum::UnSign(b) => FixedNum::Sign(a.add(b.to_i())),
                FixedNum::Float(b) => FixedNum::Float(match b {
                    Float::F32(b) => a.to_f32().add(Float::F32(b)),
                    Float::F64(b) => a.to_f64().add(Float::F64(b))
                }),
            }

            (FixedNum::UnSign(a), num) => match num {
                FixedNum::Undefined => FixedNum::Undefined,
                FixedNum::Sign(b) => FixedNum::Sign(a.to_i().add(b)),
                FixedNum::UnSign(b) => FixedNum::UnSign(a.add(b)),
                FixedNum::Float(b) => FixedNum::Float(match b {
                    Float::F32(b) => a.to_f32().add(Float::F32(b)),
                    Float::F64(b) => a.to_f64().add(Float::F64(b))
                }),
            }

            (FixedNum::Float(a), num) => match num {
                FixedNum::Undefined => FixedNum::Undefined,
                FixedNum::Sign(b) => FixedNum::Float(match a {
                    Float::F32(a) => b.to_f32().add(Float::F32(a)),
                    Float::F64(a) => b.to_f64().add(Float::F64(a))
                }),
                FixedNum::UnSign(b) => FixedNum::Float(match a {
                    Float::F32(a) => b.to_f32().add(Float::F32(a)),
                    Float::F64(a) => b.to_f64().add(Float::F64(a))
                }),
                FixedNum::Float(b) => FixedNum::Float(a.add(b)),
            }
        }
    }

    fn sub(&self) -> FixedNum {
        match self.per_cal() {
            (FixedNum::Undefined, _) => FixedNum::Undefined,
            (_, FixedNum::Undefined) => FixedNum::Undefined,

            (FixedNum::Sign(a), num) => match num {
                FixedNum::Undefined => FixedNum::Undefined,
                FixedNum::Sign(b) => FixedNum::Sign(a.sub(b)),
                FixedNum::UnSign(b) => FixedNum::Sign(a.sub(b.to_i())),
                FixedNum::Float(b) => FixedNum::Float(match b {
                    Float::F32(b) => a.to_f32().sub(Float::F32(b)),
                    Float::F64(b) => a.to_f64().sub(Float::F64(b))
                }),
            }

            (FixedNum::UnSign(a), num) => match num {
                FixedNum::Undefined => FixedNum::Undefined,
                FixedNum::Sign(b) => FixedNum::Sign(a.to_i().sub(b)),
                FixedNum::UnSign(b) => FixedNum::UnSign(a.sub(b)),
                FixedNum::Float(b) => FixedNum::Float(match b {
                    Float::F32(b) => a.to_f32().sub(Float::F32(b)),
                    Float::F64(b) => a.to_f64().sub(Float::F64(b))
                }),
            }

            (FixedNum::Float(a), num) => match num {
                FixedNum::Undefined => FixedNum::Undefined,
                FixedNum::Sign(b) => FixedNum::Float(match a {
                    Float::F32(a) => b.to_f32().sub(Float::F32(a)),
                    Float::F64(a) => b.to_f64().sub(Float::F64(a))
                }),
                FixedNum::UnSign(b) => FixedNum::Float(match a {
                    Float::F32(a) => b.to_f32().sub(Float::F32(a)),
                    Float::F64(a) => b.to_f64().sub(Float::F64(a))
                }),
                FixedNum::Float(b) => FixedNum::Float(a.sub(b)),
            }
        }
    }

    fn mul(&self) -> FixedNum {
        match self.per_cal() {
            (FixedNum::Undefined, _) => FixedNum::Undefined,
            (_, FixedNum::Undefined) => FixedNum::Undefined,

            (FixedNum::Sign(a), num) => match num {
                FixedNum::Undefined => FixedNum::Undefined,
                FixedNum::Sign(b) => FixedNum::Sign(a.mul(b)),
                FixedNum::UnSign(b) => FixedNum::Sign(a.mul(b.to_i())),
                FixedNum::Float(b) => FixedNum::Float(match b {
                    Float::F32(b) => a.to_f32().mul(Float::F32(b)),
                    Float::F64(b) => a.to_f64().mul(Float::F64(b))
                }),
            }

            (FixedNum::UnSign(a), num) => match num {
                FixedNum::Undefined => FixedNum::Undefined,
                FixedNum::Sign(b) => FixedNum::Sign(a.to_i().mul(b)),
                FixedNum::UnSign(b) => FixedNum::UnSign(a.mul(b)),
                FixedNum::Float(b) => FixedNum::Float(match b {
                    Float::F32(b) => a.to_f32().mul(Float::F32(b)),
                    Float::F64(b) => a.to_f64().mul(Float::F64(b))
                }),
            }

            (FixedNum::Float(a), num) => match num {
                FixedNum::Undefined => FixedNum::Undefined,
                FixedNum::Sign(b) => FixedNum::Float(match a {
                    Float::F32(a) => b.to_f32().mul(Float::F32(a)),
                    Float::F64(a) => b.to_f64().mul(Float::F64(a))
                }),
                FixedNum::UnSign(b) => FixedNum::Float(match a {
                    Float::F32(a) => b.to_f32().mul(Float::F32(a)),
                    Float::F64(a) => b.to_f64().mul(Float::F64(a))
                }),
                FixedNum::Float(b) => FixedNum::Float(a.mul(b)),
            }
        }
    }

    fn div(&self) -> FixedNum {
        match self.per_cal() {
            (FixedNum::Undefined, _) => FixedNum::Undefined,
            (_, FixedNum::Undefined) => FixedNum::Undefined,

            (FixedNum::Sign(a), num) => match num {
                FixedNum::Undefined => FixedNum::Undefined,
                FixedNum::Sign(b) => FixedNum::Sign(a.div(b)),
                FixedNum::UnSign(b) => FixedNum::Sign(a.div(b.to_i())),
                FixedNum::Float(b) => FixedNum::Float(match b {
                    Float::F32(b) => a.to_f32().div(Float::F32(b)),
                    Float::F64(b) => a.to_f64().div(Float::F64(b))
                }),
            }

            (FixedNum::UnSign(a), num) => match num {
                FixedNum::Undefined => FixedNum::Undefined,
                FixedNum::Sign(b) => FixedNum::Sign(a.to_i().div(b)),
                FixedNum::UnSign(b) => FixedNum::UnSign(a.div(b)),
                FixedNum::Float(b) => FixedNum::Float(match b {
                    Float::F32(b) => a.to_f32().div(Float::F32(b)),
                    Float::F64(b) => a.to_f64().div(Float::F64(b))
                }),
            }

            (FixedNum::Float(a), num) => match num {
                FixedNum::Undefined => FixedNum::Undefined,
                FixedNum::Sign(b) => FixedNum::Float(match a {
                    Float::F32(a) => b.to_f32().div(Float::F32(a)),
                    Float::F64(a) => b.to_f64().div(Float::F64(a))
                }),
                FixedNum::UnSign(b) => FixedNum::Float(match a {
                    Float::F32(a) => b.to_f32().div(Float::F32(a)),
                    Float::F64(a) => b.to_f64().div(Float::F64(a))
                }),
                FixedNum::Float(b) => FixedNum::Float(a.div(b)),
            }
        }
    }

    fn per_cal(&self) -> (FixedNum, FixedNum) {
        (self.a.cal(), self.b.cal())
    }
}

impl std::fmt::Display for Expr<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Expr< a:{:?}, b:{:?}, operation:{:?} >", self.a, self.b, self.expr_type)
    }
}