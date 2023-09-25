pub mod num_type;
pub mod manager;
pub mod operation;
pub mod calculus;
pub mod linear_algebra;
pub mod traits;
pub mod config;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use crate::{manager::{GloManager, VarManager}, operation::Expr};
    // use crate::num_type::fixed_num::FixedNum;

    #[test]
    fn it_works() {
        let mut glo = GloManager::new(
            VarManager::new(), 
            None, None
        );

        let _ = glo.add_tiny("a", true);
        let _ = glo.add_variable("b", crate::num_type::ChangeNum::Undefined);
        let a = crate::operation::Num::Cons(glo.get_cons("a").unwrap());
        let b = crate::operation::Num::Var(glo.get_var("b").unwrap());
        let _ = glo.add_variable("c", crate::num_type::ChangeNum::Expr(Expr::new(a, b, crate::operation::Op::Basic(crate::operation::BasicOp::Add))));
        // glo.add_variable("a", ChangeNum::Undefined).unwrap();
        // println!("{:?}", );
        // println!("{:?}", glo.get("b"));

        // glo.add_constant("a", FixedNum::Undefined);
        // println!("{:?}", var.get_cons("a"))
    }

}