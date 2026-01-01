# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.0] - 2026-01-01

### Added

- Initial release of tauri-plugin-sql-transaction
- Support for SQLite, MySQL, and PostgreSQL databases
- Transaction management API with begin, commit, and rollback
- TypeScript/JavaScript API wrapper
- Connection pooling using sqlx
- Automatic transaction helper function
- Manual transaction control
- Query execution (INSERT, UPDATE, DELETE)
- Data selection (SELECT queries)
- Comprehensive error handling
- Unit tests for SQLite operations
- Example Svelte application demonstrating all features
- Full API documentation in README

### Features

- **Database Support:**
  - SQLite (file-based and in-memory)
  - MySQL
  - PostgreSQL

- **Transaction API:**
  - `begin()` - Start a transaction
  - `commit()` - Commit a transaction
  - `rollback()` - Rollback a transaction
  - `transaction()` - Automatic transaction wrapper with auto-commit/rollback

- **Query API:**
  - `execute()` - Execute INSERT, UPDATE, DELETE queries
  - `select()` - Execute SELECT queries and retrieve results

- **Connection Management:**
  - `connect()` - Connect to a database with connection pooling
  - Automatic connection reuse
  - Support for multiple database connections

- **TypeScript Support:**
  - Full type definitions
  - Connection class with fluent API
  - Transaction class for manual control
  - Automatic transaction helper

### Technical Details

- Built on sqlx 0.8 with tokio runtime
- Uses Tauri 2.9.5 plugin system
- JSON-based communication between frontend and backend
- IndexMap for ordered row data
- UUID-based transaction identification

### Documentation

- Comprehensive README with installation and usage examples
- API reference for both Rust and TypeScript
- Publishing guide for crates.io and npm
- Example application with interactive demos

## [Unreleased]

### Planned

- Integration tests for MySQL and PostgreSQL
- Prepared statement caching for performance
- Batch query execution
- Connection pool configuration options
- Query builder helpers
- Migration system integration
- Additional error context and debugging info

---

## Version History

- **1.0.0** - 2026-01-01: Initial release with full transaction support for SQLite, MySQL, and PostgreSQL
