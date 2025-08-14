use std::env;
use std::fs;

use actix_web::web;
use cargo_metadata::MetadataCommand;

use crate::routes::health_route;
use crate::routes::pokemon_route;
use crate::routes::product_route;

pub fn config(cfg: &mut web::ServiceConfig) {
    health_route::config(cfg);
    product_route::config(cfg);
    pokemon_route::config(cfg);
}

pub struct Server {
    pub host: String,
    pub path: String,
    pub port: u16
}

pub struct Swagger {
    pub title: String,
    pub version: String
}

pub struct Configuration {
    pub server: Server,
    pub swagger: Swagger
}

impl Configuration {
    pub async fn init() -> Self {

        const VERSION: &str = env!("CARGO_PKG_VERSION");

        let mut rust_version = "".to_string();
        let mut actix_version = "".to_string();

        if std::env::var("CARGO_MANIFEST_DIR").is_ok() {
            let path = std::env::var("CARGO_MANIFEST_DIR").unwrap();

            let meta = MetadataCommand::new()
                .manifest_path("./Cargo.toml")
                .current_dir(&path)
                .exec()
                .unwrap();
            
            let root = meta
            .root_package()
            .unwrap();

            rust_version = root.rust_version
            .clone()
            .unwrap()
            .to_string();

            actix_version = root.dependencies
            .iter()
            .find(|dependency| dependency.name == "actix-web")
            .take()
            .unwrap()
            .req
            .to_string();
        }

        let configuration = Configuration {
            server: Server {
                host: env::var("HOST").unwrap(),
                path: "/api".to_string(),
                port: env::var("PORT").unwrap().parse::<u16>().unwrap()
            },
            swagger: Swagger {
                title: "rust-actix".to_string(),
                version: VERSION.to_owned()
            },
        };

        let log_level = env::var("RUST_LOG").unwrap();

        let mut banner = fs::read_to_string("src/config/banner.txt")
        .or_else(|_| fs::read_to_string("config/banner.txt"))
        .unwrap();

        banner = banner.replace("package.name", &configuration.swagger.title);
        banner = banner.replace("package.version", &configuration.swagger.version);
        banner = banner.replace("rust.version", &rust_version);
        banner = banner.replace("actix.version", &actix_version.replace("^", ""));
        banner = banner.replace("server.path", &configuration.server.path);
        banner = banner.replace("server.port", configuration.server.port.to_string().as_str());
        banner = banner.replace("log.level", &log_level.to_string().replace("actix_web=", ""));

        println!("{banner}");

        return configuration;
    }
}