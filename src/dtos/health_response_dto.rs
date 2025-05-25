use serde::Serialize;
use utoipa::ToSchema;

use super::disk_space_dto::DiskSpaceDto;
use super::ram_space_dto::RamSpaceDto;

#[allow(non_snake_case)]
#[derive(Serialize, ToSchema)]
pub struct HealthResponseDto {
    pub status: String,
    pub diskSpace: DiskSpaceDto,
    pub ramSpace: RamSpaceDto,
}