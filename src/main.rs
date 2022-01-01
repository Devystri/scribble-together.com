pub mod endpoints;


use actix_web::{ App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(endpoints::files::index)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}