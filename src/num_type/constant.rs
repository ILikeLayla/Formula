use super::traits::{Val, Prt};

pub mod num {
    #[derive(Debug, Copy, Clone)]
    pub enum FixedNum {
        Sign(SignNum),
        UnSign(UnSignNum),
        Float(Float),
        Undefined
    }

    #[derive(Debug, Copy, Clone)]
    pub enum SignNum {
        I8(i8), I16(i16), I32(i32), I64(i64), I128(i128), Isize(isize)
    }

    #[derive(Debug, Copy, Clone)]
    pub enum UnSignNum {
        U8(u8), U16(u16), U32(u32), U64(u64), U128(u128), Usize(usize)
    }

    #[derive(Debug, Copy, Clone)]
    pub enum Float {
        F32(f32), F64(f64)
    }
}

#[derive(Debug)]
pub struct Constant<'a> {
    name: &'a str,
    number: num::FixedNum
}

impl<'a> Constant<'a> {
    pub fn new(name: &'a str, number: num::FixedNum) -> Self {
        Self { name, number }
    }
}

impl Val for Constant<'_> {
    fn val(&self) -> num::FixedNum {
        self.number.clone()
    }
}

impl Prt for Constant<'_> {
    fn print(&self) -> String {
        format!("Constant< name:{}, number:{:?}>", self.name, self.val())
    }
}

impl std::fmt::Display for Constant<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.print())
    }
}