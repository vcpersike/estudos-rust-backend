use crate::domain::{ApiResponse, MercadoLivreProduct, MyEntity};
use reqwest;


pub struct MyService;

impl MyService {

    pub fn new() -> Self {
        MyService {}
    }

    pub fn perform_action(&self, entity: &MyEntity) {
        // Lógica para realizar uma ação, por exemplo
        println!("Action performed on entity: {}", entity.name());
    }

    pub async fn search_products(query: &str) -> Result<Vec<MercadoLivreProduct>, reqwest::Error> {
        let url = format!("https://api.mercadolibre.com/sites/MLB/search?q={}", query);
        let response = reqwest::get(url).await?.json::<ApiResponse>().await?;
        Ok(response.results)
    }
}


