use std::collections::HashMap;
use super::COUNT;
use super::warn;

pub fn init() {
    unsafe {
        if let None = COUNT {
            COUNT = Some(HashMap::new())
        } else {
            warn::repeat_init()
        }
    }
}

pub fn insert(k: &'static str, v: usize) -> Option<usize> {
    unsafe {
        if let Some(map) = COUNT.as_mut() {
            map.insert(k, v)
        } else { 
            warn::had_not_init();
            None
        }
    }
}

pub fn get(k: &str) -> Option<&usize> {
    unsafe {
        if let Some(map) = &COUNT {
            map.get(k)
        } else {
            warn::had_not_init();
            None
        }
    }
}

fn get_mut(k: &str) -> Option<&mut usize> {
    unsafe {
        if let Some(map) = COUNT.as_mut() {
            return map.get_mut(k)
        } else {
            warn::had_not_init();
            None
        }
    }
}

pub fn add(k: &str, num: usize, add: bool) {
    if let Some(val) = get_mut(k) {
        if add {
            *val += num
        } else {
            *val -= num
        }
    } else {
        warn::name_not_used();
    }
}

pub fn add_one(k: &str) {
    add(k, 1, true)
}

pub fn sub_one(k: &str) {
    add(k, 1, false)
}