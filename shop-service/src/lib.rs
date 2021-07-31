use actix_web::{get, post, put, web, Responder};
pub struct ShopServiceHandler {}
use serde::Deserialize;

impl ShopServiceHandler {
    pub fn config(cfg: &mut web::ServiceConfig) {
        cfg.service(web::scope("/shop")
            .service(get)
            .service(update)
            .service(insert));
    }
}

#[derive(Deserialize, Debug)]
struct Shop {
    id: i64,
    name: String,
    address: String,
}

#[get("/")]
async fn gett() -> impl Responder {
    "data"
}

#[get("/{id}")]
async fn get(web::Path((id)): web::Path<(i64)>) -> impl Responder {
    let query = format!("SELECT * FROM shop WHERE id={};", id);
    query
}

#[post("/update")]
async fn update(shop: web::Json<Shop>) -> impl Responder {
    let id = 09;
    println!("data");
    let query = format!(
        "UPDATE shop SET name={}, address={} WHERE id={};",
        shop.name, shop.address, id
    );
    query
}

#[put("/insert")]
async fn insert(shop: web::Json<Shop>) -> impl Responder {
    let query = format!(
        "INSERT INTO shop (name, address) VALUES ({}, {});",
        shop.name, shop.address
    );
    query
}
