mod variable;
mod constant;

pub use variable::*;
pub use constant::*;

use super::operation::{Op, Expr, BasicOp, Num};
use super::traits;