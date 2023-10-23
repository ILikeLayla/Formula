use super::config::WARN_LEVEL;

trait WarnOut {
    fn warn(self) -> String;
}

impl WarnOut for &str {
    fn warn(self) -> String {
        format!("\x1b[93[WARNING] {}\x1b[0m", self)
    }
}

fn warning(words: &str) {
    match WARN_LEVEL {
        0 => panic!("{}", words),
        1 => println!("{}", words.warn()),
        _ => (),
    }
}

pub fn had_not_init() {
    warning("SM-1")
}

pub fn name_not_used() {
    warning("SN-2")
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
        0 => panic!("D-1"),
        1 => println!("\x1b[93m[WARNING] D-1\x1b[0m"),
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

pub fn type_unchangable() {
    match WARN_LEVEL {
        0 => panic!("T-1"),
        1 => println!("\x1b[93m[WARNING] T-1\x1b[0m"),
        _ => ()
    }
}

pub fn scope_unanounced() {
    match WARN_LEVEL {
        0 => panic!("SC-1"),
        1 => println!("\x1b[93m[WARNING] SC-1\x1b[0m"),
        _ => ()
    }
}