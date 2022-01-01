pub mod endpoints;
pub mod websockets;

use actix_web::{web, App, HttpServer};

#[actix_web::main]

async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(endpoints::files::index)
            .route("/ws/", web::get().to(websockets::handler::index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}