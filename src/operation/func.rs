use super::num_type::Num;
use super::{Expr};
use std::collections::{HashSet,HashMap};

pub struct Func<'a> {
    name: &'a str,
    inp_name: HashSet<&'a str>,
    input: HashMap<&'a str, Num<'a>>,
    output: Vec<Num<'a>>,
    expr: Vec<Num<'a>>,
}

impl<'a> Func<'a> {
    pub fn new(name: &str, inp_name: Vec<&'a str>, out_name: Vec<&'a str>) -> (Self, Vec<Num>, Vec<Num>) {

    }
}