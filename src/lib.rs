pub mod num_type;
pub mod operation;
pub mod calculus;
pub mod linear_algebra;
pub mod val;
pub mod config;
pub mod static_modifier;
pub mod warn;
use std::collections::{HashSet, HashMap};

use num_type::Num;

static mut NAME:Option<HashSet<&str>> = None;
static mut GLO_VAR_MAP: Option<HashMap<&str, num_type::Variable>> = None;
static mut GLO_CONS_MAP: Option<HashMap<&str, num_type::Constant>> = None;
static mut GLO_FUNC_MAP: Option<HashMap<&str, operation::Func>> = None;
static mut COUNT: Option<HashMap<&str, usize>> = None;
static mut PLACE_HOLDER: Vec<Num> = Vec::new();

pub fn init() {
    static_modifier::count::init();
    static_modifier::glo_cons::init();
    static_modifier::glo_var::init();
    static_modifier::name::init();
    static_modifier::glo_func::init();
}

#[cfg(test)]
mod tests {
    use crate::{num_type::{Variable, Constant, Num}, init, val::Val, COUNT, NAME};

    #[test]
    fn test() {
        init();

        let mut input = Variable::new("input", Num::Undefined).unwrap();
        let output = Variable::new("output", 2.val() * input.clone() + 3.val()).unwrap();

        println!("{}", input);
        input.change_val(4.val());
        println!("{}", input);

        println!();

        println!("{}", output);
        println!("{}", output.cal());
    }
}
