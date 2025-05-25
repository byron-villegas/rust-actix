use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Rust Actix",
        description = "Proyecto base para aplicaciones Actix con ejemplos de configuración, testing y buenas prácticas.",
        terms_of_service = "https://swagger.io/terms/",
        contact(
            name = "Byron Villegas Moya",
            email = "byronvillegasm@gmail.com"
        ),
        license(
            name = "MIT",
            url = "https://github.com/byron-villegas/rust-actix/blob/main/LICENSE"
        ),
        version = "1.0.0"
    ),
    servers(
        (url = "http://localhost:8000/api", description = "Local Server"),
        (url = "https://rust-actix-luup.onrender.com/api", description = "Production Server")
    ),
    paths(
        crate::routes::health_route::health_checker_handler,
        crate::routes::product_route::get_products_handler,
        crate::routes::product_route::get_product_by_sku_handler,
        crate::routes::product_route::post_products_handler
    ),
    components(
        schemas(
            crate::dtos::health_response_dto::HealthResponseDto,
            crate::dtos::product_dto::ProductDto
        )
    ),
    tags(
        (name = "Health", description = "Health Check Endpoints"),
        (name = "Product", description = "Product Management Endpoints")
    ),
)]
pub struct ApiDoc;
