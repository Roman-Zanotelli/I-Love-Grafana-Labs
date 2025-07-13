mod balance;
mod contact;
mod transaction;

use std::{collections::HashMap, time::Duration};

use anyhow::Result as AnyResult;
use axum::{extract::{Query, State}, http::StatusCode, response::IntoResponse, routing::{get, post}};
use axum_extra::{headers::{authorization::Bearer, Authorization}, TypedHeader};
use jwt_util::decode::decode_claims;
use serde::Serialize;
use sqlx::{postgres::PgPoolOptions, query, PgPool, Pool, Postgres};

use crate::{balance::BalanceResponse, contact::ContactResponse, transaction::TransactionResponse};

#[tokio::main]
async fn main() -> AnyResult<()>{
    //TODO: observability
    Ok(axum::serve( tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap(), //Set Up Listener
        axum::Router::new()
            .route("/balance", get(balance_handler)) //Get Balance Route
            .route("/transaction", get(get_transaction_handler)) //Get Transactions Route
            .route("/transaction", post(post_transaction_handler)) //Post Transactions Route
            .route("/contact", get(get_contact_handler)) //Get Contact Route
            .route("/contact", post(post_contact_handler)) //Post Contact Route
            .with_state(init_pool(&std::env::var("DATABASE_URL")?).await?) //add connection pool
    ).await?)
}

//Init Postgre Pool
async fn init_pool(url: &str) -> AnyResult<PgPool> {
    //Create Pool
    let pool = PgPoolOptions::new()
        //Min Idle Connections
        .min_connections(2)
        //Idle Timeout
        .idle_timeout(Duration::from_secs(300))
        //Enable Testing Connection
        .test_before_acquire(true)
        //Connect
        .connect(url)
        .await?;
    Ok(pool)
}


//====================================================
//?                 ROUTE HANDLERS
//====================================================

//Balance
async fn balance_handler(TypedHeader(auth): TypedHeader<Authorization<Bearer>>, State(pool): State<Pool<Postgres>>) -> impl IntoResponse{
    match decode_claims(auth.token()){
        Ok(claims) => BalanceResponse::get_http_reponse(&pool, &claims).await, //Run Logic
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "JWT Error".to_string()), //Proxy Pre-Auth should catch (checking again for indepth security)
    }
}

//Transaction (GET)
async fn get_transaction_handler(TypedHeader(auth): TypedHeader<Authorization<Bearer>>, State(pool): State<Pool<Postgres>>, Query(params): Query<HashMap<String, String>>) -> impl IntoResponse{
    match decode_claims(auth.token()){
        Ok(claims) => {
            let query = sqlx::query("TODO");
            TransactionResponse::get_http_response(&pool, query).await
        }, //Run Logic
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "JWT Error".to_string()), //Proxy Pre-Auth should catch (checking again for indepth security)
    }
}

//Transaction (POST)
async fn post_transaction_handler(TypedHeader(auth): TypedHeader<Authorization<Bearer>>, State(pool): State<Pool<Postgres>>, Query(params): Query<HashMap<String, String>>) -> impl IntoResponse{
    match decode_claims(auth.token()){
        Ok(claims) => TransactionResponse::post_http_response(&pool, &claims, &params).await, //Run Logic
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "JWT Error".to_string()), //Proxy Pre-Auth should catch (checking again for indepth security)
    }
}

//Contact (GET)
async fn get_contact_handler(TypedHeader(auth): TypedHeader<Authorization<Bearer>>, State(pool): State<Pool<Postgres>>, Query(params): Query<HashMap<String, String>>) -> impl IntoResponse{
    match decode_claims(auth.token()){
        Ok(claims) => {
            let query = sqlx::query("TODO");
            ContactResponse::get_http_response(&pool, query).await
        }, //Run Logic
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "JWT Error".to_string()), //Proxy Pre-Auth should catch (checking again for indepth security)
    }
}

//Contact (POST)
async fn post_contact_handler(TypedHeader(auth): TypedHeader<Authorization<Bearer>>, State(pool): State<Pool<Postgres>>, Query(params): Query<HashMap<String, String>>) -> impl IntoResponse{
    match decode_claims(auth.token()){
        Ok(claims) => ContactResponse::post_http_response(&pool, &claims, &params).await, //Run Logic
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "JWT Error".to_string()), //Proxy Pre-Auth should catch (checking again for indepth security)
    }
}

//====================================================
//?                 END OF HANDLERS
//====================================================

pub trait Queriable {

    //POST logic is a bit too specific with transactions to have default impl
    async fn post_query(pool: &Pool<Postgres>, claims: &jwt_util::core::JwtClaims, params: &HashMap<String, String>) -> Result<Self, sqlx::Error> where Self: Sized + ParsableRows;

    //GET is just a SELECT statement
    async fn get_query(pool: &Pool<Postgres>, query: sqlx::query::Query<'_, Postgres, sqlx::postgres::PgArguments>) -> Result<Self, sqlx::Error> where Self: Sized + ParsableRows{
        Ok(Self::parse_rows(&query.fetch_all(pool).await?))
    }

    //Convert GET Query into Http Response
    async fn get_http_response(pool: &Pool<Postgres>, query: sqlx::query::Query<'_, Postgres, sqlx::postgres::PgArguments>) -> (StatusCode, String) where Self: Sized + ParsableRows + Serialize{
        match Self::get_query(pool, query).await{
            Ok(resp) => match serde_json::to_string(&resp) {
                Ok(json_resp) => (StatusCode::FOUND, json_resp),
                Err(json_err) => (StatusCode::INTERNAL_SERVER_ERROR, json_err.to_string()),
            },
            Err(query_err) => (StatusCode::NOT_FOUND, query_err.to_string()),
        }
    }

    //Convert POST Query into Http Response
    async fn post_http_response(pool: &Pool<Postgres>, claims: &jwt_util::core::JwtClaims, params: &HashMap<String, String>) -> (StatusCode, String) where Self: Sized + ParsableRows + Serialize{
        match Self::post_query(pool, claims, params).await{
            Ok(resp) => match serde_json::to_string(&resp) {
                Ok(json_resp) => (StatusCode::OK, json_resp),
                Err(json_err) => (StatusCode::INTERNAL_SERVER_ERROR, json_err.to_string()),
            },
            Err(query_err) => (StatusCode::NOT_MODIFIED, query_err.to_string()),
        }
    }

}

pub trait ParsableRow{
    fn parse_row(row: &sqlx::postgres::PgRow) -> Self;
}

pub trait ParsableRows{
    fn parse_rows(rows: &Vec<sqlx::postgres::PgRow>) -> Self;
}