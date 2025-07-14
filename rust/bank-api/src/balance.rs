use axum::http::StatusCode;
use jwt_util::core::JwtClaims;
use serde::Serialize;
use sqlx::{Error as SQLxError, Pool, Postgres};

#[derive(Debug, Serialize)]
#[derive(sqlx::FromRow)]
#[serde(rename_all = "lowercase")]
pub struct BalanceResponse{
    balance: i64, //account balance
    daily_send_limit: i32,
    daily_send_used: i32,
    daily_recieve_limit: i32,
    daily_recieve_used: i32,
}

impl BalanceResponse{
    async fn query(pool: &Pool<Postgres>, id: &str) -> Result<Self, SQLxError>{
        Ok(sqlx::query_as("SELECT * FROM balances WHERE user_id = $1").bind(id).fetch_one(pool).await?)
    }

    pub async fn get_http_reponse(pool: &Pool<Postgres>, claims: &JwtClaims) -> (StatusCode, String){
        match BalanceResponse::query(pool, &claims.id).await{
            Ok(resp) => match serde_json::to_string(&resp) {
                Ok(json_resp) => (StatusCode::OK, json_resp),
                Err(json_err) => (StatusCode::INTERNAL_SERVER_ERROR, json_err.to_string()),
            },
            Err(query_err) => (StatusCode::NOT_FOUND, query_err.to_string()),
        }
    }
}

