mod cal_manager;
mod limit;
mod inte;
mod infinite;
mod diff;

pub use cal_manager::*;
pub use limit::*;
pub use diff::*;
pub use inte::*;
pub use infinite::*;

use super::{num_type, operation, linear_algebra, manager::GloManager};

pub const STEP:u8 = 5;