use std::result::Result;

use serde::Serialize;
use thiserror::Error;

pub struct VerifyUseCase {}
impl VerifyUseCase {
    pub fn new() -> Self {
        Self {}
    }

    // TODO: 未実装
    pub fn execute(&self, token: &str) -> Result<VerifyResponse, VerifyError> {
        println!("{}", token);

        Ok(VerifyResponse {
            user_id: "dummy-user-id".to_string(),
        })
    }
}

#[derive(Error, Debug)]
pub enum VerifyError {}

#[derive(Serialize)]
pub struct VerifyResponse {
    pub user_id: String,
}
