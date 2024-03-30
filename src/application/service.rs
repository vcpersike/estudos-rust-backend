use crate::domain::entity::MyEntity;

pub struct MyService;

impl MyService {

    pub fn new() -> Self {
        MyService {}
    }

    pub fn perform_action(&self, entity: &MyEntity) {
        // Lógica para realizar uma ação, por exemplo
        println!("Action performed on entity: {}", entity.name());
    }
}