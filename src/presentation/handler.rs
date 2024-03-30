use actix_web::{HttpResponse, Responder};
use crate::application::service::MyService;
use crate::domain::entity::MyEntity;

pub async fn my_handler() -> impl Responder {
    let service = MyService::new();
    let entity = MyEntity {
        id: 1,
        name: "Entidade Exemplo".to_string(),
    };
    service.perform_action(&entity);
    HttpResponse::Ok().body(entity.name().clone())
}
