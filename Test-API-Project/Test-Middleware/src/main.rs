use actix_web::{App, Error, HttpResponse, HttpServer, Responder, body::MessageBody, dev::{ServiceRequest, ServiceResponse}, get, middleware::{Next, from_fn}, web};

#[actix_web::main]
async fn main() {
    HttpServer::new(|| {
        App::new()
            .service(jloka_hello)
            .service(
                web::scope("/world")
                    .route("", web::get().to(jloka_world))
                    .wrap(from_fn(my_middleware))
            )
            // .wrap(from_fn(my_middleware))
    }).bind("0.0.0.0:3000").unwrap().run().await.unwrap()
}

#[get("/hello")]
async fn jloka_hello() -> impl Responder {
    return HttpResponse::Ok().body("Hello World from Jafar Loka-01")
}

// #[get("/world")]
async fn jloka_world() -> impl Responder {
    return HttpResponse::Ok().body("Hello World from Jafar Loka-02")
}

async fn my_middleware(req: ServiceRequest, next: Next<impl MessageBody>) -> Result<ServiceResponse<impl MessageBody>, Error> {
    println!("Hello From My Middleware");
    // Ok(req.into_response(HttpResponse::Unauthorized().body("Unauthorized Test")))
    next.call(req).await
}