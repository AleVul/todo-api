use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Queryable)]
pub struct Category {
    pub id: String,
    pub name: String,
    pub color: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreateCategoryDto {
    pub name: String,
    pub color: String,
}
