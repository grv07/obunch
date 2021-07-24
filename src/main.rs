use actix_web::{get, delete, post, web, App, HttpResponse, HttpServer, Responder};

// This can be inside an module/crate or lib.
fn config(cfg: &mut web::ServiceConfig) {
    //let auth_scope = web::scope("/user").service(create_user);
    //let order_scope = web::scope("/user").service(create_user);
    //let menu_scope = web::scope("/user").service(create_user);
    //let shop_scope = web::scope("/user").service(create_user);
    //cfg.service(auth_scope);
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app = || App::new(); //.configure(config);
    HttpServer::new(app).bind("127.0.0.1:8080")?.run().await
}

