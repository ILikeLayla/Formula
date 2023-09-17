pub mod num_type;
pub mod manager;
pub mod operation;
pub mod calculus;
pub mod linear_algebra;
pub mod traits;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use crate::num_type::ChangeNum;
    use crate::operation::{BasicOp, Num};
    use super::*;

    #[test]
    fn it_works() {
        println!("{}", num_type::Variable::new("a", ChangeNum::Undefined));
        let a = num_type::Variable::new("a", ChangeNum::Undefined);
        let b = num_type::Variable::new("b", ChangeNum::Var(&a));
        println!("{}", operation::Expr::new(Num::Var(&a), Num::Var(&b), BasicOp::Mul))
    }

}
