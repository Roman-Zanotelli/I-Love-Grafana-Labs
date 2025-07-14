use std::fmt::Display;

use sqlx::{query_as, Pool, Postgres, QueryBuilder, Transaction as SQLTransaction};

use crate::{balance::select_balance_for_update, error::BankError, transaction::transaction::{BankAction, BankTransaction, BankTransactionFilter, TransactionResponse}, Queriable};

impl Queriable<BankTransactionFilter> for TransactionResponse {

    //Generate sql query for GET requests
    fn generate_get_query<'a>(claims: &'a jwt_util::core::JwtClaims, params: &'a BankTransactionFilter) -> QueryBuilder<'a, Postgres> {
        
        //BASE REQ (Associate id to either the transaction owner or contact)
        let mut qb = QueryBuilder::new("SELECT * FROM transactions WHERE (user_id = ");
        qb.push_bind(&claims.id);
        qb.push(" OR contact_id = ");
        qb.push_bind(&claims.id);
        qb.push(" )");
        //**END of BASE REQ

        //Contact param (Associate contact id to either transaction owner or contact)
        if let Some(contact_id) = &params.contact_id{
            qb.push(" AND (contact_id = ");
            qb.push_bind(contact_id);
            qb.push(" OR user_id = ");
            qb.push_bind(contact_id);
            qb.push(" )");
        }


        //transaction_id param
        if let Some(transaction_id) = &params.transaction_id{
            qb.push(" AND transaction_id = ");
            qb.push_bind(transaction_id);
        }

        //Status param
        if let Some(status) = &params.status{
            qb.push(" AND status = ");
            qb.push_bind(status);
        }

        //action param
        if let Some(action) = &params.action{
            qb.push(" AND action = ");
            qb.push_bind(action);
        }

        //less than param
        if let Some(amount) = &params.less_than{
            qb.push(" AND amount < ");
            qb.push_bind(amount);
        }


        //more than param
        if let Some(amount) = &params.more_than{
            qb.push(" AND amount > ");
            qb.push_bind(amount);
        }

        //TODO: Add more params as desired
        
        qb
    }
    
    //wrapper to handle executing the get query
    async fn get_query(pool: &Pool<Postgres>, claims: &jwt_util::core::JwtClaims, params: &BankTransactionFilter) -> Result<Self, BankError> where Self: Sized {
        Ok(TransactionResponse::new(TransactionResponse::generate_get_query(claims, params).build_query_as().fetch_all(pool).await? ))
    }


    //post query branch (calls internal logic)
    async fn post_query(pool: &Pool<Postgres>, claims: &jwt_util::core::JwtClaims, params: &BankTransactionFilter) -> Result<Self, BankError> where Self: Sized {
            //Pattern match the request to ensure values needed are present, all posts require an action and other params depending on type
            match (&params.action.ok_or(BankError::InvalidAction)?, &claims.id , &params.amount, &params.transaction_id, &params.contact_id) {
                //Confirm
                (BankAction::CONFIRM, account_id, _, Some(transaction_id), _) => Self::confirm(account_id, transaction_id, pool).await,
                //Deny
                (BankAction::DENY, account_id, _, Some(transaction_id), _) => Self::deny(account_id, transaction_id, pool).await,
                //SEND
                (BankAction::SEND, account_id, Some(amount), _, Some(contact_id)) => Self::send(account_id, contact_id, &amount, pool).await,
                //RECV
                (BankAction::RECV, account_id, Some(amount), _, Some(contact_id)) => Self::recv(account_id, contact_id, &amount, pool).await,
                //INVALID
                _ => Err(BankError::InvalidParams)
            }
        
    }
}

impl TransactionResponse{

    fn new(transactions: Vec<BankTransaction>) -> Self{
        TransactionResponse { transactions }
    }

    //Send transaction Resp
    async fn send(user_id: &str, contact_id: &str, amount: &i32, pool: &Pool<Postgres>) -> Result<Self, BankError>{
        let mut tx: SQLTransaction<'static, Postgres> = pool.begin().await?;

        //Lock User Balance
        select_balance_for_update(user_id, &mut tx).await?
            //Throw err if none
            .ok_or(BankError::NullBalance(user_id.to_owned()))?
            //Throw err if cant send amount
            .can(&BankAction::SEND, amount)?;

        //Lock Contact Balance
        select_balance_for_update(contact_id, &mut tx).await?
            //Throw err if none
            .ok_or(BankError::NullBalance(contact_id.to_owned()))?
            //Throw err if cant recv amount
            .can(&BankAction::RECV, amount)?;

        
        //Transaction logic (No commit)
        let resp = BankTransaction::send_transacion(user_id, contact_id, amount, &mut tx).await?;

        //Commit Changes
        tx.commit().await?;
        
        Ok(TransactionResponse::new(vec![resp]))
    }

    //Recv Transaction Resp
    async fn recv(user_id: &str, contact_id: &str, amount: &i32, pool: &Pool<Postgres>) -> Result<Self, BankError>{
        let mut tx: SQLTransaction<'static, Postgres> = pool.begin().await?;

        //Transaction logic (No commit)
        let resp = BankTransaction::recv_transacion(user_id, contact_id, amount, &mut tx).await?;

        //Commit Changes
        tx.commit().await?;

        Ok(TransactionResponse::new(vec![resp]))
    }


    //Confirm Transaction Resp
    async fn confirm(user_id: &str, transaction_id: &str, pool: &Pool<Postgres>) -> Result<Self, BankError>{
        let mut tx: SQLTransaction<'static, Postgres> = pool.begin().await?;
        //Select Transaction
        let resp = BankTransaction::select_for_update(user_id, transaction_id, &mut tx).await?
            //Complete Transaction
            .complete_transacion(&mut tx).await?;
        //Commit
        tx.commit().await?;
        //Return Result
        Ok(TransactionResponse::new(vec![resp]))
    }


    //Deny Transaction Resp
    async fn deny(user_id: &str, transaction_id: &str, pool: &Pool<Postgres>) -> Result<Self, BankError>{
        let mut tx: SQLTransaction<'static, Postgres> = pool.begin().await?;
        //Select Transaction
        let resp = BankTransaction::select_for_update(user_id, transaction_id, &mut tx).await?
            //Cancel Transaction
            .cancel_transacion(&mut tx).await?;
        //Commit
        tx.commit().await?;
        //Return Result
        Ok(TransactionResponse::new(vec![resp]))
    }

}


impl BankTransaction{
    async fn complete_transacion<'a>(&self,  tx: &mut SQLTransaction<'a, Postgres>) -> Result<Self, BankError>{
        todo!()
    }
    async fn cancel_transacion<'a>(&self,  tx: &mut SQLTransaction<'a, Postgres>) -> Result<Self, BankError>{
        todo!()
    }
    async fn send_transacion<'a>(user_id: &str, contact_id: &str, amount: &i32, tx: &mut SQLTransaction<'a, Postgres>) -> Result<Self, BankError>{
        todo!()
    }
    async fn recv_transacion<'a>(user_id: &str, contact_id: &str, amount: &i32, tx: &mut SQLTransaction<'a, Postgres>) -> Result<Self, BankError>{
        todo!()
    }


    //Selects Transaction for update (locking it from edits not associated with sql transaction)
    async fn select_for_update<'a>(user_id: &str, transaction_id: &str,  tx: &mut SQLTransaction<'a, Postgres>) -> Result<Self, BankError>{
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


impl Display for BankAction{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}