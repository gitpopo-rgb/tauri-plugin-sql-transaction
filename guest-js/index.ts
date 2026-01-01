import { invoke } from '@tauri-apps/api/core'

export type DbHandle = string

export interface ExecuteResult {
  rowsAffected: number
  lastInsertId?: string | null
}

export interface ExecuteOptions {
  values?: unknown[]
}

export interface SelectRow {
  [key: string]: unknown
}

export async function ping(value?: string): Promise<string | null> {
  const res = await invoke<{ value?: string }>('plugin:sql-transaction|ping', {
    payload: { value },
  })
  return res.value ?? null
}

export async function connect(url: string): Promise<DbHandle> {
  const res = await invoke<{ handle: string }>('plugin:sql-transaction|connect', {
    payload: { url },
  })
  return res.handle
}

export async function execute(
  db: DbHandle,
  query: string,
  options: ExecuteOptions = {},
): Promise<ExecuteResult> {
  const res = await invoke<{ rowsAffected: number; lastInsertId?: string }>(
    'plugin:sql-transaction|execute',
    {
      payload: { db, query, values: options.values ?? [] },
    },
  )
  return { rowsAffected: res.rowsAffected, lastInsertId: res.lastInsertId ?? null }
}

export async function select<T extends SelectRow = SelectRow>(
  db: DbHandle,
  query: string,
  options: ExecuteOptions = {},
): Promise<T[]> {
  const res = await invoke<{ rows: T[] }>('plugin:sql-transaction|select', {
    payload: { db, query, values: options.values ?? [] },
  })
  return res.rows
}

export class Transaction {
  constructor(private readonly txId: string) {}

  async execute(query: string, options: ExecuteOptions = {}): Promise<ExecuteResult> {
    const res = await invoke<{ rowsAffected: number; lastInsertId?: string }>(
      'plugin:sql-transaction|execute_in_transaction',
      {
        payload: { txId: this.txId, query, values: options.values ?? [] },
      },
    )
    return { rowsAffected: res.rowsAffected, lastInsertId: res.lastInsertId ?? null }
  }

  async commit(): Promise<void> {
    await invoke('plugin:sql-transaction|commit', { payload: { txId: this.txId } })
  }

  async rollback(): Promise<void> {
    await invoke('plugin:sql-transaction|rollback', { payload: { txId: this.txId } })
  }
}

export async function begin(db: DbHandle): Promise<Transaction> {
  const res = await invoke<{ txId: string }>('plugin:sql-transaction|begin_transaction', {
    payload: { db },
  })
  return new Transaction(res.txId)
}

/**
 * Run a transactional function with automatic commit/rollback.
 */
export async function transaction<T>(
  db: DbHandle,
  fn: (tx: Transaction) => Promise<T>,
): Promise<T> {
  const tx = await begin(db)
  try {
    const result = await fn(tx)
    await tx.commit()
    return result
  } catch (err) {
    try {
      await tx.rollback()
    } catch (_) {
      // swallow rollback error, surface original
    }
    throw err
  }
}
