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
#[pg_mapper(table = "orders")]
struct InsertOrder {
    name: String,
    address: String,
    items: Vec<Uuid>,
}

#[derive(Serialize, Deserialize, Debug, PostgresMapper)]
#[pg_mapper(table = "orders")]
struct Order {
    pub id: Uuid,
    name: String,
    address: String,
    items: Vec<Uuid>,
}

pub struct OrderServiceHandler;

impl OrderServiceHandler {
    pub fn config(cfg: &mut web::ServiceConfig) {
        cfg.service(
            web::scope("/order")
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
    let rows = execute_query(pool.get_ref(), "SELECT * FROM orders", &[])
        .await
        .unwrap();
    let mut orders = Vec::new();
    for row in rows {
        orders.push(Order::from_row(row).unwrap());
    }
    Ok(HttpResponse::Ok().json(orders))
}

#[get("/{id}")]
async fn get(pool: web::Data<Pool>, path: web::Path<Uuid>) -> Result<HttpResponse> {
    let id = path.into_inner();
    let row = execute_query_one(pool.get_ref(), "SELECT * FROM orders WHERE id=$1", &[&id])
        .await
        .unwrap();
    let order = Order::from_row(row).unwrap();
    Ok(HttpResponse::Ok().json(order))
}

#[post("/update/{id}")]
async fn update(
    pool: web::Data<Pool>,
    path: web::Path<Uuid>,
    order: web::Json<InsertOrder>,
) -> Result<HttpResponse> {
    let id = path.into_inner();
    let query = format!(
        "{} {}",
        "UPDATE orders SET name=$1, address=$2 WHERE id=$3 RETURNING",
        &Order::sql_fields()
    );
    let row = execute_query_one(pool.get_ref(), &query, &[&order.name, &order.address, &id])
        .await
        .unwrap();
    let order = Order::from_row(row).unwrap();
    Ok(HttpResponse::Ok().json(order))
}

#[put("/insert")]
async fn insert(pool: web::Data<Pool>, order: web::Json<InsertOrder>) -> Result<HttpResponse> {
    let uuid = Uuid::new_v4();
    let query = format!(
        "{} {}",
        "INSERT INTO orders (id, name, address, items) VALUES ($1, $2, $3, $4) RETURNING",
        &Order::sql_fields()
    );
    let row = execute_query_one(pool.get_ref(), &query, &[&uuid, &order.name, &order.address, &order.items])
        .await
        .unwrap();
    
    let order = Order::from_row(row).unwrap();
    Ok(HttpResponse::Ok().json(order))
}

