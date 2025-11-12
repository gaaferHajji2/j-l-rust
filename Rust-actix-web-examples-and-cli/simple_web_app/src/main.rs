mod views;
use actix_web::{web, App, HttpServer, Responder, HttpRequest};

async fn say_hello(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");

    format!("Hello {}", name)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .route("/", web::get().to(say_hello))
        .route("/{name}", web::get().to(say_hello))
        .route("/hello/jloka", web::get().to(|| async { "Hello JLoka World"}))
    }).bind("127.0.0.1:8080")?
    .workers(3)
    .run()
    .await
}
