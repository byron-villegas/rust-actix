use actix_web::{http::StatusCode, web, HttpResponse};

use crate::{dtos::product_dto::ProductDto, services::product_service::{add_product, get_products}};

pub fn get_products_cr() -> HttpResponse {
    let products = get_products();

    HttpResponse::Ok().json(products)
}

pub fn post_products(product_dto: web::Json<ProductDto>) -> HttpResponse {
    
    add_product(product_dto.0);

    HttpResponse::new (StatusCode::CREATED)
}