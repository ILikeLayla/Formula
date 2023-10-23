use super::{NUM_NAME, STATIC_SCOPE, MAX_SCOPE, warn};
use std::collections::{HashSet, HashMap};

pub fn init() {
    unsafe {
        if let None = NUM_NAME {
            let mut map = if MAX_SCOPE != 0 {HashMap::with_capacity(MAX_SCOPE)} else {HashMap::new()};
            if STATIC_SCOPE {
                map.insert("static", HashSet::new());
            }
            NUM_NAME = Some(map);
        } else {
            warn::repeat_init()
        }
    }
}

pub fn add_scope(name: &str) -> Result<(), &str> {
    unsafe {
        if let Some(name_set) = NUM_NAME.as_mut() {
            if let Some(_) = name_set.insert(name, HashSet::new()) {
                Ok(())
            } else {
                warn::name_used();
                Err("SN-1")
            }
            
        } else {
            warn::had_not_init();
            Err("SM-1")
        }
    }
}

pub fn delete_name(scope: &str, name: &str) {
    unsafe {
        if let Some(map) = NUM_NAME.as_mut() {
            if let Some(set) = map.get_mut(name) {
                set.retain(|x| x != &name)
            } else {
                warn::
            }
        }
        if let Some(name_set) = NUM_NAME.as_mut() {
            name_set.retain(|x| x != &name)
        } else {
            warn::had_not_init()
        }
    }
}

pub fn contain(k: &str) -> bool {
    unsafe {
        if let Some(set) = &NUM_NAME {
            set.contains(k)
        } else {
            warn::had_not_init();
            false
        }
    }
}