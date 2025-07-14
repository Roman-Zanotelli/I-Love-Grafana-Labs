

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres, QueryBuilder};

use crate::{ filter::TransactionFilter, transaction::{action::TAction, status::TStatus}, Queriable};

#[derive(Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub struct TransactionResponse{
    pub(super) transactions: Option<Vec<Transaction>> //list of transactions retrieved or affected
}

#[derive(Debug, Serialize, Deserialize)]
#[derive(sqlx::FromRow)]
#[serde(rename_all = "lowercase")]
pub struct Transaction{
    user_id: String, //transaction owner id
    contact_id: String, //transaction contact id
    transaction_id: String, //transaction id
    action: String, //Action
    request_timestamp: DateTime<Utc>, //timestamp of intial request
    processed_timestamp: Option<DateTime<Utc>>, //timestamp of completion
    status: TStatus //Current Status
}


