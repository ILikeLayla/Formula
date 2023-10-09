use std::collections::HashMap;
use super::warn;
use super::Variable;
use super::GLO_VAR_MAP;
use super::Num;

pub fn init() {
    unsafe {
        if let None = GLO_VAR_MAP {
            GLO_VAR_MAP = Some(HashMap::new())
        } else {
            warn::repeat_init()
        }
    }
}

pub fn insert(k: &'static str, v: Variable<'static>) -> Option<Variable<'static>> {
    unsafe {
        if let Some(map) = GLO_VAR_MAP.as_mut() {
            map.insert(k, v)
        } else { 
            warn::had_not_init();
            None
        }
    }
}

pub fn get(k: &str) -> Option<Num> {
    unsafe {
        if let Some(map) = &GLO_VAR_MAP {
            if let Some(var) = map.get(k) {
                Some(Num::Var(var))
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

// pub fn push_var(var: Variable<'static>) {
//     unsafe {
//         GLO_VAR_LIST.push(var)
//     }
// }

// pub fn get_var_by_name(aim:&str) -> Option<Num<'static>> {
//     for i in unsafe {
//         GLO_VAR_LIST.iter()
//     } {
//         if let Name::Str(name) = i.name() {
//             if name == aim {
//                 return Some(Num::Var(i))
//             }
//         }
//     };
//     return None;
// }