use std::collections::HashMap;

use axum::http::StatusCode;
use chrono::{DateTime, Utc};
use jwt_util::core::JwtClaims;
use serde::Serialize;
use sqlx::{Pool, Postgres};

use crate::{contact::Contact, Queriable};

#[derive(Debug, Serialize)]
struct TransactionResponse{
    transactions: Option<Vec<Transaction>> //list of transactions retrieved or affected
}

#[derive(Debug, Serialize)]
pub struct Transaction{
    from: Contact, //who sent money
    to: Contact, //who recieved money
    amount: u64, //amount (in pennies)
    processed_timestamp: Option<DateTime<Utc>>, //timestamp of completion
    status: Option<bool> //final staus (true = success, false = cancelled, None = pending)
}

impl Queriable for TransactionResponse {
    async fn get_query(pool: &Pool<Postgres>, claims: &jwt_util::core::JwtClaims, params: &HashMap<String, String>) {
        todo!()
    }

    async fn post_query(pool: &Pool<Postgres>, claims: &jwt_util::core::JwtClaims, params: &HashMap<String, String>) {
        todo!()
    }
}

pub async fn get_transaction(pool: &Pool<Postgres>, claims: &JwtClaims, params: &HashMap<String, String>) -> (StatusCode, String){
    (StatusCode::INTERNAL_SERVER_ERROR, "TODO".to_string())
}
pub async fn post_transaction(pool: &Pool<Postgres>, claims: &JwtClaims, params: &HashMap<String, String>) -> (StatusCode, String){
    (StatusCode::INTERNAL_SERVER_ERROR, "TODO".to_string())
}