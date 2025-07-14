use sqlx::{Pool, Postgres, QueryBuilder};

use crate::{filter::TransactionFilter, transaction::{action::TAction, post::*, transaction::TransactionResponse}, Queriable};

impl Default for TransactionResponse{
    fn default() -> Self {
        Self { transactions: None }
    }
}

impl Queriable<TransactionFilter> for TransactionResponse {
    async fn post_query(pool: &Pool<Postgres>, claims: &jwt_util::core::JwtClaims, params: &TransactionFilter) -> Result<Self, sqlx::Error> where Self: Sized {
        if let (Some(action), account_id) = (&params.action, &claims.id){
            match (action, &params.transaction_id, &params.contact_id) {
                (TAction::CONFIRM, Some(transaction_id), _) => confirm(account_id, transaction_id, pool).await,
                (TAction::DENY, Some(transaction_id), _) => deny(account_id, transaction_id, pool).await,
                (TAction::SEND(amount), _, Some(contact_id)) => send(account_id, contact_id, &amount, pool).await,
                (TAction::RECV(amount), _, Some(contact_id)) => recv(account_id, contact_id, &amount, pool).await,
                _ => Ok(TransactionResponse::default())
            }
        }else{
            Ok(TransactionResponse::default())
        }
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

        if let Some(status) = &params.status{
            qb.push(" AND status = ");
            qb.push_bind(status);
        }

        if let Some(action) = &params.action{
            qb.push(" AND action = ");
            qb.push_bind(action.as_str());
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
    
    async fn get_query(pool: &Pool<Postgres>, claims: &jwt_util::core::JwtClaims, params: &TransactionFilter) -> Result<Self, sqlx::Error> where Self: Sized {
        Ok(TransactionResponse { transactions: Some(TransactionResponse::generate_get_query(claims, params).build_query_as().fetch_all(pool).await?) })
    }
    
    async fn get_http_response(pool: &sqlx::Pool<sqlx::Postgres>, claims: &jwt_util::core::JwtClaims, params: &TransactionFilter) -> (axum::http::StatusCode, String) where Self: Sized + serde::Serialize {
        match Self::get_query(pool, &claims, &params).await{
            Ok(resp) => match serde_json::to_string(&resp) {
                Ok(json_resp) => (axum::http::StatusCode::FOUND, json_resp),
                Err(json_err) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, json_err.to_string()),
            },
            Err(query_err) => (axum::http::StatusCode::NOT_FOUND, query_err.to_string()),
        }
    }
    
    async fn post_http_response(pool: &sqlx::Pool<sqlx::Postgres>, claims: &jwt_util::core::JwtClaims, params: &TransactionFilter) -> (axum::http::StatusCode, String) where Self: Sized  + serde::Serialize{
        match Self::post_query(pool, claims, params).await{
            Ok(resp) => match serde_json::to_string(&resp) {
                Ok(json_resp) => (axum::http::StatusCode::OK, json_resp),
                Err(json_err) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, json_err.to_string()),
            },
            Err(query_err) => (axum::http::StatusCode::NOT_MODIFIED, query_err.to_string()),
        }
    }
}