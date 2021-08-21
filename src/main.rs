use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use env_logger::Env;
use shop_service::ShopServiceHandler;
//use menu_service::MenuServiceHandler;
use order_service::OrderServiceHandler;

fn config(cfg: &mut web::ServiceConfig) {
    ShopServiceHandler::config(cfg);
    OrderServiceHandler::config(cfg);
    //MenuServiceHandler::config(cfg);
}

use deadpool_postgres::{Config, ManagerConfig, Pool, RecyclingMethod};
use tokio_postgres::NoTls;

async fn get_conn_pool() -> Pool {
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let pool = get_conn_pool().await;
    let app = move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .configure(config)
    };
    HttpServer::new(app).bind("127.0.0.1:8080")?.run().await
}
