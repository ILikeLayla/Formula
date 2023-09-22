use super::{ExprManager, VarManager, calculus::CalManager, linear_algebra::AlManager};
use super::num_type::{fixed_num::FixedNum, ChangeNum, Constant, Variable};
use super::operation::{Num, Op, Expr};

#[derive(Debug)]
pub enum SearchOut<'a> {
    Cons(&'a Constant<'a>),
    Var(&'a Variable<'a>),
}

pub struct GloManager<'a> {
    expr: ExprManager<'a>,
    var: VarManager<'a>,
    cal: Option<CalManager>,
    alg: Option<AlManager>
}

impl<'a> GloManager<'a> {
    pub fn new(var: VarManager<'a>, expr: ExprManager<'a>, cal: Option<CalManager>, alg: Option<AlManager>) -> Self {
        Self {
            var, expr, cal, alg
        }
    }

    pub fn add_constant(&mut self, name: &'a str, number: FixedNum) -> Result<(), &str> {
        self.var.add_constant(name, number)
    }

    pub fn add_variable(&mut self, name: &'a str, expr: ChangeNum<'a>) -> Result<(), &str> {
        self.var.add_variable(name, expr)
    }

    pub fn add_expr(&mut self, a: &'a Num<'a>, b: &'a Num<'a>, expr_type: Op) -> &Expr {
        self.expr.new_expr(a, b, expr_type)
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
}