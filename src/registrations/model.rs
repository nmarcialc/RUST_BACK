use crate::db;
use crate::error_handler::CustomError;
use crate::schema::registrations;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[table_name = "registrations"]
pub struct Registration {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub resort_id: i32,
    
}

#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "registrations"]
pub struct Registrations {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub resort_id: i32,
}

impl Registrations {
    pub fn find_all() -> Result<Vec<Self>, CustomError> {
        let conn = db::connection()?;
        let registrations = registrations::table.load::<Registrations>(&conn)?;
        Ok(registrations)
    }

    pub fn find(id: i32) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let registration = registrations::table.filter(registrations::id.eq(id)).first(&conn)?;
        Ok(registration)
    }

    pub fn create(registration: Registration) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let registration = Registration::from(registration);
        let registration = diesel::insert_into(registrations::table)
            .values(registration)
            .get_result(&conn)?;
        Ok(registration)
    }

    pub fn update(id: i32, registration: Registration) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let registration = diesel::update(registrations::table)
            .filter(registrations::id.eq(id))
            .set(registration)
            .get_result(&conn)?;
        Ok(registration)
    }

    pub fn delete(id: i32) -> Result<usize, CustomError> {
        let conn = db::connection()?;
        let res = diesel::delete(registrations::table.filter(registrations::id.eq(id))).execute(&conn)?;
        Ok(res)
    }
}

impl Registration {
    fn from(registration: Registration) -> Registration {
        Registration {
            first_name: registration.first_name,
            last_name: registration.last_name,
            email: registration.email,
            resort_id: registration.resort_id,
        }
    }
}