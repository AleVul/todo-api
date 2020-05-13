use actix::Addr;
use crate::executor::DbExecutor;

pub struct AppState {
    pub db: Addr<DbExecutor>,
}
