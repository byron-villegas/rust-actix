use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::clients::pokeapi::dto::pokemon_entry_dto::PokemonEntryDto;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct PokedexDto {
    pub id: u32,
    pub name: String,
    pub pokemon_entries: Vec<PokemonEntryDto>
}