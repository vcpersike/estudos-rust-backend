use actix_web::{web, App, HttpServer};

pub async fn run() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(
                // Defina suas rotas e handlers aqui
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}