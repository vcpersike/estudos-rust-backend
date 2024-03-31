
mod domain;
mod application;
mod infrastructure;
mod presentation;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    infrastructure::run().await
}

