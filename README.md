# Rust Actix

Proyecto base para aplicaciones Actix con ejemplos de configuración, testing y buenas prácticas.

## Tabla de Contenidos

- [Instalación](#instalación)
- [Ejecutar Aplicación](#ejecutar-aplicación)
- [Testing](#testing)
- [Tests de Rendimiento](#tests-de-rendimiento)
- [Tests de Aceptación](#tests-de-aceptación)
- [Links de Referencia](#links-de-referencia)

## Instalación
### Instalar Rust
Para instalar rust debemos bajarlo e instalarlo de la siguiente pagina https://www.rust-lang.org/tools/install

### Actualizar Versión Rust
Para actualiza la versión de rust que ya poseemos instalada debemos ejecutar el siguiente comand

```shell
rustup update stable
```

### Instalar Paquetes
Para instalar los paquetes debemos ejecutar el siguiente comando

```shell
cargo build
```

### Borrar Build
Para borrar el build del proyecto debemos ejecutar el siguiente comando

```shell
cargo clean
```

## Ejecutar Aplicación
Se debe ejecutar el siguiente comando

```shell
cargo run --bin main
```

## Testing
### Instalar Paquete para Cobertura
Se debe ejecutar el siguiente comando

```shell
cargo install cargo-llvm-cov
```

### Ejecutar
Se debe ejecutar el siguiente comando

```shell
cargo test
```

### Ejecutar con Cobertura
Se debe ejecutar el siguiente comando

```shell
cargo llvm-cov --ignore-filename-regex "_dto.rs|main.rs|_steps.rs"
```
**NOTA:** el parametro **--ignore-filename-regex** es para indicar que tipos de archivo omitir en este caso los dto y el main ya que no son necesario cubrirlos

### Generar Reporte Cobertura Formato HTML
Se debe ejecutar el siguiente comando

```shell
cargo llvm-cov --ignore-filename-regex "_dto.rs|main.rs" --html
```
**NOTA:** el parametro **--ignore-filename-regex** es para indicar que tipos de archivo omitir en este caso los dto y el main ya que no son necesario cubrirlos

## Tests de Aceptación
### Configuración
Se debe crear un archivo de inicio en este caso cree el archivo **acceptance-test/main.rs** con el siguiente contenido

```rust
mod steps;

use std::{env, fs};

use cucumber::{writer, World as _};

#[derive(Debug, cucumber::World)]
pub struct World {
    pub host: String,
    pub endpoint: String,
    pub response: Option<reqwest::Response>,
}

impl Default for World {
    fn default() -> Self {
        Self {
            host: std::env::var("API_HOST").unwrap_or_else(|_| "http://localhost:8000/api".to_string()),
            endpoint: String::new(),
            response: None,
        }
    }
}

#[tokio::main]
async fn main() {
    let out_dir = env::current_dir().unwrap();
    let out_dir = out_dir.to_str().unwrap().split("acceptance-test").next().unwrap();
        
    let xml_file = fs::File::create(format!("{}/cucumber-report.xml", out_dir)).unwrap();
    let json_file = fs::File::create(format!("{}/cucumber-report.json", out_dir)).unwrap();

    World::cucumber()
        .with_writer(writer::JUnit::new(xml_file, 0))
        .with_writer(writer::Json::new(json_file))
        .run("acceptance-test/features")
        .await;
}
```

La estructura World es de contexto para cucumber la utilizaremos para definir parametros estaticos que deseemos re utilizar en los steps, tambien tenemos los archivos de reporte que se generaran y finalmente le indicamos desde que carpeta debe ejecutar los features

### Ejecución
Se debe ejecutar el siguiente comando

```shell
cargo run --bin cucumber
```

Al finalizar generara dos reportes **cucumber-report.xml** y **cucumber-report.json**

## Tests de Rendimiento
### Configuración
Se debe crear un archivo de inicio en este caso cree el archivo **performance-test/goose/main.rs** con el siguiente contenido

```rust
use goose::prelude::*;

async fn loadtest_products(user: &mut GooseUser) -> TransactionResult {
    let _goose_metrics = user.get("products").await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), GooseError> {
    GooseAttack::initialize()?
        .register_scenario(
            scenario!("products").register_transaction(transaction!(loadtest_products))
        )
        .execute()
        .await?;

    Ok(())
}
```

Como podemos ver creamos un test y indicamos el endpoint que deseamos testear, posteriormente registramos el escenario y la transaccion (test)

### Ejecución
Se debe ejecutar el siguiente comando

```shell
cargo run --bin goose -- --host http://localhost:8000/api/ --users 10 --run-time 10s --report-file=goose-report.html --no-reset-metrics
```

Al finalizar generara un reporte **goose-report.html**

## Links de Referencia
A continuación dejo links utilizados para realizar este proyecto

[Build a Simple API with Rust and Actix Web](https://codevoweb.com/build-a-simple-api-with-rust-and-actix-web/)

[Rust 🦀 CRUD Rest API with Docker 🐳](https://dev.to/francescoxx/rust-crud-rest-api-3n45)

[Build a REST API with Rust and MongoDB - Actix web Version](https://dev.to/hackmamba/build-a-rest-api-with-rust-and-mongodb-actix-web-version-ei1)

[Organizar Muchas Rutas Con Rust y Actix Web Con Scopes y ServiceConfig](https://rustyfullstack.com/blog/organizar-muchas-rutas-con-rust-y-actix-web)

[How To Install The cargo-llvm-cov](https://lib.rs/install/cargo-llvm-cov)

[Generate and serve Swagger in Rust](https://medium.com/@nunocarvalhodossantos/generate-and-serve-swagger-in-rust-a7be97aeabfb)

[SeaORM Custom Selects](https://www.sea-ql.org/SeaORM/docs/advanced-query/custom-select/#handling-custom-selects)

[Error Handling in Rust Actix Web](https://dev.to/chaudharypraveen98/error-handling-in-actix-web-4mm)

[Cucumber](https://cucumber-rs.github.io/cucumber/current/introduction.html)

[Goose](https://github.com/tag1consulting/goose)

[Utoipa Swagger](https://crates.io/crates/utoipa-swagger-ui)