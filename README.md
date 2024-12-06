# Instalar paquetes
Se debe ejecutar el siguiente comando

```shell
cargo build
```

# Ejecutar aplicacion
Se debe ejecutar el siguiente comando

```shell
cargo run
```

# Tests Unitarios

### Ejecutarlo
Se debe ejecutar el siguiente comando

```shell
cargo test
```

### Instalar libreria para cobertura
Se debe ejecutar el siguiente comando

```shell
cargo install cargo-llvm-cov
```

### Ejecutarlo con cobertura
Se debe ejecutar el siguiente comando

```shell
cargo llvm-cov --ignore-filename-regex "_dto.rs|main.rs"
```
**NOTA:** el parametro **--ignore-filename-regex** es para indicar que tipos de archivo omitir en este caso los dto y el main ya que no son necesario cubrirlos


### Generar reporte html de cobertura
Se debe ejecutar el siguiente comando

```shell
cargo llvm-cov --ignore-filename-regex "_dto.rs|main.rs" --html
```
**NOTA:** el parametro **--ignore-filename-regex** es para indicar que tipos de archivo omitir en este caso los dto y el main ya que no son necesario cubrirlos

# Links Referenciales
A continuación dejo links utilizados para realizar este proyecto

[Build a Simple API with Rust and Actix Web](https://codevoweb.com/build-a-simple-api-with-rust-and-actix-web/)

[Rust 🦀 CRUD Rest API with Docker 🐳](https://dev.to/francescoxx/rust-crud-rest-api-3n45)

[Build a REST API with Rust and MongoDB - Actix web Version](https://dev.to/hackmamba/build-a-rest-api-with-rust-and-mongodb-actix-web-version-ei1)

[Organizar Muchas Rutas Con Rust y Actix Web Con Scopes y ServiceConfig](https://rustyfullstack.com/blog/organizar-muchas-rutas-con-rust-y-actix-web)

[How To Install The cargo-llvm-cov](https://lib.rs/install/cargo-llvm-cov)

[Generate and serve Swagger in Rust](https://medium.com/@nunocarvalhodossantos/generate-and-serve-swagger-in-rust-a7be97aeabfb)

[SeaORM Custom Selects](https://www.sea-ql.org/SeaORM/docs/advanced-query/custom-select/#handling-custom-selects)

[Error Handling in Rust Actix Web](https://dev.to/chaudharypraveen98/error-handling-in-actix-web-4mm)
