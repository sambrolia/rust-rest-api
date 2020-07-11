use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/test1")]
async fn test() -> impl Responder {
    HttpResponse::Ok().body("look at it go!")
}

#[get("/rust")]
async fn language() -> impl Responder {
    HttpResponse::Ok().body("Rusty!")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(test)
            .service(language)
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}
