
use std::borrow::Borrow;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres, QueryBuilder, Type};

use crate::{ filter::TransactionFilter, Queriable};

#[derive(Debug, Serialize)]
pub struct TransactionResponse{
    transactions: Option<Vec<Transaction>> //list of transactions retrieved or affected
}

#[derive(Debug, Serialize, Deserialize)]
#[derive(sqlx::FromRow)]
pub struct Transaction{
    user_id: String, //transaction owner id
    contact_id: String, //transaction contact id
    transaction_id: String, //transaction id
    action: String, //Action
    request_timestamp: DateTime<Utc>, //timestamp of intial request
    processed_timestamp: Option<DateTime<Utc>>, //timestamp of completion
    status: TStatus //Current Status
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone, PartialEq, Eq, Type)]
#[serde(rename_all = "lowercase")]
enum TStatus {
    CONFIRMED,
    PENDING,
    DENIED
}

#[derive(Serialize, Deserialize, Debug)]

#[serde(tag = "action", content = "value")]
pub enum TAction {
    CONFIRM, //POST Only
    DENY, //POST Only
    SEND(#[serde(deserialize_with = "greater_than_zero")] i32),
    RECV(#[serde(deserialize_with = "greater_than_zero")] i32),
}

fn greater_than_zero<'de, D>(deser: D) -> Result<i32, D::Error> where D: serde::Deserializer<'de> {
    let v = i32::deserialize(deser)?;
    if v <= 0 {
        Err(serde::de::Error::custom("amount must be > 0"))
    } else {
        Ok(v)
    }
}


impl Default for TransactionResponse{
    fn default() -> Self {
        Self { transactions: None }
    }
}

impl Queriable<TransactionFilter> for TransactionResponse {
    async fn post_query(pool: &Pool<Postgres>, claims: &jwt_util::core::JwtClaims, params: &TransactionFilter) -> Result<Self, sqlx::Error> where Self: Sized {
        if let (Some(action), account_id) = (&params.action, &claims.id){
            match (action, &params.transaction, &params.contact) {
                (TAction::CONFIRM, Some(transaction_id), _) => confirm(account_id, transaction_id, pool).await,
                (TAction::DENY, Some(transaction_id), _) => deny(account_id, transaction_id, pool).await,
                (TAction::SEND(amount), _, Some(contact_id)) => send(account_id, contact_id, amount, pool).await,
                (TAction::RECV(amount), _, Some(contact_id)) => recv(account_id, contact_id, amount, pool).await,
                _ => Ok(TransactionResponse::default())
            }
        }else{
            Ok(TransactionResponse::default())
        }
    }
    
    fn generate_get_query<'a>(claims: &'a jwt_util::core::JwtClaims, params: &'a TransactionFilter) -> QueryBuilder<'a, Postgres> {
        let mut qb = QueryBuilder::new("SELECT * FROM transactions WHERE (user_id = ");
        qb.push_bind(&claims.id);
        qb.push(" OR contact_id = ");
        qb.push_bind(&claims.id);
        qb.push(" )");

        if let Some(contact_id) = &params.contact{
            qb.push(" AND (contact_id = ");
            qb.push_bind(contact_id);
            qb.push(" OR user_id = ");
            qb.push_bind(contact_id);
            qb.push(" )");
        }

        if let Some(transaction_id) = &params.transaction{
            qb.push(" AND transaction_id = ");
            qb.push_bind(transaction_id);
        }

        if let Some(pending) = &params.pending{
            qb.push(" AND pending = ");
            qb.push_bind(pending);
        }

        qb
    }
    
    async fn get_query(pool: &Pool<Postgres>, claims: &jwt_util::core::JwtClaims, params: &TransactionFilter) -> Result<Self, sqlx::Error> where Self: Sized {
        Ok(TransactionResponse { transactions: Some(TransactionResponse::generate_get_query(claims, params).build_query_as().fetch_all(pool).await?) })
    }
}

//POST SQL Logic
async fn confirm(account_id: &String, transaction_id: &String, pool: &Pool<Postgres>) -> Result<TransactionResponse, sqlx::Error>{
    todo!()
}
async fn deny(account_id: &String, transaction_id: &String, pool: &Pool<Postgres>) -> Result<TransactionResponse, sqlx::Error>{
    todo!()
}
async fn send(account_id: &String, contact_id: &String, amount: &i32, pool: &Pool<Postgres>) -> Result<TransactionResponse, sqlx::Error>{
    todo!()
}
async fn recv(account_id: &String, contact_id: &String, amount: &i32, pool: &Pool<Postgres>) -> Result<TransactionResponse, sqlx::Error>{
    todo!()
}