use std::collections::HashMap;

use axum::http::StatusCode;
use jwt_util::core::JwtClaims;
use sqlx::{Pool, Postgres};

pub async fn get_pending(pool: &Pool<Postgres>, claims: &JwtClaims, params: &HashMap<String, String>) -> (StatusCode, String){
    (StatusCode::INTERNAL_SERVER_ERROR, "TODO".to_string())
}
pub async fn post_pending(pool: &Pool<Postgres>, claims: &JwtClaims, params: &HashMap<String, String>) -> (StatusCode, String){
    (StatusCode::INTERNAL_SERVER_ERROR, "TODO".to_string())
}