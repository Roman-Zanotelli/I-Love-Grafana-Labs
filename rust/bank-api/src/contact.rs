use std::collections::HashMap;

use axum::http::StatusCode;
use jwt_util::core::JwtClaims;
use serde::Serialize;
use sqlx::{Pool, Postgres};

use crate::Queriable;

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

impl Queriable for ContactResponse{
    async fn get_query(pool: &Pool<Postgres>, claims: &jwt_util::core::JwtClaims, params: &HashMap<String, String>) {
        todo!()
    }

    async fn post_query(pool: &Pool<Postgres>, claims: &jwt_util::core::JwtClaims, params: &HashMap<String, String>) {
        todo!()
    }
}


pub async fn get_contact(pool: &Pool<Postgres>, claims: &JwtClaims, params: &HashMap<String, String>) -> (StatusCode, String){
    (StatusCode::INTERNAL_SERVER_ERROR, "TODO".to_string())
}
pub async fn post_contact(pool: &Pool<Postgres>, claims: &JwtClaims, params: &HashMap<String, String>) -> (StatusCode, String){
    (StatusCode::INTERNAL_SERVER_ERROR, "TODO".to_string())
}