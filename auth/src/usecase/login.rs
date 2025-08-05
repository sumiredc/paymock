use chrono::{DateTime, Duration, Utc};
use chrono_tz::{ParseError, Tz};
use jsonwebtoken::{EncodingKey, Header, encode, errors::Error};
use serde::{Deserialize, Serialize};
use std::result::Result;
use thiserror::Error;

#[derive(Default)]
pub struct LoginUseCase {}
impl LoginUseCase {
    pub fn new() -> Self {
        Self {}
    }

    pub fn execute(&self) -> Result<LoginResponse, LoginError> {
        // TODO: user_id, exp_min
        let access_token = self.generate_token(1, 15)?;
        let refresh_token = self.generate_token(1, 1440)?;

        Ok(LoginResponse {
            access_token,
            refresh_token,
        })
    }

    // トークンの生成
    fn generate_token(&self, user_id: usize, exp_min: i64) -> Result<Token, LoginError> {
        let exp = self.generate_exp(exp_min)?;
        let claims = Claims {
            sub: user_id.to_string(),
            exp: exp.timestamp(),
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret("dummy-secret".as_ref()),
        )?;

        Ok(Token {
            value: token,
            token_type: "Bearer".to_string(),
            exp: exp.to_rfc3339(),
        })
    }

    // 有効期限を生成
    fn generate_exp(&self, duration_min: i64) -> Result<DateTime<Tz>, LoginError> {
        let timezone: Tz = Tz::Japan;
        let now = Utc::now().with_timezone(&timezone);
        let exp = now + Duration::minutes(duration_min);

        Ok(exp)
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Token {
    value: String,
    token_type: String,
    exp: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: i64,
}

#[derive(Error, Debug)]
pub enum LoginError {
    #[error("Failed to generate token")]
    TokenGenerationFailed(#[from] Error),

    #[error("Parse failed to timezone")]
    ParseError(#[from] ParseError),
}

#[derive(Serialize)]
pub struct LoginResponse {
    access_token: Token,
    refresh_token: Token,
}
