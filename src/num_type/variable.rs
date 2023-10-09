use super::{traits::{Prt, Val}, Num, Name, name, glo_var, warn, fixed_num};
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
        if let Err(msg) = name::name_insert(name) {
            Err(msg)
        } else {
            let num = match num {
                Num::Expr(expr) => Num::Expr(expr),
                Num::Var(var) => Num::Var(var),
                Num::Undefined => Num::Undefined,
                _ => {warn::unacc_type(); return Err("T-1")}
            };
            glo_var::insert(name, Self { name: Name::Str(name), num });
            Ok(glo_var::get(name).unwrap())
        }
    }

    pub fn new_place_holder() -> Self {
        Self {
            name: Name::PlaceHolder,
            num: Num::Undefined,
        }
    }

    pub fn change_num(&mut self, num: Num<'a>) {
        self.num = num
    }
}

impl Variable<'_> {
    pub fn drop_name(&self) {
        match self.name {
            Name::Str(name) => name::delete_name(name),
            _ => ()
        }
    }

    pub fn name(&self) -> Name {
        self.name.clone()
    }

    pub fn cal(&self) -> fixed_num::FixedNum {
        self.num.cal()
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