use tauri::{AppHandle, command, Runtime};

use crate::models::*;
use crate::Result;
use crate::SqlTransactionExt;

#[command]
pub(crate) async fn ping<R: Runtime>(
    app: AppHandle<R>,
    payload: PingRequest,
) -> Result<PingResponse> {
    app.sql_transaction().ping(payload)
}

#[command]
pub(crate) async fn connect<R: Runtime>(
    app: AppHandle<R>,
    payload: ConnectRequest,
) -> Result<ConnectResponse> {
    app.sql_transaction().connect(payload).await
}

#[command]
pub(crate) async fn execute<R: Runtime>(
    app: AppHandle<R>,
    payload: ExecuteRequest,
) -> Result<ExecuteResponse> {
    app.sql_transaction().execute(payload).await
}

#[command]
pub(crate) async fn select<R: Runtime>(
    app: AppHandle<R>,
    payload: SelectRequest,
) -> Result<SelectResponse> {
    app.sql_transaction().select(payload).await
}

#[command]
pub(crate) async fn begin_transaction<R: Runtime>(
    app: AppHandle<R>,
    payload: BeginTransactionRequest,
) -> Result<BeginTransactionResponse> {
    app.sql_transaction().begin(payload).await
}

#[command]
pub(crate) async fn execute_in_transaction<R: Runtime>(
    app: AppHandle<R>,
    payload: TransactionExecuteRequest,
) -> Result<ExecuteResponse> {
    app.sql_transaction().execute_in_tx(payload).await
}

#[command]
pub(crate) async fn commit<R: Runtime>(
    app: AppHandle<R>,
    payload: CommitRequest,
) -> Result<AckResponse> {
    app.sql_transaction().commit(payload).await
}

#[command]
pub(crate) async fn rollback<R: Runtime>(
    app: AppHandle<R>,
    payload: RollbackRequest,
) -> Result<AckResponse> {
    app.sql_transaction().rollback(payload).await
}
