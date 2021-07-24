use actix_web::{get, delete, post, web, App, HttpResponse, HttpServer, Responder};

pub struct OrderServiceHandler;

impl OrderServiceHandler {
   pub fn config(cnf: &mut web::ServiceConfig) {
       cfg.service(
           web::scope("/order")
           .service(get_order)
           .service(create_order)
           .service(update_order)
           .service(delete_order));
   }
}

#[get("/get")]
async fn get_order() -> impl Responder {
    format!("data: {:?}\n", "order")
}

#[post("/create")]
async fn create_order(order: web::Json<Order>) -> impl Responder {
    format!("data: {:?}\n", order)
}

#[post("/update")]
async fn update_order() -> impl Responder {
    format!("order: {:?}\n", "order")
}

#[delete("/delete")]
async fn delete_order() -> impl Responder {
    format!("order: {:?}\n", "order")
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
