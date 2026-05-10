use actix_web::{App, HttpResponse, HttpServer, Responder, get};

#[actix_web::main]
async fn main() {
    HttpServer::new(|| {
        App::new().service(return_hello)
    })
    .bind("0.0.0.0:3000")
    .unwrap()
    .run()
    .await
    .unwrap();
}

#[get("/")]
async fn return_hello() -> impl Responder {
    return HttpResponse::Ok().body("Jafar Loka-01");
}