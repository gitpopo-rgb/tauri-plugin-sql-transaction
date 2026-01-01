<div align="center">

# tauri-plugin-sql-transaction

基于 Tauri 2 的跨端 SQL 事务插件，支持 SQLite / MySQL / PostgreSQL。内部复用 `tauri-plugin-sql` 与 `sqlx`，提供 Rust/TypeScript 双端 API，便于在桌面与移动端实现原子化数据库操作。

</div>

## 特性

- 多数据库支持：SQLite / MySQL / PostgreSQL（默认开启全部驱动）。
- 事务封装：begin / execute_in_tx / commit / rollback，含自动回滚的回调式 `transaction` 帮助函数。
- 前端友好：TypeScript 类型定义，简单 Promise API。
- 依赖复用：在 Rust 侧直接使用 `tauri-plugin-sql`，TS 侧使用 `@tauri-apps/plugin-sql` 的 invoke 通道。

## 安装

### Rust

```toml
# Cargo.toml
[dependencies]
tauri-plugin-sql-transaction = "1"
```

> 已默认启用 sqlite/mysql/postgres 驱动。如需精简，可在本 crate 暴露 feature 后自行裁剪。

### 前端

```bash
pnpm add @tauri-apps/plugin-sql-transaction-api
```

## 使用示例（前端）

```ts
import { connect, transaction, select } from 'tauri-plugin-sql-transaction-api'

// 连接数据库（示例为 SQLite，本地文件会自动创建）
const db = await connect('sqlite:app.db')

// 事务封装：成功自动提交，异常自动回滚
await transaction(db, async (tx) => {
	await tx.execute('INSERT INTO todos (title) VALUES (?)', { values: ['hello'] })
	const rows = await select(db, 'SELECT * FROM todos')
	console.log(rows)
})
```

## Rust 注册

```rust
use tauri_plugin_sql_transaction;

fn main() {
	tauri::Builder::default()
		.plugin(tauri_plugin_sql_transaction::init())
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}
```

## API（前端）

- `connect(url: string): Promise<DbHandle>`
- `execute(db, query, { values? }): Promise<{ rowsAffected: number; lastInsertId?: string | null }>`
- `select<T = Record<string, unknown>>(db, query, { values? }): Promise<T[]>`
- `begin(db): Promise<Transaction>`
- `Transaction.execute(query, { values? })`
- `Transaction.commit()` / `Transaction.rollback()`
- `transaction(db, fn)` 回调式封装，自动提交/回滚。

## API（Rust 命令）

- `connect(url)`
- `execute(db, query, values)`
- `select(db, query, values)`
- `begin_transaction(db)`
- `execute_in_transaction(tx_id, query, values)`
- `commit(tx_id)` / `rollback(tx_id)`

## 测试与示例

- 推荐首先使用 SQLite（免安装）在 `examples/tauri-app` 中演示 CRUD + 事务回滚。
- 单元测试：围绕事务的提交/回滚/错误回滚场景。
- 集成测试：可选对 MySQL/PostgreSQL（需本地或容器）。

## 发布

1. `pnpm build` 生成前端 dist。
2. `cargo publish` 发布 Rust crate。
3. `pnpm publish` 发布 npm 包。

## 状态

当前为 1.0.0 初版骨架，核心事务接口已就绪，仍需补充：
- 更丰富的错误分类与日志
- 完整测试矩阵（SQLite 默认，MySQL/PostgreSQL 可选）
- 示例页面的事务演示
# Tauri Plugin SQL Transaction

A Tauri plugin that provides SQL transaction support for SQLite, MySQL, and PostgreSQL databases.

## Features

- 🔄 Full transaction support (begin, commit, rollback)
- 🗄️ Multi-database support: SQLite, MySQL, PostgreSQL
- 🛡️ Type-safe TypeScript API
- 🎯 Connection pooling via sqlx
- 🔌 Easy-to-use transaction helper functions

## Installation

### Rust (Cargo.toml)

```toml
[dependencies]
tauri-plugin-sql-transaction = "1.0.0"
```

### JavaScript/TypeScript

```bash
pnpm add tauri-plugin-sql-transaction-api
# or
npm install tauri-plugin-sql-transaction-api
# or
yarn add tauri-plugin-sql-transaction-api
```

## Setup

### Rust Setup

In your `lib.rs` or `main.rs`:

```rust
use tauri_plugin_sql_transaction;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_sql_transaction::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

### Permissions

Add the plugin permissions to your `capabilities/default.json`:

```json
{
  "permissions": [
    "sql-transaction:default"
  ]
}
```

## Usage

### TypeScript API

#### Basic Connection and Queries

```typescript
import { connect, execute, select } from 'tauri-plugin-sql-transaction-api'

// Connect to a database
const db = await connect('sqlite:app.db')
// Or: const db = await connect('mysql://user:pass@localhost/mydb')
// Or: const db = await connect('postgres://user:pass@localhost/mydb')

// Execute a query
const result = await execute(db, 'INSERT INTO users (name) VALUES (?)', {
  values: ['Alice']
})
console.log(`Inserted ${result.rowsAffected} rows, last ID: ${result.lastInsertId}`)

// Select data
const rows = await select(db, 'SELECT * FROM users WHERE id = ?', {
  values: [1]
})
console.log(rows)
```

#### Manual Transaction Control

```typescript
import { begin } from 'tauri-plugin-sql-transaction-api'

const db = await connect('sqlite:app.db')

// Begin a transaction
const tx = await begin(db)

try {
  // Execute multiple operations
  await tx.execute('INSERT INTO users (name) VALUES (?)', { values: ['Bob'] })
  await tx.execute('UPDATE accounts SET balance = balance - 100 WHERE user = ?', {
    values: ['Bob']
  })
  
  // Commit the transaction
  await tx.commit()
} catch (error) {
  // Rollback on error
  await tx.rollback()
  throw error
}
```

#### Automatic Transaction Helper

```typescript
import { transaction } from 'tauri-plugin-sql-transaction-api'

const db = await connect('sqlite:app.db')

// Automatically commits on success, rolls back on error
const result = await transaction(db, async (tx) => {
  const r1 = await tx.execute('INSERT INTO users (name) VALUES (?)', {
    values: ['Charlie']
  })
  
  await tx.execute('INSERT INTO logs (user_id, action) VALUES (?, ?)', {
    values: [r1.lastInsertId, 'created']
  })
  
  return r1.lastInsertId
})

console.log(`Created user with ID: ${result}`)
```

### Connection URLs

- **SQLite**: `sqlite:database.db` or `sqlite::memory:` (paths are relative to app config directory)
- **MySQL**: `mysql://username:password@host:port/database`
- **PostgreSQL**: `postgres://username:password@host:port/database` or `postgresql://...`

## API Reference

### TypeScript

#### `connect(url: string): Promise<DbHandle>`

Connect to a database and return a connection handle.

#### `execute(db: DbHandle, query: string, options?: ExecuteOptions): Promise<ExecuteResult>`

Execute a query (INSERT, UPDATE, DELETE) and return the result.

- **ExecuteResult**: `{ rowsAffected: number, lastInsertId?: string | null }`
- **ExecuteOptions**: `{ values?: unknown[] }`

#### `select<T>(db: DbHandle, query: string, options?: ExecuteOptions): Promise<T[]>`

Execute a SELECT query and return rows as objects.

#### `begin(db: DbHandle): Promise<Transaction>`

Begin a new transaction.

#### `transaction<T>(db: DbHandle, fn: (tx: Transaction) => Promise<T>): Promise<T>`

Execute a function within a transaction. Automatically commits on success, rolls back on error.

#### Transaction Methods

- `execute(query: string, options?: ExecuteOptions): Promise<ExecuteResult>`
- `commit(): Promise<void>`
- `rollback(): Promise<void>`

## Development

### Build

```bash
# Build Rust library
cargo build

# Build TypeScript API
pnpm install
pnpm build
```

### Testing

```bash
# Run Rust tests
cargo test

# Check types
pnpm tsc --noEmit
```

## Example

See the [examples/tauri-app](examples/tauri-app) directory for a complete working example.

## Requirements

- Tauri 2.x
- Rust 1.77.2+
- Node.js (for building TypeScript API)

## License

MIT or Apache-2.0

## Credits

Built with [sqlx](https://github.com/launchbadge/sqlx) and [Tauri](https://tauri.app/).
