use super::{val::Val, Num, Name, name, glo_var, warn, fixed_num, count};

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
            count::insert(name, 0);
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
}

impl Val for Variable<'_> {
    fn val(&self) -> Num<'_> {
        Num::Fixed(self.cal())
    }
}

impl<'a> std::fmt::Display for Variable<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Variable< name:{:?} >", self.name)
    }
}