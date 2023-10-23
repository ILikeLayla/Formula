pub mod count;
pub mod glo_cons;
pub mod glo_var;
pub mod num_name;
pub mod glo_func;
pub mod scope_name;

use super::{NUM_NAME, GLO_CONS_MAP, GLO_VAR_MAP, COUNT, GLO_FUNC_MAP, SCOPE_NAME};
use super::num_type::{Variable, Constant, Num};
use super::operation::Func;
use super::warn;
use super::config::{MAX_SCOPE, STATIC_SCOPE};
