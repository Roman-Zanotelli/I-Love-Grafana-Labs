use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::{Pool, Postgres, Row};

use crate::{ ParsableRow, ParsableRows, Queriable};

#[derive(Debug, Serialize)]
pub struct TransactionResponse{
    transactions: Option<Vec<Transaction>> //list of transactions retrieved or affected
}

#[derive(Debug, Serialize)]
pub struct Transaction{
    from: String, //who sent money
    to: String, //who recieved money
    amount: i32, //amount (in pennies)
    request_timestamp: DateTime<Utc>, //timestamp of intial request
    processed_timestamp: Option<DateTime<Utc>>, //timestamp of completion
    status: Option<bool> //final staus (true = success, false = cancelled, None = pending)
}


impl ParsableRow for Transaction {
    fn parse_row(row: &sqlx::postgres::PgRow) -> Self {
        Transaction{
            from: row.get("from"),
            to: row.get("to"),
            amount: row.get("amount"),
            request_timestamp: row.get("request_timestamp"),
            processed_timestamp: row.get("processed_timestamp"),
            status: row.get("status"),
            
        }
    }
}

impl ParsableRows for TransactionResponse {
    fn parse_rows(rows: &Vec<sqlx::postgres::PgRow>) -> Self {
        todo!()
    }
}


impl Queriable for TransactionResponse {
    async fn post_query(pool: &Pool<Postgres>, claims: &jwt_util::core::JwtClaims, params: &HashMap<String, String>) -> Result<Self, sqlx::Error> where Self: Sized {
        //TODO: Write SQL Statement
        let rows = sqlx::query("SQL TODO").fetch_all(pool).await?;
        Ok(Self::parse_rows(&rows))
    }
    
    fn generate_get_query<'a>(claims: &'a jwt_util::core::JwtClaims, params: &'a HashMap<String, String>) -> sqlx::query::Query<'a, Postgres, sqlx::postgres::PgArguments> {
        todo!()
    }
}

