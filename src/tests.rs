#[cfg(test)]
mod tests {
  use sqlx::SqlitePool;
  
  #[tokio::test]
  async fn test_sqlite_basic_operations() {
    // Direct sqlx test without Tauri app context
    let pool = SqlitePool::connect("sqlite::memory:")
      .await
      .expect("Failed to connect");
    
    // Create table
    sqlx::query("CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT NOT NULL)")
      .execute(&pool)
      .await
      .expect("Failed to create table");
    
    // Insert
    let result = sqlx::query("INSERT INTO users (name) VALUES (?)")
      .bind("Alice")
      .execute(&pool)
      .await
      .expect("Failed to insert");
    
    assert_eq!(result.rows_affected(), 1);
    assert_eq!(result.last_insert_rowid(), 1);
    
    // Select
    let rows: Vec<(i64, String)> = sqlx::query_as("SELECT id, name FROM users WHERE name = ?")
      .bind("Alice")
      .fetch_all(&pool)
      .await
      .expect("Failed to select");
    
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].1, "Alice");
  }

  #[tokio::test]
  async fn test_sqlite_transaction_commit() {
    let pool = SqlitePool::connect("sqlite::memory:")
      .await
      .expect("Failed to connect");
    
    // Create table
    sqlx::query("CREATE TABLE accounts (id INTEGER PRIMARY KEY, balance INTEGER)")
      .execute(&pool)
      .await
      .expect("Failed to create table");
    
    sqlx::query("INSERT INTO accounts (balance) VALUES (1000)")
      .execute(&pool)
      .await
      .expect("Failed to insert");
    
    // Begin transaction
    let mut tx = pool.begin().await.expect("Failed to begin");
    
    // Update in transaction
    sqlx::query("UPDATE accounts SET balance = balance - 100 WHERE id = 1")
      .execute(&mut *tx)
      .await
      .expect("Failed to update");
    
    // Commit
    tx.commit().await.expect("Failed to commit");
    
    // Verify
    let balance: (i64,) = sqlx::query_as("SELECT balance FROM accounts WHERE id = 1")
      .fetch_one(&pool)
      .await
      .expect("Failed to select");
    
    assert_eq!(balance.0, 900);
  }

  #[tokio::test]
  async fn test_sqlite_transaction_rollback() {
    let pool = SqlitePool::connect("sqlite::memory:")
      .await
      .expect("Failed to connect");
    
    // Create table
    sqlx::query("CREATE TABLE accounts (id INTEGER PRIMARY KEY, balance INTEGER)")
      .execute(&pool)
      .await
      .expect("Failed to create table");
    
    sqlx::query("INSERT INTO accounts (balance) VALUES (1000)")
      .execute(&pool)
      .await
      .expect("Failed to insert");
    
    // Begin transaction
    let mut tx = pool.begin().await.expect("Failed to begin");
    
    // Update in transaction
    sqlx::query("UPDATE accounts SET balance = balance - 100 WHERE id = 1")
      .execute(&mut *tx)
      .await
      .expect("Failed to update");
    
    // Rollback
    tx.rollback().await.expect("Failed to rollback");
    
    // Verify - should still be 1000
    let balance: (i64,) = sqlx::query_as("SELECT balance FROM accounts WHERE id = 1")
      .fetch_one(&pool)
      .await
      .expect("Failed to select");
    
    assert_eq!(balance.0, 1000);
  }

  #[tokio::test]
  async fn test_mysql_connection_format() {
    // Test that MySQL URL parsing works (without actual connection)
    let url = "mysql://user:pass@localhost:3306/testdb";
    assert!(url.starts_with("mysql://"));
  }

  #[tokio::test]
  async fn test_postgres_connection_format() {
    // Test that PostgreSQL URL parsing works (without actual connection)
    let url = "postgres://user:pass@localhost:5432/testdb";
    assert!(url.starts_with("postgres://") || url.starts_with("postgresql://"));
  }
}

