use actix_web::{get, post, web, HttpResponse};

use crate::{controllers::product_controller::{get_product_by_sku_cr, get_products_cr, post_products}, dtos::product_dto::ProductDto};

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
#[get("")]
async fn get_products_handler() -> HttpResponse  {
    return get_products_cr();
}

#[utoipa::path(
    get,
    path = "/products/{sku}",
    tag = "Product",
    summary = "Find product by SKU",
    description = "Endpoint to retrieve a product by its SKU",
    operation_id = "getProductBySku",
    params(
        ("sku" = String, Path, description = "SKU of the product to retrieve")
    ),
    responses(
        (status = 200, description = "Product retrived", body = ProductDto)
    )
)]
#[get("/{sku}")]
async fn get_product_by_sku_handler(sku: web::Path<String>) -> HttpResponse  {
    return get_product_by_sku_cr(sku.into_inner());
}

#[utoipa::path(
    post,
    path = "/products",
    tag = "Product",
    summary = "Save a new product",
    description = "Endpoint to save a new product",
    operation_id = "saveProduct",
    request_body(
        content = ProductDto,
        description = "Product data to be saved"
    ),
    responses(
        (status = 201, description = "Product created")
    )
)]
#[post("")]
async fn post_products_handler(product_dto: web::Json<ProductDto>) -> HttpResponse  {
    return post_products(product_dto);
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(web::scope("/products")
            .service(get_products_handler)
            .service(get_product_by_sku_handler)
            .service(post_products_handler)
        );
}