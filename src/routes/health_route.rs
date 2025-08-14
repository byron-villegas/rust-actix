use actix_web::{get, web, HttpResponse};

use crate::{controllers::health_controller::get_health_check_info};

#[utoipa::path(
    get,
    path = "/health",
    tag = "Health",
    summary = "Health Check Endpoint",
    description = "Endpoint to check the health of the service",
    operation_id = "healthChecker",
    responses(
        (status = 200, description = "Health check successful", body = crate::dtos::health_response_dto::HealthResponseDto)
    )
)]
#[get("")]
async fn health_checker_handler() -> HttpResponse  {
    return get_health_check_info();
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(web::scope("/health")
            .service(health_checker_handler)
        );
}