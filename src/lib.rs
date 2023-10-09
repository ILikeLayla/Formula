pub mod num_type;
pub mod operation;
pub mod calculus;
pub mod linear_algebra;
pub mod traits;
pub mod config;
pub mod static_modifier;
pub mod warn;

use num_type::{Variable, Constant};
use std::collections::HashMap;
use config::*;

#[cfg(test)]
mod tests {
    use crate::{num_type::Variable, NAME, static_modifier::*, COUNT};

    #[test]
    fn test() {
        count::init();
        count::init();
        count::insert("a", 0);
        // println!("{:?}", COUNT);
        unsafe { println!("{:?}", COUNT) };
    }
}

static mut NAME:Vec<String> = Vec::new();
static mut GLO_VAR_MAP: Option<HashMap<&str, Variable>> = None;
static mut GLO_CONS_MAP: Option<HashMap<&str, Constant>> = None;
static mut COUNT: Option<HashMap<&str, usize>> = None;