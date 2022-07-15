use actix_web::{HttpResponse, get, Responder};

#[get("/ping")]
async fn ping() -> impl Responder {
    HttpResponse::Ok().body("pong!")
}
