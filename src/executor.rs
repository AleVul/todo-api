//! This holds the data base executor
//! actor.

use super::schema::categories::dsl::*;
use crate::categories::Category;
use crate::diesel::RunQueryDsl;
use actix::{Actor, Handler, SyncContext};
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::result::Error;

pub struct DbExecutor(pub Pool<ConnectionManager<PgConnection>>);

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}

impl Handler<Category> for DbExecutor {
    type Result = Result<Category, Error>;

    fn handle(&mut self, msg: Category, _: &mut Self::Context) -> Self::Result {
        let conn: &PgConnection = &self.0.get().unwrap();

        let _ = diesel::insert_into(categories)
            .values(&msg)
            .execute(conn)
            .expect("Error creating category");

        Ok(msg)
    }
}
