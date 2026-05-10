use actix_web::{App, HttpResponse, HttpServer, Responder, get, web::{Query, Path}};
use serde::Deserialize;

#[derive(Deserialize)]
struct JLoka {
    name: String,
    age: i32
}

#[actix_web::main]
async fn main() {
    HttpServer::new(|| {
        App::new().service(return_hello).service(get_name).service(get_query_data)
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
async fn get_name(name: Path<String>) -> impl Responder {
    let msg: String = format!("The name is: {}", name.into_inner());
    return HttpResponse::Ok().body(msg)
}

#[get("/query")]
async fn get_query_data(query_data: Query<JLoka>) -> impl  Responder {
    let jloka_msg = format!("The name is: {}, the age is: {}", query_data.name, query_data.age);
    return HttpResponse::Ok().body(jloka_msg);
}