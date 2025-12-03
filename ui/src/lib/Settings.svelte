<script>
  import { onMount } from 'svelte';

  let connections = [];
  let secrets = [];
  let activeTab = 'connections';

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
    }
  }

  async function loadSecrets() {
    const res = await fetch('/api/secrets');
    if (res.ok) {
      secrets = await res.json();
    }
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
    } else {
      message = 'Error saving connection';
    }
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
    } else {
      message = 'Error saving secret';
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
      <h3>Connections</h3>
      <ul>
        {#each connections as conn}
          <li>
            <strong>{conn.name}</strong> ({conn.driver})
            <br>
            <small>{conn.connection_string_template}</small>
            {#if conn.secret_ref}
              <br>
              <small>Secret: {conn.secret_ref}</small>
            {/if}
          </li>
        {/each}
      </ul>

      <h4>Add Connection</h4>
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
        <button type="submit">Save Connection</button>
      </form>
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
  }
  .tabs button.active {
    background-color: #007bff;
    color: white;
  }
  .section {
    border: 1px solid #ccc;
    padding: 1rem;
    border-radius: 4px;
  }
  form {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    max-width: 400px;
    margin-top: 1rem;
  }
  label {
    display: flex;
    flex-direction: column;
  }
  input, select {
    padding: 0.5rem;
  }
  button[type="submit"] {
    margin-top: 0.5rem;
    padding: 0.5rem;
    background-color: #28a745;
    color: white;
    border: none;
    cursor: pointer;
  }
  .message {
    padding: 0.5rem;
    background-color: #d4edda;
    color: #155724;
    margin-bottom: 1rem;
    border-radius: 4px;
  }
</style>
