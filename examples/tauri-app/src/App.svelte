<script>
  import Greet from './lib/Greet.svelte'
  import { connect } from 'tauri-plugin-sql-transaction-api'

	let response = $state('')
	let db = $state(null)

	function updateResponse(returnValue) {
		response += `[${new Date().toLocaleTimeString()}] ` + (typeof returnValue === 'string' ? returnValue : JSON.stringify(returnValue)) + '<br>'
	}

	async function initDb() {
		try {
			db = await connect('sqlite:demo.db')
			updateResponse('Connected to SQLite database')
			
			// Create table
			await db.execute('CREATE TABLE IF NOT EXISTS users (id INTEGER PRIMARY KEY, name TEXT NOT NULL, email TEXT)')
			updateResponse('Table created')
		} catch (error) {
			updateResponse('Error: ' + error)
		}
	}

	async function insertUser() {
		if (!db) {
			updateResponse('Please connect to database first')
			return
		}
		
		try {
			const result = await db.execute(
				'INSERT INTO users (name, email) VALUES (?, ?)',
				{ values: ['Alice', 'alice@example.com'] }
			)
			updateResponse(`Inserted user, ID: ${result.lastInsertId}`)
		} catch (error) {
			updateResponse('Error: ' + error)
		}
	}

	async function selectUsers() {
		if (!db) {
			updateResponse('Please connect to database first')
			return
		}
		
		try {
			const rows = await db.select('SELECT * FROM users')
			updateResponse(`Found ${rows.length} users: ${JSON.stringify(rows)}`)
		} catch (error) {
			updateResponse('Error: ' + error)
		}
	}

	async function testTransaction() {
		if (!db) {
			updateResponse('Please connect to database first')
			return
		}
		
		try {
			// Using automatic transaction helper
			const result = await db.transaction(async (tx) => {
				const r1 = await tx.execute(
					'INSERT INTO users (name, email) VALUES (?, ?)',
					{ values: ['Bob', 'bob@example.com'] }
				)
				
				const r2 = await tx.execute(
					'INSERT INTO users (name, email) VALUES (?, ?)',
					{ values: ['Charlie', 'charlie@example.com'] }
				)
				
				return { id1: r1.lastInsertId, id2: r2.lastInsertId }
			})
			
			updateResponse(`Transaction committed: ${JSON.stringify(result)}`)
		} catch (error) {
			updateResponse('Transaction rolled back: ' + error)
		}
	}

	async function testRollback() {
		if (!db) {
			updateResponse('Please connect to database first')
			return
		}
		
		try {
			const tx = await db.begin()
			
			await tx.execute(
				'INSERT INTO users (name, email) VALUES (?, ?)',
				{ values: ['Dave', 'dave@example.com'] }
			)
			updateResponse('Inserted Dave, rolling back...')
			
			await tx.rollback()
			updateResponse('Transaction rolled back')
			
			const rows = await db.select('SELECT * FROM users WHERE name = ?', { values: ['Dave'] })
			updateResponse(`Dave should not exist: ${rows.length === 0 ? 'PASS' : 'FAIL'}`)
		} catch (error) {
			updateResponse('Error: ' + error)
		}
	}

	async function clearData() {
		if (!db) {
			updateResponse('Please connect to database first')
			return
		}
		
		try {
			await db.execute('DELETE FROM users')
			updateResponse('All users deleted')
		} catch (error) {
			updateResponse('Error: ' + error)
		}
	}
</script>

<main class="container">
  <h1>SQL Transaction Demo</h1>

  <div class="row">
    <a href="https://vite.dev" target="_blank">
      <img src="/vite.svg" class="logo vite" alt="Vite Logo" />
    </a>
    <a href="https://tauri.app" target="_blank">
      <img src="/tauri.svg" class="logo tauri" alt="Tauri Logo" />
    </a>
    <a href="https://svelte.dev" target="_blank">
      <img src="/svelte.svg" class="logo svelte" alt="Svelte Logo" />
    </a>
  </div>

  <p>
    Test SQL transactions with SQLite database.
  </p>

  <div class="row">
    <Greet />
  </div>

  <div class="demo-section">
    <h2>Database Operations</h2>
    
    <div class="button-group">
      <button onclick="{initDb}">1. Connect & Init DB</button>
      <button onclick="{insertUser}">2. Insert User</button>
      <button onclick="{selectUsers}">3. Select Users</button>
    </div>
    
    <div class="button-group">
      <button onclick="{testTransaction}">4. Test Transaction (Commit)</button>
      <button onclick="{testRollback}">5. Test Rollback</button>
      <button onclick="{clearData}">6. Clear Data</button>
    </div>
    
    <div class="output">
      <h3>Output:</h3>
      <div class="log">{@html response}</div>
    </div>
  </div>

</main>

<style>
  .logo.vite:hover {
    filter: drop-shadow(0 0 2em #747bff);
  }

  .logo.svelte:hover {
    filter: drop-shadow(0 0 2em #ff3e00);
  }

  .demo-section {
    margin-top: 2rem;
    width: 100%;
  }

  .button-group {
    display: flex;
    gap: 0.5rem;
    flex-wrap: wrap;
    margin: 1rem 0;
  }

  .button-group button {
    padding: 0.6rem 1rem;
    font-size: 0.9rem;
  }

  .output {
    margin-top: 2rem;
    text-align: left;
  }

  .output h3 {
    margin-bottom: 0.5rem;
  }

  .log {
    background: #1a1a1a;
    border: 1px solid #333;
    border-radius: 4px;
    padding: 1rem;
    min-height: 200px;
    max-height: 400px;
    overflow-y: auto;
    font-family: monospace;
    font-size: 0.85rem;
    line-height: 1.5;
  }
</style>
