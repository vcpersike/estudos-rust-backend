use actix_web::{web, App, HttpServer};
use crate::presentation::handler::{my_handler, search_handler};



pub async fn run() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/hello", web::get().to(my_handler))
            .route("/search", web::post().to(search_handler))
            .route("/holla", web::get().to(my_handler))
            // Você pode adicionar mais rotas aqui conforme necessário
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
