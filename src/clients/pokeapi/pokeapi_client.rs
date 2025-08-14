use reqwest::Client;

use crate::clients::pokeapi::dto::pokedex_dto::PokedexDto;

pub async fn get_pokedex_by_id(id: String) -> Result<PokedexDto, reqwest::Error> {
    let url = "https://pokeapi.co/api/v2/pokedex/".to_string() + &id;

    let client = Client::new();

    let response = client
        .get(url)
        .send()
        .await?;

    let dto = response
        .json::<PokedexDto>()
        .await?;

    Ok(dto)
}