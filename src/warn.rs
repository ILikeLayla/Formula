use super::config::WARN_LEVEL;

fn warning(words: &str) {
    match WARN_LEVEL {
        0 => panic!("{}", words),
        1 => println!("\x1b[93[WARNING] {}\x1b[0m", words),
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
    warning("SM-2")
}

pub fn unacc_type() {
    warning("T-1")
}

pub fn delete_before_no_borrow() {
    warning("D-1")
}

pub fn name_used() {
    warning("N-1")
}

pub fn type_unchangable() {
    warning("T-1")
}

pub fn scope_unanounced() {
    warning("SC-1")
}