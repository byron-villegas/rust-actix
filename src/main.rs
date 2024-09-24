mod config;
mod controllers;
mod dtos;
mod routes;
mod services;

#[cfg(test)]
mod tests;

use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};

use crate::config::config::config;
use crate::config::config::Configuration;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    
    let configuration = Configuration::init().await;

    env_logger::init();

    println!("Server started successfully on http://{0}:{1}{2}", configuration.server.host, configuration.server.port, configuration.server.path);

    HttpServer::new(move || {
        App::new()
            .service(web::scope(&configuration.server.path)
            .configure(config))
            .wrap(Logger::default())
    })
    .bind((configuration.server.host, configuration.server.port))?
    .run()
    .await
}
