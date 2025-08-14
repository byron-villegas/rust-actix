use actix_web::{get, web, HttpResponse};

use crate::clients::pokeapi::dto::pokedex_dto::PokedexDto;
use crate::controllers::pokemon_controller;


#[utoipa::path(
    get,
    path = "/pokedex",
    tag = "Pokemon",
    summary = "Find all pokemon of the pokedex",
    description = "Endpoint to retrieve all pokemons in the pokedex",
    operation_id = "getPokedex",
    responses(
        (status = 200, description = "return pokedex successfully", body = PokedexDto)
    )
)]
#[get("/pokedex")]
async fn get_pokedex_handler() -> HttpResponse  {
    return pokemon_controller::get_pokedex_cr().await;
}


pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(web::scope("/pokemon")
            .service(get_pokedex_handler));
}