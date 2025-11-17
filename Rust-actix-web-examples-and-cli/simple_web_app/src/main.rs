mod views;
mod to_do;
mod operation;
mod state;
mod json_serialization;
use actix_web::{App, HttpServer};

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
