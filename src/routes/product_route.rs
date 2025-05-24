use actix_web::{get, post, web, HttpResponse};

use crate::{controllers::product_controller::{get_product_by_sku_cr, get_products_cr, post_products}, dtos::product_dto::ProductDto};

#[get("/products")]
async fn get_products_handler() -> HttpResponse  {
    return get_products_cr();
}

#[get("/products/{sku}")]
async fn get_product_by_sku_handler(sku: web::Path<String>) -> HttpResponse  {
    return get_product_by_sku_cr(sku.into_inner());
}

#[post("/products")]
async fn post_products_handler(product_dto: web::Json<ProductDto>) -> HttpResponse  {
    return post_products(product_dto);
}