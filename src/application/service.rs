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
        match reqwest::get(&url).await {
            Ok(response) => {
                match response.json::<ApiResponse>().await {
                    Ok(api_response) => Ok(api_response.results),
                    Err(e) => {
                        println!("Erro ao desserializar a resposta: {:?}", e);
                        Err(e)
                    }
                }
            },
            Err(e) => {
                println!("Erro ao acessar a API: {:?}", e);
                Err(e)
            }
        }
    }

}


