mod variable;
mod constant;
mod num;

pub use variable::*;
pub use constant::*;
pub use num::*;

use super::operation::{Op, Expr, BasicOp};
use super::val;
use super::static_modifier::{name, glo_cons, glo_var, count};
use super::warn;

#[derive(Debug, Clone)]
pub enum Name<'a> {
    Str(&'a str),
    PlaceHolder
}