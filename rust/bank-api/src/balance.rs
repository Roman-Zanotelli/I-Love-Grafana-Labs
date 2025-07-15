use std::ops::Neg;

use axum::http::StatusCode;
use jwt_util::core::JwtClaims;
use serde::Serialize;
use sqlx::{query, query_as, Executor, Pool, Postgres};
use uuid::Uuid;

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


pub(super) async fn select_balance_for_update<'a>(user_id: &Uuid,  tx: &mut sqlx::Transaction<'a, Postgres>) -> Result<Option<BalanceResponse>, BankError>{
    Ok(query_as::<_, BalanceResponse>(r#"
        SELECT * FROM balances
        WHERE user_id = $1
        FOR UPDATE
    "#).bind(user_id).fetch_optional(tx.as_mut()).await?)
}

pub(super) async fn transfer_balance<'a>(from: &Uuid, to: &Uuid, amount: &i32, tx:  &mut sqlx::Transaction<'a, Postgres>) -> Result<(), BankError>{
    //Lock User Balance
        select_balance_for_update(from, tx).await?
            //Throw err if none
            .ok_or(BankError::NullBalance(from.to_string()))?
            //Throw err if cant send amount
            .can(&BankAction::SEND, amount)?;

        //Lock Contact Balance
        select_balance_for_update(to, tx).await?
            //Throw err if none
            .ok_or(BankError::NullBalance(to.to_string()))?
            //Throw err if cant recv amount
            .can(&BankAction::RECV, amount)?;
        
    change_balance(from, &(amount.abs().neg()), tx).await?;
    change_balance(to, &(amount.abs()), tx).await
}

pub(super) async fn change_balance<'a>(user_id: &Uuid, amount: &i32,  tx: &mut sqlx::Transaction<'a, Postgres>) -> Result<(), BankError>{
    match amount {
        ..0 => {
            query::<Postgres>(r#"
            UPDATE balances
            SET
                balance = balance + $1,
                daily_send_used = daily_send_used + $2,
            WHERE account_id = $3
            "#).bind(amount).bind(amount.abs()).bind(user_id).execute(tx.as_mut()).await?;
        },
        0.. => {
            query::<Postgres>(r#"
            UPDATE balances
            SET
                balance = balance + $1,
                daily_recieve_used = daily_recieve_used + $1,
            WHERE account_id = $2
            "#).bind(amount).bind(user_id).execute(tx.as_mut()).await?;
        }
    };
    Ok(())
}