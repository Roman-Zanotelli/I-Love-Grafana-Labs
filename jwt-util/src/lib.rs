use core::fmt;
use std::sync::OnceLock;

use chrono::Utc;
use jsonwebtoken::{decode, encode, errors::Error as JwtError, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct JwtClaims {
    pub id: String,
    pub exp: usize,
    pub iss: String,
}

impl fmt::Display for JwtClaims{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "id={}, exp={}, iss={}", self.id, self.exp, self.iss)
    }
}

//Stuff needed for JWT
//Immutable
static JWT_VALIDATION: OnceLock<Validation> = OnceLock::new();
static JWT_HEADER: OnceLock<Header> = OnceLock::new();
static JWT_DURATION: OnceLock<usize> = OnceLock::new();
static JWT_ISS: OnceLock<String> = OnceLock::new();
static JWT_DECODE_KEY: OnceLock<DecodingKey> = OnceLock::new();
static JWT_ENCODE_KEY: OnceLock<EncodingKey> = OnceLock::new();

//Public entry point/logic for encoding a JWT Claim for user id
pub fn encode_claims(id: &str) -> Result<String, JwtError> {
    encode(
        JWT_HEADER.get_or_init(||init_header()), 
        &JwtClaims{
            id: id.to_string(),
            exp: Utc::now().timestamp() as usize + JWT_DURATION.get_or_init(|| init_duration()),
            iss: JWT_ISS.get_or_init(||init_iss()).to_string()
        },
        JWT_ENCODE_KEY.get_or_init(|| init_encode_key())
    )
    .track(id)
}

//Internal logic for decoding a signed JWT claim
fn inner_decode(signed_claim: &str) -> Result<JwtClaims, JwtError> {
    Ok(
        decode::<JwtClaims>(
            signed_claim, 
            JWT_DECODE_KEY.get_or_init(|| init_decode_key()), 
            JWT_VALIDATION.get_or_init(|| init_validation())
        )?
        .claims
    )
    .track(signed_claim)
}

//Default Sync version of decode
#[cfg(feature = "sync-decode")]
pub fn decode_claims(signed_claim: &str) -> Result<JwtClaims, JwtError> {
    inner_decode(signed_claim)
}


//Async version of decode
#[cfg(feature = "async-decode")]
pub async fn decode_claims(signed_claim: &str) -> Result<JwtClaims, JwtError> {
    inner_decode(signed_claim)
}




//Initialization methods for static resources
fn init_validation() -> Validation{
    todo!()
}
fn init_header() -> Header{
    todo!()
}
fn init_duration() -> usize{
    todo!()
}
fn init_iss() -> String{
    todo!()
}
fn init_decode_key() -> DecodingKey{
    todo!()
}
fn init_encode_key() -> EncodingKey{
    todo!()
}

trait Track {
    fn track(self, input: &str) -> Self;
}
impl Track for Result<JwtClaims, JwtError>{
    fn track(self, input: &str) -> Self{
        //TODO: Tracking
        return self
    }
}
impl Track for Result<String, JwtError>{
    fn track(self, input: &str) -> Self{
        //TODO: Tracking
        return self
    }
}