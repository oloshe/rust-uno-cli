#[derive(Debug)]
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
}