use super::{STEP, num_type::{fixed_num, Constant}, GloManager};

pub fn get_po_small<'a>(glo_manager:&'a mut GloManager<'a>, name:&'a str) -> Result<&'static Constant<'a>, &'a str> {
    glo_manager.add_constant(name,
        fixed_num::FixedNum::Float(
            fixed_num::Float::F64(
                0.1_f64.powi(0-STEP as i32)
            )
        ) 
    )
}

pub fn get_po_huge<'a>(glo_manager:&'a mut GloManager<'a>, name:&'a str) -> Result<&'static Constant<'a>, &'a str> {
    glo_manager.add_constant(name,
        fixed_num::FixedNum::UnSign(
            fixed_num::UnSignNum::U128(
                10_u128.pow(STEP.into())
            )
        ) 
    )
}

pub fn get_ne_small<'a>(glo_manager:&'a mut GloManager<'a>, name:&'a str) -> Result<&'static Constant<'a>, &'a str> {
    glo_manager.add_constant(name,
        fixed_num::FixedNum::Float(
            fixed_num::Float::F64(
                -1.0_f64 * 0.1_f64.powi(0-STEP as i32)
            )
        ) 
    )
}

pub fn get_ne_huge<'a: 'static>(glo_manager:&'a mut GloManager<'a>, name:&'a str) -> Result<&'a Constant<'a>, &'a str> {
    glo_manager.add_constant(name,
        fixed_num::FixedNum::Sign(
            fixed_num::SignNum::I128(
                -1_i128 * 10_i128.pow(STEP.into())
            )
        ) 
    )
}