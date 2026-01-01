use tauri::{
  plugin::{Builder, TauriPlugin},
  Manager, Runtime,
};

pub use models::*;

#[cfg(desktop)]
mod desktop;
#[cfg(mobile)]
mod mobile;

mod commands;
mod error;
mod models;
#[cfg(test)]
mod tests;

pub use error::{Error, Result};

#[cfg(desktop)]
use desktop::SqlTransaction;
#[cfg(mobile)]
use mobile::SqlTransaction;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the sql-transaction APIs.
pub trait SqlTransactionExt<R: Runtime> {
  fn sql_transaction(&self) -> &SqlTransaction<R>;
}

impl<R: Runtime, T: Manager<R>> crate::SqlTransactionExt<R> for T {
  fn sql_transaction(&self) -> &SqlTransaction<R> {
    self.state::<SqlTransaction<R>>().inner()
  }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
  Builder::new("sql-transaction")
    .invoke_handler(tauri::generate_handler![
      commands::ping,
      commands::connect,
      commands::execute,
      commands::select,
      commands::begin_transaction,
      commands::execute_in_transaction,
      commands::commit,
      commands::rollback
    ])
    .setup(|app, api| {
      #[cfg(mobile)]
      let sql_transaction = mobile::init(app, api)?;
      #[cfg(desktop)]
      let sql_transaction = desktop::init(app, api)?;
      app.manage(sql_transaction);
      Ok(())
    })
    .build()
}
