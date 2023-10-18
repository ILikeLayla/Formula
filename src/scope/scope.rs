use std::collections::{HashMap, HashSet};

use super::{Constant, Variable, Num, FixedNum, Expr, Func, warn};

pub struct Scope<'a, 'b> {
    scope_name: &'a str,
    name_using: HashSet<&'a str>,
    cons: HashMap<&'a str, Constant<'b>>,
    var: HashMap<&'a str, Variable<'b>>,
    func: HashMap<&'a str, Func<'b>>,
    borrow_count: HashMap<&'a str, usize>,
    parent_scope: Vec<&'b Self>,
}

impl<'a: 'b, 'b: 'a> Scope<'a, 'b> {
    pub fn new(scope_name: &'a str, parent_scope: Vec<&'a Self>) -> Self {
        Self {
            scope_name, parent_scope,
            name_using: HashSet::new(),
            cons: HashMap::new(),
            var: HashMap::new(),
            func: HashMap::new(),
            borrow_count: HashMap::new()
        }
    }

    pub fn add_cons(&mut self, name: &'a str, val: Num<'a>) -> Result<(), &str> {
        if self.name_used(name) {
            warn::name_used();
            Err("SN-1")
        } else {
            let buf = Constant::new(name, val);
            if let Ok(cons) = buf {
                self.borrow_count.insert(name, 0);
                self.name_using.insert(name);
                self.cons.insert(name, cons);
                // Ok(Num::Cons(
                //     self.cons.get(name).unwrap()
                // ))
                Ok(())
            } else if let Err(msg) = buf {
                Err(msg)
            } else {
                Err("")
            }
        }
    }
}

impl Scope<'_, '_> {
    pub fn name_used(&self, k: &str) -> bool {
        if ! self.parent_scope.is_empty() {
            for i in self.parent_scope.iter() {
                if i.name_used(k) {
                    return true
                }
            };
        }
        if self.name_using.contains(k) {
            return true
        };
        return false
    }
}

