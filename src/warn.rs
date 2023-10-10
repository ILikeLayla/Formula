use super::config::WARN_LEVEL;

pub fn had_not_init() {
    match WARN_LEVEL {
        0 => panic!("SM-1"),
        1 => println!("\x1b[93m[WARNING] SM-1\x1b[0m"),
        _ => (),
    }
}

pub fn name_not_used() {
    match WARN_LEVEL {
        0 => panic!("SN-2"),
        1 => println!("\x1b[93m[WARNING] SN-2\x1b[0m"),
        _ => (),
    }
}

pub fn repeat_init() {
    match WARN_LEVEL {
        0 => panic!("SM-2"),
        1 => println!("\x1b[93m[WARNING] SM-2\x1b[0m"),
        _ => ()
    }
}

pub fn unacc_type() {
    match WARN_LEVEL {
        0 => panic!("T-1"),
        1 => println!("\x1b[93m[WARNING] T-1\x1b[0m"),
        _ => ()
    }
}

pub fn delete_before_no_borrow() {
    match WARN_LEVEL {
        0 => panic!("SC-1"),
        1 => println!("\x1b[93m[WARNING] SC-1\x1b[0m"),
        _ => ()
    }
}

pub fn name_used() {
    match WARN_LEVEL {
        0 => panic!("SN-1"),
        1 => println!("\x1b[93m[WARNING] SN-1\x1b[0m"),
        _ => ()
    }
}