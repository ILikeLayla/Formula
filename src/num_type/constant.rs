use super::{Num, name, glo_cons, fixed_num, Name, warn, val::Val, count};

#[derive(Debug, Clone)]
pub struct Constant<'a> {
    name: Name<'a>,
    num: Num<'a>
}

impl<'a: 'static> Constant<'a> {
    pub fn new(name: &'a str, num: Num<'a>) -> Result<Num<'a>, &'a str> {
        if let Err(msg) = name::name_insert(name) {
            Err(msg)
        } else {
            let num = match num {
                Num::Fixed(fix) => Num::Fixed(fix),
                Num::Cons(cons) => Num::Cons(cons),
                Num::Undefined => Num::Undefined,
                _ => {warn::unacc_type(); return Err("T-1")}
            };
            glo_cons::insert(name, Self { name: Name::Str(name), num });
            count::insert(name, 0);
            Ok(glo_cons::get(name).unwrap())
        }
    }
}

impl Constant<'_> {
    pub fn drop_name(&self) {
        match self.name {
            Name::Str(name) => {
                if count::get(self.name.to_str()) == Some(&0) {
                    name::delete_name(name); 
                    let _ = count::remove(name);
                }
            },
            _ => ()
        }
    }

    pub fn droppable(&self) -> bool {
        count::check_zero(self.name().to_str())
    }

    pub fn name(&self) -> Name {
        self.name.clone()
    }

    pub fn cal(&self) -> fixed_num::FixedNum {
        self.num.cal()
    }

    pub fn new_place_holder() -> Self {
        Self {
            name: Name::PlaceHolder,
            num: Num::Undefined,
        }
    }
    
    pub fn val_print(&self) -> String {
        format!("{}", self.num)
    }
}

impl<'a> Constant<'a> {
    pub fn expr(&'a self) -> Num {
        match self.num {
            Num::Cons(cons) => Num::Cons(cons),
            _ => Num::Undefined
        }
    }
}

impl Val for Constant<'_> {
    fn val(&self) -> Num<'_> {
        Num::Fixed(self.cal())
    }
}

impl std::fmt::Display for Constant<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self.name {
            Name::Str(str) => str,
            Name::PlaceHolder => "PLACEHOLDER"
        };
        write!(f, "{} = {:?}", name, self.num)
    }
}