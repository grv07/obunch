use actix_web::{ web, App, HttpServer};
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
    let app = || App::new().configure(config);
    HttpServer::new(app).bind("127.0.0.1:8080")?.run().await
}

