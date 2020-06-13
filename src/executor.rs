//! This holds the data base executor
//! actor.

use crate::categories::{Category, QueryCategories};
use crate::schema::categories::dsl::*;
use actix::{Actor, Handler, Message, SyncContext};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::result::Error;
use std::convert::TryInto;

pub struct DbExecutor(pub Pool<ConnectionManager<PgConnection>>);

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}

impl Message for Category {
    type Result = Result<Category, Error>;
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

impl Message for QueryCategories {
    type Result = Result<Vec<Category>, Error>;
}

impl Handler<QueryCategories> for DbExecutor {
    type Result = Result<Vec<Category>, Error>;

    fn handle(&mut self, msg: QueryCategories, _: &mut Self::Context) -> Self::Result {
        let conn: &PgConnection = &self.0.get().unwrap();
        let mut query = categories.into_boxed();

        if let Some(name_field) = msg.name {
            query = query.filter(name.eq(name_field));
        }

        if let Some(color_field) = msg.color {
            query = query.filter(color.eq(color_field))
        }

        query
            .limit(msg.per_page.try_into().unwrap())
            .load::<Category>(conn)
    }
}
