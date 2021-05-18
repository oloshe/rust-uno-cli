use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct Player {
    id: String,
    name: String,
}

impl Player {
    pub fn new(id: &str, name: &str) -> Player {
        Player{ id: id.to_string(), name: name.to_string() }
    }
    pub fn get_id(&self) -> &String {
        &self.id
    }
    
    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn set_id(&mut self, id: &str) {
        self.id = id.to_string()
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string()
    }

    pub fn reset(&mut self, id: &str, name: &str) {
        self.set_id(id);
        self.set_name(name);
    }
}