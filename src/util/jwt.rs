use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};

/// Our claims struct, it needs to derive `Serialize` and/or `Deserialize`
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    token: String,
    exp: usize,
}

pub fn create_jwt(token: String) -> Result<String, jsonwebtoken::errors::Error> {
    let jwt_secret = dotenv::var("JWT_SECRET").expect("JWT_SECRET must be set");

    let claims = Claims {
        token: token.to_string().to_owned(),
        exp: 10000000000,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_ref()),
    )
}
