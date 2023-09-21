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
    use crate::num_type::fixed_num::FixedNum;

    #[test]
    fn it_works() {
        let mut glo: manager::GloManager<'static> = manager::GloManager::new(true, false);
        let a = glo.add_constant("a", FixedNum::Undefined).expect("TODO");
    }

}