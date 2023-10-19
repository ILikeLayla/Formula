use super::static_modifier::scope_name;

pub struct Scope<'a> {
    name: &'a str,
    parent_scopes: Vec<&'a str>,
}

impl<'a> Scope<'a> {
    pub fn new(name: &'a str, parent_scopes: Vec<&'a str>) -> Result<Self, &'a str> {
        
        Ok(Self {
            name, parent_scopes
        })
    }
}