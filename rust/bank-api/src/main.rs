mod balance;
mod contact;
mod transaction;
mod error;

use std::time::Duration;

use anyhow::Result as AnyResult;
use axum::{extract::{Query, State}, http::StatusCode, response::IntoResponse, routing::{get, post}, Json};
use axum_extra::{headers::{authorization::Bearer, Authorization}, TypedHeader};
use jwt_util::decode::decode_claims;
use metrics_exporter_prometheus::PrometheusBuilder;
use serde::{de::DeserializeOwned, Serialize};
use sqlx::{postgres::PgPoolOptions, PgPool, Pool, Postgres, QueryBuilder};
use tower_http::trace::TraceLayer;


use crate::{balance::BalanceResponse, contact::contact::{ContactFilter, ContactResponse}, error::BankError, transaction::transaction::{BankTransactionFilter, TransactionResponse}};

#[tokio::main]
async fn main() -> AnyResult<()>{
    //TODO: observability
    let recorder_handle = PrometheusBuilder::new()
        .install_recorder()?;
     let metrics_handle = recorder_handle.clone();


    Ok(axum::serve( tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap(), //Set Up Listener
        axum::Router::new()
            .route("/balance", get(balance_handler)) //Get Balance Route
            .route("/transaction", get(get_transaction_handler)) //Get Transactions Route
            .route("/transaction", post(post_transaction_handler)) //Post Transactions Route
            .route("/contact", get(get_contact_handler)) //Get Contact Route
            .route("/contact", post(post_contact_handler)) //Post Contact Route
            .route("/metrics", get(|| async move {Ok::<_, std::convert::Infallible>(metrics_handle.render())})) //Metrics Route for prometheus
            .with_state(init_pool(&std::env::var("DATABASE_URL")?).await?) //add connection pool
            .layer(TraceLayer::new_for_http()) //Tower trace layer for auto instrumentation
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

//todo: write a decode_claims wrapper or something that consumes a closure to execute the logic without needing to rewrite all the match cases (could do 2 closurs response on OK and response on Err that handles error pattern matching once)

//Balance
async fn balance_handler(TypedHeader(auth): TypedHeader<Authorization<Bearer>>, State(pool): State<Pool<Postgres>>) -> impl IntoResponse{
    match decode_claims(auth.token()){
        Ok(claims) => BalanceResponse::get_http_reponse(&pool, &claims).await, //Run Logic
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "JWT Error".to_string()), //Proxy Pre-Auth should catch (checking again for indepth security)
    }
}

//Transaction (GET)
async fn get_transaction_handler(TypedHeader(auth): TypedHeader<Authorization<Bearer>>, State(pool): State<Pool<Postgres>>, Query(params): Query<BankTransactionFilter>) -> impl IntoResponse{
    match decode_claims(auth.token()){
        Ok(claims) => TransactionResponse::get_http_response(&pool, &claims, &params).await, //Run Logic
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "JWT Error".to_string()), //Proxy Pre-Auth should catch (checking again for indepth security)
    }
}

//Transaction (POST)
async fn post_transaction_handler(TypedHeader(auth): TypedHeader<Authorization<Bearer>>, State(pool): State<Pool<Postgres>>, Json(params): Json<BankTransactionFilter>) -> impl IntoResponse{
    match decode_claims(auth.token()){
        Ok(claims) => TransactionResponse::post_http_response(&pool, &claims, &params).await, //Run Logic
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "JWT Error".to_string()), //Proxy Pre-Auth should catch (checking again for indepth security)
    }
}

//Contact (GET)
async fn get_contact_handler(TypedHeader(auth): TypedHeader<Authorization<Bearer>>, State(pool): State<Pool<Postgres>>, Query(params): Query<ContactFilter>) -> impl IntoResponse{
    match decode_claims(auth.token()){
        Ok(claims) => ContactResponse::get_http_response(&pool, &claims, &params).await, //Run Logic
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "JWT Error".to_string()), //Proxy Pre-Auth should catch (checking again for indepth security)
    }
}

//Contact (POST)
async fn post_contact_handler(TypedHeader(auth): TypedHeader<Authorization<Bearer>>, State(pool): State<Pool<Postgres>>, Json(params): Json<ContactFilter>) -> impl IntoResponse{
    match decode_claims(auth.token()){
        Ok(claims) => ContactResponse::post_http_response(&pool, &claims, &params).await, //Run Logic
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "JWT Error".to_string()), //Proxy Pre-Auth should catch (checking again for indepth security)
    }
}



//====================================================
//?                 END OF HANDLERS
//====================================================

//====================================================
//?                    Traits
//====================================================

pub trait Queriable<Filter> where  Filter: DeserializeOwned{

    //Impl
    async fn post_query(pool: &Pool<Postgres>, claims: &jwt_util::core::JwtClaims, params: &Filter) -> Result<Self, BankError> where Self: Sized;
    fn generate_get_query<'a>(claims: &'a jwt_util::core::JwtClaims, params: &'a Filter) -> QueryBuilder<'a, Postgres>;
    async fn get_query(pool: &Pool<Postgres>, claims: &jwt_util::core::JwtClaims, params: &Filter) -> Result<Self, BankError> where Self: Sized;

    //Default
    //Convert GET Query into Http Response
    async fn get_http_response(pool: &Pool<Postgres>, claims: &jwt_util::core::JwtClaims, params: &Filter) -> (StatusCode, String) where Self: Sized + Serialize {
        
        //Calls get query impl
        match Self::get_query(pool, &claims, &params).await{
            
            //If Ok convert to json
            Ok(resp) => match serde_json::to_string(&resp) {
                //respond
                Ok(json_resp) => (StatusCode::FOUND, json_resp),
                //json failed to parse
                Err(json_err) => (StatusCode::INTERNAL_SERVER_ERROR, json_err.to_string()),
            },

            //if err
            //TODO: Add more granular error response/tracking
            Err(query_err) => (StatusCode::NOT_FOUND, query_err.to_string()),
        }

    }

    //Convert POST Query into Http Response
    async fn post_http_response(pool: &Pool<Postgres>, claims: &jwt_util::core::JwtClaims, params: &Filter) -> (StatusCode, String) where Self: Sized  + Serialize{

        //Calls post query impl
        match Self::post_query(pool, claims, params).await{

            //If Ok convert to json
            Ok(resp) => match serde_json::to_string(&resp) {
                //respond
                Ok(json_resp) => (StatusCode::OK, json_resp),
                //json failed to parse
                Err(json_err) => (StatusCode::INTERNAL_SERVER_ERROR, json_err.to_string()),
            },

            //if err
            //TODO: Add more granular error response/tracking
            Err(query_err) => (StatusCode::NOT_MODIFIED, query_err.to_string()),
        }

    }

}


//====================================================
//?                 END OF Traits
//====================================================