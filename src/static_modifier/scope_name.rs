use std::collections::HashSet;

use super::{SCOPE_NAME, warn};

pub fn init() {
    unsafe {
        if let None = SCOPE_NAME {
            SCOPE_NAME = Some(HashSet::new())
        } else {
            warn::repeat_init()
        }
    }
}

pub fn contains(k: &str) -> bool {
    unsafe {
        if let Some(names) = SCOPE_NAME {
            return names.contains(k)
        } else {
            warn::had_not_init();
            false
        }
    }
}

pub fn insert(k: &str) -> Result<(), &str> {
    unsafe {
        if let Some(names) = SCOPE_NAME.as_mut() {
            if names.insert(k) {
                return Ok(())
            } else {
                warn::name_used();
                return Err("SN-1")
            }
        } else {
            warn::had_not_init();
            return Err("SM-1")
        }
    }
}