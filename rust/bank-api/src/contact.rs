use std::collections::HashMap;

use axum::http::StatusCode;
use jwt_util::core::JwtClaims;
use serde::Serialize;
use sqlx::{Pool, Postgres, Row};

use crate::{Parsable, Queriable};

#[derive(Debug, Serialize)]
struct ContactResponse{
    contacts: Option<Vec<Contact>> //list of contacts affect or retrieved
}

#[derive(Debug, Serialize)]
pub struct Contact{
    name: String, //Display Name
    id: String, //Acount ID
    saved: Option<bool> //True if favorite, False if just saved, None if not saved
}

impl Parsable for Contact{
    fn parse_row(row: &sqlx::postgres::PgRow) -> Self {
        Contact{
            name: row.get("name"),
            id: row.get("id"),
            saved: row.get("saved"),
        }
    }
}


fn parse_rows(row: &Vec<sqlx::postgres::PgRow>) -> Option<Vec<Contact>> {
        todo!()
}

impl Queriable for ContactResponse{
    async fn get_query(pool: &Pool<Postgres>, claims: &jwt_util::core::JwtClaims, params: &HashMap<String, String>) -> anyhow::Result<String> {
        //TODO: Write SQL Statement
        let row = sqlx::query("SQL TODO").fetch_all(pool).await?;
        Ok(serde_json::to_string(&ContactResponse{
            contacts: parse_rows(&row),
        })?)
    }

    async fn post_query(pool: &Pool<Postgres>, claims: &jwt_util::core::JwtClaims, params: &HashMap<String, String>) -> anyhow::Result<String> {
        //TODO: Write SQL Statement
        let row = sqlx::query("SQL TODO").fetch_all(pool).await?;
        Ok(serde_json::to_string(&ContactResponse{
            contacts: parse_rows(&row),
        })?)
    }
}


pub async fn get_contact(pool: &Pool<Postgres>, claims: &JwtClaims, params: &HashMap<String, String>) -> (StatusCode, String){
    match ContactResponse::get_query(pool, claims, params).await{
        Ok(resp) => (StatusCode::OK, resp),
        Err(err) => (StatusCode::NOT_FOUND, err.to_string()),
    }
}
pub async fn post_contact(pool: &Pool<Postgres>, claims: &JwtClaims, params: &HashMap<String, String>) -> (StatusCode, String){
    match ContactResponse::post_query(pool, claims, params).await{
        Ok(resp) => (StatusCode::OK, resp),
        Err(err) => (StatusCode::NOT_MODIFIED, err.to_string()),
    }
}