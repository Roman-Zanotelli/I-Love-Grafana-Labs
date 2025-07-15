//=========================================
//Core Logic used bwtween encode/decode
//=========================================
//#[cfg(feature = "core")]
pub mod core{
    use uuid::Uuid;

    //Claim data
    #[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
    pub struct JwtClaims {
        pub id: Uuid,
        pub exp: usize,
        pub iss: String,
    }
    
    //Formatting
    impl core::fmt::Display for JwtClaims{
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "id={}, exp={}, iss={}", self.id, self.exp, self.iss)
            }
        }

    //Observability Tracking Trait (Arc is used in both implementations so that a background task can hold reference to the result without copying it)
    pub(crate) trait Track {
        fn track(self, input: &Uuid) -> Self;
    }
}


//=========================================
//Decoding Logic
//=========================================
//#[cfg(feature = "decode")]
pub mod decode{
    use std::sync::{Arc, OnceLock};

    use jsonwebtoken::errors::Error as JwtError;
    use uuid::Uuid;

    use crate::core::{JwtClaims, Track};

    //Static resources for decoding
    static JWT_DECODE_KEY: OnceLock<jsonwebtoken::DecodingKey> = OnceLock::new();
    static JWT_VALIDATION: OnceLock<jsonwebtoken::Validation> = OnceLock::new();

    //Internal logic for decoding a signed JWT claim
    fn _inner_decode(signed_claim: &Uuid) -> Result<JwtClaims, JwtError>{
        
            jsonwebtoken::decode::<JwtClaims>(
                &signed_claim.to_string(), 
                JWT_DECODE_KEY.get_or_init(|| _decode_key()), 
                JWT_VALIDATION.get_or_init(|| _validation())
            )
            .map(|token| token.claims)
        .track(signed_claim)
    }

    pub fn decode_claims(signed_claim: &Uuid) -> Result<JwtClaims, JwtError> {
        _inner_decode(signed_claim)
    }


    //init validation
    fn _validation() -> jsonwebtoken::Validation{
        jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::HS256)
    }

    //init decode key
    fn _decode_key() -> jsonwebtoken::DecodingKey{
        jsonwebtoken::DecodingKey::from_secret(std::env::var("JWT_SECRET").unwrap_or("Default Secret".to_string()).as_bytes())
    }

    //tracking implementation for decoding
    impl Track for Result<JwtClaims, JwtError>{
        fn track(self, input: &Uuid) -> Self{
            //TODO: Tracking
            return self
        }
    }
}


//=========================================
//Encoding Logic
//=========================================
//#[cfg(feature = "encode")]
pub mod encode{
    use std::sync::{Arc, OnceLock};

    use chrono::Utc;
    use jsonwebtoken::errors::Error as JwtError;
    use uuid::Uuid;

    use crate::core::{JwtClaims, Track};

    //Static resources for encoding 
    static JWT_HEADER: OnceLock<jsonwebtoken::Header> = OnceLock::new();
    static JWT_ENCODE_KEY: OnceLock<jsonwebtoken::EncodingKey> = OnceLock::new();
    static JWT_DURATION: OnceLock<usize> = OnceLock::new();
    static JWT_ISS: OnceLock<String> = OnceLock::new();

    //Encode a claim by user id (later add a more flexible method for general claims)
    pub fn encode_claims(id: Uuid) -> Result<String, JwtError> {
        let claim = JwtClaims{
                    id,
                    exp: Utc::now().timestamp() as usize + JWT_DURATION.get_or_init(|| _duration()),
                    iss: JWT_ISS.get_or_init(|| _iss()).clone()
        };
        
        jsonwebtoken::encode(
            JWT_HEADER.get_or_init(|| _header()), 
            &claim,
            JWT_ENCODE_KEY.get_or_init(|| _encode_key())
        ).track(&id)
    }


    //init header
    fn _header() -> jsonwebtoken::Header{
        jsonwebtoken::Header::new(jsonwebtoken::Algorithm::HS256)
    }

    //init duration
    fn _duration() -> usize{
    std::env::var("JWT_DURATION").unwrap_or("999999".to_string()).parse().unwrap()
    }

    //init iss
    fn _iss() -> String{
    std::env::var("JWT_ISS").unwrap_or("Default ISS".to_string())
    }

    //init encode key
    fn _encode_key() -> jsonwebtoken::EncodingKey{
        jsonwebtoken::EncodingKey::from_secret(std::env::var("JWT_SECRET").unwrap_or("Default Secret".to_string()).as_bytes())
    }

    //tracking implementation for encoding
    impl Track for Result<String, JwtError>{
        fn track(self, input: &Uuid) -> Self{
            //TODO: Tracking
            return self
        }
    }
}

