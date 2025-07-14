use sqlx::{query_as, Pool, Postgres};
use sqlx::Executor;

use crate::transaction;
use crate::transaction::transaction::{Transaction, TransactionResponse};
const SELECT_FOR_UPDATE: &str = r#"
    SELECT * FROM  transactions
    WHERE user_id = $1 AND transaction_id = $2
    FOR UPDATE
"#;

const UPDATE_STATUS: &str = r#"
    UPDATE transactions
    SET status = $3, processed_timestamp = now()
    WHERE user_id = $1 AND transaction_id = $2
    RETURNING user_id, contact_id, transaction_id, action, request_timestamp, processed_timestamp, status
"#;
//POST SQL Logic
pub(super) async fn confirm(user_id: &str, transaction_id: &str, pool: &Pool<Postgres>) -> Result<TransactionResponse, sqlx::Error>{
    let mut tx: sqlx::Transaction<'static, Postgres> = pool.begin().await?;
    match select_transaction_for_update(user_id, transaction_id, &mut tx).await?{
        Some(transaction) => {

            todo!()
        },
        None => {Ok(TransactionResponse::default())},
    }
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

async fn select_transaction_for_update<'a>(user_id: &str, transaction_id: &str,  tx: &mut sqlx::Transaction<'a, Postgres>) -> Result<Option<Transaction>, sqlx::Error>{
    query_as::<_, Transaction>(r#"
        SELECT * FROM  transactions
        WHERE contact_id = $1 AND transaction_id = $2
        FOR UPDATE
    "#).bind(user_id).bind(transaction_id).fetch_optional(tx.as_mut()).await
}