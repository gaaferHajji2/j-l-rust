use actix_web::{App, HttpResponse, HttpServer, Responder, get};

#[actix_web::main]
async fn main() {
    HttpServer::new(|| {
        App::new().service(jloka_hello).service(jloka_world)
    }).bind("0.0.0.0:3000").unwrap().run().await.unwrap()
}

#[get("/hello")]
async fn jloka_hello() -> impl Responder {
    return HttpResponse::Ok().body("Hello World from Jafar Loka-01")
}

#[get("/world")]
async fn jloka_world() -> impl Responder {
    return HttpResponse::Ok().body("Hello World from Jafar Loka-02")
}