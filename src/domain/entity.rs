pub struct MyEntity {
    pub id: i32,
    pub name: String,
}


impl MyEntity {

    pub fn name(&self) -> &String {
        &self.name
    }
}
