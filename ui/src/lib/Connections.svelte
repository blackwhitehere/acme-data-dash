<script>
  import { onMount, createEventDispatcher } from 'svelte';

  const dispatch = createEventDispatcher();

  let activeTab = 'databases';
  let databases = [];
  let apis = [];
  let files = [];
  let message = '';

  let showDatabaseForm = false;
  let showApiForm = false;
  let showFileForm = false;

  let odbcIniContent = '';
  let odbcInstContent = '[ODBC Drivers]\n# Add your drivers here\n\n[Driver Configurations]\n# Driver-specific settings';
  let editMode = 'form'; // 'form' or 'direct'
  let odbcIniEdited = '';

  let newDatabase = {
    name: '',
    driver: 'sqlite',
    driver_path: '',
    dsn: '',
    server: '',
    port: '',
    database: '',
    connection_params: ''
  };

  let driverMode = 'existing'; // 'existing' or 'new'
  let availableDrivers = [
    { name: 'SQLite', value: 'sqlite', defaultPath: '/usr/lib/x86_64-linux-gnu/odbc/libsqlite3odbc.so' },
    { name: 'PostgreSQL', value: 'postgresql', defaultPath: '/usr/local/lib/psqlodbcw.so' },
    { name: 'MySQL', value: 'mysql', defaultPath: '/usr/local/lib/libmyodbc8w.so' },
    { name: 'SQL Server', value: 'sqlserver', defaultPath: '/opt/microsoft/msodbcsql18/lib64/libmsodbcsql-18.3.so' },
    { name: 'Oracle', value: 'oracle', defaultPath: '/opt/oracle/instantclient_21_1/libsqora.so.21.1' },
    { name: 'DuckDB', value: 'duckdb', defaultPath: '/usr/local/lib/libduckdb_odbc.so' }
  ];

  $: selectedDriver = availableDrivers.find(d => d.value === newDatabase.driver);

  let newApi = {
    name: '',
    openapi_spec: ''
  };

  let newFile = {
    name: '',
    file_path: ''
  };

  onMount(async () => {
    await loadConnections();
  });

  async function loadConnections() {
    try {
      const res = await fetch('/api/connections');
      if (res.ok) {
        const connections = await res.json();
        // Categorize connections by type
        databases = connections.filter(c => c.connection_type === 'database' || !c.connection_type);
        apis = connections.filter(c => c.connection_type === 'api');
        files = connections.filter(c => c.connection_type === 'file');
        generateOdbcIni();
        dispatch('update');
      }
    } catch (e) {
      console.error('Failed to load connections', e);
    }
  }

  function generateOdbcIni() {
    let ini = '# ODBC Data Sources\n[ODBC Data Sources]\n';
    let inst = '[ODBC Drivers]\n';
    
    // Collect unique drivers
    const usedDrivers = new Map(); // name -> path

    databases.forEach(db => {
      ini += `${db.name} = ${db.driver}\n`;
      
      // Try to find driver path in connection string
      let driverPath = '';
      if (db.connection_string_template) {
        const match = db.connection_string_template.match(/Driver=([^;]+)/i);
        if (match) {
          driverPath = match[1];
        }
      }
      
      // Fallback to default path if not found
      if (!driverPath) {
        const defaultDriver = availableDrivers.find(d => d.value === db.driver);
        if (defaultDriver) {
          driverPath = defaultDriver.defaultPath;
        }
      }
      
      if (db.driver && driverPath) {
        usedDrivers.set(db.driver, driverPath);
      }
    });
    ini += '\n';

    // Generate odbcinst.ini content
    usedDrivers.forEach((path, name) => {
      inst += `${name} = Installed\n`;
    });
    inst += '\n';
    
    usedDrivers.forEach((path, name) => {
      inst += `[${name}]\n`;
      inst += `Description = ${name} ODBC Driver\n`;
      inst += `Driver = ${path}\n\n`;
    });

    databases.forEach(db => {
      ini += `[${db.name}]\n`;
      ini += `Driver = ${db.driver}\n`;
      
      if (db.connection_string_template) {
        const parts = db.connection_string_template.split(';').filter(p => p.trim());
        parts.forEach(part => {
          const [key, value] = part.split('=').map(s => s?.trim());
          // Skip Driver and DSN as they are handled by section name and Driver line
          if (key && value && key.toLowerCase() !== 'driver' && key.toLowerCase() !== 'dsn') {
            ini += `${key} = ${value}\n`;
          }
        });
      }
      
      ini += '\n';
    });

    odbcIniContent = ini;
    odbcIniEdited = ini;
    odbcInstContent = inst;
  }

  async function saveDatabase() {
    try {
      // Build connection string with all parameters
      let parts = [`DSN=${newDatabase.dsn}`];
      
      if (driverMode === 'new' && newDatabase.driver_path) {
        parts.push(`Driver=${newDatabase.driver_path}`);
      }
      
      if (newDatabase.server) parts.push(`Server=${newDatabase.server}`);
      if (newDatabase.port) parts.push(`Port=${newDatabase.port}`);
      if (newDatabase.database) parts.push(`Database=${newDatabase.database}`);
      if (newDatabase.connection_params) {
        parts.push(newDatabase.connection_params);
      }
      
      const connectionString = parts.join(';');

      const payload = {
        name: newDatabase.dsn || newDatabase.name,
        driver: newDatabase.driver,
        connection_string_template: connectionString,
        connection_type: 'database',
        secret_ref: null
      };

      const res = await fetch('/api/connections', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(payload)
      });

      if (res.ok) {
        message = 'Database connection saved!';
        await loadConnections();
        newDatabase = { name: '', driver: 'sqlite', driver_path: '', dsn: '', server: '', port: '', database: '', connection_params: '' };
        driverMode = 'existing';
        showDatabaseForm = false;
        setTimeout(() => message = '', 3000);
      } else {
        message = 'Error saving database connection';
        setTimeout(() => message = '', 3000);
      }
    } catch (e) {
      message = `Error: ${e.message}`;
      setTimeout(() => message = '', 3000);
    }
  }

  async function saveOdbcIniDirect() {
    try {
      // Parse the edited ODBC.ini content
      const lines = odbcIniEdited.split('\n');
      const dsnSections = [];
      let currentSection = null;
      
      for (const line of lines) {
        const trimmed = line.trim();
        
        // Skip comments and empty lines
        if (!trimmed || trimmed.startsWith('#') || trimmed === '[ODBC Data Sources]') continue;
        
        // Check if it's a section header
        const sectionMatch = trimmed.match(/^\[(.+)\]$/);
        if (sectionMatch) {
          // Save previous section if it exists
          if (currentSection && currentSection.name && currentSection.driver) {
            dsnSections.push(currentSection);
          }
          // Start new section
          currentSection = {
            name: sectionMatch[1].trim(),
            driver: '',
            params: []
          };
          continue;
        }
        
        // Parse key-value pairs
        if (currentSection && trimmed.includes('=')) {
          const eqIndex = trimmed.indexOf('=');
          const key = trimmed.substring(0, eqIndex).trim();
          const value = trimmed.substring(eqIndex + 1).trim();
          
          if (key.toLowerCase() === 'driver') {
            currentSection.driver = value;
          } else if (key && value) {
            currentSection.params.push(`${key}=${value}`);
          }
        }
      }
      
      // Don't forget the last section
      if (currentSection && currentSection.name && currentSection.driver) {
        dsnSections.push(currentSection);
      }
      
      // Save each DSN to the backend
      for (const dsn of dsnSections) {
        // Build connection string with DSN parameter
        const connParams = [`DSN=${dsn.name}`, ...dsn.params];
        const connectionString = connParams.join(';');
        
        await fetch('/api/connections', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({
            name: dsn.name,
            driver: dsn.driver,
            connection_string_template: connectionString,
            connection_type: 'database',
            secret_ref: null
          })
        });
      }
      
      message = `Successfully saved ${dsnSections.length} DSN(s) from ODBC.ini!`;
      await loadConnections();
      setTimeout(() => message = '', 3000);
    } catch (e) {
      message = `Error: ${e.message}`;
      setTimeout(() => message = '', 3000);
    }
  }

  async function saveApi() {
    try {
      const payload = {
        name: newApi.name,
        driver: 'openapi',
        connection_string_template: newApi.openapi_spec,
        connection_type: 'api',
        secret_ref: null
      };

      const res = await fetch('/api/connections', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(payload)
      });

      if (res.ok) {
        message = 'API connection saved!';
        await loadConnections();
        newApi = { name: '', openapi_spec: '' };
        showApiForm = false;
        setTimeout(() => message = '', 3000);
      } else {
        message = 'Error saving API connection';
        setTimeout(() => message = '', 3000);
      }
    } catch (e) {
      message = `Error: ${e.message}`;
      setTimeout(() => message = '', 3000);
    }
  }

  async function saveFile() {
    try {
      const payload = {
        name: newFile.name,
        driver: 'file',
        connection_string_template: newFile.file_path,
        connection_type: 'file',
        secret_ref: null
      };

      const res = await fetch('/api/connections', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(payload)
      });

      if (res.ok) {
        message = 'File connection saved!';
        await loadConnections();
        newFile = { name: '', file_path: '' };
        showFileForm = false;
        setTimeout(() => message = '', 3000);
      } else {
        message = 'Error saving file connection';
        setTimeout(() => message = '', 3000);
      }
    } catch (e) {
      message = `Error: ${e.message}`;
      setTimeout(() => message = '', 3000);
    }
  }

  async function deleteConnection(name) {
    if (!confirm(`Delete connection "${name}"?`)) return;

    try {
      const res = await fetch(`/api/connections/${encodeURIComponent(name)}`, {
        method: 'DELETE'
      });

      if (res.ok) {
        message = 'Connection deleted!';
        await loadConnections();
        setTimeout(() => message = '', 3000);
      }
    } catch (e) {
      message = `Error: ${e.message}`;
      setTimeout(() => message = '', 3000);
    }
  }
</script>

<div class="connections">
  <h3>Connections</h3>

  {#if message}
    <div class="message">{message}</div>
  {/if}

  <div class="sub-tabs">
    <button 
      class:active={activeTab === 'databases'} 
      on:click={() => activeTab = 'databases'}
    >
      Databases
      <span class="tooltip">ⓘ
        <span class="tooltip-text">Configure database connections using ODBC format. Requires DSN and Driver configuration.</span>
      </span>
    </button>
    <button 
      class:active={activeTab === 'apis'} 
      on:click={() => activeTab = 'apis'}
    >
      APIs
      <span class="tooltip">ⓘ
        <span class="tooltip-text">Define API endpoints using OpenAPI Specification (OAS) format.</span>
      </span>
    </button>
    <button 
      class:active={activeTab === 'files'} 
      on:click={() => activeTab = 'files'}
    >
      Files
      <span class="tooltip">ⓘ
        <span class="tooltip-text">Configure file-based data sources by specifying file paths.</span>
      </span>
    </button>
  </div>

  <div class="sub-content">
    {#if activeTab === 'databases'}
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
          Edit ODBC Files
        </button>
      </div>

      {#if editMode === 'form'}
        <div class="form-mode">
          <h4>Database Connections ({databases.length})</h4>
          
          {#if databases.length === 0}
            <p class="empty-state">No database connections configured.</p>
          {:else}
            <table>
              <thead>
                <tr>
                  <th>Name</th>
                  <th>Driver</th>
                  <th>Connection String</th>
                  <th>Actions</th>
                </tr>
              </thead>
              <tbody>
                {#each databases as db}
                  <tr>
                    <td class="name">{db.name}</td>
                    <td>{db.driver}</td>
                    <td class="connection-string">{db.connection_string_template}</td>
                    <td>
                      <button class="delete-button" on:click={() => deleteConnection(db.name)}>
                        Delete
                      </button>
                    </td>
                  </tr>
                {/each}
              </tbody>
            </table>
          {/if}

          {#if !showDatabaseForm}
            <button class="add-button" on:click={() => showDatabaseForm = true}>
              + Add Database Connection
            </button>
          {/if}

          {#if showDatabaseForm}
            <div class="add-form">
              <h4>Add Database Connection (ODBC)</h4>
              <div class="info-banner">
                <strong>💡 What is ODBC?</strong> ODBC (Open Database Connectivity) is a standard way to connect to databases. 
                Each connection needs a DSN (Data Source Name) and a driver.
              </div>
              <form on:submit|preventDefault={saveDatabase}>
                <label>
                  DSN (Data Source Name):
                  <span class="tooltip">ⓘ
                    <span class="tooltip-text">This is the unique identifier for your database connection. Choose a meaningful name like "ProductionDB" or "AnalyticsWarehouse". This name will appear in odbc.ini.</span>
                  </span>
                  <input type="text" bind:value={newDatabase.dsn} required placeholder="ProductionDB" />
                </label>

                <div class="driver-section">
                  <div class="section-header">
                    <strong>Driver Configuration:</strong>
                    <span class="tooltip">ⓘ
                      <span class="tooltip-text">Choose an existing driver if you've already installed one, or configure a new driver by providing its path.</span>
                    </span>
                  </div>
                  <div class="radio-group">
                    <label class="radio-label">
                      <input type="radio" bind:group={driverMode} value="existing" />
                      Use Existing Driver
                    </label>
                    <label class="radio-label">
                      <input type="radio" bind:group={driverMode} value="new" />
                      Configure New Driver
                    </label>
                  </div>
                  
                  {#if driverMode === 'existing'}
                    <select bind:value={newDatabase.driver}>
                      {#each availableDrivers as driver}
                        <option value={driver.value}>{driver.name}</option>
                      {/each}
                    </select>
                  {:else}
                    <div class="driver-config">
                      <label>
                        Driver Type:
                        <select bind:value={newDatabase.driver}>
                          {#each availableDrivers as driver}
                            <option value={driver.value}>{driver.name}</option>
                          {/each}
                        </select>
                      </label>
                      <label>
                        Driver Library Path:
                        <input 
                          type="text" 
                          bind:value={newDatabase.driver_path} 
                          placeholder={selectedDriver?.defaultPath || '/path/to/driver.so'}
                          required={driverMode === 'new'}
                        />
                        <small>Example: {selectedDriver?.defaultPath}</small>
                      </label>
                    </div>
                  {/if}
                </div>

                <div class="connection-details">
                  <h5>Connection Details</h5>
                  <div class="grid-2">
                    <label>
                      Server/Host:
                      <input type="text" bind:value={newDatabase.server} placeholder="localhost" />
                    </label>
                    <label>
                      Port:
                      <input type="text" bind:value={newDatabase.port} placeholder="5432" />
                    </label>
                  </div>
                  <label>
                    Database Name:
                    <input type="text" bind:value={newDatabase.database} placeholder="mydb" />
                  </label>
                </div>

                <label>
                  Additional Parameters:
                  <span class="tooltip">ⓘ
                    <span class="tooltip-text">Add extra connection settings as key=value pairs separated by semicolons. Use {'{'}{'{'} PASSWORD {'}'}{'}'}  or {'{'}{'{'} API_KEY {'}'}{'}'}  as placeholders for secrets that will be replaced at runtime.</span>
                  </span>
                  <textarea 
                    bind:value={newDatabase.connection_params} 
                    placeholder="UID=myuser;PWD={'{'}{'{'} PASSWORD {'}'}{'}'}; SSLMode=require;Encrypt=yes"
                    rows="3"
                  ></textarea>
                  <small>Use <code>{'{'}{'{'} PASSWORD {'}'}{'}'}  </code> or <code>{'{'}{'{'} API_KEY {'}'}{'}'}  </code> as placeholders for secrets</small>
                </label>

                <div class="form-actions">
                  <button type="submit">Save Database Connection</button>
                  <button type="button" class="cancel" on:click={() => {
                    showDatabaseForm = false;
                    newDatabase = { name: '', driver: 'sqlite', driver_path: '', dsn: '', server: '', port: '', database: '', connection_params: '' };
                    driverMode = 'existing';
                  }}>Cancel</button>
                </div>
              </form>
            </div>
          {/if}

          <div class="file-preview">
            <h4>Current ODBC Configuration</h4>
            <div class="preview-section">
              <h5>odbc.ini</h5>
              <pre>{odbcIniContent || '# No database connections configured'}</pre>
            </div>
            <div class="preview-section">
              <h5>odbcinst.ini</h5>
              <pre>{odbcInstContent}</pre>
            </div>
          </div>
        </div>
      {:else}
        <div class="direct-mode">
          <h4>Edit ODBC Configuration Files</h4>
          <p class="help-text">
            Edit the ODBC configuration files directly below. Changes will update all database connections when saved.
          </p>
          
          <div class="file-editor-large">
            <h5>odbc.ini</h5>
            <textarea bind:value={odbcIniEdited} rows="30"></textarea>
          </div>

          <div class="file-editor-large">
            <h5>odbcinst.ini (Read-only preview)</h5>
            <textarea value={odbcInstContent} rows="20" readonly></textarea>
          </div>

          <div class="form-actions">
            <button class="save-button" on:click={saveOdbcIniDirect}>Save ODBC Configuration</button>
            <button class="cancel" on:click={() => {
              odbcIniEdited = odbcIniContent;
              editMode = 'form';
            }}>Cancel</button>
          </div>
        </div>
      {/if}

    {:else if activeTab === 'apis'}
      <h4>API Connections ({apis.length})</h4>
      
      {#if apis.length === 0}
        <p class="empty-state">No API connections configured.</p>
      {:else}
        <table>
          <thead>
            <tr>
              <th>Name</th>
              <th>OpenAPI Specification</th>
              <th>Actions</th>
            </tr>
          </thead>
          <tbody>
            {#each apis as api}
              <tr>
                <td class="name">{api.name}</td>
                <td class="connection-string">
                  <a href={api.connection_string_template} target="_blank" rel="noopener noreferrer" class="api-link">
                    {api.connection_string_template}
                    <span class="external-icon">↗</span>
                  </a>
                </td>
                <td>
                  <button class="delete-button" on:click={() => deleteConnection(api.name)}>
                    Delete
                  </button>
                </td>
              </tr>
            {/each}
          </tbody>
        </table>
      {/if}

      {#if !showApiForm}
        <button class="add-button" on:click={() => showApiForm = true}>
          + Add API Connection
        </button>
      {/if}

      {#if showApiForm}
        <div class="add-form">
          <h4>Add API Connection</h4>
          <form on:submit|preventDefault={saveApi}>
            <label>
              Connection Name:
              <input type="text" bind:value={newApi.name} required placeholder="my-api" />
            </label>
            <label>
              OpenAPI Specification (JSON or YAML URL):
              <input 
                type="text" 
                bind:value={newApi.openapi_spec} 
                required 
                placeholder="https://api.example.com/openapi.json" 
              />
              <small>Provide URL to OpenAPI 3.0+ specification</small>
            </label>
            <div class="form-actions">
              <button type="submit">Save API Connection</button>
              <button type="button" class="cancel" on:click={() => {
                showApiForm = false;
                newApi = { name: '', openapi_spec: '' };
              }}>Cancel</button>
            </div>
          </form>
        </div>
      {/if}

    {:else if activeTab === 'files'}
      <h4>File Connections ({files.length})</h4>
      
      {#if files.length === 0}
        <p class="empty-state">No file connections configured.</p>
      {:else}
        <table>
          <thead>
            <tr>
              <th>Name</th>
              <th>File Path</th>
              <th>Actions</th>
            </tr>
          </thead>
          <tbody>
            {#each files as file}
              <tr>
                <td class="name">{file.name}</td>
                <td class="connection-string">{file.connection_string_template}</td>
                <td>
                  <button class="delete-button" on:click={() => deleteConnection(file.name)}>
                    Delete
                  </button>
                </td>
              </tr>
            {/each}
          </tbody>
        </table>
      {/if}

      {#if !showFileForm}
        <button class="add-button" on:click={() => showFileForm = true}>
          + Add File Connection
        </button>
      {/if}

      {#if showFileForm}
        <div class="add-form">
          <h4>Add File Connection</h4>
          <form on:submit|preventDefault={saveFile}>
            <label>
              Connection Name:
              <input type="text" bind:value={newFile.name} required placeholder="my-file" />
            </label>
            <label>
              File Path:
              <input 
                type="text" 
                bind:value={newFile.file_path} 
                required 
                placeholder="/path/to/data.csv" 
              />
              <small>Absolute or relative path to the data file</small>
            </label>
            <div class="form-actions">
              <button type="submit">Save File Connection</button>
              <button type="button" class="cancel" on:click={() => {
                showFileForm = false;
                newFile = { name: '', file_path: '' };
              }}>Cancel</button>
            </div>
          </form>
        </div>
      {/if}
    {/if}
  </div>
</div>

<style>
  .connections {
    padding: 0;
  }

  h3 {
    margin: 0 0 1rem 0;
    color: #333;
    font-size: 1.25rem;
  }

  h4 {
    margin: 0 0 1rem 0;
    color: #495057;
    font-size: 1.1rem;
  }

  h5 {
    margin: 0 0 0.5rem 0;
    color: #6c757d;
    font-size: 0.95rem;
    font-weight: 600;
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

  .sub-tabs {
    display: flex;
    gap: 0.5rem;
    margin-bottom: 1.5rem;
    border-bottom: 1px solid #e0e0e0;
  }

  .sub-tabs button {
    padding: 0.6rem 1.2rem;
    background: none;
    border: none;
    border-bottom: 2px solid transparent;
    cursor: pointer;
    font-weight: 500;
    color: #6c757d;
    transition: all 0.2s;
    position: relative;
  }

  .sub-tabs button:hover {
    color: #495057;
  }

  .sub-tabs button.active {
    color: #007bff;
    border-bottom-color: #007bff;
  }

  .tooltip {
    display: inline-block;
    margin-left: 0.25rem;
    cursor: help;
    position: relative;
    font-size: 0.85rem;
  }

  .tooltip-text {
    visibility: hidden;
    width: 250px;
    background-color: #333;
    color: #fff;
    text-align: left;
    border-radius: 6px;
    padding: 0.75rem;
    position: absolute;
    z-index: 1000;
    bottom: 125%;
    left: 50%;
    margin-left: -125px;
    opacity: 0;
    transition: opacity 0.3s;
    font-size: 0.85rem;
    font-weight: normal;
    line-height: 1.4;
  }

  .tooltip:hover .tooltip-text {
    visibility: visible;
    opacity: 1;
  }

  .sub-content {
    margin-top: 1rem;
  }

  .mode-toggle {
    display: flex;
    gap: 0.5rem;
    margin-bottom: 1.5rem;
  }

  .mode-toggle button {
    padding: 0.5rem 1rem;
    border: 1px solid #ddd;
    background: white;
    cursor: pointer;
    border-radius: 4px;
    font-size: 0.9rem;
    transition: all 0.2s;
  }

  .mode-toggle button.active {
    background-color: #007bff;
    color: white;
    border-color: #007bff;
  }

  .empty-state {
    text-align: center;
    color: #999;
    font-style: italic;
    padding: 2rem;
    background: #f8f9fa;
    border-radius: 4px;
    margin-bottom: 1.5rem;
  }

  table {
    width: 100%;
    border-collapse: collapse;
    margin-bottom: 1.5rem;
  }

  thead {
    background-color: #f8f9fa;
    border-bottom: 2px solid #dee2e6;
  }

  th {
    padding: 0.75rem;
    text-align: left;
    font-weight: 600;
    color: #495057;
    font-size: 0.85rem;
    text-transform: uppercase;
  }

  tbody tr {
    border-bottom: 1px solid #e9ecef;
  }

  tbody tr:hover {
    background-color: #f8f9fa;
  }

  td {
    padding: 0.75rem;
    color: #333;
  }

  .name {
    font-weight: 600;
    color: #007bff;
  }

  .connection-string {
    font-family: 'Courier New', monospace;
    font-size: 0.85rem;
    color: #495057;
  }

  .delete-button {
    padding: 0.4rem 0.8rem;
    background-color: #dc3545;
    color: white;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    font-size: 0.85rem;
  }

  .delete-button:hover {
    background-color: #c82333;
  }

  .add-button {
    padding: 0.6rem 1.2rem;
    background-color: #007bff;
    color: white;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    font-weight: 500;
    margin-bottom: 1.5rem;
  }

  .add-button:hover {
    background-color: #0056b3;
  }

  .add-form {
    margin-bottom: 1.5rem;
    padding: 1.5rem;
    background: #f8f9fa;
    border-radius: 4px;
    border: 1px solid #e0e0e0;
  }

  form {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  label {
    display: flex;
    flex-direction: column;
    font-weight: 500;
    color: #495057;
  }

  input, select, textarea {
    padding: 0.6rem;
    border: 1px solid #ced4da;
    border-radius: 4px;
    font-size: 0.95rem;
    margin-top: 0.25rem;
  }

  input:focus, select:focus, textarea:focus {
    outline: none;
    border-color: #007bff;
    box-shadow: 0 0 0 2px rgba(0, 123, 255, 0.1);
  }

  textarea {
    font-family: 'Courier New', monospace;
    font-size: 0.9rem;
    resize: vertical;
    background: #2d2d2d;
    color: #f8f8f2;
  }

  textarea[readonly] {
    background: #f8f9fa;
    color: #6c757d;
    cursor: not-allowed;
  }

  small {
    color: #6c757d;
    font-size: 0.85rem;
    margin-top: 0.25rem;
    font-weight: normal;
  }

  .form-actions {
    display: flex;
    gap: 0.75rem;
    margin-top: 0.5rem;
  }

  button[type="submit"], .save-button {
    padding: 0.6rem 1.2rem;
    background-color: #28a745;
    color: white;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    font-weight: 500;
  }

  button[type="submit"]:hover, .save-button:hover {
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

  .file-preview {
    margin-top: 2rem;
    padding: 1.5rem;
    background: #f8f9fa;
    border-radius: 4px;
    border: 1px solid #e0e0e0;
  }

  .preview-section {
    margin-bottom: 1.5rem;
  }

  .preview-section:last-child {
    margin-bottom: 0;
  }

  .preview-section pre {
    background: #2d2d2d;
    color: #f8f8f2;
    padding: 1rem;
    border-radius: 4px;
    overflow-x: auto;
    font-family: 'Courier New', monospace;
    font-size: 0.85rem;
    margin: 0;
  }

  .direct-mode {
    margin-top: 1rem;
  }

  .help-text {
    color: #6c757d;
    font-size: 0.9rem;
    margin-bottom: 1.5rem;
    line-height: 1.5;
  }

  .file-editor-large {
    margin-bottom: 1.5rem;
  }

  .file-editor-large textarea {
    min-height: 400px;
  }

  .info-banner {
    background: #e7f3ff;
    border-left: 4px solid #007bff;
    padding: 1rem;
    margin-bottom: 1.5rem;
    border-radius: 4px;
    color: #004085;
    font-size: 0.9rem;
    line-height: 1.5;
  }

  .driver-section {
    background: white;
    padding: 1rem;
    border-radius: 4px;
    border: 1px solid #dee2e6;
  }

  .section-header {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    margin-bottom: 0.75rem;
    color: #495057;
  }

  .radio-group {
    display: flex;
    gap: 1.5rem;
    margin: 0.75rem 0;
  }

  .radio-label {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    cursor: pointer;
    font-weight: normal;
  }

  .radio-label input[type="radio"] {
    margin: 0;
    cursor: pointer;
  }

  .driver-config {
    margin-top: 1rem;
    padding: 1rem;
    background: #f8f9fa;
    border-radius: 4px;
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .connection-details {
    background: white;
    padding: 1rem;
    border-radius: 4px;
    border: 1px solid #dee2e6;
  }

  .connection-details h5 {
    margin: 0 0 1rem 0;
    color: #495057;
    font-size: 0.95rem;
    font-weight: 600;
  }

  .grid-2 {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 1rem;
  }

  code {
    background: #f8f9fa;
    padding: 0.2rem 0.4rem;
    border-radius: 3px;
    font-family: 'Courier New', monospace;
    font-size: 0.85rem;
    color: #d63384;
  }

  .api-link {
    color: #007bff;
    text-decoration: none;
    word-break: break-all;
  }

  .api-link:hover {
    text-decoration: underline;
  }

  .external-icon {
    margin-left: 0.25rem;
    font-size: 0.8rem;
    opacity: 0.7;
  }

  /* Fix table overflow */
  .sub-content {
    margin-top: 1rem;
    overflow-x: auto;
  }

  table {
    width: 100%;
    min-width: 600px;
    border-collapse: collapse;
    margin-bottom: 1.5rem;
  }

  .connection-string {
    font-family: 'Courier New', monospace;
    font-size: 0.85rem;
    color: #495057;
    max-width: 400px;
    word-break: break-all;
    overflow-wrap: break-word;
  }
</style>
