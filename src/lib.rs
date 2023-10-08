use num_type::{Variable, Constant};

pub mod num_type;
pub mod manager;
pub mod operation;
pub mod calculus;
pub mod linear_algebra;
pub mod traits;
pub mod config;
pub mod event;

static mut NAME:Vec<String> = Vec::new();
static mut GLO_VAR_LIST: Vec<Variable> = Vec::new();
static mut GLO_CONS_LIST: Vec<Constant> = Vec::new();

pub mod con_list {
    use crate::{num_type::Constant, GLO_CONS_LIST, num_type::Name};

    pub fn push_cons(cons: Constant) {
        unsafe {
            GLO_CONS_LIST.push(cons)
        }
    }

    pub fn get_cons_by_name(aim:&str) -> &Constant {
        for i in unsafe {
            GLO_CONS_LIST.iter()
        } {
            if let Name::Str(name) = i.name() {
                if name == aim {
                    return i
                }
            }
        };
        return &Constant::new_place_holder()
    }
}

pub mod var_list {
    use crate::{num_type::Variable, GLO_VAR_LIST};

    pub fn push_var(var: Variable) {
        unsafe {
            GLO_VAR_LIST.push(var)
        }
    }

    pub fn get_var_by_name(aim:&str) -> &Constant {
        for i in unsafe {
            GLO_CONS_LIST.iter()
        } {
            if let Name::Str(name) = i.name() {
                if name == aim {
                    return i
                }
            }
        };
        return &Constant::new_place_holder()
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

#[cfg(test)]
mod tests {
    use crate::{num_type::Variable, NAME, name};

    #[test]
    fn test() {
        let a = Variable::new("a", crate::num_type::Num::Undefined).unwrap();
        // let b = Variable::new("b", a).unwrap();
        println!("{:?}", a);
    }
}