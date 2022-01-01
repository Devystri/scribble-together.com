pub mod endpoints;
pub mod websockets;
pub mod chunk;

use actix_web::{web, App, HttpServer};

#[actix_web::main]

async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/ws/").route(web::get().to(websockets::handler::index)))
            .service(endpoints::get::index)
            .service(endpoints::files::index)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}