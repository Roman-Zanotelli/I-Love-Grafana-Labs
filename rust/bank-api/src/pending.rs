use std::collections::HashMap;

use axum::http::StatusCode;
use jwt_util::core::JwtClaims;
use serde::Serialize;
use sqlx::{types::chrono::{DateTime, Utc}, Pool, Postgres};

use crate::{transaction::Transaction, Queriable};

#[derive(Debug, Serialize)]
struct PendingResponse{
    pending_transactions: Option<Vec<PendingTransaction>> //list of pending transactions recieved or affected
}

#[derive(Debug, Serialize)]
pub struct PendingTransaction{
    transaction: Transaction, //transaction pending
    request_timestamp: DateTime<Utc>, //timestamp of intial request
}

impl Queriable for PendingResponse {
    async fn get_query(pool: &Pool<Postgres>, claims: &jwt_util::core::JwtClaims, params: &HashMap<String, String>) {
        todo!()
    }

    async fn post_query(pool: &Pool<Postgres>, claims: &jwt_util::core::JwtClaims, params: &HashMap<String, String>) {
        todo!()
    }
}

pub async fn get_pending(pool: &Pool<Postgres>, claims: &JwtClaims, params: &HashMap<String, String>) -> (StatusCode, String){
    (StatusCode::INTERNAL_SERVER_ERROR, "TODO".to_string())
}
pub async fn post_pending(pool: &Pool<Postgres>, claims: &JwtClaims, params: &HashMap<String, String>) -> (StatusCode, String){
    (StatusCode::INTERNAL_SERVER_ERROR, "TODO".to_string())
}