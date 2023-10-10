pub mod num_type;
pub mod operation;
pub mod calculus;
pub mod linear_algebra;
pub mod val;
pub mod config;
pub mod static_modifier;
pub mod warn;
use std::collections::HashMap;

static mut NAME:Vec<String> = Vec::new();
static mut GLO_VAR_MAP: Option<HashMap<&str, num_type::Variable>> = None;
static mut GLO_CONS_MAP: Option<HashMap<&str, num_type::Constant>> = None;
static mut COUNT: Option<HashMap<&str, usize>> = None;

pub fn init() {
    static_modifier::count::init();
    static_modifier::glo_cons::init();
    static_modifier::glo_var::init();
}

#[cfg(test)]
mod tests {
    use crate::{num_type::{Variable, Constant}, init, val::Val};

    #[test]
    fn test() {
        init();
        let e = Constant::new("e", 2.71828_f32.val()).unwrap();
        let pi = Constant::new("pi", 
            3.1415926535897932384626.val()
        ).unwrap();
        let a = Variable::new("a", e.clone() + pi.clone()).unwrap();
        // a.drop();
        println!("1");
        e.drop();
        // e.drop();
        // println!("{:?}", a.cal());
        // println!("{:?}", a)
    }
}