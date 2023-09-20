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

    #[test]
    fn it_works() {
        let mut glo: manager::GloManager<'static> = manager::GloManager::new(true, false);
        println!("{:?}", calculus::get_ne_huge(&mut glo, "x").unwrap())
    }

}
