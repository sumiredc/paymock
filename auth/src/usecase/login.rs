use chrono::{DateTime, Duration, Utc};
use chrono_tz::Tz;
use jsonwebtoken::{EncodingKey, Header};
use redis::{Client, Commands, RedisError};
use serde::{Deserialize, Serialize};
use std::result::Result;
use thiserror::Error;

pub struct LoginUseCase {
    client: Client,
}
impl LoginUseCase {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub fn execute(&self) -> Result<LoginResponse, LoginError> {
        let mut conn = self
            .client
            .get_connection()
            .map_err(LoginError::RedisConnectionFailed)?;

        // TODO: user_id, exp
        let user_id = 1;
        let access_token_exp = 15;
        let refresh_token_exp = 1440;

        let access_token = self.generate_token(user_id, access_token_exp)?;
        let refresh_token = self.generate_token(user_id, refresh_token_exp)?;

        conn.set::<String, usize, ()>(refresh_token.value.clone(), user_id)
            .map_err(LoginError::RedisSetCommandFailed)?;

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

        let token = jsonwebtoken::encode(
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
    #[error("Failed to generate token: {0}")]
    TokenGenerationFailed(#[from] jsonwebtoken::errors::Error),

    #[error("Parse failed to timezone: {0}")]
    ParseError(#[from] chrono::ParseError),

    #[error("Failed to connect to Redis: {0}")]
    RedisConnectionFailed(RedisError),

    #[error("Failed to execute Redis SET command: {0}")]
    RedisSetCommandFailed(RedisError),
}

#[derive(Serialize)]
pub struct LoginResponse {
    access_token: Token,
    refresh_token: Token,
}
