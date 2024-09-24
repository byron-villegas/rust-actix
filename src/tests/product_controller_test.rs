#[cfg(test)]
mod tests {
    use std::fs;

    use actix_web::{test, web, App};

    use crate::{dtos::product_dto::ProductDto, routes::product_route::{get_products_handler, post_products_handler}};

    #[actix_web::test]
    async fn test_get_products_endpoint() {
        let app = test::init_service(App::new().service(web::scope("/api").service(get_products_handler))).await;
        let req = test::TestRequest::get().uri("/api/products").to_request();
        let resp = test::call_service(&app, req).await;

        assert!(resp.response().status().is_success());
    }

    #[actix_web::test]
    async fn test_post_products_endpoint() {
        let file = fs::read_to_string("src/data/productos.json").unwrap();

        let products: Vec<ProductDto> = serde_json::from_str(file.as_str()).unwrap();

        let product = &products[0];

        let app = test::init_service(App::new().service(web::scope("/api").service(post_products_handler))).await;
        let req = test::TestRequest::post().uri("/api/products").set_json(product).to_request();
        let resp = test::call_service(&app, req).await;

        assert!(resp.response().status().is_success());
    }
}
