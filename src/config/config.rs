use std::env;
use std::fs;

use actix_web::web;

use crate::routes::health_route::health_checker_handler;
use crate::routes::product_route::get_product_by_sku_handler;
use crate::routes::product_route::{get_products_handler, post_products_handler};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(health_checker_handler)
    .service(get_products_handler)
    .service(get_product_by_sku_handler)
    .service(post_products_handler);
}

pub struct Server {
    pub host: String,
    pub path: String,
    pub port: u16
}

pub struct Configuration {
    pub server: Server
}

impl Configuration {
    pub async fn init() -> Self {

        let mut host = "127.0.0.1".to_string();

        if env::var_os("RUST_LOG").is_none() {
            env::set_var("RUST_LOG", "actix_web=info");
        }

        if env::var("HOST").is_ok() {
            host = env::var("HOST").unwrap();
        }

        let configuration = Configuration {
            server: Server {
                host: host,
                path: "/api".to_string(),
                port: 8000
            }
        };

        let log_level = env::var_os("RUST_LOG").unwrap();

        let mut banner = fs::read_to_string("src/config/banner.txt").unwrap();

        banner = banner.replace("server.path", &configuration.server.path);
        banner = banner.replace("server.port", configuration.server.port.to_string().as_str());
        banner = banner.replace("log.level", &log_level.to_str().unwrap().replace("actix_web=", ""));

        println!("{banner}");

        return configuration;
    }
}