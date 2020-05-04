mod categories;

use actix_web::{web, App, HttpResponse, HttpServer};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(categories::init_config)
            .route("/health", web::get().to(HttpResponse::Ok))
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}
