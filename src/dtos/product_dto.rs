use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::characteristic_dto::Characteristic;

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, ToSchema)]
pub struct ProductDto {
    pub id: i32,
    pub sku: i32,
    pub imagen: String,
    pub nombre: String,
    pub descripcion: String,
    pub caracteristicas: Vec<Characteristic>,
    pub marca: String,
    pub precio: i32
}