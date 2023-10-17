use super::num_type::{Variable, Num};
use super::{count, name, glo_func};
use std::collections::{HashSet,HashMap};

pub struct Func<'a> {
    name: &'a str,
    inp_name: HashSet<&'a str>,
    out_name: HashSet<&'a str>,
    input: HashMap<&'a str, Num<'a>>,
    output: HashMap<&'a str, Num<'a>>,
    expr: Vec<Num<'a>>,
}

impl<'a: 'static> Func<'a> {
    pub fn new(name: &'a str, inp_name: Vec<&'a str>, out_name: Vec<&'a str>) -> Result<(&'a Self, HashMap<&'a str, Num<'a>>, HashMap<&'a str, Num<'a>>), &'a str> {
        if let Err(msg) = name::insert(name) {
            Err(msg)
        } else {
            let inp_name: HashSet<&str> = inp_name.into_iter().collect();
            let out_name: HashSet<&str> = out_name.into_iter().collect();
            let mut input = HashMap::new();
            let mut output = HashMap::new();
            for i in inp_name.iter() {
                let buf = *i;
                input.insert(buf, Variable::new_without_name_check(buf, Num::Undefined));
            };
            for i in out_name.iter() {
                let buf = *i;
                output.insert(buf, Variable::new_without_name_check(buf, Num::Undefined));
            };
            let expr = Vec::with_capacity(out_name.len());

            glo_func::insert(name, Self {
                name, inp_name, out_name, expr,
                input: input.clone(), output: output.clone()
            });
            count::insert(name, 0);

            let out = glo_func::get(name).unwrap();
            Ok( (out, input, output))
        }
    }
}