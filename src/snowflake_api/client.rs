// src/snowflake_api/snowflake_api

use reqwest::{Client as HttpClient, Error};
use serde::{Serialize, Deserialize};
use super::models::{StatementExecutionRequest, StatementExecutionResponse}; // Assuming you define your request/response models here.

pub struct Client {
    http_client: HttpClient,
    token: String,
}

impl Client {
    pub fn new(token: &str) -> Self {
        Client {
            http_client: HttpClient::new(),
            token: token.to_string(),
        }
    }

    pub async fn submit_sql_statement(&self, request_body: &StatementExecutionRequest) -> Result<StatementExecutionResponse, Error> {
        let res = self.http_client.post("https://org-account.snowflakecomputing.com/api/v2/statements")
            .bearer_auth(&self.token)
            .json(&request_body)
            .send()
            .await?
            .json::<StatementExecutionResponse>()
            .await?;

        Ok(res)
    }
}
