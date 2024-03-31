use actix_web::{web, HttpResponse, Responder};
use crate::application::MyService;
use crate::domain::{MyEntity, SearchQuery};


pub async fn my_handler() -> impl Responder {
    let service = MyService::new();
    let entity = MyEntity {
        id: 1,
        name: "Entidade Exemplo".to_string(),
    };
    service.perform_action(&entity);
    HttpResponse::Ok().body(entity.name().clone())
}

pub async fn search_handler(body: web::Json<SearchQuery>) -> impl Responder {
    match MyService::search_products(&body.query).await {
        Ok(products) => HttpResponse::Ok().json(products),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
