use actix_web::{App, HttpResponse, HttpServer, Responder, get, web};

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
    return HttpResponse::Ok().body("Jafar Loka-01")
}

#[get("/name/{name}")]
async fn get_name(name: web::Path<String>) -> impl Responder {
    let msg: String = format!("The name is: {}", name.into_inner());
    return HttpResponse::Ok().body(msg)
}