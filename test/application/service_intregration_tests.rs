// Em tests/service_integration_tests.rs

use mockito::{mock, server_url};

#[tokio::test]
async fn test_search_products_integration() {
    let _m = mock("GET", "/sites/MLB/search")
        .with_status(200)
        .with_body(r#"{"results":[{"title":"Produto Teste"}]}"#)
        .create();

    std::env::set_var("BASE_URL", &mockito::server_url());  // Definindo a variável de ambiente para o teste

    let service = MyService::new();
    let products = service.search_products("Produto Teste").await.unwrap();

    assert_eq!(products.len(), 1);
    assert_eq!(products[0].title, "Produto Teste");

    // Lembre-se de limpar a variável de ambiente após o teste, se necessário.
}
