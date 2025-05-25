use actix_web::{get, HttpResponse};

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
#[get("/health")]
async fn health_checker_handler() -> HttpResponse  {
    return get_health_check_info();
}