#[cfg(test)]
mod tests {
    use std::env::set_var;

    use actix_web::{test, web, App};
    use utoipa::OpenApi;
    use utoipa_swagger_ui::SwaggerUi;

    use crate::config::config::Configuration;
    use crate::config::config::config;
    use crate::doc::api_doc::ApiDoc;

    #[actix_web::test]
    async fn test_config() {
        set_var("HOST", "127.0.0.1");
        set_var("PORT", "8000");
        set_var("RUST_LOG", "actix_web=info");

        let configuration = Configuration::init().await;

        let app = test::init_service(App::new().service(web::scope(&configuration.server.path)
        .configure(config))
        .service(SwaggerUi::new("/swagger-ui/{_:.*}")
        .url("/api-doc/openapi.json", ApiDoc::openapi())))
        .await;

        let req = test::TestRequest::get().uri("/api/health").to_request();
        let resp = test::call_service(&app, req).await;

        assert!(resp.response().status().is_success());
    }
}
