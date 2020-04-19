use chrono::serde::ts_seconds::{deserialize as from_ts, serialize as to_ts};
use chrono::{DateTime, Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TokenClaims {
    pub user_id: i64,
    #[serde(serialize_with = "to_ts", deserialize_with = "from_ts")]
    iat: DateTime<Utc>,
    #[serde(serialize_with = "to_ts", deserialize_with = "from_ts")]
    exp: DateTime<Utc>,
}

pub fn encode_token(user_id: i64, secret: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let claims = TokenClaims {
        user_id,
        iat: Utc::now(),
        exp: Utc::now() + Duration::days(30),
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
}

pub fn decode_token(token: &str, secret: &str) -> Result<TokenClaims, jsonwebtoken::errors::Error> {
    let data = decode::<TokenClaims>(
        &token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )?;

    Ok(data.claims)
}
