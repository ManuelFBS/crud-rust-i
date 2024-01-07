use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use core::sync::atomic::AtomicU16;
use core::sync::atomic::Ordering;
use std::sync::Arc;

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {} !!!", &name)
}

async fn health_check(thread_index: u16) -> HttpResponse {
    HttpResponse::Ok()
        .append_header(("thread-id", thread_index.to_string()))
        .finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting the server...");
    let thread_counter = Arc::new(AtomicU16::new(1));
    HttpServer::new(move || {
        println!(
            "Starting the thread: {}",
            thread_counter.fetch_add(1, Ordering::SeqCst)
        );
        let thread_index = thread_counter.load(Ordering::SeqCst);
        App::new()
            .route("/", web::get().to(greet))
            .route("/health", web::get().to(move || health_check(thread_index)))
            .route("/str", web::get().to(|| async { "Hello Rust" }))
            .route("/{name}", web::get().to(greet))
    })
    .bind(("localhost", 8085))?
    .run()
    .await
}
