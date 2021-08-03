use actix_web::{get, post, put, web, Responder};
use deadpool_postgres::{Config, Manager, ManagerConfig, Pool, RecyclingMethod};
use tokio_postgres::{NoTls, Error, Client};
use serde::Deserialize;

pub struct ShopServiceHandler {}

impl ShopServiceHandler {
    pub fn config(cfg: &mut web::ServiceConfig) {
        cfg.service(
            web::scope("/shop")
                .service(get)
                .service(update)
                .service(insert),
        );
    }
}

fn get_conn_pool() -> Pool {
    let mut cfg = Config::new();
    cfg.dbname = Some("obunch".to_string());
    cfg.user = Some("gaurav".to_string());
    cfg.host = Some("127.0.0.1".to_string());
    cfg.password = Some("test123".to_string());
    cfg.manager = Some(ManagerConfig { recycling_method: RecyclingMethod::Fast });

    cfg.create_pool(NoTls).unwrap()
}

#[derive(Deserialize, Debug)]
struct Shop {
    pub id: i64,
    name: String,
    address: String,
}

#[get("/{id}")]
async fn get(path: web::Path<i64>) -> impl Responder {
    let pool = get_conn_pool();
    let mut client = pool.get().await.unwrap(); 
    let statement = client.prepare("SELECT * FROM shop WHERE id=$1").await.unwrap();
    let (id) = path.into_inner();
    let row = client.query(&statement, &[&id]).await.unwrap();
    //let row = format!("SELECT * FROM shop WHERE id={};", id).await.unwrap();
    println!("{:?}", row);
    ""
}

#[post("/update/{id}")]
async fn update(path: web::Path<i64>, shop: web::Json<Shop>) -> impl Responder {
    let pool = get_conn_pool();
    let mut client = pool.get().await.unwrap(); 
    let statement = client.prepare("SELECT * FROM shop WHERE id=$1").await.unwrap();
    let (id) = path.into_inner();
    let row = client.query(&statement, &[&id]).await.unwrap();
    let query = format!(
        "UPDATE shop SET name=\"{}\", address=\"{}\" WHERE id={};",
        shop.name, shop.address, id
    );
    query
}

#[put("/insert")]
async fn insert(shop: web::Json<Shop>) -> impl Responder {
    let pool = get_conn_pool();
    let mut client = pool.get().await.unwrap(); 
    let statement = client.prepare("INSERT INTO shop (name, address) VALUES ($1, $2)").await.unwrap();
    let row = client.query(&statement, &[&shop.name, &shop.address]).await.unwrap();
    let query = format!(
        "INSERT INTO shop (name, address) VALUES (\"{}\", \"{}\");",
        shop.name, shop.address
    );
    query
}
