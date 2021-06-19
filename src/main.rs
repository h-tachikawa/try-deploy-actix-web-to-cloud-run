mod http_client;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::Serialize;

#[derive(Serialize)]
struct MyResponse {
    str: String,
    num: isize,
    arr: Vec<isize>,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, World!")
}

#[get("/json")]
async fn get_json() -> impl Responder {
    HttpResponse::Ok().json(MyResponse {
        str: "test".to_string(),
        num: 100,
        arr: vec![1, 2, 3],
    })
}

#[get("/json_via_external_source")]
async fn get_json_via_external_source() -> impl Responder {
    let result = http_client::http_client_module::get(
        "https://jsonplaceholder.typicode.com/todos".to_string(),
    )
    .await;
    HttpResponse::Ok()
        .content_type("application/json")
        .body(result)
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .service(get_json)
            .service(get_json_via_external_source)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
