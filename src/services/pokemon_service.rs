use crate::clients::pokeapi::{self, dto::pokedex_dto::PokedexDto};

pub async fn get_pokedex() -> PokedexDto {
    let pokedex: PokedexDto = pokeapi::pokeapi_client::get_pokedex_by_id("1".to_string())
        .await
        .expect("Failed to fetch pokedex");

    return pokedex;
}