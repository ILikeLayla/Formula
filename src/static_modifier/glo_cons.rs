use std::collections::HashMap;
use super::warn;
use super::Constant;
use super::GLO_CONS_MAP;
use super::Num;

pub fn init() {
    unsafe {
        if let None = GLO_CONS_MAP {
            GLO_CONS_MAP = Some(HashMap::new())
        } else {
            warn::repeat_init()
        }
    }
}

pub fn insert(k: &'static str, v: Constant<'static>) -> Option<Constant<'static>> {
    unsafe {
        if let Some(map) = GLO_CONS_MAP.as_mut() {
            map.insert(k, v)
        } else { 
            warn::had_not_init();
            None
        }
    }
}

pub fn get(k: &str) -> Option<Num> {
    unsafe {
        if let Some(map) = &GLO_CONS_MAP {
            if let Some(cons) = map.get(k) {
                Some(Num::Cons(cons))
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

// pub fn push_cons(cons: Constant<'static>) {
//     unsafe {
//         GLO_CONS_LIST.push(cons)
//     }
// }

// pub fn get_cons_by_name(aim: &str) -> Option<Num<'static>> {
//     for i in unsafe {
//         GLO_CONS_LIST.iter()
//     } {
//         if let Name::Str(name) = i.name() {
//             if name == aim {
//                 return Some(Num::Cons(i))
//             }
//         }
//     };
//     return None
// }