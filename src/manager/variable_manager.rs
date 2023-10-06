use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use super::num_type::*;

pub struct VarManager<'a> {
    var_map: HashMap<&'a str, &'a Variable<'a>>,
    var_vec: Vec<Variable<'a>>,
    con_map: HashMap<&'a str, &'a Constant<'a>>,
    con_vec: Vec<Constant<'a>>,
    name_used: HashSet<&'a str>,
}

impl VarManager<'_> {
    pub fn new() -> Self {
        Self {
            var_map: HashMap::new(),
            con_map: HashMap::new(),
            var_vec: Vec::new(),
            con_vec: Vec::new(),
            name_used: HashSet::new()
        }
    }
}

impl<'a> VarManager<'a> {
    pub fn add_variable(&'a mut self, name: &'a str, expr: ChangeNum<'a>) -> Result<Num, &str> {
        if self.name_used.insert(name) {
            let var: Variable<'a> = Variable::new(name, expr);
            self.var_vec.push(var);
            let _ = self.var_map.insert(name, self.var_vec.last().unwrap());
            Ok(self.get_var(name).unwrap())
        } else {
            Err("Name is used.")
        }
    }

    pub fn add_constant(&'a mut self, name: &'a str, number: fixed_num::FixedNum) -> Result<Num, &str> {
        if self.name_used.insert(name) {
            let con: Constant<'a> = Constant::new(name, number);
            self.con_vec.push(con);
            let _ = self.con_map.insert(name, self.con_vec.last().unwrap());
            Ok(self.get_cons(name).unwrap())
        } else {
            Err("Name is used.")
        }
    }

    pub fn get_cons(&self, name: &str) -> Option<Num> {
        if let Some(cons) = self.con_map.get(name) {
            Some(Num::Cons(*cons))
        } else {
            None
        }
    }

    pub fn get_var(&'a self, name: &'a str) -> Option<Num> {
        if let Some(var) = self.var_map.get(name) {
            Some(Num::Var(*var))
        } else {
            None
        }
    }
}