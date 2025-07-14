use sqlx::{Pool, Postgres};

use crate::transaction::transaction::TransactionResponse;

//POST SQL Logic
pub(super) async fn confirm(account_id: &str, transaction_id: &str, pool: &Pool<Postgres>) -> Result<TransactionResponse, sqlx::Error>{
    todo!()
}
pub(super) async fn deny(account_id: &str, transaction_id: &str, pool: &Pool<Postgres>) -> Result<TransactionResponse, sqlx::Error>{
    todo!()
}
pub(super) async fn send(account_id: &str, contact_id: &str, amount: &i32, pool: &Pool<Postgres>) -> Result<TransactionResponse, sqlx::Error>{
    todo!()
}
pub(super) async fn recv(account_id: &str, contact_id: &str, amount: &i32, pool: &Pool<Postgres>) -> Result<TransactionResponse, sqlx::Error>{
    todo!()
}