use actix_web::{get, delete, post, web, App, HttpResponse, HttpServer, Responder};

pub struct ShopServiceHandler;

impl ShopServiceHandler {
   pub fn config(cnf: &mut web::ServiceConfig) {
       cfg.service(
           web::scope("/shop")
           .service(get_shop)
           .service(create_shop)
           .service(update_shop)
           .service(delete_shop));
   }
}

#[get("/get")]
async fn get_shop() -> impl Responder {
    format!("data: {:?}\n", "shop")
}

#[post("/create")]
async fn create_shop(shop: web::Json<Shop>) -> impl Responder {
    format!("data: {:?}\n", shop)
}

#[post("/update")]
async fn update_shop() -> impl Responder {
    format!("data: {:?}\n", "shop")
}

#[delete("/delete")]
async fn delete_shop() -> impl Responder {
    format!("data: {:?}\n", "update menu")
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
