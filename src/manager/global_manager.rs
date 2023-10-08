use super::{VarManager, calculus::CalManager, linear_algebra::AlManager};
use super::num_type::{fixed_num, Constant, Variable, Num};
use super::operation::{Op, Expr};
use super::config;
use super::event::Eva;

pub struct GloManager<'a> {
    var: VarManager<'a>,
    event: Eva,
    cal: Option<CalManager>,
    alg: Option<AlManager>
}

impl<'a> GloManager<'a> {
    pub fn new(var: VarManager<'a>, event: Eva, cal: Option<CalManager>, alg: Option<AlManager>) -> Self {
        Self {
            var, cal, alg, event
        }
    }

    // pub fn add_constant(&mut self, name: &'a str, number: fixed_num::FixedNum) -> Result<Num, &str> {
    //     self.var.add_constant(name, number)
    // }

    // pub fn add_variable(&'a mut self, name: &'a str, expr: ChangeNum<'a>) -> Result<Num, &str> {
    //     self.var.add_variable(name, expr)
    // }

    // pub fn get_var(&'a self, name: &'a str) -> Option<Num> {
    //     self.var.get_var(name)
    // }

    // pub fn get_cons(&self, name: &'a str) -> Option<Num> {
    //     self.var.get_cons(name)
    // }

    // pub fn add_tiny(&mut self, name:&'a str, positive: bool) -> Result<Num, &str> {
    //     let pos:f64 = if positive {1.0} else {-1.0};
    //     let num = fixed_num::FixedNum::Float(fixed_num::Float::F64(pos * (0.1_f64).powi(config::STEP.into())));
    //     self.add_constant(name, num)
    // }

    // pub fn add_huge(&'a mut self, name:&'a str, positive: bool) -> Result<Num, &str> {
    //     let pos:i128 = if positive {1} else {-1};
    //     let num = fixed_num::FixedNum::Sign(fixed_num::SignNum::I128(pos * 10_i128.pow(config::STEP.into())));
    //     self.add_constant(name, num)
    // }

}