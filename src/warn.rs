use super::WARN_LEVEL;

pub fn had_not_init() {
    match WARN_LEVEL {
        0 => panic!("The map hadn't beed initiallized!"),
        1 => println!("\x1b[93m[WARNING] The map hadn't beed initiallized!\x1b[0m"),
        _ => (),
    }
}

pub fn name_not_used() {
    match WARN_LEVEL {
        0 => panic!("The name is not used!"),
        1 => println!("\x1b[93m[WARNING] The name is not used!\x1b[0m"),
        _ => (),
    }
}

pub fn repeat_init() {
    match WARN_LEVEL {
        0 => panic!("The map already had beed initilallized!"),
        1 => println!("\x1b[93m[WARNING] The map already had beed initilallized!\x1b[0m"),
        _ => ()
    }
}

pub fn unacc_type() {
    match WARN_LEVEL {
        0 => panic!("The type is unacceptable!"),
        1 => println!("\x1b[93m[WARNING] The type is unacceptable!\x1b[0m"),
        _ => ()
    }
}