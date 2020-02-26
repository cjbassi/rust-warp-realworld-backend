use chrono::{Duration, Utc};
use jsonwebtoken::errors::Result;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// According to the RealWorld API spec, clients are supposed to prefix the token with this string
// in the Authorization header.
const TOKEN_PREFIX: &str = "Token ";

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Claims {
    sub: Uuid,
    exp: u64, // seconds since the epoch
}

impl Claims {
    fn new(user_id: Uuid) -> Self {
        Self {
            sub: user_id,
            exp: (Utc::now() + Duration::weeks(3)).timestamp() as u64,
        }
    }

    pub fn user_id(&self) -> Uuid {
        self.sub
    }
}

pub fn encode_token(secret: &str, sub: Uuid) -> String {
    encode(
        &Header::default(),
        &Claims::new(sub),
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .unwrap()
}

pub fn decode_token(secret: &str, token: &str) -> Result<Claims> {
    decode::<Claims>(
        token.trim_start_matches(TOKEN_PREFIX),
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )
    .map(|token_data| token_data.claims)
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn encode_decode_token() {
        let sub = Uuid::new_v4();
        let token = encode_token("secret", sub);
        let decoded = decode::<Claims>(
            &token,
            &DecodingKey::from_secret("secret".as_ref()),
            &Validation::default(),
        );
        if let Err(e) = &decoded {
            println!("decode err: {}", e);
        }

        assert!(decoded.is_ok());
    }
}
