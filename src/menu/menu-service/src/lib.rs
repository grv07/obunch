use actix_web::{get, delete, post, web, App, HttpResponse, HttpServer, Responder};

pub struct MenuServiceHandler;

impl MenuServiceHandler {
   pub fn config(cnf: &mut web::ServiceConfig) {
       cfg.service(
           web::scope("/menu")
           .service(get_menu)
           .service(create_menu)
           .service(update_menu)
           .service(delete_menu));
   }
}

#[get("/get")]
async fn get_menu(menu: web::Json<Menu>) -> impl Responder {
    format!("data: {:?}\n", menu)
}

#[post("/create")]
async fn create_menu(menu: web::Json<Menu>) -> impl Responder {
    format!("data: {:?}\n", menu)
}

#[post("/update")]
async fn update_menu() -> impl Responder {
    format!("data: {:?}\n", "update menu")
}

#[delete("/delete")]
async fn delete_menu() -> impl Responder {
    format!("menu: {:?}\n", "drop")
}

