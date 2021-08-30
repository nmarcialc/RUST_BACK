use crate::registrations::{Registration, Registrations};
use crate::error_handler::CustomError;
use actix_web::{delete, get, post, put, web, HttpResponse};
use serde_json::json;

#[get("/registrations")]
async fn find_all() -> Result<HttpResponse, CustomError> {
    let registrations = Registrations::find_all()?;
    Ok(HttpResponse::Ok().json(registrations))
}

#[get("/registrations/{id}")]
async fn find(id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let registration = Registrations::find(id.into_inner())?;
    Ok(HttpResponse::Ok().json(registration))
}

#[post("/registrations")]
async fn create(registration: web::Json<Registration>) -> Result<HttpResponse, CustomError> {
    
    let registration = Registrations::create(registration.into_inner())?;
    Ok(HttpResponse::Ok().json(registration))
}

#[put("/registrations/{id}")]
async fn update(
    id: web::Path<i32>,
    registration: web::Json<Registration>,
) -> Result<HttpResponse, CustomError> {
    let registration = Registrations::update(id.into_inner(), registration.into_inner())?;
    Ok(HttpResponse::Ok().json(registration))
}

#[delete("/registrations/{id}")]
async fn delete(id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let deleted_employee = Registrations::delete(id.into_inner())?;
    Ok(HttpResponse::Ok().json(json!({ "deleted": deleted_employee })))
}

pub fn init_routes(comfig: &mut web::ServiceConfig) {
    comfig.service(find_all);
    comfig.service(find);
    comfig.service(create);
    comfig.service(update);
    comfig.service(delete);
}