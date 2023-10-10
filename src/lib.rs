pub mod num_type;
pub mod operation;
pub mod calculus;
pub mod linear_algebra;
pub mod traits;
pub mod config;
pub mod static_modifier;
pub mod warn;

use num_type::{Variable, Constant};
use static_modifier::{count, glo_cons, glo_var};
use std::collections::HashMap;
use config::*;

pub fn init() {
    count::init();
    glo_cons::init();
    glo_var::init();
}

#[cfg(test)]
mod tests {
    use crate::{num_type::{Variable, Constant}, NAME, static_modifier::*, COUNT, init};

    #[test]
    fn test() {
        init();
        let e = Constant::new("e", 
            crate::num_type::Num::Fixed(
                crate::num_type::fixed_num::FixedNum::Float(
                    crate::num_type::fixed_num::Float::F64(2.71828)
                )
            )
        ).unwrap();
        let pi = Constant::new("pi", 
            crate::num_type::Num::Fixed(
                crate::num_type::fixed_num::FixedNum::Float(
                    crate::num_type::fixed_num::Float::F64(3.1415926)
                )
            )
        ).unwrap();
        let mut a = Variable::new("a", e.clone() + pi.clone()).unwrap();
        println!("{:?}", a.cal())
    }
}

static mut NAME:Vec<String> = Vec::new();
static mut GLO_VAR_MAP: Option<HashMap<&str, Variable>> = None;
static mut GLO_CONS_MAP: Option<HashMap<&str, Constant>> = None;
static mut COUNT: Option<HashMap<&str, usize>> = None;