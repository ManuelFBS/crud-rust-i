mod repository;
mod user;

use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {} !!!", &name)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let port = std::env::var("PORT").unwrap_or("8085".to_string());
    let address = format!("127.0.0.1:{}", port);

    println!("Starting the server...");
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/health", web::get().to(|| HttpResponse::Ok()))
            .route("/str", web::get().to(|| async { "Hello Rust" }))
            .route("/{name}", web::get().to(greet))
    })
    .bind(&address)?
    .run()
    .await
}
