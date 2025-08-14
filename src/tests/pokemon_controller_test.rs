#[cfg(test)]
mod tests {
    use actix_web::{test, web, App};

    use crate::routes::pokemon_route::get_pokedex_handler;

    #[actix_web::test]
    async fn test_pokemon_endpoint() {
        let app = test::init_service(App::new()
            .service(web::scope("/api")
                .service(web::scope("/pokemon")
                    .service(get_pokedex_handler))))
            .await;
        let req = test::TestRequest::get().uri("/api/pokemon/pokedex").to_request();
        let resp = test::call_service(&app, req).await;

        assert!(resp.response().status().is_success());
    }
}
