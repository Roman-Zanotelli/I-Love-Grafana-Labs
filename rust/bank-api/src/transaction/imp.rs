use std::fmt::Display;

use sqlx::{query_as, Pool, Postgres, QueryBuilder};

use crate::{balance::select_balance_for_update, error::BankError, transaction::transaction::{BankTransaction, TAction, TransactionFilter, TransactionResponse}, Queriable};

impl Queriable<TransactionFilter> for TransactionResponse {
    fn generate_get_query<'a>(claims: &'a jwt_util::core::JwtClaims, params: &'a TransactionFilter) -> QueryBuilder<'a, Postgres> {
        let mut qb = QueryBuilder::new("SELECT * FROM transactions WHERE (user_id = ");
        qb.push_bind(&claims.id);
        qb.push(" OR contact_id = ");
        qb.push_bind(&claims.id);
        qb.push(" )");

        if let Some(contact_id) = &params.contact_id{
            qb.push(" AND (contact_id = ");
            qb.push_bind(contact_id);
            qb.push(" OR user_id = ");
            qb.push_bind(contact_id);
            qb.push(" )");
        }

        if let Some(transaction_id) = &params.transaction_id{
            qb.push(" AND transaction_id = ");
            qb.push_bind(transaction_id);
        }

        if let Some(status) = &params.status{
            qb.push(" AND status = ");
            qb.push_bind(status);
        }

        if let Some(action) = &params.action{
            qb.push(" AND action = ");
            qb.push_bind(action);
        }

        if let Some(amount) = &params.less_than{
            qb.push(" AND amount < ");
            qb.push_bind(amount);
        }

        if let Some(amount) = &params.more_than{
            qb.push(" AND amount > ");
            qb.push_bind(amount);
        }
        
        qb
    }
    
    async fn get_query(pool: &Pool<Postgres>, claims: &jwt_util::core::JwtClaims, params: &TransactionFilter) -> Result<Self, BankError> where Self: Sized {
        Ok(TransactionResponse { transactions: TransactionResponse::generate_get_query(claims, params).build_query_as().fetch_all(pool).await? })
    }

    async fn post_query(pool: &Pool<Postgres>, claims: &jwt_util::core::JwtClaims, params: &TransactionFilter) -> Result<Self, BankError> where Self: Sized {
            match (&claims.id, &params.action , &params.amount, &params.transaction_id, &params.contact_id) {
                (account_id, Some(TAction::CONFIRM), _, Some(transaction_id), _) => Self::confirm(account_id, transaction_id, pool).await,
                (account_id, Some(TAction::DENY), _, Some(transaction_id), _) => Self::deny(account_id, transaction_id, pool).await,
                (account_id, Some(TAction::SEND), Some(amount), _, Some(contact_id)) => Self::send(account_id, contact_id, &amount, pool).await,
                (account_id, Some(TAction::RECV), Some(amount), _, Some(contact_id)) => Self::recv(account_id, contact_id, &amount, pool).await,
                _ => Err(BankError::InvalidParams)
            }
        
    }
}

impl TransactionResponse{
    async fn send(user_id: &str, contact_id: &str, amount: &i32, pool: &Pool<Postgres>) -> Result<Self, BankError>{
        let mut tx: sqlx::Transaction<'static, Postgres> = pool.begin().await?;

        //Lock User Balance
        select_balance_for_update(user_id, &mut tx).await?
            //Throw err if none
            .ok_or(BankError::NullBalance(user_id.to_owned()))?
            //Throw err if cant send amount
            .can(&TAction::SEND, amount)?;

        //Lock Contact Balance
        select_balance_for_update(contact_id, &mut tx).await?
            //Throw err if none
            .ok_or(BankError::NullBalance(contact_id.to_owned()))?
            //Throw err if cant recv amount
            .can(&TAction::RECV, amount)?;

        //TODO: Transfer if both succeed
        let resp = todo!();


        //Commit Changes
        tx.commit().await?;
        resp
    }

    async fn recv(user_id: &str, contact_id: &str, amount: &i32, pool: &Pool<Postgres>) -> Result<Self, BankError>{
        let mut tx: sqlx::Transaction<'static, Postgres> = pool.begin().await?;
        //TODO: generate a pending request transaction
        let resp = todo!();
        
        //Commit Changes
        tx.commit().await?;
        resp
    }

    async fn confirm(user_id: &str, transaction_id: &str, pool: &Pool<Postgres>) -> Result<Self, BankError>{
        let mut tx: sqlx::Transaction<'static, Postgres> = pool.begin().await?;
        //Select Transaction
        let resp = BankTransaction::select_for_update(user_id, transaction_id, &mut tx).await?
            //Complete Transaction
            .complete_transacion(&mut tx).await;
        //Commit
        tx.commit().await?;
        //Return Result
        resp
    }

    async fn deny(user_id: &str, transaction_id: &str, pool: &Pool<Postgres>) -> Result<Self, BankError>{
        let mut tx: sqlx::Transaction<'static, Postgres> = pool.begin().await?;
        //Select Transaction
        let resp = BankTransaction::select_for_update(user_id, transaction_id, &mut tx).await?
            //Cancel Transaction
            .cancel_transacion(&mut tx).await;
        //Commit
        tx.commit().await?;
        //Return Result
        resp
    }

}


impl BankTransaction{
    async fn complete_transacion<'a>(&self,  tx: &mut sqlx::Transaction<'a, Postgres>) -> Result<TransactionResponse, BankError>{
        todo!()
    }
    async fn cancel_transacion<'a>(&self,  tx: &mut sqlx::Transaction<'a, Postgres>) -> Result<TransactionResponse, BankError>{
        todo!()
    }
    async fn select_for_update<'a>(user_id: &str, transaction_id: &str,  tx: &mut sqlx::Transaction<'a, Postgres>) -> Result<Self, BankError>{
    //Setup Query
    query_as::<_, Self>(
        r#"
        SELECT * FROM  transactions
        WHERE contact_id = $1 AND transaction_id = $2
        FOR UPDATE 
        "#
    )
    //Bind Values
    .bind(user_id).bind(transaction_id)
    //Execute returning optional BankTransaction
    .fetch_optional(tx.as_mut()).await?
    //If it doesnt exist return custom error
    .ok_or(BankError::InvalidBankTransactionUpdateSelection)
    }

    
}


impl Display for TAction{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}