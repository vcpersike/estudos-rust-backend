use actix_web::{web, HttpResponse, Responder, http::StatusCode};
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
    println!("Recebendo pedido de busca: {}", body.query);
    match MyService::search_products(&body.query).await {
        Ok(products) => {
            println!("Produtos encontrados: {}", products.len());
            HttpResponse::Ok().json(products)
        },
        Err(e) => {
            println!("Erro durante a busca: {}", e);
            HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR)
        },
    }
}
