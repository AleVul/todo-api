use crate::categories::Category;
use std::sync::Mutex;

pub struct AppState {
    pub categories: Mutex<Vec<Category>>,
}
