mod errors;
mod models;

use models::{User};
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello")
}

#[get("/create")]
async fn createuser(user: web::Json<User>) -> impl Responder {
    format!("data: {:?}\n", user.name)
}

#[post("/login")]
async fn login() -> String {
    "data\n".to_string()
}

// This can be inside an module/crate or lib.
fn config(cfg: &mut web::ServiceConfig) {
    let authscope = web::scope("/user").service(createuser);
    cfg.service(authscope);
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app = || App::new().service(hello).service(login).configure(config);
    HttpServer::new(app).bind("127.0.0.1:8080")?.run().await
}
