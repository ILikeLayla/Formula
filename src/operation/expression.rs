use crate::num_type::fixed_num::{Float, FixedNum};
use crate::static_modifier::count;
use super::num_type::Num;
use super::op::{Op, BasicOp, Tri, Expo};
use super::{RUDE_DIV, FULL_DISPLAY};


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
    pub fn drop_borrow(&mut self) {
        let mut check = (false, false);
        
        match self.a {
            Num::Var(var) => count::sub_one(var.name().to_str()),
            Num::Cons(cons) => count::sub_one(cons.name().to_str()),
            Num::Expr(_) => check.0 = true,
            _ => ()
        }

        match self.b {
            Num::Var(var) => count::sub_one(var.name().to_str()),
            Num::Cons(cons) => count::sub_one(cons.name().to_str()),
            Num::Expr(_) => check.1 = true,
            _ => ()
        }

        match check {
            (true, true) => {
                self.a.drop_borrow(); 
                self.b.drop_borrow()
            },
            (true, false) => self.a.drop_borrow(),
            (false, true) => self.b.drop_borrow(),
            (false, false) => (),
        }
        
        self.a = Num::Undefined;
        self.b = Num::Undefined;
    }

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
                FixedNum::Sign(b) => FixedNum::Float(a.div(b)),
                FixedNum::UnSign(b) => FixedNum::Sign(a.rude_div(b.to_i())),
                FixedNum::Float(b) => FixedNum::Float(match b {
                    Float::F32(b) => a.to_f32().div(Float::F32(b)),
                    Float::F64(b) => a.to_f64().div(Float::F64(b))
                }),
            }

            (FixedNum::UnSign(a), num) => match num {
                FixedNum::Undefined => FixedNum::Undefined,
                FixedNum::Sign(b) => FixedNum::Float(a.div(b.to_u())),
                FixedNum::UnSign(b) => {
                    if RUDE_DIV {
                        FixedNum::UnSign(a.rude_div(b))
                    } else {
                        FixedNum::Float(a.div(b))
                    }
                },
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
        let a = if FULL_DISPLAY { self.a.val_print() } else { self.a.symbol() };
        let b = if FULL_DISPLAY { self.b.val_print() } else { self.b.symbol() };

        // println!("{}, {}", a, b);

        let formula = match self.expr_type {
            Op::Basic(base) => {
                let sign = match base {
                    BasicOp::Add => "+",
                    BasicOp::Sub => "-",
                    BasicOp::Mul => "*",
                    BasicOp::Div => "/",
                };
                format!("({} {} {})", a, sign, b)
            }
            Op::Expo(expo) => {
                let sign = match expo {
                    Expo::Exp => "exp",
                    Expo::Log => "log",
                };
                format!("{}({}, {})", sign, a, b)
            },
            Op::Tri(tri) => {
                let sign = match tri {
                    Tri::Sin => "sin",
                    Tri::Con => "cos",
                    Tri::Tan => "tan",
                    Tri::Arcsin => "arcsin",
                    Tri::Arccos => "arccos",
                    Tri::Arctan => "acrtan",
                };
                format!("{}({})", sign, a)
            },
        };
        write!(f, "{}", formula)
    }
}