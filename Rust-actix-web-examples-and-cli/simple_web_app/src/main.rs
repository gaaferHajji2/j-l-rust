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
        .configure(views::views_factory)
    }).bind("127.0.0.1:8080")?
    .workers(3)
    .run()
    .await
}
