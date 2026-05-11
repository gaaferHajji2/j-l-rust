use std::sync::Mutex;

use actix_web::{App, HttpResponse, HttpServer, Responder, get, post, web::{self, Json, Path, Query}};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
struct JLoka {
    name: String,
    age: i32
}

struct JLokaMutex {
    name: Mutex<String>,
    age: Mutex<i32>
}

#[actix_web::main]
async fn main() {

    let jloka: JLoka = JLoka {name: "Jafar-Loka-01".to_string(), age: 26};
    let jloka_mutex: web::Data<JLokaMutex> = web::Data::new(
        JLokaMutex {    name: Mutex::new(String::from("Jafar Loka-01 Mutex Example")), 
                        age: Mutex::new(26)
        }
    );

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(jloka.clone()))
            .app_data(jloka_mutex.clone())
            .service(return_hello)
            .service(get_name)
            .service(get_query_data)
            .service(get_jloka_data)
            .service(get_response)
            .service(return_jloka)
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

#[get("/jloka")]
async fn return_jloka(jloka: web::Data<JLokaMutex>) -> impl Responder {
    let msg : String = format!(
        "Name is: {}, and Age is: {}", 
        jloka.name.lock().unwrap(), 
        jloka.age.lock().unwrap()
    );
    return HttpResponse::Ok().body(msg)
}

#[get("/name/{name}")]
async fn get_name(name: Path<String>, jloka_data: web::Data<JLokaMutex>) -> impl Responder {
    *jloka_data.name.lock().unwrap() = name.into_inner();
    let msg: String = format!("The name is: {}", jloka_data.name.lock().unwrap());
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