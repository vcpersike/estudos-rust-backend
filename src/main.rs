use actix_web::{web, App, HttpServer};

mod domain;
mod application;
mod infrastructure;
mod presentation;

use crate::presentation::handler::my_handler;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().route("/", web::get().to(my_handler))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

