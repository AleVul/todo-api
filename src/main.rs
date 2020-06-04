#[macro_use]
extern crate diesel;
extern crate actix;
extern crate dotenv;
extern crate num_cpus;

mod categories;
mod executor;
mod schema;
mod state;

use actix::SyncArbiter;
use actix_web::{web, App, HttpResponse, HttpServer};
use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use executor::DbExecutor;
use state::AppState;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let manager =
        ConnectionManager::<PgConnection>::new("postgres://postgres:test@localhost:5432/todo");
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    let addr = SyncArbiter::start(num_cpus::get(), move || DbExecutor(pool.clone()));
    let state = web::Data::new(AppState { db: addr.clone() });

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
