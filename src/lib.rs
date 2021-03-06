#[macro_use]
extern crate diesel;
extern crate actix;
extern crate dotenv;
extern crate num_cpus;
extern crate pretty_env_logger;
#[macro_use]
extern crate log;

mod categories;
mod executor;
mod schema;
mod state;

use actix::SyncArbiter;
use actix_web::dev::Server;
use actix_web::{web, App, HttpResponse, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use executor::DbExecutor;
use state::AppState;
use std::net::TcpListener;

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let db_url = "postgres://postgres:test@localhost:5432/todo";
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    let manager2 = ConnectionManager::<SqliteConnection>::new(db_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    let addr = SyncArbiter::start(num_cpus::get(), move || DbExecutor(pool.clone()));
    let state = web::Data::new(AppState { db: addr });

    info!("Starting server");
    let server = HttpServer::new(move || {
        trace!("Creating app instance");
        App::new()
            .app_data(state.clone())
            .configure(categories::init_config)
            .route("/health", web::get().to(HttpResponse::Ok))
    })
    .listen(listener)?
    .run();

    Ok(server)
}

pub struct Config<T> {
    listener: TcpListener,
    manager: ConnectionManager<T>,
}

impl<T> Config<T> {
    pub fn new(listener: TcpListener, db_url: &str) -> Self {
        Config {
            listener,
            manager: ConnectionManager::<T>::new(db_url),
        }
    }
}
