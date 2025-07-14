

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{ transaction::{action::TAction, status::TStatus}};

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


#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub struct TransactionFilter{
    pub transaction_id: Option<String>, //filter by transaction id
    pub contact_id: Option<String>, //filter by contact id
    pub status: Option<TStatus>, // filter by pending status
    pub action: Option<TAction>, //action to preform (POST)
    pub less_than: Option<i32>, //less than amount
    pub more_than: Option<i32> //more than amount
}

