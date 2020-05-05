use super::models::{Category, CreateCategoryDto};
use crate::state::AppState;
use actix_web::{delete, get, patch, post, web, HttpResponse, Result};

#[post("/categories")]
async fn create(
    _dto: web::Json<CreateCategoryDto>,
    data: web::Data<AppState>,
) -> Result<HttpResponse> {
    let len = data.categories.lock().unwrap().len();

    let obj = Category {
        id: (len + 1) as u64,
        name: String::from("Foo"),
        color: String::from("Bar"),
    };

    Ok(HttpResponse::Ok().json(obj))
}

#[get("/categories")]
async fn read(data: web::Data<AppState>) -> Result<HttpResponse> {
    let _cats = data.categories.lock().unwrap();
    let obj = Category {
        id: 1,
        name: String::from("Foo"),
        color: String::from("Bar"),
    };

    Ok(HttpResponse::Ok().json(obj))
}

#[patch("/categories/{id}")]
async fn update(id: web::Path<u64>, _dto: web::Json<CreateCategoryDto>) -> Result<HttpResponse> {
    let obj = Category {
        id: *id,
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
