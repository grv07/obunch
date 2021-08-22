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
#[pg_mapper(table = "menu")]
struct InsertMenu {
    name: String,
    is_avail: bool,
    item_list: Vec<Uuid>,
}

#[derive(Serialize, Deserialize, Debug, PostgresMapper)]
#[pg_mapper(table = "menu")]
struct Menu {
    pub id: Uuid,
    name: String,
    is_avail: bool,
    item_list: Vec<Uuid>,
}

pub struct MenuServiceHandler;

impl MenuServiceHandler {
    pub fn config(cfg: &mut web::ServiceConfig) {
        cfg.service(
            web::scope("/menu")
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
    let rows = execute_query(pool.get_ref(), "SELECT * FROM menu", &[])
        .await
        .unwrap();
    let mut menus = Vec::new();
    for row in rows {
        menus.push(Menu::from_row(row).unwrap());
    }
    Ok(HttpResponse::Ok().json(menus))
}

#[get("/{id}")]
async fn get(pool: web::Data<Pool>, path: web::Path<Uuid>) -> Result<HttpResponse> {
    let id = path.into_inner();
    let row = execute_query_one(pool.get_ref(), "SELECT * FROM menu WHERE id=$1", &[&id])
        .await
        .unwrap();
    let menu = Menu::from_row(row).unwrap();
    Ok(HttpResponse::Ok().json(menu))
}

#[post("/update/{id}")]
async fn update(
    pool: web::Data<Pool>,
    path: web::Path<Uuid>,
    menu: web::Json<InsertMenu>,
) -> Result<HttpResponse> {
    let id = path.into_inner().to_string();
    let query = format!(
        "{} {}",
        "UPDATE menu SET name=$1, is_avail=$2 WHERE id=$3 RETURNING",
        &Menu::sql_fields()
    );
    let row = execute_query_one(pool.get_ref(), &query, &[&menu.name, &menu.is_avail, &id])
        .await
        .unwrap();
    let menu = Menu::from_row(row).unwrap();
    Ok(HttpResponse::Ok().json(menu))
}

#[put("/insert")]
async fn insert(pool: web::Data<Pool>, menu: web::Json<InsertMenu>) -> Result<HttpResponse> {
    let uuid = Uuid::new_v4();
    let query = format!(
        "{} {}",
        "INSERT INTO menu (name, is_avail) VALUES ($1, $2, $3) RETURNING",
        &Menu::sql_fields()
    );
    let row = execute_query_one(pool.get_ref(), &query, &[&uuid, &menu.name, &menu.is_avail])
        .await
        .unwrap();
    
    let menu = Menu::from_row(row).unwrap();
    Ok(HttpResponse::Ok().json(menu))
}

