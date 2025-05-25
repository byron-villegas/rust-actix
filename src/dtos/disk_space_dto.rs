use serde::Serialize;
use utoipa::ToSchema;

#[allow(non_snake_case)]
#[derive(Serialize, ToSchema)]
pub struct DiskSpaceDto {
    pub status: String,
    pub total: i64,
    pub free: i64,
    pub used: i64,
}