use serde::Serialize;
use std::result::Result;
use thiserror::Error;

#[derive(Default)]
pub struct SendUseCase {}
impl SendUseCase {
    pub fn new() -> Self {
        Self {}
    }

    // TODO: 未実装
    pub fn execute(&self, user_id: String) -> Result<SendResponse, SendError> {
        println!("{}", user_id);

        Ok(SendResponse {})
    }
}

#[derive(Error, Debug)]
pub enum SendError {}

#[derive(Serialize)]
pub struct SendResponse {}
