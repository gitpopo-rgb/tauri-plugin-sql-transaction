use serde::{ser::Serializer, Serialize};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error("database is not loaded: {0}")]
  DatabaseNotLoaded(String),

  #[error("transaction not found: {0}")]
  TransactionNotFound(String),

  #[error("transaction already finished: {0}")]
  TransactionFinished(String),

  #[error(transparent)]
  Sql(#[from] tauri_plugin_sql::Error),

  #[error(transparent)]
  Sqlx(#[from] sqlx::Error),

  #[error(transparent)]
  Io(#[from] std::io::Error),
  #[cfg(mobile)]
  #[error(transparent)]
  PluginInvoke(#[from] tauri::plugin::mobile::PluginInvokeError),
}

impl Serialize for Error {
  fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    serializer.serialize_str(self.to_string().as_ref())
  }
}
