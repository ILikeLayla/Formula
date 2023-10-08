pub mod num_type;
pub mod operation;
pub mod calculus;
pub mod linear_algebra;
pub mod traits;
pub mod config;
pub mod event;

use num_type::{Variable, Constant};
use std::collections::HashMap;
use config::*;

#[cfg(test)]
mod tests {
    use crate::{num_type::Variable, NAME, name, count, COUNT};

    #[test]
    fn test() {
        count::init();
        count::insert("a", 0);
        // println!("{:?}", COUNT);
        unsafe { println!("{:?}", COUNT) };
    }
}

static mut NAME:Vec<String> = Vec::new();
static mut GLO_VAR_LIST: Vec<Variable> = Vec::new();
static mut GLO_CONS_LIST: Vec<Constant> = Vec::new();
static mut COUNT: Option<HashMap<&str, usize>> = None;

pub mod con_list {
    use crate::{num_type::{Constant, Num}, GLO_CONS_LIST, num_type::Name};

    pub fn push_cons(cons: Constant<'static>) {
        unsafe {
            GLO_CONS_LIST.push(cons)
        }
    }

    pub fn get_cons_by_name(aim: &str) -> Option<Num<'static>> {
        for i in unsafe {
            GLO_CONS_LIST.iter()
        } {
            if let Name::Str(name) = i.name() {
                if name == aim {
                    return Some(Num::Cons(i))
                }
            }
        };
        return None
    }
}

pub mod var_list {
    use crate::{num_type::Variable, GLO_VAR_LIST, num_type::{Num, Name}};

    pub fn push_var(var: Variable<'static>) {
        unsafe {
            GLO_VAR_LIST.push(var)
        }
    }

    pub fn get_var_by_name(aim:&str) -> Option<Num<'static>> {
        for i in unsafe {
            GLO_VAR_LIST.iter()
        } {
            if let Name::Str(name) = i.name() {
                if name == aim {
                    return Some(Num::Var(i))
                }
            }
        };
        return None;
    }
}

pub mod name {
    use super::NAME;

    pub fn name_insert(name: &str) -> Result<(), &str> {
        if  unsafe {
            NAME.contains(&name.to_string())
        } {
            Err("Name is used")
        } else {
            unsafe {
                NAME.push(name.to_string())
            }
            Ok(())
        }
    }
    
    pub fn delete_name(name: &str) {
        unsafe {
            NAME.retain(|x| x != name)
        }
    }
}

pub mod count {
    use std::collections::HashMap;
    use super::{WARN_LEVEL, COUNT};

    pub fn init() {
        unsafe {
            if let None = COUNT {
                COUNT = Some(HashMap::new())
            } else {
                match WARN_LEVEL {
                    0 => panic!("The map already had beed initilallized!"),
                    1 => println!("[WARNING] The map already had beed initilallized!"),
                    _ => ()
                }
            }
        }
    }

    pub fn insert(k: &'static str, v: usize) -> Option<usize> {
        unsafe {
            if let Some(map) = COUNT.as_mut() {
                map.insert(k, v)
            } else { match WARN_LEVEL {
                0 => panic!("The map hadn't beed initiallized!"),
                1 => {println!("[WARNING] The map hadn't beed initiallized!"); None},
                _ => None
            } }
        }
    }

    pub fn get(k: &str) -> Option<&usize> {
        unsafe {
            if let Some(map) = &COUNT {
                map.get(k)
            } else { match WARN_LEVEL {
                0 => panic!("The name is not used!"),
                1 => {println!("[WARNING] The name is not used!"); None},
                _ => None
            } }
        }
    }
}