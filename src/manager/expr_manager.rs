use super::operation::{Num, Expr, Op};

pub struct ExprManager<'a> {
    expr: Vec<Expr<'a>>
}

impl<'a> ExprManager<'a> {
    pub fn new() -> Self {
        Self {
            expr: Vec::new()
        }
    }

    pub fn new_expr(&mut self, a: &'a Num<'a>, b: &'a Num<'a>, op: Op) -> &Expr {
        let expr = Expr::new(a, b, op);
        self.expr.push(expr);
        return &self.expr[self.expr.len()-1]
    }
}