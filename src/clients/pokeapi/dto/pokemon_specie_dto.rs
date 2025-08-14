use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct PokemonSpecieDto {
    pub name: String,
    pub url: String
}