mod variable;
mod constant;
mod num;

pub use variable::*;
pub use constant::*;
pub use num::*;

use super::operation::{Op, Expr, BasicOp};
use super::traits;
use super::name::*;
use super::con_list::*;
use super::var_list::*;

#[derive(Debug, Clone)]
pub enum Name<'a> {
    Str(&'a str),
    PlaceHolder
}