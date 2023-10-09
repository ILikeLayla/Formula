use super::warn;
use super::NAME;

pub fn name_insert(name: &str) -> Result<(), &str> {
    if  unsafe {
        NAME.contains(&name.to_string())
    } {
        warn::name_used();
        Err("SN-1")
    } else {
        unsafe {
            NAME.push(name.to_string())
        }
        Ok(())
    }
}

pub fn delete_name(name: &str) {
    unsafe {
        NAME.retain(|x| x != name)
    }
}