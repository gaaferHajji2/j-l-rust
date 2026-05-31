use actix_web::{App, HttpResponse, HttpServer, Responder, guard, post, web::{self, Form, ServiceConfig}};
use serde::Deserialize;

#[derive(Deserialize)]
struct Info {
    name: String,
    age: i32
}

#[actix_web::main]
async fn main() {
    HttpServer::new( move || {
        App::new()
        .service(get_form_data)
        .service(
            web::scope("/api")
            .guard(guard::Get())
            .route("/hello", web::get().to(hello))
            .route("/world", web::post().to(world))
        )
        .service(web::scope("").configure(config))
        
    }).bind("0.0.0.0:3000").unwrap().run().await.unwrap()
}

async fn hello() -> impl Responder {
    HttpResponse::Ok().body("JLoka From Hello Path")
}

async fn world() -> impl Responder {
    HttpResponse::Ok().body("JLoka From World Path")
}

fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/hello")
                .route("/world", web::get().to(|| async {
                    HttpResponse::Ok().body("Hello Jafar-Loka World from the service config")
                }
            )
        )
    );
}

#[post("/info")]
async fn get_form_data(info: Form<Info>) -> impl Responder {
    let msg = format!("Name is: {}, age is: {}", info.name, info.age);

    return HttpResponse::Ok().body(msg);
}