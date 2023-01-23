use actix_web::{ post, Responder, HttpResponse };

#[post("/playground")]
pub async fn run_playground() -> impl Responder {
    HttpResponse::Ok().body("42 is the answer to life, the universe and everything.");
}