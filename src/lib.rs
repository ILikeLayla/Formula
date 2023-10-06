pub mod num_type;
pub mod manager;
pub mod operation;
pub mod calculus;
pub mod linear_algebra;
pub mod traits;
pub mod config;
pub mod event;

static mut EXPR_LIST:Vec<operation::Expr> = Vec::new();

#[cfg(test)]
mod tests {
    use crate::{manager::{GloManager, VarManager}, event::Eva, operation::{Expr, Op, BasicOp}, num_type::{Num, fixed_num::FixedNum}};

    #[test]
    fn test() {
        let mut glo: GloManager<'static> = GloManager::new(VarManager::new(), Eva::new(), None, None);
        let tiny = glo.add_tiny("tiny", true).unwrap();
        let a = tiny + Num::Fixed(FixedNum::Undefined);

        // let a = Expr::new(Num::Fixed(FixedNum::Undefined), tiny.clone(), Op::Basic(BasicOp::Add));
        // let b = Expr::new(Num::Fixed(FixedNum::Undefined), tiny.clone(), Op::Basic(BasicOp::Add));
        // let c = &a + &b;
        // println!("{:?}", c);


    }
}