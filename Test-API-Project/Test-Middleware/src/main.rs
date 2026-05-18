use actix_web::{App, Error, HttpMessage, HttpRequest, HttpResponse, HttpServer, Responder, body::MessageBody, dev::{ServiceRequest, ServiceResponse}, get, middleware::{Next, from_fn}, web};

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
async fn jloka_hello(req: HttpRequest) -> impl Responder {
    match req.extensions().get::<String>() {
        Some(msg) => HttpResponse::Ok().body(format!("The message from JLoka is: {}", msg)),
        None => HttpResponse::Ok().body("No Message Found")
    }
}

// #[get("/world")]
async fn jloka_world(req: HttpRequest) -> impl Responder {
    
    match req.extensions().get::<String>() {
        Some(msg) => HttpResponse::Ok().body(format!("The message from JLoka is: {}", msg)),
        None => HttpResponse::Ok().body("No Message Found")
    }
}

async fn my_middleware(req: ServiceRequest, next: Next<impl MessageBody>) -> Result<ServiceResponse<impl MessageBody>, Error> {
    println!("Hello From My Middleware");
    req.extensions_mut().insert("Hello From JLoka Middleware".to_string());
    // Ok(req.into_response(HttpResponse::Unauthorized().body("Unauthorized Test")))
    next.call(req).await
}