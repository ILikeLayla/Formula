mod variable;
mod constant;
mod num;

pub use variable::*;
pub use constant::*;
pub use num::*;

use super::operation::{Op, Expr, BasicOp, Func};
use super::val;
use super::static_modifier::{num_name, glo_cons, glo_var, count};
use super::warn;
use super::config::RUDE_DIV;

#[derive(Debug, Clone, Copy)]
pub enum Name<'a> {
    Str(&'a str),
    PlaceHolder
}

impl Name<'_> {
    pub fn to_str(&self) -> &str {
        match self {
            Name::Str(a) => a,
            Name::PlaceHolder => "",
        }
    }
}

impl std::fmt::Display for Name<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Name::Str(str) => write!(f, "{}", str),
            Name::PlaceHolder => write!(f, "PLACEHOLDER"),
        }
    }
}