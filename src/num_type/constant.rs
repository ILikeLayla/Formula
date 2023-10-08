use super::traits::{Val, Prt};
use super::{Num, name_insert, fixed_num, delete_name, Name};

#[derive(Debug)]
pub struct Constant<'a> {
    name: Name<'a>,
    num: Num<'a>
}

impl<'a> Constant<'a> {
    pub fn new(name: &'a str, num: Num<'a>) -> Result<Self, &'a str> {
        if let Err(msg) = name_insert(name) {
            Err(msg)
        } else {
            let num = match num {
                Num::Expr(expr) => Num::Expr(expr),
                Num::Var(var) => Num::Var(var),
                Num::Undefined => Num::Undefined,
                _ => return Err("The type in unacceptable.")
            };
            Ok(Self {
                name: Name::Str(name), num
            })
        }
    }
}

impl Constant<'_> {
    pub fn drop_name(&self) {
        match self.name {
            Name::Str(name) => delete_name(name),
            _ => ()
        }
    }

    pub fn name(&self) -> Name {
        self.name.clone()
    }

    pub fn new_place_holder() -> Self {
        Self {
            name: Name::PlaceHolder,
            num: Num::Undefined,
        }
    }
}

impl Val for Constant<'_> {
    fn val(&self) -> fixed_num::FixedNum {
        match self.num {
            Num::Cons(cons) => cons.val(),
            Num::Fixed(fix) => fix,
            Num::Undefined => fixed_num::FixedNum::Undefined,
            _ => fixed_num::FixedNum::Undefined,
        }
    }
}

impl Prt for Constant<'_> {
    fn print(&self) -> String {
        format!("Constant< name:{:?}, number:{:?}>", self.name, self.val())
    }
}

impl std::fmt::Display for Constant<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.print())
    }
}