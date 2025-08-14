use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::clients::pokeapi::dto::pokemon_specie_dto::PokemonSpecieDto;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct PokemonEntryDto {
    pub entry_number: u32,
    pub pokemon_species: PokemonSpecieDto
}