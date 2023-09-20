use crate::num_type::{Constant, Variable};
use crate::operation::Expr;
use super::{ExprManager, VarManager, calculus::CalManager, linear_algebra::AlManager};
use super::num_type::{fixed_num::FixedNum, ChangeNum};
use super::operation::{Num, Op};

pub struct GloManager<'a> {
    expr: ExprManager<'a>,
    var: VarManager<'a>,
    cal: Option<CalManager>,
    alg: Option<AlManager>
}

impl<'a> GloManager<'a> {
    pub fn new(cal: bool, alg: bool) -> Self {
        Self {
            expr: ExprManager::new(),
            var: VarManager::new(),
            cal: if cal {Some(CalManager::new())} else {None},
            alg: if alg {Some(AlManager::new())} else {None}
        }
    }

    pub fn add_constant(&mut self, name: &'a str, number: FixedNum) -> Result<&Constant<'a>, &str> {
        self.var.add_constant(name, number)
    }

    pub fn add_variable(&mut self, name: &'a str, expr: ChangeNum) -> Result<&Variable<'a>, &str> {
        self.var.add_variable(name, expr)
    }

    pub fn add_expr(&mut self, a: &'a Num<'a>, b: &'a Num<'a>, expr_type: Op) -> &Expr {
        self.expr.new_expr(a, b, expr_type)
    }
}