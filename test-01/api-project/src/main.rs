use actix_web::{App, HttpResponse, HttpServer, Responder, get, post, web::{Json, Path, Query}};
use serde::Deserialize;

#[derive(Deserialize, Serialize)]
struct JLoka {
    name: String,
    age: i32
}

#[actix_web::main]
async fn main() {
    HttpServer::new(|| {
        App::new()
            .service(return_hello)
            .service(get_name)
            .service(get_query_data)
            .service(get_jloka_data)
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
#[post("/jloka")]
async fn get_jloka_data(jloka_data: Json<JLoka>) -> impl Responder {
    let data = format!("The username is: {} & age is: {}", jloka_data.name, jloka_data.age);
    return HttpResponse::Ok().body(data);
}