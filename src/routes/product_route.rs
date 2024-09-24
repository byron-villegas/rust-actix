use actix_web::{get, post, web, HttpResponse};

use crate::{controllers::product_controller::{post_products, get_products_cr}, dtos::product_dto::ProductDto};

#[get("/products")]
async fn get_products_handler() -> HttpResponse  {
    return get_products_cr();
}

#[post("/products")]
async fn post_products_handler(product_dto: web::Json<ProductDto>) -> HttpResponse  {
    return post_products(product_dto);
}