use super::{val::Val, Num, Name, num_name, var, warn, fixed_num, count};
use std::cell::{RefCell, Ref};

#[derive(Debug, Clone)]
pub struct Variable<'a> {
    name: Name<'a>,
    num: RefCell<Num<'a>>
}


impl<'a: 'static> Variable<'a> {
    pub fn new(name:&'a str, num: Num<'a>) -> Result<Num<'a>, &'a str> {
        if let Err(msg) = num_name::insert(name) {
            Err(msg)
        } else {
            let num = match num {
                Num::Expr(expr) => Num::Expr(expr),
                Num::Var(var) => Num::Var(var),
                Num::Undefined => Num::Undefined,
                _ => {warn::unacc_type(); return Err("T-1")}
            };
            var::insert(name, Self { name: Name::Str(name), num: RefCell::new(num) });
            count::insert(name, 0);
            Ok(var::get(name).unwrap())
        }
    }

    pub fn new_without_name_check(name:&'a str, num: Num<'a>) -> Num<'a> {
        let num = match num {
            Num::Expr(expr) => Num::Expr(expr),
            Num::Var(var) => Num::Var(var),
            Num::Undefined => Num::Undefined,
            _ => {warn::unacc_type(); Num::Undefined}
        };
        var::insert(name, Self { name: Name::Str(name), num: RefCell::new(num) });
        count::insert(name, 0);
        var::get(name).unwrap()
    }

    pub fn new_place_holder() -> Self {
        Self {
            name: Name::PlaceHolder,
            num: RefCell::new(Num::Undefined),
        }
    }

    pub fn change_num(&self, num: Num<'a>) {
        self.num.borrow_mut().replace(num)
    }

}

impl Variable<'_> {
    pub fn drop_name(&self) {
        match self.name {
            Name::Str(name) => {
                if count::get(self.name.to_str()) == Some(&0) {
                    num_name::delete_name(name); 
                    let _ = count::remove(name);
                }
            },
            _ => ()
        }
    }

    pub fn name(&self) -> Name {
        self.name.clone()
    }

    pub fn cal(&self) -> fixed_num::FixedNum {
        self.num.borrow().cal()
    }

    pub fn drop_borrow(&mut self) {
        self.num.borrow_mut().drop_borrow() ;
        self.num.borrow_mut().replace(Num::Undefined)
    }

    pub fn droppable(&self) -> bool {
        count::check_zero(self.name().to_str())
    }

    pub fn val_print(&self) -> String {
        format!("({})", self.num.borrow())
    }
}

impl<'a> Variable<'a> {
    pub fn expr(&'a self) -> Num {
        self.num.borrow().clone()
    }
}

impl Val for Variable<'_> {
    fn val(&self) -> Num<'_> {
        Num::Fixed(self.cal())
    }
}

impl<'a> std::fmt::Display for Variable<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self.name {
            Name::Str(str) => str,
            Name::PlaceHolder => "PLACEHOLDER"
        };

        let words = match &*self.num.borrow() {
            Num::Expr(expr) => {
                let num_words = format!("{}", *expr);
                (&num_words[1 .. (num_words.len()-1)]).to_string()
            },
            num => format!("{}", num)
        };
        
        write!(f, "{} = {}", name, words)
    }
}