use thiserror::Error;
use sqlx::Error as SqlxError;
use serde_json::Error as JsonError;

use crate::transaction::transaction::BankAction;


#[derive(Debug, Error)]
pub enum BankError {
    #[error("database error: {0}")]
    Sqlx(#[from] SqlxError),

    #[error("invalid json: {0}")]
    Json(#[from] JsonError),

    #[error("invalid {action} transacion, reason: {reason}")]
    InvalidTransaction{
        reason: String,
        action: BankAction
    },

    #[error("balance for id {0} does not exist")]
    NullBalance(String),
    
    #[error("could not select")]
    InvalidBankTransactionUpdateSelection,

    #[error("one or more missing/invalid params")]
    InvalidParams,

    #[error("missing/invalid action")]
    InvalidAction,

    #[error("transaction not pending")]
    NotPending,

    #[error("transaction could not be created")]
    TransactionCreateFailed,

}