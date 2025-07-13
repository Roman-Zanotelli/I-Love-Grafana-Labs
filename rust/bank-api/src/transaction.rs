use std::collections::HashMap;

use axum::http::StatusCode;
use chrono::{DateTime, Utc};
use jwt_util::core::JwtClaims;
use serde::Serialize;
use sqlx::{Pool, Postgres, Row};

use crate::{contact::Contact, Parsable, Queriable};

#[derive(Debug, Serialize)]
struct TransactionResponse{
    transactions: Option<Vec<Transaction>> //list of transactions retrieved or affected
}

#[derive(Debug, Serialize)]
pub struct Transaction{
    from: String, //who sent money
    to: String, //who recieved money
    amount: i32, //amount (in pennies)
    processed_timestamp: Option<DateTime<Utc>>, //timestamp of completion
    status: Option<bool> //final staus (true = success, false = cancelled, None = pending)
}

impl Parsable for Transaction {
    fn parse_row(row: &sqlx::postgres::PgRow) -> Self {
        Transaction{
            from: row.get("from"),
            to: row.get("to"),
            amount: row.get("amount"),
            processed_timestamp: row.get("processed_timestamp"),
            status: row.get("status"),
        }
    }
}

fn parse_rows(row: &Vec<sqlx::postgres::PgRow>) -> Option<Vec<Transaction>> {
        todo!()
}

impl Queriable for TransactionResponse {
    async fn get_query(pool: &Pool<Postgres>, claims: &jwt_util::core::JwtClaims, params: &HashMap<String, String>) -> anyhow::Result<String> {
        //TODO: Write SQL Statement
        let row = sqlx::query("SQL TODO").fetch_all(pool).await?;
        Ok(serde_json::to_string(&TransactionResponse{
            transactions: parse_rows(&row),
        })?)
    }

    async fn post_query(pool: &Pool<Postgres>, claims: &jwt_util::core::JwtClaims, params: &HashMap<String, String>) -> anyhow::Result<String> {
        //TODO: Write SQL Statement
        let row = sqlx::query("SQL TODO").fetch_all(pool).await?;
        Ok(serde_json::to_string(&TransactionResponse{
            transactions: parse_rows(&row),
        })?)
    }
}

pub async fn get_transaction(pool: &Pool<Postgres>, claims: &JwtClaims, params: &HashMap<String, String>) -> (StatusCode, String){
    match TransactionResponse::get_query(pool, claims, params).await{
        Ok(resp) => (StatusCode::OK, resp),
        Err(err) => (StatusCode::NOT_FOUND, err.to_string()),
    }
}
pub async fn post_transaction(pool: &Pool<Postgres>, claims: &JwtClaims, params: &HashMap<String, String>) -> (StatusCode, String){
    match TransactionResponse::post_query(pool, claims, params).await{
        Ok(resp) => (StatusCode::OK, resp),
        Err(err) => (StatusCode::NOT_MODIFIED, err.to_string()),
    }
}