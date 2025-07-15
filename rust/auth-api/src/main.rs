use std::time::Duration;

use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::{get, post}, Json};
use metrics_exporter_prometheus::PrometheusBuilder;
use serde::Deserialize;

use anyhow::Result as AnyResult;
use thiserror::Error;

use jwt_util::encode::encode_claims;

use sqlx::{postgres::PgPoolOptions, Error as SqlxError, PgPool, Pool, Postgres, Row};

use argon2::{Argon2, PasswordHasher, PasswordVerifier};
use tower_http::trace::TraceLayer;
use uuid::Uuid;


#[tokio::main]
async fn main() -> AnyResult<()>{

    let tracking_guard = tracking_util::TrackingGuard::init_from_env()?;
    Ok(axum::serve( tokio::net::TcpListener::bind("0.0.0.0:80").await.unwrap(), //Set Up Listener
        axum::Router::new()
            .route("/sign_in", post(sign_in)) //Create sign in route
            .route("/sign_up", post(sign_up)) //Create sign up route
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

//Auth Errors
#[derive(Debug, Error)]
enum AuthError {
    #[error("Invalid Auth")]
    InvalidAuth,
    #[error("Invalid Pass Conf")]
    InvalidPassConf,
    #[error("Email/Username Taken")]
    EmailUserTaken,
    #[error("Database Error Occured!")]
    Db(#[from] SqlxError),
    #[error("Password Hashing Error Occured")]
    PasswordHashError,
}

impl From<argon2::password_hash::Error> for AuthError {
    fn from(_: argon2::password_hash::Error) -> Self {
        AuthError::PasswordHashError
    }
}



trait Req {
    async fn validate(self, pool: Pool<Postgres>) -> impl IntoResponse;
}


//====================================================
//?             START OF SIGN IN 
//====================================================


//Request Struct
#[derive(Debug, Deserialize)]
struct SignInRequest{
    email: String, //email
    pass: String //clear-text pass
}


//Request Logic
impl Req for SignInRequest{
    async fn validate(self, pool: Pool<Postgres>) -> impl IntoResponse{
        //Validate inside postgre
        match self.validate_db(&pool).await {
            //If valid encode claim
            Ok(account_id) => match encode_claims(account_id){
                //Return valid JWT
                Ok(jwt) => (StatusCode::OK, jwt),
                //If JWT Err
                Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
            },
            //If Invalid
            Err(AuthError::InvalidAuth) => (StatusCode::UNAUTHORIZED, AuthError::InvalidAuth.to_string()),
            //If Internal Err
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        };   
    }
}

impl SignInRequest{
    async fn validate_db(&self, pool: &Pool<Postgres>) -> Result<Uuid, AuthError>{
        //Prepare Statement
        let row = sqlx::query("SELECT * FROM auth WHERE email = $1")
            //Bind email
            .bind(&self.email)
            //Fetch first
            .fetch_one(pool).await?;

        //Verify Hash
        match Argon2::default().verify_password(self.pass.as_bytes(), &argon2::PasswordHash::new(row.try_get("hash")?)?){
            //If Valid Return Account Id
            Ok(_) => Ok(row.try_get("account_id")?),
            //Else Return Err
            Err(_) => Err(AuthError::InvalidAuth),
        }  
    }
}

//Route Handler
async fn sign_in(State(pool): State<Pool<Postgres>>,Json(req): Json<SignInRequest>) -> impl IntoResponse{
    req.validate(pool).await
}


//====================================================
//?                 END OF SIGN IN
//====================================================


//====================================================
//?                START OF SIGN UP
//====================================================


//Request Struct
#[derive(Debug, Deserialize)]
struct SignUpRequest{
    username: String,
    email: String,
    pass: String,
    conf: String //Must match pass
}


//Request Logic
impl Req for SignUpRequest{
    async fn validate(self, pool: Pool<Postgres>) -> impl IntoResponse{
        //Check to make sure conf and pass match
        if self.conf != self.pass {
            return (StatusCode::BAD_REQUEST, AuthError::InvalidPassConf.to_string());
        }

        //Hash password
        if let Ok(password_hash) = Argon2::default().hash_password(self.pass.as_bytes(), &argon2::password_hash::SaltString::generate(&mut argon2::password_hash::rand_core::OsRng)){
            //Prepare SQL
            match sqlx::query("INSERT INTO auth (email, username, password_hash, account_id) VALUES ($1, $2, $3, $4) ON CONFLICT (email, username) DO NOTHING")
                //Bind Email
                .bind(self.email)
                //Bind Username
                .bind(self.username)
                //Bind Hased Password
                .bind(password_hash.to_string())
                //Bind New User ID
                .bind(Uuid::new_v4())
                //Execute SQL
                .execute(&pool).await{
                    //If Success, Check Rows Affected
                    Ok(res) => match res.rows_affected() {
                        //On Username/Email Conflict
                        0 => (StatusCode::CONFLICT, AuthError::EmailUserTaken.to_string()),
                        //On Successful Creation
                        _ => (StatusCode::CREATED, "Account Created".to_string())
                    },
                    //If SQL Err
                    Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
                }  
        //If Hash Err
        }else{
            (StatusCode::INTERNAL_SERVER_ERROR, AuthError::PasswordHashError.to_string())
        }
    }
}

//Route Handler
async fn sign_up(State(pool): State<Pool<Postgres>>, Json(req): Json<SignUpRequest>) -> impl IntoResponse{
    req.validate(pool).await
}


//====================================================
//?                 END OF SIGN UP
//====================================================