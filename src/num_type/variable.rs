use crate::var_list::get_var_by_name;

use super::{fixed_num::*, Expr, traits::{Prt, Val}, Op, BasicOp, Num, Constant, name_insert, delete_name, push_var, Name};
use std::cell::RefCell;

// #[derive(Debug, Clone, Copy)]
// pub enum ChangeNum<'a> {
//     Var(&'a Variable<'a>),
//     Expr(&'a Expr<'a>),
//     Undefined,
// }

#[derive(Debug, Clone)]
pub struct Variable<'a> {
    name: Name<'a>,
    num: Num<'a>
}

impl<'a: 'static> Variable<'a> {
    pub fn new(name:&'a str, num: Num<'a>) -> Result<Num<'a>, &'a str> {
        if let Err(msg) = name_insert(name) {
            Err(msg)
        } else {
            let num = match num {
                Num::Expr(expr) => Num::Expr(expr),
                Num::Var(var) => Num::Var(var),
                Num::Undefined => Num::Undefined,
                _ => return Err("The type in unacceptable.")
            };
            push_var(Self { name: Name::Str(name), num });
            Ok(get_var_by_name(name).unwrap())
        }
    }

    pub fn new_place_holder() -> Self {
        Self {
            name: Name::PlaceHolder,
            num: Num::Undefined,
        }
    }
}

impl Variable<'_> {
    pub fn drop_name(&self) {
        match self.name {
            Name::Str(name) => delete_name(name),
            _ => ()
        }
    }

    pub fn name(&self) -> Name {
        self.name.clone()
    }
}

// impl Val for Variable<'_> {
//     fn val(&self) -> FixedNum {
//         match self.expr {
//             ChangeNum::Var(num) => num.val().clone(),
//             ChangeNum::Expr(expr) => expr.val().clone(),
//             ChangeNum::Undefined => FixedNum::Undefined,
//         }
//     }
// }

impl Prt for Variable<'_> {
    fn print(&self) -> String {
        // format!("Variable< name:{}, expr:{:?} >", self.name, self.val())
        format!("Variable< name:{:?} >", self.name )
    }
}

impl<'a> std::fmt::Display for Variable<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.print())
    }
}

// impl<'a> std::ops::Add for &Variable<'a> {
//     type Output = Num<'a>;
//     fn add(self, rhs: Self) -> Self::Output {
//         Num::Expr(())
//     }
// }