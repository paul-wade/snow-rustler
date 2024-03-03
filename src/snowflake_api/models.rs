use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct StatementExecutionRequest {
    statement: String,
    timeout: Option<i64>,
    database: Option<String>,
    schema: Option<String>,
    warehouse: Option<String>,
    role: Option<String>,
    bindings: serde_json::Value,
    parameters: ExecutionParameters,
}

#[derive(Serialize, Deserialize)]
pub struct ExecutionParameters {
    timezone: Option<String>,
    query_tag: Option<String>,
    // Add other parameters as needed
}

#[derive(Serialize, Deserialize)]
pub(crate) struct StatementExecutionResponse {
    code: String,
    sql_state: String,
    message: String,
    statement_handle: String,
    // Include other fields from the example response
}
