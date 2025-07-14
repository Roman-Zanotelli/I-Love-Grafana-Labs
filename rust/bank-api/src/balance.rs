use axum::http::StatusCode;
use jwt_util::core::JwtClaims;
use serde::Serialize;
use sqlx::{query_as, Executor, Pool, Postgres};

use crate::{error::BankError, transaction::transaction::BankAction};

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
    async fn query<'e, E>(exec: E, id: &str) -> Result<Self, BankError> where E: Executor<'e, Database = Postgres>,{
        Ok(sqlx::query_as("SELECT * FROM balances WHERE user_id = $1").bind(id).fetch_one(exec).await?)
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

    pub fn can(&self, action: &BankAction, amount: &i32) -> Result<(), BankError>{
        match action {
            BankAction::SEND => todo!(),
            BankAction::RECV => todo!(),
            _ => todo!()
        }
    }
}


pub(super) async fn select_balance_for_update<'a>(user_id: &str,  tx: &mut sqlx::Transaction<'a, Postgres>) -> Result<Option<BalanceResponse>, BankError>{
    Ok(query_as::<_, BalanceResponse>(r#"
        SELECT * FROM balances
        WHERE user_id = $1
        FOR UPDATE
    "#).bind(user_id).fetch_optional(tx.as_mut()).await?)
}
