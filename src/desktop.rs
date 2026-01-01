use std::{collections::HashMap, sync::Arc};

use indexmap::IndexMap;
use serde::de::DeserializeOwned;
use serde_json::Value as JsonValue;
use sqlx::{Column, Row};
use tauri::{plugin::PluginApi, AppHandle, Manager, Runtime};
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::{models::*, Error, Result};

pub fn init<R: Runtime, C: DeserializeOwned>(
  app: &AppHandle<R>,
  _api: PluginApi<R, C>,
) -> Result<SqlTransaction<R>> {
  Ok(SqlTransaction::new(app.clone()))
}

/// Access to the sql-transaction APIs.
#[derive(Clone)]
pub struct SqlTransaction<R: Runtime> {
  app: AppHandle<R>,
  state: Arc<SqlState>,
}

#[derive(Default)]
struct SqlState {
  pools: RwLock<HashMap<String, DbPool>>, // key: db url/handle
  txs: RwLock<HashMap<Uuid, Box<dyn DbTransaction>>>, // key: tx id
}

enum DbPool {
  Sqlite(sqlx::Pool<sqlx::Sqlite>),
  MySql(sqlx::Pool<sqlx::MySql>),
  Postgres(sqlx::Pool<sqlx::Postgres>),
}

trait DbTransaction: Send + Sync {
  fn execute(&mut self, query: String, values: Vec<JsonValue>) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(u64, Option<String>)>> + Send + '_>>;
  fn commit(self: Box<Self>) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<()>> + Send>>;
  fn rollback(self: Box<Self>) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<()>> + Send>>;
}

struct SqliteTransaction(sqlx::Transaction<'static, sqlx::Sqlite>);
struct MySqlTransaction(sqlx::Transaction<'static, sqlx::MySql>);
struct PostgresTransaction(sqlx::Transaction<'static, sqlx::Postgres>);

impl<R: Runtime> SqlTransaction<R> {
  pub fn new(app: AppHandle<R>) -> Self {
    Self {
      app,
      state: Arc::new(SqlState::default()),
    }
  }

  pub async fn connect(&self, payload: ConnectRequest) -> Result<ConnectResponse> {
    let pool = Self::create_pool(&payload.url, &self.app).await?;
    let mut guard = self.state.pools.write().await;
    guard.insert(payload.url.clone(), pool);
    Ok(ConnectResponse {
      handle: payload.url,
    })
  }

  async fn create_pool<R2: Runtime>(url: &str, app: &AppHandle<R2>) -> Result<DbPool> {
    let scheme = url.split_once(':')
      .ok_or_else(|| Error::DatabaseNotLoaded(format!("Invalid URL: {}", url)))?
      .0;

    match scheme {
      "sqlite" => {
        let path = Self::map_sqlite_path(url, app)?;
        let pool = sqlx::SqlitePool::connect(&path).await?;
        Ok(DbPool::Sqlite(pool))
      }
      "mysql" => {
        let pool = sqlx::MySqlPool::connect(url).await?;
        Ok(DbPool::MySql(pool))
      }
      "postgres" | "postgresql" => {
        let pool = sqlx::PgPool::connect(url).await?;
        Ok(DbPool::Postgres(pool))
      }
      _ => Err(Error::DatabaseNotLoaded(format!("Unsupported database type: {}", scheme))),
    }
  }

  fn map_sqlite_path<R2: Runtime>(url: &str, app: &AppHandle<R2>) -> Result<String> {
    let db_path = url.strip_prefix("sqlite:").unwrap_or(url);
    
    if db_path == ":memory:" {
      return Ok("sqlite::memory:".to_string());
    }

    let app_dir = app.path().app_config_dir()
      .map_err(|e| Error::DatabaseNotLoaded(format!("Failed to get app dir: {}", e)))?;
    
    std::fs::create_dir_all(&app_dir)
      .map_err(|e| Error::DatabaseNotLoaded(format!("Failed to create app dir: {}", e)))?;

    let full_path = app_dir.join(db_path);
    Ok(format!("sqlite:{}", full_path.display()))
  }

  pub async fn execute(&self, payload: ExecuteRequest) -> Result<ExecuteResponse> {
    let guard = self.state.pools.read().await;
    let pool = guard
      .get(&payload.db)
      .ok_or_else(|| Error::DatabaseNotLoaded(payload.db.clone()))?;
    
    let (rows_affected, last_insert_id) = Self::execute_query(pool, &payload.query, payload.values).await?;
    Ok(ExecuteResponse {
      rows_affected,
      last_insert_id,
    })
  }

  async fn execute_query(pool: &DbPool, query: &str, values: Vec<JsonValue>) -> Result<(u64, Option<String>)> {
    match pool {
      DbPool::Sqlite(pool) => {
        let mut q = sqlx::query(query);
        for value in values {
          q = Self::bind_value_sqlite(q, value);
        }
        let result = q.execute(pool).await?;
        Ok((result.rows_affected(), Some(result.last_insert_rowid().to_string())))
      }
      DbPool::MySql(pool) => {
        let mut q = sqlx::query(query);
        for value in values {
          q = Self::bind_value_mysql(q, value);
        }
        let result = q.execute(pool).await?;
        Ok((result.rows_affected(), Some(result.last_insert_id().to_string())))
      }
      DbPool::Postgres(pool) => {
        let mut q = sqlx::query(query);
        for value in values {
          q = Self::bind_value_postgres(q, value);
        }
        let result = q.execute(pool).await?;
        Ok((result.rows_affected(), None))
      }
    }
  }

  pub async fn select(&self, payload: SelectRequest) -> Result<SelectResponse> {
    let guard = self.state.pools.read().await;
    let pool = guard
      .get(&payload.db)
      .ok_or_else(|| Error::DatabaseNotLoaded(payload.db.clone()))?;
    
    let rows = Self::select_query(pool, &payload.query, payload.values).await?;
    Ok(SelectResponse { rows })
  }

  async fn select_query(pool: &DbPool, query: &str, values: Vec<JsonValue>) -> Result<Vec<IndexMap<String, JsonValue>>> {
    match pool {
      DbPool::Sqlite(pool) => {
        let mut q = sqlx::query(query);
        for value in values {
          q = Self::bind_value_sqlite(q, value);
        }
        let rows = q.fetch_all(pool).await?;
        Self::rows_to_json_sqlite(rows)
      }
      DbPool::MySql(pool) => {
        let mut q = sqlx::query(query);
        for value in values {
          q = Self::bind_value_mysql(q, value);
        }
        let rows = q.fetch_all(pool).await?;
        Self::rows_to_json_mysql(rows)
      }
      DbPool::Postgres(pool) => {
        let mut q = sqlx::query(query);
        for value in values {
          q = Self::bind_value_postgres(q, value);
        }
        let rows = q.fetch_all(pool).await?;
        Self::rows_to_json_postgres(rows)
      }
    }
  }

  pub async fn begin(&self, payload: BeginTransactionRequest) -> Result<BeginTransactionResponse> {
    let guard = self.state.pools.read().await;
    let pool = guard
      .get(&payload.db)
      .ok_or_else(|| Error::DatabaseNotLoaded(payload.db.clone()))?;

    let tx: Box<dyn DbTransaction> = match pool {
      DbPool::Sqlite(pool) => {
        let tx = pool.begin().await?;
        Box::new(SqliteTransaction(tx))
      }
      DbPool::MySql(pool) => {
        let tx = pool.begin().await?;
        Box::new(MySqlTransaction(tx))
      }
      DbPool::Postgres(pool) => {
        let tx = pool.begin().await?;
        Box::new(PostgresTransaction(tx))
      }
    };
    drop(guard);

    let tx_id = Uuid::new_v4();
    self.state.txs.write().await.insert(tx_id, tx);

    Ok(BeginTransactionResponse {
      tx_id: tx_id.to_string(),
    })
  }

  pub async fn execute_in_tx(&self, payload: TransactionExecuteRequest) -> Result<ExecuteResponse> {
    let tx_id = Uuid::parse_str(&payload.tx_id)
      .map_err(|_| Error::TransactionNotFound(payload.tx_id.clone()))?;

    let mut txs = self.state.txs.write().await;
    let tx = txs
      .get_mut(&tx_id)
      .ok_or_else(|| Error::TransactionNotFound(payload.tx_id.clone()))?;

    let (rows_affected, last_insert_id) = tx.execute(payload.query, payload.values).await?;
    Ok(ExecuteResponse {
      rows_affected,
      last_insert_id,
    })
  }

  pub async fn commit(&self, payload: CommitRequest) -> Result<AckResponse> {
    let tx_id = Uuid::parse_str(&payload.tx_id)
      .map_err(|_| Error::TransactionNotFound(payload.tx_id.clone()))?;

    let tx = self
      .state
      .txs
      .write()
      .await
      .remove(&tx_id)
      .ok_or_else(|| Error::TransactionNotFound(payload.tx_id.clone()))?;

    tx.commit().await?;
    Ok(AckResponse { ok: true })
  }

  pub async fn rollback(&self, payload: RollbackRequest) -> Result<AckResponse> {
    let tx_id = Uuid::parse_str(&payload.tx_id)
      .map_err(|_| Error::TransactionNotFound(payload.tx_id.clone()))?;

    let tx = self
      .state
      .txs
      .write()
      .await
      .remove(&tx_id)
      .ok_or_else(|| Error::TransactionNotFound(payload.tx_id.clone()))?;

    tx.rollback().await?;
    Ok(AckResponse { ok: true })
  }

  pub fn ping(&self, payload: PingRequest) -> Result<PingResponse> {
    Ok(PingResponse {
      value: payload.value,
    })
  }

  fn bind_value_sqlite<'q>(query: sqlx::query::Query<'q, sqlx::Sqlite, sqlx::sqlite::SqliteArguments<'q>>, value: JsonValue) -> sqlx::query::Query<'q, sqlx::Sqlite, sqlx::sqlite::SqliteArguments<'q>> {
    if value.is_null() {
      query.bind(None::<String>)
    } else if let Some(s) = value.as_str() {
      query.bind(s.to_owned())
    } else if let Some(n) = value.as_i64() {
      query.bind(n)
    } else if let Some(n) = value.as_f64() {
      query.bind(n)
    } else if let Some(b) = value.as_bool() {
      query.bind(b)
    } else {
      query.bind(value.to_string())
    }
  }

  fn bind_value_mysql<'q>(query: sqlx::query::Query<'q, sqlx::MySql, sqlx::mysql::MySqlArguments>, value: JsonValue) -> sqlx::query::Query<'q, sqlx::MySql, sqlx::mysql::MySqlArguments> {
    if value.is_null() {
      query.bind(None::<String>)
    } else if let Some(s) = value.as_str() {
      query.bind(s.to_owned())
    } else if let Some(n) = value.as_i64() {
      query.bind(n)
    } else if let Some(n) = value.as_f64() {
      query.bind(n)
    } else if let Some(b) = value.as_bool() {
      query.bind(b)
    } else {
      query.bind(value.to_string())
    }
  }

  fn bind_value_postgres<'q>(query: sqlx::query::Query<'q, sqlx::Postgres, sqlx::postgres::PgArguments>, value: JsonValue) -> sqlx::query::Query<'q, sqlx::Postgres, sqlx::postgres::PgArguments> {
    if value.is_null() {
      query.bind(None::<String>)
    } else if let Some(s) = value.as_str() {
      query.bind(s.to_owned())
    } else if let Some(n) = value.as_i64() {
      query.bind(n)
    } else if let Some(n) = value.as_f64() {
      query.bind(n)
    } else if let Some(b) = value.as_bool() {
      query.bind(b)
    } else {
      query.bind(value.to_string())
    }
  }

  fn rows_to_json_sqlite(rows: Vec<sqlx::sqlite::SqliteRow>) -> Result<Vec<IndexMap<String, JsonValue>>> {
    let mut result = Vec::new();
    for row in rows {
      let mut map = IndexMap::new();
      for (i, col) in row.columns().iter().enumerate() {
        let value = Self::decode_sqlite_value(&row, i)?;
        map.insert(col.name().to_string(), value);
      }
      result.push(map);
    }
    Ok(result)
  }

  fn rows_to_json_mysql(rows: Vec<sqlx::mysql::MySqlRow>) -> Result<Vec<IndexMap<String, JsonValue>>> {
    let mut result = Vec::new();
    for row in rows {
      let mut map = IndexMap::new();
      for (i, col) in row.columns().iter().enumerate() {
        let value = Self::decode_mysql_value(&row, i)?;
        map.insert(col.name().to_string(), value);
      }
      result.push(map);
    }
    Ok(result)
  }

  fn rows_to_json_postgres(rows: Vec<sqlx::postgres::PgRow>) -> Result<Vec<IndexMap<String, JsonValue>>> {
    let mut result = Vec::new();
    for row in rows {
      let mut map = IndexMap::new();
      for (i, col) in row.columns().iter().enumerate() {
        let value = Self::decode_postgres_value(&row, i)?;
        map.insert(col.name().to_string(), value);
      }
      result.push(map);
    }
    Ok(result)
  }

  fn decode_sqlite_value(row: &sqlx::sqlite::SqliteRow, idx: usize) -> Result<JsonValue> {
    use sqlx::ValueRef;
    let raw = row.try_get_raw(idx)?;
    if raw.is_null() {
      return Ok(JsonValue::Null);
    }
    
    // Try common types
    if let Ok(v) = row.try_get::<i64, _>(idx) {
      return Ok(JsonValue::Number(v.into()));
    }
    if let Ok(v) = row.try_get::<f64, _>(idx) {
      return Ok(serde_json::Number::from_f64(v).map(JsonValue::Number).unwrap_or(JsonValue::Null));
    }
    if let Ok(v) = row.try_get::<String, _>(idx) {
      return Ok(JsonValue::String(v));
    }
    if let Ok(v) = row.try_get::<bool, _>(idx) {
      return Ok(JsonValue::Bool(v));
    }
    
    Ok(JsonValue::Null)
  }

  fn decode_mysql_value(row: &sqlx::mysql::MySqlRow, idx: usize) -> Result<JsonValue> {
    use sqlx::ValueRef;
    let raw = row.try_get_raw(idx)?;
    if raw.is_null() {
      return Ok(JsonValue::Null);
    }
    
    if let Ok(v) = row.try_get::<i64, _>(idx) {
      return Ok(JsonValue::Number(v.into()));
    }
    if let Ok(v) = row.try_get::<f64, _>(idx) {
      return Ok(serde_json::Number::from_f64(v).map(JsonValue::Number).unwrap_or(JsonValue::Null));
    }
    if let Ok(v) = row.try_get::<String, _>(idx) {
      return Ok(JsonValue::String(v));
    }
    if let Ok(v) = row.try_get::<bool, _>(idx) {
      return Ok(JsonValue::Bool(v));
    }
    
    Ok(JsonValue::Null)
  }

  fn decode_postgres_value(row: &sqlx::postgres::PgRow, idx: usize) -> Result<JsonValue> {
    use sqlx::ValueRef;
    let raw = row.try_get_raw(idx)?;
    if raw.is_null() {
      return Ok(JsonValue::Null);
    }
    
    if let Ok(v) = row.try_get::<i64, _>(idx) {
      return Ok(JsonValue::Number(v.into()));
    }
    if let Ok(v) = row.try_get::<i32, _>(idx) {
      return Ok(JsonValue::Number(v.into()));
    }
    if let Ok(v) = row.try_get::<f64, _>(idx) {
      return Ok(serde_json::Number::from_f64(v).map(JsonValue::Number).unwrap_or(JsonValue::Null));
    }
    if let Ok(v) = row.try_get::<String, _>(idx) {
      return Ok(JsonValue::String(v));
    }
    if let Ok(v) = row.try_get::<bool, _>(idx) {
      return Ok(JsonValue::Bool(v));
    }
    
    Ok(JsonValue::Null)
  }
}

impl DbTransaction for SqliteTransaction {
  fn execute(&mut self, query: String, values: Vec<JsonValue>) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(u64, Option<String>)>> + Send + '_>> {
    Box::pin(async move {
      let mut q = sqlx::query(&query);
      for value in values {
        if value.is_null() {
          q = q.bind(None::<String>);
        } else if let Some(s) = value.as_str() {
          q = q.bind(s.to_owned());
        } else if let Some(n) = value.as_i64() {
          q = q.bind(n);
        } else if let Some(n) = value.as_f64() {
          q = q.bind(n);
        } else if let Some(b) = value.as_bool() {
          q = q.bind(b);
        } else {
          q = q.bind(value.to_string());
        }
      }
      let result = q.execute(&mut *self.0).await?;
      Ok((result.rows_affected(), Some(result.last_insert_rowid().to_string())))
    })
  }

  fn commit(self: Box<Self>) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<()>> + Send>> {
    Box::pin(async move {
      self.0.commit().await?;
      Ok(())
    })
  }

  fn rollback(self: Box<Self>) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<()>> + Send>> {
    Box::pin(async move {
      self.0.rollback().await?;
      Ok(())
    })
  }
}

impl DbTransaction for MySqlTransaction {
  fn execute(&mut self, query: String, values: Vec<JsonValue>) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(u64, Option<String>)>> + Send + '_>> {
    Box::pin(async move {
      let mut q = sqlx::query(&query);
      for value in values {
        if value.is_null() {
          q = q.bind(None::<String>);
        } else if let Some(s) = value.as_str() {
          q = q.bind(s.to_owned());
        } else if let Some(n) = value.as_i64() {
          q = q.bind(n);
        } else if let Some(n) = value.as_f64() {
          q = q.bind(n);
        } else if let Some(b) = value.as_bool() {
          q = q.bind(b);
        } else {
          q = q.bind(value.to_string());
        }
      }
      let result = q.execute(&mut *self.0).await?;
      Ok((result.rows_affected(), Some(result.last_insert_id().to_string())))
    })
  }

  fn commit(self: Box<Self>) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<()>> + Send>> {
    Box::pin(async move {
      self.0.commit().await?;
      Ok(())
    })
  }

  fn rollback(self: Box<Self>) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<()>> + Send>> {
    Box::pin(async move {
      self.0.rollback().await?;
      Ok(())
    })
  }
}

impl DbTransaction for PostgresTransaction {
  fn execute(&mut self, query: String, values: Vec<JsonValue>) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(u64, Option<String>)>> + Send + '_>> {
    Box::pin(async move {
      let mut q = sqlx::query(&query);
      for value in values {
        if value.is_null() {
          q = q.bind(None::<String>);
        } else if let Some(s) = value.as_str() {
          q = q.bind(s.to_owned());
        } else if let Some(n) = value.as_i64() {
          q = q.bind(n);
        } else if let Some(n) = value.as_f64() {
          q = q.bind(n);
        } else if let Some(b) = value.as_bool() {
          q = q.bind(b);
        } else {
          q = q.bind(value.to_string());
        }
      }
      let result = q.execute(&mut *self.0).await?;
      Ok((result.rows_affected(), None))
    })
  }

  fn commit(self: Box<Self>) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<()>> + Send>> {
    Box::pin(async move {
      self.0.commit().await?;
      Ok(())
    })
  }

  fn rollback(self: Box<Self>) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<()>> + Send>> {
    Box::pin(async move {
      self.0.rollback().await?;
      Ok(())
    })
  }
}
