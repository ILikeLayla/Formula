use super::{VarManager, calculus::CalManager, linear_algebra::AlManager};
use super::num_type::{fixed_num, ChangeNum, Constant, Variable};
use super::operation::{Num, Op, Expr};
use super::config;

#[derive(Debug)]
pub enum SearchOut<'a> {
    Cons(&'a Constant<'a>),
    Var(&'a Variable<'a>),
}

pub struct GloManager<'a> {
    var: VarManager<'a>,
    cal: Option<CalManager>,
    alg: Option<AlManager>
}

impl<'a> GloManager<'a> {
    pub fn new(var: VarManager<'a>, cal: Option<CalManager>, alg: Option<AlManager>) -> Self {
        Self {
            var, cal, alg
        }
    }

    pub fn add_constant(&mut self, name: &'a str, number: fixed_num::FixedNum) -> Result<(), &str> {
        self.var.add_constant(name, number)
    }

    pub fn add_variable(&mut self, name: &'a str, expr: ChangeNum<'a>) -> Result<(), &str> {
        self.var.add_variable(name, expr)
    }

    pub fn get_var(&self, name: &'a str) -> Option<&Variable> {
        self.var.get_var(name)
    }

    pub fn get_cons(&self, name: &'a str) -> Option<&Constant> {
        self.var.get_cons(name)
    }

    pub fn get(&self, name: &'a str) -> Option<SearchOut> {
        if let Some(cons) = self.var.get_cons(name) {
            Some(SearchOut::Cons(cons))
        } else if let Some(var) = self.var.get_var(name) {
            Some(SearchOut::Var(var))
        } else {
            None
        }
    }

    pub fn add_tiny(&mut self, name:&'a str, positive: bool) -> Result<(), &str> {
        let pos:f64 = if positive {1.0} else {-1.0};
        self.add_constant(name, fixed_num::FixedNum::Float(fixed_num::Float::F64(pos * (0.1_f64).powi(config::STEP.into()))))
    }

    pub fn add_huge(&mut self, name:&'a str, positive: bool) -> Result<(), &str> {
        let pos:i128 = if positive {1} else {-1};
        self.add_constant(name, fixed_num::FixedNum::Sign(fixed_num::SignNum::I128(pos * 10_i128.pow(config::STEP.into()))))
    }

}