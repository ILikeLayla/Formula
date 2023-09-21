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
    use super::*;
    use std::rc::Rc;
    use std::cell::RefCell;
    use crate::manager::{ExprManager, GloManager, VarManager};
    use crate::num_type::fixed_num::FixedNum;

    #[test]
    fn it_works() {
        let var = VarManager::new();
        let expr = ExprManager::new();
        let mut glo = GloManager::new(var, expr, None, None);

        glo.add_constant("a", FixedNum::Undefined).unwrap();
        println!("{:?}", glo.get_cons("a"))
        // let _ = var.add_constant("a", FixedNum::Undefined);
        // println!("{:?}", var.get_cons("a"))
    }

}