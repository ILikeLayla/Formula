pub mod count;
pub mod glo_cons;
pub mod glo_var;
pub mod name;
pub mod glo_func;

use super::{NAME, GLO_CONS_MAP, GLO_VAR_MAP, COUNT, GLO_FUNC_MAP};
use super::num_type::{Variable, Constant, Num};
use super::operation::Func;
use super::warn;
