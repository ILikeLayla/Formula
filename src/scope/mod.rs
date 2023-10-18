mod scope;

pub use scope::*;

use super::num_type::{Constant, Num, Variable, fixed_num::FixedNum};
use super::operation::{Expr, Func};
use super::warn;