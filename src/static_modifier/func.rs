use std::collections::HashMap;
use super::warn;
use super::Func;
use super::FUNC;
use super::Num;

pub fn init() {
    unsafe {
        if let None = FUNC {
            FUNC = Some(HashMap::new())
        } else {
            warn::repeat_init()
        }
    }
}

pub fn insert_scope(scope: &str) -> Option<HashMap<&str, Func<'_>>>{
    unsafe {
        if let Some(map) = FUNC.as_mut() {
            map.insert(scope, HashMap::new())
        } else { 
            warn::had_not_init();
            None
        }
    }
}

pub fn get<'a>(scope: &str, k: &str) -> Option<Num<'a>> {
    unsafe {
        if let Some(func_map) = &FUNC {
            if let Some(map) = func_map.get(scope) {
                if let Some(func) = map.get(k) {
                    Some(Num::Func(func))
                } else {
                    warn::name_not_used();
                    None
                }
            } else {
                warn::scope_unanounced();
                None
            }
        } else {
            warn::had_not_init();
            None
        }
    }
}

pub fn remove(k: &str) {
    match unsafe {
        FUNC.as_mut()
    } {
        Some(map) => {
            if map.contains_key(&k) {
                map.remove(k);
            } else {
                ()
            }
        },
        None => {
            ()
        }
    }
}