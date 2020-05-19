use crate::executor::DbExecutor;
use actix::Addr;

pub struct AppState {
    pub db: Addr<DbExecutor>,
}
