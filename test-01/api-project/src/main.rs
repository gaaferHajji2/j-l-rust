use actix_web::{App, HttpResponse, HttpServer, web::get};

#[actix_web::main]
async fn main() {
    HttpServer::new(|| {
        App::new().route("/", get().to(|| async { HttpResponse::Ok().body("get")}))
    })
    .bind("0.0.0.0:3000")
    .unwrap()
    .run()
    .await
    .unwrap();
}
