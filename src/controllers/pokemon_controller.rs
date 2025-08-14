use actix_web::{HttpResponse};

use crate::{services::pokemon_service};

pub async fn get_pokedex_cr() -> HttpResponse {
    let pokedex = pokemon_service::get_pokedex().await;

    HttpResponse::Ok().json(pokedex)
}