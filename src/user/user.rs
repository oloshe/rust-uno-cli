pub struct User {
    id: String,
    name: String,
}
impl User {
    pub fn new(id: String, name: String) -> User {
        User { id, name }
    }
}