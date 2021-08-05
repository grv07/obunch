use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use env_logger::Env;
use shop_service::ShopServiceHandler;
//use menu_service::MenuServiceHandler;
//use order_service::OrderServiceHandler;

fn config(cfg: &mut web::ServiceConfig) {
    ShopServiceHandler::config(cfg);
    //OrderServiceHandler::config(cfg);
    //MenuServiceHandler::config(cfg);
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let app = || {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .configure(config)
    };
    HttpServer::new(app).bind("127.0.0.1:8080")?.run().await
}
