mod balance;
mod contact;
mod pending;
mod transaction;

use std::{collections::HashMap, time::Duration};

use anyhow::Result as AnyResult;
use axum::{extract::{Query, State}, http::StatusCode, response::IntoResponse, routing::{get, post}};
use axum_extra::{headers::{authorization::Bearer, Authorization}, TypedHeader};
use jwt_util::decode::decode_claims;
use sqlx::{postgres::PgPoolOptions, PgPool, Pool, Postgres};

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
            .route("/pending", get(get_pending_handler)) //Get Pending Route
            .route("/pending", post(post_pending_handler)) //Post Pending Route
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
        Ok(claims) => balance::balance(&pool, &claims).await, //Run Logic
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "JWT Error".to_string()), //Proxy Pre-Auth should catch (checking again for indepth security)
    }
}

//Transaction (GET)
async fn get_transaction_handler(TypedHeader(auth): TypedHeader<Authorization<Bearer>>, State(pool): State<Pool<Postgres>>, Query(params): Query<HashMap<String, String>>) -> impl IntoResponse{
    match decode_claims(auth.token()){
        Ok(claims) => transaction::get_transaction(&pool, &claims, &params).await, //Run Logic
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "JWT Error".to_string()), //Proxy Pre-Auth should catch (checking again for indepth security)
    }
}

//Transaction (POST)
async fn post_transaction_handler(TypedHeader(auth): TypedHeader<Authorization<Bearer>>, State(pool): State<Pool<Postgres>>, Query(params): Query<HashMap<String, String>>) -> impl IntoResponse{
    match decode_claims(auth.token()){
        Ok(claims) => transaction::post_transaction(&pool, &claims, &params).await, //Run Logic
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "JWT Error".to_string()), //Proxy Pre-Auth should catch (checking again for indepth security)
    }
}

//Contact (GET)
async fn get_contact_handler(TypedHeader(auth): TypedHeader<Authorization<Bearer>>, State(pool): State<Pool<Postgres>>, Query(params): Query<HashMap<String, String>>) -> impl IntoResponse{
    match decode_claims(auth.token()){
        Ok(claims) => contact::get_contact(&pool, &claims, &params).await, //Run Logic
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "JWT Error".to_string()), //Proxy Pre-Auth should catch (checking again for indepth security)
    }
}

//Contact (POST)
async fn post_contact_handler(TypedHeader(auth): TypedHeader<Authorization<Bearer>>, State(pool): State<Pool<Postgres>>, Query(params): Query<HashMap<String, String>>) -> impl IntoResponse{
    match decode_claims(auth.token()){
        Ok(claims) => contact::post_contact(&pool, &claims, &params).await, //Run Logic
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "JWT Error".to_string()), //Proxy Pre-Auth should catch (checking again for indepth security)
    }
}

//Pending (GET)
async fn get_pending_handler(TypedHeader(auth): TypedHeader<Authorization<Bearer>>, State(pool): State<Pool<Postgres>>, Query(params): Query<HashMap<String, String>>) -> impl IntoResponse{
    match decode_claims(auth.token()){
        Ok(claims) => pending::get_pending(&pool, &claims, &params).await, //Run Logic
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "JWT Error".to_string()), //Proxy Pre-Auth should catch (checking again for indepth security)
    }
}

//Pending (POST)
async fn post_pending_handler(TypedHeader(auth): TypedHeader<Authorization<Bearer>>, State(pool): State<Pool<Postgres>>, Query(params): Query<HashMap<String, String>>) -> impl IntoResponse{
    match decode_claims(auth.token()){
        Ok(claims) => pending::post_pending(&pool, &claims, &params).await, //Run Logic
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "JWT Error".to_string()), //Proxy Pre-Auth should catch (checking again for indepth security)
    }
}


//====================================================
//?                 END OF HANDLERS
//====================================================

pub trait Queriable {
    async fn get_query(pool: &Pool<Postgres>, claims: &jwt_util::core::JwtClaims, params: &HashMap<String, String>);
    async fn post_query(pool: &Pool<Postgres>, claims: &jwt_util::core::JwtClaims, params: &HashMap<String, String>);
}
