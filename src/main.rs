mod errors;
mod models;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use models::User;

// TODO: Make it seprate trait/module.
mod auth {
    use serde::Deserialize;
    #[derive(Deserialize)]
    pub struct Login {
        pub name: String,
        password: String,
    }
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello")
}

#[post("/create")]
async fn create_user(user: web::Json<User>) -> impl Responder {
    // Make a create user call.
    format!("data: {:?}\n", user.name)
}

#[post("/login")]
async fn login(login: web::Json<auth::Login>) -> impl Responder {
    format!("pass: {:?} \n", login.name)
}

// This can be inside an module/crate or lib.
fn config(cfg: &mut web::ServiceConfig) {
    let authscope = web::scope("/user").service(create_user);
    cfg.service(authscope);
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app = || App::new().service(hello).service(login).configure(config);
    HttpServer::new(app).bind("127.0.0.1:8080")?.run().await
}
