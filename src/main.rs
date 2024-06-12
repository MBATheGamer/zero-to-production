use actix_web::{web, App, HttpServer, Responder};

async fn hello_world() -> impl Responder {
    format!("Hello, World!")
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    HttpServer::new(|| App::new().route("/", web::get().to(hello_world)))
        .bind("127.0.0.1:8000")?
        .run()
        .await
}
