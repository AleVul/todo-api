use super::models::{Category, CreateCategoryDto, QueryCategories};
use crate::state::AppState;
use actix_web::{delete, get, patch, post, web, HttpResponse, Result};
use uuid::Uuid;

#[post("/categories")]
async fn create(
    dto: web::Json<CreateCategoryDto>,
    state: web::Data<AppState>,
) -> Result<HttpResponse> {
    let res = state.db.send(Category::from(dto.into_inner())).await;

    // TODO: rethink this how to make it more idiomic in rust.
    match res {
        Ok(res2) => match res2 {
            Ok(cat) => Ok(HttpResponse::Ok().json(cat)),
            _ => Ok(HttpResponse::Ok().body("data")),
        },
        _ => Ok(HttpResponse::Ok().body("data")),
    }
}

#[get("/categories")]
async fn read(
    state: web::Data<AppState>,
    query: web::Query<QueryCategories>,
) -> Result<HttpResponse> {
    let res = state.db.send(query.into_inner()).await;

    match res {
        Ok(res2) => match res2 {
            Ok(cats) => Ok(HttpResponse::Ok().json(cats)),
            _ => Ok(HttpResponse::Ok().body("data")),
        },
        _ => Ok(HttpResponse::Ok().body("data")),
    }
}

#[patch("/categories/{id}")]
async fn update(
    _id: web::Path<String>,
    _dto: web::Json<CreateCategoryDto>,
) -> Result<HttpResponse> {
    let obj = Category {
        id: Uuid::new_v4().to_string(),
        name: String::from("Foo"),
        color: String::from("Bar"),
    };

    Ok(HttpResponse::Ok().json(obj))
}

#[delete("/categories/{id}")]
async fn delete(id: web::Path<u64>) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(*id))
}

pub fn init_config(config: &mut web::ServiceConfig) {
    config
        .service(create)
        .service(read)
        .service(update)
        .service(delete);
}
