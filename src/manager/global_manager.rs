use super::{VarManager, calculus::CalManager, linear_algebra::AlManager};
use super::num_type::{fixed_num, ChangeNum, Constant, Variable};
use super::operation::{Num, Op, Expr};
use super::config;
use super::event::Eva;
use std::cell::RefCell;

#[derive(Debug)]
pub enum SearchOut<'a> {
    Cons(&'a Constant<'a>),
    Var(&'a Variable<'a>),
}

pub struct GloManager<'a> {
    var: RefCell<VarManager<'a>>,
    event: Eva,
    cal: Option<RefCell<CalManager>>,
    alg: Option<RefCell<AlManager>>
}

impl<'a> GloManager<'a> {
    pub fn new(var: VarManager<'a>, event: Eva, cal: Option<CalManager>, alg: Option<AlManager>) -> Self {
        let var = RefCell::new(var);
        let cal = if let Some(man) = cal {
            Some(RefCell::new(man))
        } else {
            None
        };
        let alg = if let Some(man) = alg {
            Some(RefCell::new(man))
        } else {
            None
        };
        Self {
            var, cal, alg, event
        }
    }

    pub fn add_constant(&self, name: &'a str, number: fixed_num::FixedNum) -> Result<(), String> {
        // self.var.borrow_mut().add_constant(name, number)
        if let Err(msg) = self.var.borrow_mut().add_constant(name, number) {
            Err(msg.to_string())
        } else { Ok(()) }
    }

    pub fn add_variable(&self, name: &'a str, expr: ChangeNum<'a>) -> Result<(), String> {
        // self.var.borrow_mut().add_variable(name, expr)
        if let Err(msg) = self.var.borrow_mut().add_variable(name, expr) {
            Err(msg.to_string())
        } else { Ok(()) }
    }

    // pub fn get_var(&self, name: &'a str) -> Option<&Variable<'a>> {
    //     self.var.borrow().get_var(name)
    // }

    // pub fn get_cons(&self, name: &'a str) -> Option<&Constant<'a>> {
    //     self.var.borrow().get_cons(name)
    // }

    // pub fn get(&self, name: &'a str) -> Option<SearchOut> {
    //     if let Some(cons) = self.var.borrow().get_cons(name) {
    //         Some(SearchOut::Cons(cons))
    //     } else if let Some(var) = self.var.borrow().get_var(name) {
    //         Some(SearchOut::Var(var))
    //     } else {
    //         None
    //     }
    // }

    pub fn add_tiny(&mut self, name:&'a str, positive: bool) -> Result<(), String> {
        let pos:f64 = if positive {1.0} else {-1.0};
        let num = fixed_num::FixedNum::Float(fixed_num::Float::F64(pos * (0.1_f64).powi(config::STEP.into())));
        self.add_constant(name, num)
    }

    pub fn add_huge(&mut self, name:&'a str, positive: bool) -> Result<(), String> {
        let pos:i128 = if positive {1} else {-1};
        let num = fixed_num::FixedNum::Sign(fixed_num::SignNum::I128(pos * 10_i128.pow(config::STEP.into())));
        self.add_constant(name, num)
    }

}