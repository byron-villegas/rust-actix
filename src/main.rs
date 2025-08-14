mod clients;
mod config;
mod controllers;
mod dtos;
mod routes;
mod services;
mod doc;
mod repositories;

#[cfg(test)]
mod tests;

use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::config::config::config;
use crate::config::config::Configuration;

use crate::doc::api_doc::ApiDoc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    
    let configuration = Configuration::init().await;

    env_logger::init();

    println!("Server started successfully on http://{0}:{1}{2}", configuration.server.host, configuration.server.port, configuration.server.path);

    HttpServer::new(move || {
        App::new()
            .service(web::scope(&configuration.server.path)
            .configure(config))
            .service(SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-doc/openapi.json", ApiDoc::openapi()))
            .wrap(Logger::default())
    })
    .bind((configuration.server.host, configuration.server.port))?
    .run()
    .await
}
