<script>
  import { onMount } from 'svelte';

  let connections = [];
  let secrets = [];
  let activeTab = 'connections';
  let editMode = 'form'; // 'form' or 'direct'
  let showAddForm = false;
  let odbcIniContent = '';
  let odbcIniEdited = '';

  let newConnection = {
    name: '',
    driver: 'sqlite',
    connection_string_template: '',
    secret_ref: ''
  };

  let newSecret = {
    key: '',
    value: ''
  };

  let message = '';

  onMount(async () => {
    await loadConnections();
    await loadSecrets();
  });

  async function loadConnections() {
    const res = await fetch('/api/connections');
    if (res.ok) {
      connections = await res.json();
      generateOdbcIni();
    }
  }

  async function loadSecrets() {
    const res = await fetch('/api/secrets');
    if (res.ok) {
      secrets = await res.json();
    }
  }

  function generateOdbcIni() {
    let ini = '';
    connections.forEach(conn => {
      ini += `[${conn.name}]\n`;
      ini += `Driver = ${conn.driver}\n`;
      
      // Parse connection string template into key-value pairs
      const parts = conn.connection_string_template.split(';').filter(p => p.trim());
      parts.forEach(part => {
        const [key, value] = part.split('=').map(s => s.trim());
        if (key && value) {
          ini += `${key} = ${value}\n`;
        }
      });
      
      if (conn.secret_ref) {
        ini += `# Secret Reference: ${conn.secret_ref}\n`;
      }
      ini += '\n';
    });
    odbcIniContent = ini;
    odbcIniEdited = ini;
  }

  async function saveConnection() {
    const payload = { ...newConnection };
    if (payload.secret_ref === '') payload.secret_ref = null;

    const res = await fetch('/api/connections', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(payload)
    });

    if (res.ok) {
      message = 'Connection saved!';
      await loadConnections();
      newConnection = { name: '', driver: 'sqlite', connection_string_template: '', secret_ref: '' };
      showAddForm = false;
      setTimeout(() => message = '', 3000);
    } else {
      message = 'Error saving connection';
      setTimeout(() => message = '', 3000);
    }
  }

  async function saveOdbcIniDirect() {
    // Parse the ODBC INI format and convert back to connection profiles
    const sections = odbcIniEdited.split(/\n\[/).filter(s => s.trim());
    const newConnections = [];
    
    for (let section of sections) {
      const lines = section.split('\n').filter(l => l.trim() && !l.trim().startsWith('#'));
      if (lines.length === 0) continue;
      
      let name = lines[0].replace(/^\[|\]$/g, '').trim();
      let driver = '';
      let connStr = '';
      let secretRef = null;
      
      for (let i = 1; i < lines.length; i++) {
        const line = lines[i].trim();
        if (line.startsWith('#')) {
          const match = line.match(/Secret Reference:\s*(.+)/);
          if (match) secretRef = match[1].trim();
          continue;
        }
        
        const [key, ...valueParts] = line.split('=');
        const value = valueParts.join('=').trim();
        
        if (key.trim().toLowerCase() === 'driver') {
          driver = value;
        } else if (key && value) {
          if (connStr) connStr += ';';
          connStr += `${key.trim()}=${value}`;
        }
      }
      
      if (name && driver) {
        newConnections.push({ name, driver, connection_string_template: connStr, secret_ref: secretRef });
      }
    }
    
    // Save all connections
    for (const conn of newConnections) {
      await fetch('/api/connections', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(conn)
      });
    }
    
    message = 'Connections saved from ODBC.ini!';
    await loadConnections();
    setTimeout(() => message = '', 3000);
  }

  async function saveSecret() {
    const res = await fetch('/api/secrets', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(newSecret)
    });

    if (res.ok) {
      message = 'Secret saved!';
      await loadSecrets();
      newSecret = { key: '', value: '' };
      setTimeout(() => message = '', 3000);
    } else {
      message = 'Error saving secret';
      setTimeout(() => message = '', 3000);
    }
  }
</script>

<div class="settings">
  <h2>Settings</h2>
  
  {#if message}
    <div class="message">{message}</div>
  {/if}

  <div class="tabs">
    <button class:active={activeTab === 'connections'} on:click={() => activeTab = 'connections'}>Connections</button>
    <button class:active={activeTab === 'secrets'} on:click={() => activeTab = 'secrets'}>Secrets</button>
  </div>

  {#if activeTab === 'connections'}
    <div class="section">
      <div class="header-row">
        <h3>Connection Profiles</h3>
        <div class="mode-toggle">
          <button 
            class:active={editMode === 'form'} 
            on:click={() => editMode = 'form'}
          >
            Form Mode
          </button>
          <button 
            class:active={editMode === 'direct'} 
            on:click={() => editMode = 'direct'}
          >
            Edit ODBC.ini
          </button>
        </div>
      </div>

      {#if editMode === 'form'}
        <div class="form-mode">
          <div class="odbc-preview">
            <h4>ODBC.ini Preview</h4>
            <pre>{odbcIniContent || '# No connections configured'}</pre>
          </div>

          <div class="connection-list">
            <h4>Configured Connections ({connections.length})</h4>
            {#if connections.length === 0}
              <p class="empty-state">No connections configured yet.</p>
            {:else}
              <ul>
                {#each connections as conn}
                  <li>
                    <strong>{conn.name}</strong> ({conn.driver})
                    <br>
                    <small>{conn.connection_string_template}</small>
                    {#if conn.secret_ref}
                      <br>
                      <small class="secret-ref">🔒 Secret: {conn.secret_ref}</small>
                    {/if}
                  </li>
                {/each}
              </ul>
            {/if}
          </div>

          {#if !showAddForm}
            <button class="add-button" on:click={() => showAddForm = true}>
              + Add New Connection
            </button>
          {/if}

          {#if showAddForm}
            <div class="add-form">
              <h4>Add New Connection</h4>
              <form on:submit|preventDefault={saveConnection}>
                <label>
                  Name:
                  <input type="text" bind:value={newConnection.name} required />
                </label>
                <label>
                  Driver:
                  <select bind:value={newConnection.driver}>
                    <option value="sqlite">SQLite</option>
                    <option value="postgres">PostgreSQL</option>
                    <option value="mysql">MySQL</option>
                    <option value="odbc">ODBC</option>
                  </select>
                </label>
                <label>
                  Connection String Template:
                  <input type="text" bind:value={newConnection.connection_string_template} required placeholder="e.g. DSN=mydb;UID=user;PWD={'{{PASSWORD}}'}" />
                </label>
                <label>
                  Secret Reference (Optional):
                  <select bind:value={newConnection.secret_ref}>
                    <option value="">None</option>
                    {#each secrets as secret}
                      <option value={secret}>{secret}</option>
                    {/each}
                  </select>
                </label>
                <div class="form-actions">
                  <button type="submit">Save Connection</button>
                  <button type="button" class="cancel" on:click={() => {
                    showAddForm = false;
                    newConnection = { name: '', driver: 'sqlite', connection_string_template: '', secret_ref: '' };
                  }}>Cancel</button>
                </div>
              </form>
            </div>
          {/if}
        </div>
      {:else}
        <div class="direct-mode">
          <h4>Edit ODBC.ini Directly</h4>
          <p class="help-text">
            Edit the ODBC configuration file format below. Use <code>[SectionName]</code> for each connection.
            Changes will update all connection profiles when saved.
          </p>
          <textarea bind:value={odbcIniEdited} rows="20"></textarea>
          <div class="form-actions">
            <button class="save-odbc" on:click={saveOdbcIniDirect}>Save Changes</button>
            <button class="cancel" on:click={() => odbcIniEdited = odbcIniContent}>Reset</button>
          </div>
        </div>
      {/if}
    </div>
  {/if}

  {#if activeTab === 'secrets'}
    <div class="section">
      <h3>Secrets</h3>
      <ul>
        {#each secrets as secret}
          <li>{secret}</li>
        {/each}
      </ul>

      <h4>Add Secret</h4>
      <form on:submit|preventDefault={saveSecret}>
        <label>
          Key:
          <input type="text" bind:value={newSecret.key} required />
        </label>
        <label>
          Value:
          <input type="password" bind:value={newSecret.value} required />
        </label>
        <button type="submit">Save Secret</button>
      </form>
    </div>
  {/if}
</div>

<style>
  .settings {
    padding: 1rem;
  }
  .tabs {
    margin-bottom: 1rem;
  }
  .tabs button {
    padding: 0.5rem 1rem;
    margin-right: 0.5rem;
    cursor: pointer;
    border: 1px solid #ccc;
    background: white;
    border-radius: 4px 4px 0 0;
  }
  .tabs button.active {
    background-color: #007bff;
    color: white;
    border-color: #007bff;
  }
  .section {
    border: 1px solid #ccc;
    padding: 1.5rem;
    border-radius: 4px;
    background: white;
  }
  .header-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1rem;
  }
  .header-row h3 {
    margin: 0;
  }
  .mode-toggle {
    display: flex;
    gap: 0.5rem;
  }
  .mode-toggle button {
    padding: 0.4rem 0.8rem;
    border: 1px solid #ddd;
    background: white;
    cursor: pointer;
    border-radius: 4px;
    font-size: 0.9rem;
  }
  .mode-toggle button.active {
    background-color: #007bff;
    color: white;
    border-color: #007bff;
  }
  .odbc-preview {
    background: #f8f9fa;
    padding: 1rem;
    border-radius: 4px;
    margin-bottom: 1.5rem;
    border: 1px solid #e0e0e0;
  }
  .odbc-preview h4 {
    margin-top: 0;
    color: #555;
  }
  .odbc-preview pre {
    background: #2d2d2d;
    color: #f8f8f2;
    padding: 1rem;
    border-radius: 4px;
    overflow-x: auto;
    font-family: 'Courier New', monospace;
    font-size: 0.9rem;
    margin: 0;
  }
  .connection-list {
    margin-bottom: 1.5rem;
  }
  .connection-list h4 {
    margin-bottom: 0.75rem;
    color: #555;
  }
  .connection-list ul {
    list-style: none;
    padding: 0;
    margin: 0;
  }
  .connection-list li {
    padding: 0.75rem;
    border: 1px solid #e0e0e0;
    border-radius: 4px;
    margin-bottom: 0.5rem;
    background: #fafafa;
  }
  .connection-list li strong {
    color: #333;
  }
  .connection-list li small {
    color: #666;
  }
  .secret-ref {
    color: #28a745 !important;
    font-weight: 500;
  }
  .empty-state {
    color: #999;
    font-style: italic;
    padding: 1rem;
    text-align: center;
  }
  .add-button {
    padding: 0.75rem 1.5rem;
    background-color: #007bff;
    color: white;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    font-size: 1rem;
    font-weight: 500;
    display: block;
    width: 100%;
    max-width: 300px;
  }
  .add-button:hover {
    background-color: #0056b3;
  }
  .add-form {
    margin-top: 1.5rem;
    padding: 1.5rem;
    background: #f8f9fa;
    border-radius: 4px;
    border: 1px solid #e0e0e0;
  }
  .add-form h4 {
    margin-top: 0;
    color: #555;
  }
  .direct-mode {
    margin-top: 1rem;
  }
  .direct-mode h4 {
    margin-bottom: 0.5rem;
    color: #555;
  }
  .help-text {
    color: #666;
    font-size: 0.9rem;
    margin-bottom: 1rem;
    line-height: 1.5;
  }
  .help-text code {
    background: #f0f0f0;
    padding: 0.2rem 0.4rem;
    border-radius: 3px;
    font-family: monospace;
  }
  textarea {
    width: 100%;
    padding: 0.75rem;
    border: 1px solid #ddd;
    border-radius: 4px;
    font-family: 'Courier New', monospace;
    font-size: 0.9rem;
    resize: vertical;
    background: #2d2d2d;
    color: #f8f8f2;
  }
  form {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }
  label {
    display: flex;
    flex-direction: column;
    font-weight: 500;
    color: #555;
  }
  input, select {
    padding: 0.6rem;
    border: 1px solid #ddd;
    border-radius: 4px;
    font-size: 0.95rem;
    margin-top: 0.25rem;
  }
  input:focus, select:focus, textarea:focus {
    outline: none;
    border-color: #007bff;
    box-shadow: 0 0 0 2px rgba(0, 123, 255, 0.1);
  }
  .form-actions {
    display: flex;
    gap: 0.75rem;
    margin-top: 1rem;
  }
  button[type="submit"], .save-odbc {
    padding: 0.6rem 1.2rem;
    background-color: #28a745;
    color: white;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    font-weight: 500;
  }
  button[type="submit"]:hover, .save-odbc:hover {
    background-color: #218838;
  }
  .cancel {
    padding: 0.6rem 1.2rem;
    background-color: #6c757d;
    color: white;
    border: none;
    border-radius: 4px;
    cursor: pointer;
  }
  .cancel:hover {
    background-color: #5a6268;
  }
  .message {
    padding: 0.75rem 1rem;
    background-color: #d4edda;
    color: #155724;
    margin-bottom: 1rem;
    border-radius: 4px;
    border: 1px solid #c3e6cb;
    font-weight: 500;
  }
</style>
