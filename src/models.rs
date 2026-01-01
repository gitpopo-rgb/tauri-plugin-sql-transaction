use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PingRequest {
  pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PingResponse {
  pub value: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectRequest {
  pub url: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectResponse {
  pub handle: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecuteRequest {
  pub db: String,
  pub query: String,
  #[serde(default)]
  pub values: Vec<Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecuteResponse {
  pub rows_affected: u64,
  pub last_insert_id: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SelectRequest {
  pub db: String,
  pub query: String,
  #[serde(default)]
  pub values: Vec<Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SelectResponse {
  pub rows: Vec<IndexMap<String, Value>>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BeginTransactionRequest {
  pub db: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BeginTransactionResponse {
  pub tx_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionExecuteRequest {
  pub tx_id: String,
  pub query: String,
  #[serde(default)]
  pub values: Vec<Value>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CommitRequest {
  pub tx_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RollbackRequest {
  pub tx_id: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AckResponse {
  pub ok: bool,
}
