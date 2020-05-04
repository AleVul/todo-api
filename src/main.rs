mod categories;
mod state;

use actix_web::{web, App, HttpResponse, HttpServer};
use state::AppState;
use std::sync::Mutex;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let state = web::Data::new(AppState {
        categories: Mutex::new(Vec::<categories::Category>::new()),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .configure(categories::init_config)
            .route("/health", web::get().to(HttpResponse::Ok))
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}
