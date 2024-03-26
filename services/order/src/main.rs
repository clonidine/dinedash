use actix_web::{web, App, HttpResponse, HttpServer};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let addr = ("127.0.0.1", 8000);
    println!("Server running on {}:{}", addr.0, addr.1);
    HttpServer::new(|| App::new().route("/", web::get().to(HttpResponse::Ok)))
        .bind(addr)?
        .run()
        .await
}
