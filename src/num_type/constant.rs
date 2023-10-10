use super::{Num, name, glo_cons, fixed_num, Name, warn, val::Val, count};

#[derive(Debug)]
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
                let _ = count::remove(name); 
                name::delete_name(name)
            },
            _ => ()
        }
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
}

impl Val for Constant<'_> {
    fn val(&self) -> Num<'_> {
        Num::Fixed(self.cal())
    }
}

impl std::fmt::Display for Constant<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Constant< name:{:?}, number:{:?}>", self.name, self.val())
    }
}