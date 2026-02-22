use actix_web::{App, HttpServer};
use fintrack::api;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server starting at http://127.0.0.1:8080");

    HttpServer::new(|| {
        App::new()
            .configure(api::configure_auth)
            .configure(api::configure_users)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}