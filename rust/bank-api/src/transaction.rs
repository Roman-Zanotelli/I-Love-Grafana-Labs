use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::{query, query_as, Pool, Postgres, QueryBuilder, Row};

use crate::{ filter::TransactionFilter, ParsableRow, ParsableRows, Queriable};

#[derive(Debug, Serialize)]
pub struct TransactionResponse{
    transactions: Option<Vec<Transaction>> //list of transactions retrieved or affected
}

#[derive(Debug, Serialize)]
pub struct Transaction{
    from: String, //who sent money
    to: String, //who recieved money
    id: String,
    amount: i32, //amount (in pennies)
    request_timestamp: DateTime<Utc>, //timestamp of intial request
    processed_timestamp: Option<DateTime<Utc>>, //timestamp of completion
    pending: bool,
    status: Option<bool> //final staus (true = success, false = cancelled, None = pending)
}


impl ParsableRow for Transaction {
    fn parse_row(row: &sqlx::postgres::PgRow) -> Self {
        Transaction{
            from: row.get("user_id"),
            to: row.get("contact_id"),
            id: row.get("tansaction_id"),
            amount: row.get("amount"),
            request_timestamp: row.get("request_timestamp"),
            processed_timestamp: row.get("processed_timestamp"),
            status: row.get("status"),
            pending: row.get("pending"),
            
        }
    }
}

impl ParsableRows for TransactionResponse {
    fn parse_rows(rows: &Vec<sqlx::postgres::PgRow>) -> Self {
        todo!()
    }
}


impl Queriable<TransactionFilter> for TransactionResponse {
    async fn post_query(pool: &Pool<Postgres>, claims: &jwt_util::core::JwtClaims, params: &TransactionFilter) -> Result<Self, sqlx::Error> where Self: Sized {
        todo!()
    }
    
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

        if let Some(pending) = &params.include_pending{
            qb.push(" AND pending = ");
            qb.push_bind(pending);
        }

        qb
    }
}

