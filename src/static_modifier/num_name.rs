use super::warn;
use super::NAME;
use std::collections::HashSet;

pub fn init() {
    unsafe {
        if let None = NAME {
            NAME = Some(HashSet::new())
        } else {
            warn::repeat_init()
        }
    }
}

pub fn insert<'a: 'static>(name: &'a str) -> Result<(), &str> {
    unsafe {
        if let Some(name_set) = NAME.as_mut() {
            if name_set.insert(name) {
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

pub fn delete_name(name: &str) {
    unsafe {
        if let Some(name_set) = NAME.as_mut() {
            name_set.retain(|x| x != &name)
        } else {
            warn::had_not_init()
        }
    }
}

pub fn contain(k: &str) -> bool {
    unsafe {
        if let Some(set) = &NAME {
            set.contains(k)
        } else {
            warn::had_not_init();
            false
        }
    }
}