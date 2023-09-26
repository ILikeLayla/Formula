use super::{BasicOp, Op, Num, Expr};

pub struct CalEve<'a> {
    to_cal: Expr<'a>,
}

impl<'a> CalEve<'a> {
    pub fn new(to_cal: Expr<'a>) -> Self {
        Self {
            to_cal
        }
    }
}

pub struct CalEveList<'a> {
    to_cal: Vec<CalEve<'a>>
}

impl CalEveList<'_> {
    pub fn new() -> Self {
        Self {
            to_cal: Vec::new()
        }
    }
}