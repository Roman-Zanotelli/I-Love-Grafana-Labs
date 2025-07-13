use std::collections::HashMap;

use axum::http::StatusCode;
use jwt_util::core::JwtClaims;
use serde::Serialize;
use sqlx::{postgres::PgRow, Pool, Postgres, Row};

use anyhow::Result as AnyResult;

#[derive(Debug, Serialize)]
struct BalanceResponse{
    balance: i64, //account balance
    daily_transfer_max: i32, //daily transfer limit
    daily_transfer_used: i32, //daily transfer used
}

impl BalanceResponse{
    async fn query(pool: &Pool<Postgres>, id: &str) -> AnyResult<String>{
        BalanceResponse::parse(sqlx::query("SELECT * FROM balances WHERE id = $1").bind(id).fetch_one(pool).await?) 
    }
    
    fn parse(row: PgRow) -> AnyResult<String>{
        Ok(serde_json::to_string(&BalanceResponse{
            balance: row.get("balance"),
            daily_transfer_max: row.get("daily_transfer_max"),
            daily_transfer_used: row.get("daily_transfer_used"),
        })?)
    }
}

pub async fn balance(pool: &Pool<Postgres>, claims: &JwtClaims) -> (StatusCode, String){
    match BalanceResponse::query(pool, &claims.id).await{
        Ok(resp) => (StatusCode::OK, resp),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
    }
}