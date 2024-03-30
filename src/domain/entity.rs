pub struct MyEntity {
    pub id: i32,
    pub name: String,
}


impl MyEntity {
    pub fn new(id: i32, name: String) -> Self {
        Self { id, name }
    }

    // Getter para o nome
    pub fn name(&self) -> &String {
        &self.name
    }
}
