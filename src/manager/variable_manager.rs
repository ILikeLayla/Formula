use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use super::num_type::*;

pub struct VarManager<'a> {
    var_map: HashMap<&'a str, Variable<'a>>,
    con_map: HashMap<&'a str, Constant<'a>>,
    name_used: HashSet<&'a str>,
}

impl<'a> VarManager<'a> {
    pub fn new() -> Self {
        Self {
            var_map: HashMap::new(),
            con_map: HashMap::new(),
            name_used: HashSet::new()
        }
    }

    pub fn add_variable(&mut self, name: &'a str, expr: ChangeNum<'a>) -> Result<(), &str> {
        if self.name_used.insert(name) {
            let var = Variable::new(name, expr);
            let _ = self.var_map.insert(name, var);
            Ok(())
        } else {
            Err("Name is used.")
        }
    }

    pub fn add_constant(&mut self, name: &'a str, number: fixed_num::FixedNum) -> Result<(), &str> {
        if self.name_used.insert(name) {
            let con = Constant::new(name, number);
            let _ = self.con_map.insert(name, con);
            Ok(())
        } else {
            Err("Name is used.")
        }
    }

    pub fn get_var(&self, name: &'a str) -> Option<&Variable<'a>> {
        self.var_map.get(name)
    }

    pub fn get_cons(&self, name: &'a str) -> Option<&Constant<'a>> {
        self.con_map.get(name)
    }

    // pub fn used_name(&self) -> Vec<&str> {
    //     self.used_name().clone()
    // }
}