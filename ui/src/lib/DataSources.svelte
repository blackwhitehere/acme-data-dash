<script>
  import { onMount } from 'svelte';
  import Connections from './Connections.svelte';
  import Secrets from './Secrets.svelte';

  let activeTab = 'data-sources';
  let dataSources = [];
  let connections = [];
  let secrets = [];
  let showAddForm = false;
  let message = '';

  let newDataSource = {
    name: '',
    connection_name: '',
    secret_key: ''
  };

  onMount(async () => {
    await loadAll();
  });

  async function loadAll() {
    await Promise.all([
      loadDataSources(),
      loadConnections(),
      loadSecrets()
    ]);
  }

  async function loadDataSources() {
    try {
      const res = await fetch('/api/data-sources');
      if (res.ok) {
        dataSources = await res.json();
      }
    } catch (e) {
      console.error('Failed to load data sources', e);
    }
  }

  async function loadConnections() {
    try {
      const res = await fetch('/api/connections');
      if (res.ok) {
        connections = await res.json();
      }
    } catch (e) {
      console.error('Failed to load connections', e);
    }
  }

  async function loadSecrets() {
    try {
      const res = await fetch('/api/secrets');
      if (res.ok) {
        secrets = await res.json();
      }
    } catch (e) {
      console.error('Failed to load secrets', e);
    }
  }

  async function saveDataSource() {
    try {
      const res = await fetch('/api/data-sources', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(newDataSource)
      });

      if (res.ok) {
        message = 'Data source configured successfully!';
        await loadDataSources();
        newDataSource = { name: '', connection_name: '', secret_key: '' };
        showAddForm = false;
        setTimeout(() => message = '', 3000);
      } else {
        const error = await res.text();
        message = `Error: ${error}`;
        setTimeout(() => message = '', 5000);
      }
    } catch (e) {
      message = `Error: ${e.message}`;
      setTimeout(() => message = '', 5000);
    }
  }

  async function deleteDataSource(name) {
    if (!confirm(`Delete data source "${name}"?`)) return;

    try {
      const res = await fetch(`/api/data-sources/${encodeURIComponent(name)}`, {
        method: 'DELETE'
      });

      if (res.ok) {
        message = 'Data source deleted!';
        await loadDataSources();
        setTimeout(() => message = '', 3000);
      }
    } catch (e) {
      message = `Error: ${e.message}`;
      setTimeout(() => message = '', 5000);
    }
  }
</script>

<div class="data-sources">
  <h2>Data Sources</h2>

  {#if message}
    <div class="message">{message}</div>
  {/if}

  <div class="tabs">
    <button 
      class:active={activeTab === 'data-sources'} 
      on:click={() => activeTab = 'data-sources'}
    >
      Data Sources
      <span class="tooltip">ⓘ
        <span class="tooltip-text">A Data Source combines a Connection with a Secret to provide complete access credentials</span>
      </span>
    </button>
    <button 
      class:active={activeTab === 'connections'} 
      on:click={() => activeTab = 'connections'}
    >
      Connections
    </button>
    <button 
      class:active={activeTab === 'secrets'} 
      on:click={() => activeTab = 'secrets'}
    >
      Secrets
    </button>
  </div>

  <div class="content">
    {#if activeTab === 'data-sources'}
      <div class="section">
        <div class="section-header">
          <h3>Configured Data Sources</h3>
          <p class="help-text">
            A valid Data Source is a combination of a Connection and Secret. 
            The Secret values will bind to placeholders in the Connection definition.
          </p>
        </div>

        {#if dataSources.length === 0}
          <p class="empty-state">No data sources configured yet.</p>
        {:else}
          <div class="table-container">
            <table>
              <thead>
                <tr>
                  <th>Name</th>
                  <th>Connection</th>
                  <th>Secret</th>
                  <th>Status</th>
                  <th>Actions</th>
                </tr>
              </thead>
              <tbody>
                {#each dataSources as ds}
                  <tr>
                    <td class="name">{ds.name}</td>
                    <td>{ds.connection_name}</td>
                    <td>
                      <span class="secret-ref">🔒 {ds.secret_key}</span>
                    </td>
                    <td>
                      <span class="status-badge {ds.is_valid ? 'valid' : 'invalid'}">
                        {ds.is_valid ? 'Valid' : 'Invalid'}
                      </span>
                    </td>
                    <td>
                      <button class="delete-button" on:click={() => deleteDataSource(ds.name)}>
                        Delete
                      </button>
                    </td>
                  </tr>
                {/each}
              </tbody>
            </table>
          </div>
        {/if}

        {#if !showAddForm}
          <button class="add-button" on:click={() => showAddForm = true}>
            + Add New Data Source
          </button>
        {/if}

        {#if showAddForm}
          <div class="add-form">
            <h4>Configure New Data Source</h4>
            <form on:submit|preventDefault={saveDataSource}>
              <label>
                Data Source Name:
                <input 
                  type="text" 
                  bind:value={newDataSource.name} 
                  placeholder="my-database" 
                  required 
                />
              </label>

              <label>
                Select Connection:
                <select bind:value={newDataSource.connection_name} required>
                  <option value="">-- Choose Connection --</option>
                  {#each connections as conn}
                    <option value={conn.name}>
                      {conn.name} ({conn.driver})
                    </option>
                  {/each}
                </select>
                {#if connections.length === 0}
                  <small class="hint">No connections available. Create one in the Connections tab first.</small>
                {/if}
              </label>

              <label>
                Select Secret:
                <select bind:value={newDataSource.secret_key} required>
                  <option value="">-- Choose Secret --</option>
                  {#each secrets as secret}
                    <option value={secret}>{secret}</option>
                  {/each}
                </select>
                {#if secrets.length === 0}
                  <small class="hint">No secrets available. Create one in the Secrets tab first.</small>
                {/if}
              </label>

              <div class="form-actions">
                <button type="submit" disabled={connections.length === 0 || secrets.length === 0}>
                  Configure Data Source
                </button>
                <button 
                  type="button" 
                  class="cancel" 
                  on:click={() => {
                    showAddForm = false;
                    newDataSource = { name: '', connection_name: '', secret_key: '' };
                  }}
                >
                  Cancel
                </button>
              </div>
            </form>
          </div>
        {/if}
      </div>
    {:else if activeTab === 'connections'}
      <Connections on:update={loadConnections} />
    {:else if activeTab === 'secrets'}
      <Secrets on:update={loadSecrets} />
    {/if}
  </div>
</div>

<style>
  .data-sources {
    padding: 1rem;
  }

  h2 {
    margin-bottom: 1.5rem;
    color: #333;
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

  .tabs {
    display: flex;
    gap: 0.5rem;
    margin-bottom: 1.5rem;
    border-bottom: 2px solid #dee2e6;
  }

  .tabs button {
    padding: 0.75rem 1.5rem;
    background: none;
    border: none;
    border-bottom: 3px solid transparent;
    cursor: pointer;
    font-weight: 500;
    color: #6c757d;
    transition: all 0.2s;
    position: relative;
  }

  .tabs button:hover {
    color: #495057;
    background-color: #f8f9fa;
  }

  .tabs button.active {
    color: #007bff;
    border-bottom-color: #007bff;
  }

  .tooltip {
    display: inline-block;
    margin-left: 0.25rem;
    cursor: help;
    position: relative;
  }

  .tooltip-text {
    visibility: hidden;
    width: 300px;
    background-color: #333;
    color: #fff;
    text-align: left;
    border-radius: 6px;
    padding: 0.75rem;
    position: absolute;
    z-index: 1000;
    bottom: 125%;
    left: 50%;
    margin-left: -150px;
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

  .content {
    background: white;
    border-radius: 8px;
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
  }

  .section {
    padding: 1.5rem;
  }

  .section-header {
    margin-bottom: 1.5rem;
  }

  .section-header h3 {
    margin: 0 0 0.5rem 0;
    color: #333;
  }

  .help-text {
    color: #6c757d;
    font-size: 0.9rem;
    margin: 0;
    line-height: 1.5;
  }

  .empty-state {
    text-align: center;
    color: #999;
    font-style: italic;
    padding: 2rem;
    background: #f8f9fa;
    border-radius: 4px;
  }

  .table-container {
    overflow-x: auto;
    margin-bottom: 1.5rem;
  }

  table {
    width: 100%;
    border-collapse: collapse;
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
    letter-spacing: 0.5px;
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

  .secret-ref {
    color: #28a745;
    font-weight: 500;
    font-size: 0.9rem;
  }

  .status-badge {
    display: inline-block;
    padding: 0.25rem 0.6rem;
    border-radius: 12px;
    font-size: 0.8rem;
    font-weight: 600;
    text-transform: uppercase;
  }

  .status-badge.valid {
    background-color: #d4edda;
    color: #155724;
  }

  .status-badge.invalid {
    background-color: #f8d7da;
    color: #721c24;
  }

  .delete-button {
    padding: 0.4rem 0.8rem;
    background-color: #dc3545;
    color: white;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    font-size: 0.85rem;
    font-weight: 500;
  }

  .delete-button:hover {
    background-color: #c82333;
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
    margin: 0 0 1rem 0;
    color: #333;
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

  input, select {
    padding: 0.6rem;
    border: 1px solid #ced4da;
    border-radius: 4px;
    font-size: 0.95rem;
    margin-top: 0.25rem;
  }

  input:focus, select:focus {
    outline: none;
    border-color: #007bff;
    box-shadow: 0 0 0 2px rgba(0, 123, 255, 0.1);
  }

  .hint {
    color: #dc3545;
    font-size: 0.85rem;
    margin-top: 0.25rem;
    font-style: italic;
  }

  .form-actions {
    display: flex;
    gap: 0.75rem;
    margin-top: 0.5rem;
  }

  button[type="submit"] {
    padding: 0.6rem 1.2rem;
    background-color: #28a745;
    color: white;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    font-weight: 500;
  }

  button[type="submit"]:hover:not(:disabled) {
    background-color: #218838;
  }

  button[type="submit"]:disabled {
    background-color: #6c757d;
    cursor: not-allowed;
    opacity: 0.6;
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
</style>
