use actix::Message;
use diesel::result::Error;
use serde::{Deserialize, Serialize};
// required for `table_name` macro.
use crate::schema::categories;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "categories"]
pub struct Category {
    pub id: String,
    pub name: String,
    pub color: String,
}

impl From<CreateCategoryDto> for Category {
    fn from(dto: CreateCategoryDto) -> Self {
        Category {
            id: Uuid::new_v4().to_string(),
            name: dto.name,
            color: dto.color,
        }
    }
}

impl Message for Category {
    type Result = Result<Category, Error>;
}

/**
 * DTO to create a `Category` entity
 * in the database.
 *
 * Note: should we derive `Serialize` here?
 */
#[derive(Serialize, Deserialize)]
pub struct CreateCategoryDto {
    pub name: String,
    pub color: String,
}
