pub mod num_type;
pub mod operation;
pub mod calculus;
pub mod linear_algebra;
pub mod val;
pub mod config;
pub mod static_modifier;
pub mod warn;
pub mod scope;
use std::collections::{HashSet, HashMap};

static mut SCOPE_NAME: Option<HashSet<&str>> = None;
static mut NUM_NAME:Option<HashMap<&str, HashSet<&str>>> = None;
static mut VAR: Option<HashMap<&str, HashMap<&str, num_type::Variable>>> = None;
static mut CONS: Option<HashMap<&str, HashMap<&str, num_type::Constant>>> = None;
static mut FUNC: Option<HashMap<&str, HashMap<&str, operation::Func>>> = None;
static mut COUNT: Option<HashMap<&str, HashMap<&str, usize>>> = None;
// static mut PLACE_HOLDER: Vec<Num> = Vec::new();

pub fn init() {
    static_modifier::count::init();
    static_modifier::cons::init();
    static_modifier::var::init();
    static_modifier::num_name::init();
    static_modifier::func::init();
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        
    }
}