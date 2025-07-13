use std::collections::HashMap;

use axum::http::StatusCode;
use jwt_util::core::JwtClaims;
use serde::Serialize;
use sqlx::{Pool, Postgres};

use anyhow::Result as AnyResult;

#[derive(Debug, Serialize)]
struct BalanceResponse{
    balance: i64, //account balance
    daily_transfer_max: u32, //daily transfer limit
    daily_transfer_used: u32, //daily transfer used
}

impl BalanceResponse{
    async fn query(pool: &Pool<Postgres>, id: &str) -> AnyResult<String>{
        todo!()
    }
}

pub async fn balance(pool: &Pool<Postgres>, claims: &JwtClaims) -> (StatusCode, String){
    match BalanceResponse::query(pool, &claims.id).await{
        Ok(resp) => (StatusCode::OK, resp),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
    }
}