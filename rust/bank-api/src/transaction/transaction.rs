

use std::fmt::Display;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::Type;




#[derive(Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub struct TransactionResponse{
    pub(super) transactions: Vec<BankTransaction> //list of transactions retrieved or affected
}

#[derive(Debug, Serialize, Deserialize)]
#[derive(sqlx::FromRow)]
#[serde(rename_all = "lowercase")]
pub struct BankTransaction{
    pub user_id: String, //transaction owner id
    pub contact_id: String, //transaction contact id
    pub transaction_id: String, //transaction id
    pub transaction_action: TAction, //Action
    pub transaction_amount: i32,
    pub request_timestamp: DateTime<Utc>, //timestamp of intial request
    pub processed_timestamp: Option<DateTime<Utc>>, //timestamp of completion
    pub status: TStatus //Current Status
}
#[derive(Debug, Serialize, Deserialize, Copy, Clone, PartialEq, Eq, Type)]
#[serde(rename_all = "lowercase")]
pub enum TAction {
    CONFIRM, //POST Only
    DENY, //POST Only
    SEND,
    RECV
}


#[derive(Debug, Serialize, Deserialize, Copy, Clone, PartialEq, Eq, Type)]
#[serde(rename_all = "lowercase")]
pub enum TStatus {
    CONFIRMED,
    PENDING,
    DENIED
}


#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub struct TransactionFilter{
    pub transaction_id: Option<String>, //filter by transaction id
    pub contact_id: Option<String>, //filter by contact id
    pub status: Option<TStatus>, // filter by pending status
    pub action: Option<TAction>, //action to preform
    pub amount: Option<i32>, //amount to send
    pub less_than: Option<i32>, //less than amount
    pub more_than: Option<i32> //more than amount
}

