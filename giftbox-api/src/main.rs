use actix_web::{get, post, web, middleware::Logger, App, HttpResponse, HttpRequest, HttpServer, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct Box {
    items: Vec<String>,
}


#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[get("/boxes/{id}")]
async fn get_box(req: HttpRequest) -> impl Responder {
    let box_id: String = req.match_info().query("id").parse().unwrap();
    HttpResponse::Ok().body(box_id)
}

#[post("/boxes")]
async fn post_box(giftbox: web::Json<Box>) -> impl Responder {
    HttpResponse::Ok()
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .wrap(Logger::default())
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
