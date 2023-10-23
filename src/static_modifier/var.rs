use std::collections::HashMap;
use super::warn;
use super::Variable;
use super::VAR;
use super::Num;

pub fn init() {
    unsafe {
        if let None = VAR {
            VAR = Some(HashMap::new())
        } else {
            warn::repeat_init()
        }
    }
}

pub fn insert_scope(scope: &str) -> Option<HashMap<&str, Variable<'_>>>{
    unsafe {
        if let Some(map) = VAR.as_mut() {
            map.insert(scope, HashMap::new())
        } else { 
            warn::had_not_init();
            None
        }
    }
}

pub fn get<'a>(scope: &str, k: &str) -> Option<Num<'a>> {
    unsafe {
        if let Some(var_map) = &VAR {
            if let Some(map) = var_map.get(scope) {
                if let Some(var) = map.get(k) {
                    Some(Num::Var(var))
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
        VAR.as_mut()
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