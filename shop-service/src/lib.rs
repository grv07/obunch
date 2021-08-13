use actix_web::{get, post, put, web, HttpResponse, Result};
use deadpool_postgres::Pool;
use serde::{Deserialize, Serialize};
use tokio_pg_mapper::FromTokioPostgresRow;
use tokio_pg_mapper_derive::PostgresMapper;
use tokio_postgres::row::Row;
use tokio_postgres::types::ToSql;
use tokio_postgres::Error;

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
                .service(list)
                .service(get)
                .service(update)
                .service(insert),
        );
    }
}

async fn execute_query_one(
    pool: &Pool,
    raw_query: &str,
    param: &[&(dyn ToSql + Sync)],
) -> Result<Row, Error> {
    let client = pool.get().await.unwrap();
    let statement = client.prepare(raw_query).await.unwrap();
    client.query_one(&statement, param).await
}

async fn execute_query(
    pool: &Pool,
    raw_query: &str,
    param: &[&(dyn ToSql + Sync)],
) -> Result<Vec<Row>, Error> {
    let client = pool.get().await.unwrap();
    let statement = client.prepare(raw_query).await.unwrap();
    client.query(&statement, param).await
}

#[get("/list")]
async fn list(pool: web::Data<Pool>) -> Result<HttpResponse> {
    let rows = execute_query(pool.get_ref(), "SELECT * FROM shop", &[])
        .await
        .unwrap();
    let mut shops = Vec::new();
    for row in rows {
        shops.push(Shop::from_row(row).unwrap());
    }
    Ok(HttpResponse::Ok().json(shops))
}

#[get("/{id}")]
async fn get(pool: web::Data<Pool>, path: web::Path<i32>) -> Result<HttpResponse> {
    let id = path.into_inner();
    let row = execute_query_one(pool.get_ref(), "SELECT * FROM shop WHERE id=$1", &[&id])
        .await
        .unwrap();
    let shop = Shop::from_row(row).unwrap();
    Ok(HttpResponse::Ok().json(shop))
}

#[post("/update/{id}")]
async fn update(
    pool: web::Data<Pool>,
    path: web::Path<i32>,
    shop: web::Json<Shop>,
) -> Result<HttpResponse> {
    let id = path.into_inner();
    let query = format!(
        "{} {}",
        "UPDATE shop SET name=$1, address=$2 WHERE id=$3 RETURNING",
        &Shop::sql_fields()
    );
    let row = execute_query_one(pool.get_ref(), &query, &[&shop.name, &shop.address, &id])
        .await
        .unwrap();
    let shop = Shop::from_row(row).unwrap();
    Ok(HttpResponse::Ok().json(shop))
}

#[put("/insert")]
async fn insert(pool: web::Data<Pool>, shop: web::Json<Shop>) -> Result<HttpResponse> {
    let query = format!(
        "{} {}",
        "INSERT INTO shop (name, address) VALUES ($1, $2) RETURNING",
        &Shop::sql_fields()
    );
    let row = execute_query_one(pool.get_ref(), &query, &[&shop.name, &shop.address])
        .await
        .unwrap();

    let shop = Shop::from_row(row).unwrap();
    Ok(HttpResponse::Ok().json(shop))
}
