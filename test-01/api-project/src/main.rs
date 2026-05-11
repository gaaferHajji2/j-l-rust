use actix_web::{App, HttpResponse, HttpServer, Responder, get, post, web::{self, Json, Path, Query}};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
struct JLoka {
    name: String,
    age: i32
}

#[actix_web::main]
async fn main() {

    let jloka = JLoka {name: "Jafar-Loka-01".to_string(), age: 26};

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(jloka.clone()))
            .service(return_hello)
            .service(get_name)
            .service(get_query_data)
            .service(get_jloka_data)
            .service(get_response)
    })
    .bind("0.0.0.0:3000")
    .unwrap()
    .run()
    .await
    .unwrap();
}

#[get("/")]
async fn return_hello(jloka: web::Data<JLoka>) -> impl Responder {
    let msg : String = format!("Name is: {}, and Age is: {}", jloka.name, jloka.age);
    return HttpResponse::Ok().body(msg)
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

#[get("/get-jloka-info")]
async fn get_response()-> impl Responder {
    let jloka_data: JLoka = JLoka { name: "Jafar-Loka-01".to_string(), age: 26};
    let json_data: String = serde_json::to_string(&jloka_data).unwrap();
    return HttpResponse::Ok().json(json_data);
}