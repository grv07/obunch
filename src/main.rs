mod errors;
mod models;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello")
}

#[get("/create")]
async fn createuser() -> impl Responder {
    "data"
}

#[post("/login")]
async fn login() -> String {
    "data\n".to_string()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let authscope = web::scope("/user").service(createuser);
    let app = || App::new().service(hello).service(login).service(authscope);
    HttpServer::new(app).bind("127.0.0.1:8080")?.run().await
}
