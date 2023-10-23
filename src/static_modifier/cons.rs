use std::collections::HashMap;
use super::warn;
use super::Constant;
use super::CONS;
use super::Num;

pub fn init() {
    unsafe {
        if let None = CONS {
            CONS = Some(HashMap::new())
        } else {
            warn::repeat_init()
        }
    }
}

pub fn insert_scope(scope: &str) -> Option<HashMap<&str, Constant<'_>>>{
    unsafe {
        if let Some(map) = CONS.as_mut() {
            map.insert(scope, HashMap::new())
        } else { 
            warn::had_not_init();
            None
        }
    }
}

pub fn get<'a>(scope: &str, k: &str) -> Option<Num<'a>> {
    unsafe {
        if let Some(cons_map) = &CONS {
            if let Some(map) = cons_map.get(scope) {
                if let Some(cons) = map.get(k) {
                    Some(Num::Cons(cons))
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
        CONS.as_mut()
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