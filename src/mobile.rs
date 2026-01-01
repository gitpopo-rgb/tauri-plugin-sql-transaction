use serde::de::DeserializeOwned;
use tauri::{
  plugin::{PluginApi, PluginHandle},
  AppHandle, Runtime,
};

use crate::models::*;

#[cfg(target_os = "ios")]
tauri::ios_plugin_binding!(init_plugin_sql_transaction);

// initializes the Kotlin or Swift plugin classes
pub fn init<R: Runtime, C: DeserializeOwned>(
  _app: &AppHandle<R>,
  api: PluginApi<R, C>,
) -> crate::Result<SqlTransaction<R>> {
  #[cfg(target_os = "android")]
  let handle = api.register_android_plugin("", "ExamplePlugin")?;
  #[cfg(target_os = "ios")]
  let handle = api.register_ios_plugin(init_plugin_sql_transaction)?;
  Ok(SqlTransaction(handle))
}

/// Access to the sql-transaction APIs.
pub struct SqlTransaction<R: Runtime>(PluginHandle<R>);

impl<R: Runtime> SqlTransaction<R> {
  pub fn ping(&self, payload: PingRequest) -> crate::Result<PingResponse> {
    self
      .0
      .run_mobile_plugin("ping", payload)
      .map_err(Into::into)
  }

  pub fn connect(&self, payload: ConnectRequest) -> crate::Result<ConnectResponse> {
    self
      .0
      .run_mobile_plugin("connect", payload)
      .map_err(Into::into)
  }

  pub fn execute(&self, payload: ExecuteRequest) -> crate::Result<ExecuteResponse> {
    self
      .0
      .run_mobile_plugin("execute", payload)
      .map_err(Into::into)
  }

  pub fn select(&self, payload: SelectRequest) -> crate::Result<SelectResponse> {
    self
      .0
      .run_mobile_plugin("select", payload)
      .map_err(Into::into)
  }

  pub fn begin(&self, payload: BeginTransactionRequest) -> crate::Result<BeginTransactionResponse> {
    self
      .0
      .run_mobile_plugin("begin_transaction", payload)
      .map_err(Into::into)
  }

  pub fn execute_in_tx(&self, payload: TransactionExecuteRequest) -> crate::Result<ExecuteResponse> {
    self
      .0
      .run_mobile_plugin("execute_in_transaction", payload)
      .map_err(Into::into)
  }

  pub fn commit(&self, payload: CommitRequest) -> crate::Result<AckResponse> {
    self
      .0
      .run_mobile_plugin("commit", payload)
      .map_err(Into::into)
  }

  pub fn rollback(&self, payload: RollbackRequest) -> crate::Result<AckResponse> {
    self
      .0
      .run_mobile_plugin("rollback", payload)
      .map_err(Into::into)
  }
}
