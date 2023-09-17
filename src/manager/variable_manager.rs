use std::collections::{HashMap, HashSet};
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
            let _ = self.var_map.insert(name, Variable::new(name, expr));
            Ok(())
        } else {
            Err("Name is used.")
        }
    }

    pub fn add_constant(&mut self, name: &'a str, number: num::FixedNum) -> Result<(), &str> {
        if self.name_used.insert(name) {
            let _ = self.con_map.insert(name, Constant::new(name, number));
            Ok(())
        } else {
            Err("Name is used.")
        }
    }

    pub fn used_name(&self) -> Vec<&str> {
        self.used_name().clone()
    }
}