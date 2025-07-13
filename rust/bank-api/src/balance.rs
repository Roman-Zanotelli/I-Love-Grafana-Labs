use axum::http::StatusCode;
use jwt_util::core::JwtClaims;
use serde::Serialize;
use sqlx::{postgres::PgRow, Error as SQLxError, Pool, Postgres, Row};

#[derive(Debug, Serialize)]
pub struct BalanceResponse{
    balance: i64, //account balance
    daily_send_limit: i32,
    daily_send_used: i32,
    daily_recieve_limit: i32,
    daily_recieve_used: i32,
}

impl BalanceResponse{
    async fn query(pool: &Pool<Postgres>, id: &str) -> Result<Self, SQLxError>{
        
        Ok(BalanceResponse::parse(sqlx::query("SELECT * FROM balances WHERE user_id = $1").bind(id).fetch_one(pool).await?))
    }

    fn parse(row: PgRow) -> Self{
        BalanceResponse{
            balance: row.get("balance"),
            daily_send_limit: row.get("daily_send_limit"),
            daily_send_used: row.get("daily_send_used"),
            daily_recieve_limit: row.get("daily_recieve_limit"),
            daily_recieve_used: row.get("daily_recieve_used"),
        }
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

