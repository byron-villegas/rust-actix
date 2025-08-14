use crate::{dtos::product_dto::ProductDto, repositories::product_repository};

pub fn get_products() -> Vec<ProductDto> {
    let products: Vec<ProductDto> = product_repository::get_products();

    return products;
}

pub fn get_product_by_sku(sku: String) -> Option<ProductDto> {
    let products = product_repository::get_products();

    let product: Option<ProductDto> = products
        .into_iter()
        .find(|p| p.sku == sku.parse::<i32>().unwrap());

    return product;
}

pub fn add_product(product_dto: ProductDto) {
    product_repository::add_product(product_dto);
}