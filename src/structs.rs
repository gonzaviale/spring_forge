use std::collections::HashMap;

pub struct Entity {
    name: String,
    attributes: HashMap<String, String>,
}

impl Entity {
    pub fn new(name: String, attributes: HashMap<String, String>) -> Entity {
        Entity { name, attributes }
    }
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
    pub fn _get_attributes(&self) -> HashMap<String, String> {
        self.attributes.clone()
    }
    pub fn _get_attribute(&self, name: &str) -> String {
        self.attributes.get(name).unwrap().clone()
    }
    pub fn add_attribute(&mut self, name: String, value: String) {
        self.attributes.insert(name, value);
    }
    pub fn _remove_attribute(&mut self, name: &str) {
        self.attributes.remove(name);
    }
    pub fn _update_attribute(&mut self, name: &str, value: String) {
        self.attributes.insert(name.to_string(), value);
    }
    pub fn _has_attribute(&self, name: &str) -> bool {
        self.attributes.contains_key(name)
    } 
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
}
