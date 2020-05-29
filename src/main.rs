use actix_web::{web, App, HttpServer};
use chat::{index};


#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    HttpServer::new(|| {
        App::new()
        .route("/home", web::get().to(index))
    })
    .bind("127.0.0.1:3002")?
    .run()
    .await

}
