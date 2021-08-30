use crate::resorts::{Resort, Resorts};
use crate::error_handler::CustomError;
use actix_web::{delete, get, post, put, web, HttpResponse};
use serde_json::json;

#[get("/resorts")]
async fn find_all() -> Result<HttpResponse, CustomError> {
    let resorts = Resorts::find_all()?;
    Ok(HttpResponse::Ok().json(resorts))
}

#[get("/resorts/{id}")]
async fn find(id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let resort = Resorts::find(id.into_inner())?;
    Ok(HttpResponse::Ok().json(resort))
}

#[post("/resorts")]
async fn create(resort: web::Json<Resort>) -> Result<HttpResponse, CustomError> {
    
    let resort = Resorts::create(resort.into_inner())?;
    Ok(HttpResponse::Ok().json(resort))
}

#[put("/resorts/{id}")]
async fn update(
    id: web::Path<i32>,
    resort: web::Json<Resort>,
) -> Result<HttpResponse, CustomError> {
    let resort = Resorts::update(id.into_inner(), resort.into_inner())?;
    Ok(HttpResponse::Ok().json(resort))
}

#[delete("/resorts/{id}")]
async fn delete(id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let deleted_employee = Resorts::delete(id.into_inner())?;
    Ok(HttpResponse::Ok().json(json!({ "deleted": deleted_employee })))
}

pub fn init_routes(comfig: &mut web::ServiceConfig) {
    comfig.service(find_all);
    comfig.service(find);
    comfig.service(create);
    comfig.service(update);
    comfig.service(delete);
}