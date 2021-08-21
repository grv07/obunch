use actix_web::{get, post, put, web, HttpResponse, Result};
use deadpool_postgres::Pool;
use serde::{Deserialize, Serialize};
use tokio_pg_mapper::FromTokioPostgresRow;
use tokio_pg_mapper_derive::PostgresMapper;
use tokio_postgres::row::Row;
use tokio_postgres::types::{ToSql};
use tokio_postgres::Error;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, PostgresMapper)]
#[pg_mapper(table = "item")]
struct InsertItem {
    name: String,
    is_avail: bool,
}

#[derive(Serialize, Deserialize, Debug, PostgresMapper)]
#[pg_mapper(table = "item")]
struct Item {
    pub id: Uuid,
    name: String,
    is_avail: bool,
}

pub struct ItemServiceHandler;

impl ItemServiceHandler {
    pub fn config(cfg: &mut web::ServiceConfig) {
        cfg.service(
            web::scope("/item")
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
    let rows = execute_query(pool.get_ref(), "SELECT * FROM item", &[])
        .await
        .unwrap();
    let mut items = Vec::new();
    for row in rows {
        items.push(Item::from_row(row).unwrap());
    }
    Ok(HttpResponse::Ok().json(items))
}

#[get("/{id}")]
async fn get(pool: web::Data<Pool>, path: web::Path<Uuid>) -> Result<HttpResponse> {
    let id = path.into_inner();
    let row = execute_query_one(pool.get_ref(), "SELECT * FROM item WHERE id=$1", &[&id])
        .await
        .unwrap();
    let item = Item::from_row(row).unwrap();
    Ok(HttpResponse::Ok().json(item))
}

#[post("/update/{id}")]
async fn update(
    pool: web::Data<Pool>,
    path: web::Path<Uuid>,
    item: web::Json<InsertItem>,
) -> Result<HttpResponse> {
    let id = path.into_inner().to_string();
    let query = format!(
        "{} {}",
        "UPDATE item SET name=$1, is_avail=$2 WHERE id=$3 RETURNING",
        &Item::sql_fields()
    );
    let row = execute_query_one(pool.get_ref(), &query, &[&item.name, &item.is_avail, &id])
        .await
        .unwrap();
    let item = Item::from_row(row).unwrap();
    Ok(HttpResponse::Ok().json(item))
}

#[put("/insert")]
async fn insert(pool: web::Data<Pool>, item: web::Json<InsertItem>) -> Result<HttpResponse> {
    let uuid = Uuid::new_v4();
    let query = format!(
        "{} {}",
        "INSERT INTO item (name, is_avail) VALUES ($1, $2, $3) RETURNING",
        &Item::sql_fields()
    );
    let row = execute_query_one(pool.get_ref(), &query, &[&uuid, &item.name, &item.is_avail])
        .await
        .unwrap();
    
    let item = Item::from_row(row).unwrap();
    Ok(HttpResponse::Ok().json(item))
}

