use std::collections::HashMap;

use axum::http::StatusCode;
use jwt_util::core::JwtClaims;
use serde::Serialize;
use sqlx::{types::chrono::{DateTime, Utc}, Pool, Postgres, Row};

use crate::{transaction::Transaction, Parsable, Queriable};

#[derive(Debug, Serialize)]
struct PendingResponse{
    pending_transactions: Option<Vec<PendingTransaction>> //list of pending transactions recieved or affected
}

#[derive(Debug, Serialize)]
pub struct PendingTransaction{
    transaction: Transaction, //transaction pending
    request_timestamp: DateTime<Utc>, //timestamp of intial request
}

impl Parsable for PendingTransaction {
    fn parse_row(row: &sqlx::postgres::PgRow) -> Self{
        PendingTransaction{
            transaction: Transaction::parse_row(row),
            request_timestamp: row.get("request_timestamp"),
        }
    }
}
fn parse_rows(row: &Vec<sqlx::postgres::PgRow>) -> Option<Vec<PendingTransaction>> {
        todo!()
}


impl Queriable for PendingResponse {
    async fn get_query(pool: &Pool<Postgres>, claims: &jwt_util::core::JwtClaims, params: &HashMap<String, String>) -> anyhow::Result<String> {
        //TODO: Write SQL Statement
        let row = sqlx::query("SQL TODO").fetch_all(pool).await?;
        Ok(serde_json::to_string(&PendingResponse{
            pending_transactions: parse_rows(&row),
        })?)
    }

    async fn post_query(pool: &Pool<Postgres>, claims: &jwt_util::core::JwtClaims, params: &HashMap<String, String>) -> anyhow::Result<String> {
        //TODO: Write SQL Statement
        let row = sqlx::query("SQL TODO").fetch_all(pool).await?;
        Ok(serde_json::to_string(&PendingResponse{
            pending_transactions: parse_rows(&row),
        })?)
    }
}

pub async fn get_pending(pool: &Pool<Postgres>, claims: &JwtClaims, params: &HashMap<String, String>) -> (StatusCode, String){
    match PendingResponse::get_query(pool, claims, params).await{
        Ok(resp) => (StatusCode::OK, resp),
        Err(err) => (StatusCode::NOT_FOUND, err.to_string()),
    }
}
pub async fn post_pending(pool: &Pool<Postgres>, claims: &JwtClaims, params: &HashMap<String, String>) -> (StatusCode, String){
     match PendingResponse::post_query(pool, claims, params).await{
        Ok(resp) => (StatusCode::OK, resp),
        Err(err) => (StatusCode::NOT_MODIFIED, err.to_string()),
    }
}