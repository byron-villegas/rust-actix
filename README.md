# Rust Actix

Proyecto base para aplicaciones Actix con ejemplos de configuración, testing y buenas prácticas.

## Tabla de Contenidos

- [Instalación](#instalación)
- [Variables de Entorno](#variables-de-entorno)
- [Ejecutar Aplicación](#ejecutar-aplicación)
- [Testing](#testing)
- [Tests de Aceptación](#tests-de-aceptación)
- [Tests de Rendimiento](#tests-de-rendimiento)
- [Swagger](#swagger)
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

## Variables de Entorno
Este proyecto utiliza dotenv por lo que podemos crear el archivo **.env** con las siguientes variables

```text
HOST=127.0.0.1
PORT=8000
RUST_LOG=actix_web=info
```

Estas son las variables que se obtienen del entorno, la variable **HOST** se utiliza para indicarle el host de ejecución de la aplicación, la variable **PORT** se utiliza para indicar el puerto de ejecución de la aplicación y la variable **RUST_LOG** se utiliza para configurar el nivel de log de actix

## Ejecutar Aplicación
Se debe ejecutar el siguiente comando

```shell
cargo run --bin main
```

### Docker
A continuacion dejo los comandos a utilizar para generar la imagen y posteriormente ejecutarla

#### Imagen
Para generar la imagen debemos utilizar el siguiente comando

```shell
docker build -t rust-actix .
```

#### Ejecutar
Para ejecutar la imagen debemos utilizar el siguiente comando

```shell
docker run -p 8000:8000 rust-actix
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
cargo llvm-cov --ignore-filename-regex "_dto.rs|main.rs|api_doc.rs|_steps.rs"
```
**NOTA:** el parametro **--ignore-filename-regex** es para indicar que tipos de archivo omitir en este caso los dto y el main ya que no son necesario cubrirlos

### Generar Reporte Cobertura Formato HTML
Se debe ejecutar el siguiente comando

```shell
cargo llvm-cov --ignore-filename-regex "_dto.rs|main.rs|api_doc.rs|_steps.rs" --html
```
**NOTA:** el parametro **--ignore-filename-regex** es para indicar que tipos de archivo omitir en este caso los dto y el main ya que no son necesario cubrirlos

Al terminar de ejecutarse generara un reporte de cobertura en la ruta **/target/llvm-cov/html** donde se encuentra el archivo **index.html**

## Tests de Aceptación
### Configuración
Se debe crear un archivo de inicio en este caso cree el archivo **acceptance-test/main.rs** con el siguiente contenido

```rust
mod steps;

use std::{env, fs};

use cucumber::{writer::{self}, World as _, WriterExt};

#[derive(Debug, cucumber::World)]
pub struct World {
    pub host: String,
    pub endpoint: String,
    pub response: Option<reqwest::Response>,
}

impl Default for World {
    fn default() -> Self {
        Self {
            host: env::var("API_HOST").unwrap_or_else(|_| "http://localhost:8000/api".to_string()),
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
    .with_writer(
        writer::Basic::stdout() // And output to STDOUT.
            .summarized()       // Simultaneously, add execution summary.
            .tee::<World, _>(writer::JUnit::for_tee(xml_file, 0))
            .tee::<World, _>(writer::Json::for_tee(json_file)) // Then, output to XML file.
            .normalized()       // First, normalize events order.
    )
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

## Swagger
### Documentar Endpoints
Para documentar los endpoints debemos hacerlo de forma manual mediante anotaciones especiales

#### General
La documentacion general se realiza creando una estructura especial llamada **ApiDoc** a continuacion el detalle

```rust
#[derive(OpenApi)]
#[openapi(
    info(
        title = "Rust Actix",
        description = "Proyecto base para aplicaciones Actix con ejemplos de configuración, testing y buenas prácticas.",
        terms_of_service = "https://swagger.io/terms/",
        contact(
            name = "Byron Villegas Moya",
            email = "byronvillegasm@gmail.com"
        ),
        license(
            name = "MIT",
            url = "https://github.com/byron-villegas/rust-actix/blob/main/LICENSE"
        ),
        version = "1.0.0"
    ),
    servers(
        (url = "http://localhost:8000/api", description = "Local Server"),
        (url = "https://rust-actix-luup.onrender.com/api", description = "Production Server")
    ),
    paths(
        crate::routes::health_route::health_checker_handler,
        crate::routes::product_route::get_products_handler,
        crate::routes::product_route::get_product_by_sku_handler,
        crate::routes::product_route::post_products_handler
    ),
    components(
        schemas(
            crate::dtos::health_response_dto::HealthResponseDto,
            crate::dtos::product_dto::ProductDto
        )
    ),
    tags(
        (name = "Health", description = "Health Check Endpoints"),
        (name = "Product", description = "Product Management Endpoints")
    ),
)]
pub struct ApiDoc;
```

Como podemos ver definimos la informacion del proyecto, servidores, paths, componentes, tags

#### Rutas
Para documentar los endpoints debemos colocar una anotacion especial en la ruta

Ejemplo
product_route

```rust
#[utoipa::path(
    get,
    path = "/products",
    tag = "Product",
    summary = "Get all products",
    description = "Endpoint to retrieve all products",
    operation_id = "getProducts",
    responses(
        (status = 200, description = "return product successfully", body = [ProductDto])
    )
)]
#[get("/products")]
async fn get_products_handler() -> HttpResponse  {
    return get_products_cr();
}
```

Como podemos ver documentamos el metodo, url, tag, summary, etc del endpoint

#### DTO
Para documentar los dto debemos colocar una anotacion especial en el dto

Ejemplo
ProductDto

```rust
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, ToSchema)]
pub struct ProductDto {
    pub id: i32,
    pub sku: i32,
    pub imagen: String,
    pub nombre: String,
    pub descripcion: String,
    pub caracteristicas: Vec<Characteristic>,
    pub marca: String,
    pub precio: i32
}
```

Tenemos que usar la funcion **ToSchema** para que genere el esquema automaticamente

### Configurar Swagger UI
Para configurar Swagger UI simplemente agregamos el siguiente codigo al archivo **main.rs**

```rust
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
```

Como podemos ver definimos el servicio **SwagerUi** con su ruta **swagger-ui** y la ruta del documento open api autogenerado **/api-doc/openapi.json**

Cuando ejecutemos a la aplicacion debemos entrar a la pagina **/swagger-ui/**

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