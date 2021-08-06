use actix_web::{get, post, put, web, HttpResponse, Responder, Result};
use deadpool_postgres::{Config, ManagerConfig, Pool, RecyclingMethod};
use serde::{Deserialize, Serialize};
use tokio_pg_mapper::FromTokioPostgresRow;
use tokio_pg_mapper_derive::PostgresMapper;
use tokio_postgres::row::Row;
use tokio_postgres::types::ToSql;
use tokio_postgres::{Error, NoTls};

#[derive(Serialize, Deserialize, Debug, PostgresMapper)]
#[pg_mapper(table = "shop")]
struct Shop {
    pub id: i32,
    name: String,
    address: String,
}

pub struct ShopServiceHandler;

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
    cfg.manager = Some(ManagerConfig {
        recycling_method: RecyclingMethod::Fast,
    });

    cfg.create_pool(NoTls).unwrap()
}

async fn execute_query(raw_query: &str, param: &[&(dyn ToSql + Sync)]) -> Result<Row, Error> {
    let pool = get_conn_pool();
    let client = pool.get().await.unwrap();
    let statement = client.prepare(raw_query).await.unwrap();
    client.query_one(&statement, param).await
}

#[get("/{id}")]
async fn get(path: web::Path<i32>) -> Result<HttpResponse> {
    let id = path.into_inner();
    let row = execute_query("SELECT * FROM shop WHERE id=$1", &[&id])
        .await
        .unwrap();
    let shop = Shop::from_row(row).unwrap();
    Ok(HttpResponse::Ok().json(shop))
}

#[post("/update/{id}")]
async fn update(path: web::Path<i32>, shop: web::Json<Shop>) -> impl Responder {
    let id = path.into_inner();
    let _row = execute_query(
        "UPDATE shop SET name=$1, address=$2 WHERE id=$3;",
        &[&shop.name, &shop.address, &id],
    )
    .await
    .unwrap();
    ""
}

#[put("/insert")]
async fn insert(shop: web::Json<Shop>) -> Result<HttpResponse> {
    let row = execute_query(
        "INSERT INTO shop (name, address) VALUES ($1, $2) RETURNING id, name, address",
        &[&shop.name, &shop.address],
    )
    .await
    .unwrap();
    let shop = Shop::from_row(row).unwrap();
    Ok(HttpResponse::Ok().json(shop))
}
