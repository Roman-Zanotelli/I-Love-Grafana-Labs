use sqlx::{Pool, Postgres};

use crate::transaction::transaction::TransactionResponse;

//POST SQL Logic
pub(super) async fn confirm(account_id: &String, transaction_id: &String, pool: &Pool<Postgres>) -> Result<TransactionResponse, sqlx::Error>{
    todo!()
}
pub(super) async fn deny(account_id: &String, transaction_id: &String, pool: &Pool<Postgres>) -> Result<TransactionResponse, sqlx::Error>{
    todo!()
}
pub(super) async fn send(account_id: &String, contact_id: &String, amount: &i32, pool: &Pool<Postgres>) -> Result<TransactionResponse, sqlx::Error>{
    todo!()
}
pub(super) async fn recv(account_id: &String, contact_id: &String, amount: &i32, pool: &Pool<Postgres>) -> Result<TransactionResponse, sqlx::Error>{
    todo!()
}