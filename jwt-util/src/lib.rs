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

//JWT
//Immutable, Internal Only
static _JWT_VALIDATION: OnceLock<Validation> = OnceLock::new();
static _JWT_HEADER: OnceLock<Header> = OnceLock::new();
static _JWT_DURATION: OnceLock<usize> = OnceLock::new();
static _JWT_ISS: OnceLock<String> = OnceLock::new();
static _JWT_DECODE_KEY: OnceLock<DecodingKey> = OnceLock::new();
static _JWT_ENCODE_KEY: OnceLock<EncodingKey> = OnceLock::new();

//Internal logic for decoding a signed JWT claim
fn _inner_decode(signed_claim: &str) -> Result<JwtClaims, JwtError> {
    Ok(
        decode::<JwtClaims>(
            signed_claim, 
            _JWT_DECODE_KEY.get_or_init(|| _decode_key()), 
            _JWT_VALIDATION.get_or_init(|| _validation())
        )?
        .claims
    )
    .track(signed_claim)
}

//Default Sync Decode
#[cfg(feature = "sync-decode")]
pub fn decode_claims(signed_claim: &str) -> Result<JwtClaims, JwtError> {
    _inner_decode(signed_claim)
}


//Async Decode
#[cfg(feature = "async-decode")]
pub async fn decode_claims(signed_claim: &str) -> Result<JwtClaims, JwtError> {
    _inner_decode(signed_claim)
}

//Optional Sync Encode 
#[cfg(feature = "encode")]
pub fn encode_claims(id: &str) -> Result<String, JwtError> {
    encode(
        _JWT_HEADER.get_or_init(|| _header()), 
        &JwtClaims{
            id: id.to_string(),
            exp: Utc::now().timestamp() as usize + _JWT_DURATION.get_or_init(|| _duration()),
            iss: _JWT_ISS.get_or_init(|| _iss()).clone()
        },
        _JWT_ENCODE_KEY.get_or_init(|| _encode_key())
    )
    .track(id)
}




//Initialization methods for static resources
fn _validation() -> Validation{
    todo!()
}
fn _header() -> Header{
    todo!()
}
fn _duration() -> usize{
    todo!()
}
fn _iss() -> String{
    todo!()
}
fn _decode_key() -> DecodingKey{
    todo!()
}
fn _encode_key() -> EncodingKey{
    todo!()
}


//Observability Tracking
trait _Track {
    fn track(self, input: &str) -> Self;
}
impl _Track for Result<JwtClaims, JwtError>{
    fn track(self, input: &str) -> Self{
        //TODO: Tracking
        return self
    }
}
impl _Track for Result<String, JwtError>{
    fn track(self, input: &str) -> Self{
        //TODO: Tracking
        return self
    }
}