#[derive(Debug)]
pub struct Player<'a> {
    id: &'a str,
    name: &'a str,
}

impl Player<'_> {
    pub fn new<'a>(id: &'a str, name: &'a str) -> Player<'a> {
        Player{ id, name }
    }
    pub fn get_id(&self) -> &str {
        self.id
    }
    
    pub fn get_name(&self) -> &str {
        self.name
    }
}