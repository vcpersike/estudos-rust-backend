use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct MercadoLivreProduct {
    pub id: String,
    pub title: String,
    pub price: f64,
    pub currency_id: String,
    // Adicione mais campos conforme a resposta da API
}


#[derive(Deserialize)]
pub struct ApiResponse {
    pub results: Vec<MercadoLivreProduct>,
    pub paging: Paging,

}

#[derive(Deserialize)]
pub struct Paging {
    pub total: i32,
    pub limit: i32,
}

#[derive(Deserialize, Serialize)]

pub struct SearchQuery {
    pub query: String,
}

