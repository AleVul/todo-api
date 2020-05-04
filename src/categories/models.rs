use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Category {
    pub id: u64,
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreateCategoryDto {
    pub name: String,
}
