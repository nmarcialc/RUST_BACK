use crate::db;
use crate::error_handler::CustomError;
use crate::schema::resorts;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[table_name = "resorts"]
pub struct Resort {
    pub name: String,
    pub description: String,
   
}

#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "resorts"]
pub struct Resorts {
    pub id: i32,
    pub name: String,
    pub description: String,
}

impl Resorts {
    pub fn find_all() -> Result<Vec<Self>, CustomError> {
        let conn = db::connection()?;
        let resorts = resorts::table.load::<Resorts>(&conn)?;
        Ok(resorts)
    }

    pub fn find(id: i32) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let resort = resorts::table.filter(resorts::id.eq(id)).first(&conn)?;
        Ok(resort)
    }

    pub fn create(resort: Resort) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let resort = Resort::from(resort);
        let resort = diesel::insert_into(resorts::table)
            .values(resort)
            .get_result(&conn)?;
        Ok(resort)
    }

    pub fn update(id: i32, resort: Resort) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let resort = diesel::update(resorts::table)
            .filter(resorts::id.eq(id))
            .set(resort)
            .get_result(&conn)?;
        Ok(resort)
    }

    pub fn delete(id: i32) -> Result<usize, CustomError> {
        let conn = db::connection()?;
        let res = diesel::delete(resorts::table.filter(resorts::id.eq(id))).execute(&conn)?;
        Ok(res)
    }
}

impl Resort {
    fn from(resort: Resort) -> Resort {
        Resort {
            name: resort.name,
            description: resort.description,
            
        }
    }
}