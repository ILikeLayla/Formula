use std::collections::HashMap;
use super::warn;
use super::Func;
use super::GLO_FUNC_MAP;
use super::Num;

pub fn init() {
    unsafe {
        if let None = GLO_FUNC_MAP {
            GLO_FUNC_MAP = Some(HashMap::new())
        } else {
            warn::repeat_init()
        }
    }
}

pub fn insert(k: &'static str, v: Func<'static>) -> Option<Func<'static>> {
    unsafe {
        if let Some(map) = GLO_FUNC_MAP.as_mut() {
            map.insert(k, v)
        } else { 
            warn::had_not_init();
            None
        }
    }
}

pub fn get<'a: 'static>(k: &'a str) -> Option<&Func> {
    unsafe {
        if let Some(map) = &GLO_FUNC_MAP {
            if let Some(func) = map.get(k) {
                Some(func)
            } else {
                warn::name_not_used();
                None
            }
        } else {
            warn::had_not_init();
            None
        }
    }
}

pub fn remove(k: &str) -> Result<(), &str> {
    match unsafe {
        GLO_FUNC_MAP.as_mut()
    } {
        Some(map) => {
            if map.contains_key(&k) {
                map.remove(k);
                Ok(())
            } else {
                Err("SN-2")
            }
        },
        None => {
            Err("SM-1")
        }
    }
}