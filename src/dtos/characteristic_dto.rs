use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, ToSchema)]
pub struct Characteristic {
    titulo: String,
    valor: String
}