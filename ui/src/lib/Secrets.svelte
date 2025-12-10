<script>
  import { onMount, createEventDispatcher } from 'svelte';

  const dispatch = createEventDispatcher();

  let activeTab = 'environment';
  let envSecrets = [];
  let fileSecrets = [];
  let unixGroups = [];
  let message = '';

  let showEnvForm = false;
  let showFileForm = false;
  let showGroupForm = false;

  let newEnvSecret = {
    key: '',
    value: ''
  };

  let newFileSecret = {
    name: '',
    file_path: '',
    format: 'key-value'
  };

  let newUnixGroup = {
    group_name: '',
    file_path: '',
    permissions: '640'
  };

  onMount(async () => {
    await loadSecrets();
  });

  async function loadSecrets() {
    try {
      const res = await fetch('/api/secrets');
      if (res.ok) {
        const allSecrets = await res.json();
        // For now, treat all as environment secrets
        // In a full implementation, we'd categorize by type
        envSecrets = allSecrets.map(key => ({ key, source: 'environment' }));
        dispatch('update');
      }
    } catch (e) {
      console.error('Failed to load secrets', e);
    }

    // Load Unix groups
    try {
      const res = await fetch('/api/unix-groups');
      if (res.ok) {
        unixGroups = await res.json();
      }
    } catch (e) {
      console.error('Failed to load Unix groups', e);
    }
  }

  async function saveEnvSecret() {
    try {
      const res = await fetch('/api/secrets', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(newEnvSecret)
      });

      if (res.ok) {
        message = 'Environment variable secret saved!';
        await loadSecrets();
        newEnvSecret = { key: '', value: '' };
        showEnvForm = false;
        setTimeout(() => message = '', 3000);
      } else {
        message = 'Error saving secret';
        setTimeout(() => message = '', 3000);
      }
    } catch (e) {
      message = `Error: ${e.message}`;
      setTimeout(() => message = '', 3000);
    }
  }

  async function saveFileSecret() {
    try {
      // Save file path as a special type of secret
      const payload = {
        key: `file:${newFileSecret.name}`,
        value: JSON.stringify({
          path: newFileSecret.file_path,
          format: newFileSecret.format
        })
      };

      const res = await fetch('/api/secrets', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(payload)
      });

      if (res.ok) {
        message = 'File-based secret configuration saved!';
        await loadSecrets();
        newFileSecret = { name: '', file_path: '', format: 'key-value' };
        showFileForm = false;
        setTimeout(() => message = '', 3000);
      } else {
        message = 'Error saving file secret';
        setTimeout(() => message = '', 3000);
      }
    } catch (e) {
      message = `Error: ${e.message}`;
      setTimeout(() => message = '', 3000);
    }
  }

  async function deleteSecret(key) {
    if (!confirm(`Delete secret "${key}"?`)) return;

    try {
      const res = await fetch(`/api/secrets/${encodeURIComponent(key)}`, {
        method: 'DELETE'
      });

      if (res.ok) {
        message = 'Secret deleted!';
        await loadSecrets();
        setTimeout(() => message = '', 3000);
      }
    } catch (e) {
      message = `Error: ${e.message}`;
      setTimeout(() => message = '', 3000);
    }
  }

  async function saveUnixGroup() {
    try {
      const res = await fetch('/api/unix-groups', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(newUnixGroup)
      });

      if (res.ok) {
        message = 'Unix group configuration saved!';
        await loadSecrets();
        newUnixGroup = { group_name: '', file_path: '', permissions: '640' };
        showGroupForm = false;
        setTimeout(() => message = '', 3000);
      } else {
        message = 'Error saving Unix group';
        setTimeout(() => message = '', 3000);
      }
    } catch (e) {
      message = `Error: ${e.message}`;
      setTimeout(() => message = '', 3000);
    }
  }

  async function deleteUnixGroup(groupName) {
    if (!confirm(`Delete Unix group configuration "${groupName}"?`)) return;

    try {
      const res = await fetch(`/api/unix-groups/${encodeURIComponent(groupName)}`, {
        method: 'DELETE'
      });

      if (res.ok) {
        message = 'Unix group configuration deleted!';
        await loadSecrets();
        setTimeout(() => message = '', 3000);
      }
    } catch (e) {
      message = `Error: ${e.message}`;
      setTimeout(() => message = '', 3000);
    }
  }
</script>

<div class="secrets">
  <h3>Secrets</h3>

  {#if message}
    <div class="message">{message}</div>
  {/if}

  <div class="sub-tabs">
    <button 
      class:active={activeTab === 'environment'} 
      on:click={() => activeTab = 'environment'}
    >
      Environment Variables
      <span class="tooltip">ⓘ
        <span class="tooltip-text">Configure environment variables to hold secret values like passwords and API keys.</span>
      </span>
    </button>
    <button 
      class:active={activeTab === 'files'} 
      on:click={() => activeTab = 'files'}
    >
      Files
      <span class="tooltip">ⓘ
        <span class="tooltip-text">Load secrets from key-value formatted files.</span>
      </span>
    </button>
    <button 
      class:active={activeTab === 'groups'} 
      on:click={() => activeTab = 'groups'}
    >
      Unix Groups
      <span class="tooltip">ⓘ
        <span class="tooltip-text">Configure Unix group permissions for secret files to control read access.</span>
      </span>
    </button>
    <button 
      class:active={activeTab === 'managers'} 
      on:click={() => activeTab = 'managers'}
    >
      Secret Managers
      <span class="tooltip">ⓘ
        <span class="tooltip-text">Integration with secret management services like AWS Secrets Manager, HashiCorp Vault, etc.</span>
      </span>
    </button>
  </div>

  <div class="sub-content">
    {#if activeTab === 'environment'}
      <h4>Environment Variable Secrets ({envSecrets.length})</h4>
      <p class="help-text">
        Configure environment variables that will hold secret values. These can be referenced in connection configurations.
      </p>

      {#if envSecrets.length === 0}
        <p class="empty-state">No environment variable secrets configured.</p>
      {:else}
        <table>
          <thead>
            <tr>
              <th>Key</th>
              <th>Source</th>
              <th>Actions</th>
            </tr>
          </thead>
          <tbody>
            {#each envSecrets as secret}
              <tr>
                <td class="key">
                  <span class="secret-icon">🔒</span>
                  {secret.key}
                </td>
                <td class="source">Environment Variable</td>
                <td>
                  <button class="delete-button" on:click={() => deleteSecret(secret.key)}>
                    Delete
                  </button>
                </td>
              </tr>
            {/each}
          </tbody>
        </table>
      {/if}

      {#if !showEnvForm}
        <button class="add-button" on:click={() => showEnvForm = true}>
          + Add Environment Variable Secret
        </button>
      {/if}

      {#if showEnvForm}
        <div class="add-form">
          <h4>Add Environment Variable Secret</h4>
          <form on:submit|preventDefault={saveEnvSecret}>
            <label>
              Secret Key (Environment Variable Name):
              <input 
                type="text" 
                bind:value={newEnvSecret.key} 
                required 
                placeholder="DATABASE_PASSWORD" 
              />
              <small>Use uppercase with underscores (e.g., API_KEY, DB_PASSWORD)</small>
            </label>
            <label>
              Secret Value:
              <input 
                type="password" 
                bind:value={newEnvSecret.value} 
                required 
                placeholder="Your secret value" 
              />
              <small>This value will be stored securely</small>
            </label>
            <div class="form-actions">
              <button type="submit">Save Secret</button>
              <button type="button" class="cancel" on:click={() => {
                showEnvForm = false;
                newEnvSecret = { key: '', value: '' };
              }}>Cancel</button>
            </div>
          </form>
        </div>
      {/if}

    {:else if activeTab === 'files'}
      <h4>File-Based Secrets ({fileSecrets.length})</h4>
      <p class="help-text">
        Configure files that contain secrets in key-value format. The system will read secrets from these files.
      </p>

      {#if fileSecrets.length === 0}
        <p class="empty-state">No file-based secrets configured.</p>
      {:else}
        <table>
          <thead>
            <tr>
              <th>Name</th>
              <th>File Path</th>
              <th>Format</th>
              <th>Actions</th>
            </tr>
          </thead>
          <tbody>
            {#each fileSecrets as secret}
              <tr>
                <td class="key">
                  <span class="secret-icon">📄</span>
                  {secret.name}
                </td>
                <td class="path">{secret.file_path}</td>
                <td>{secret.format}</td>
                <td>
                  <button class="delete-button" on:click={() => deleteSecret(secret.key)}>
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
          + Add File-Based Secret
        </button>
      {/if}

      {#if showFileForm}
        <div class="add-form">
          <h4>Add File-Based Secret</h4>
          <form on:submit|preventDefault={saveFileSecret}>
            <label>
              Configuration Name:
              <input 
                type="text" 
                bind:value={newFileSecret.name} 
                required 
                placeholder="my-secrets" 
              />
            </label>
            <label>
              File Path:
              <input 
                type="text" 
                bind:value={newFileSecret.file_path} 
                required 
                placeholder="/path/to/secrets.env" 
              />
              <small>Absolute or relative path to the secrets file</small>
            </label>
            <label>
              File Format:
              <select bind:value={newFileSecret.format}>
                <option value="key-value">Key-Value (.env format)</option>
                <option value="json">JSON</option>
                <option value="yaml">YAML</option>
              </select>
              <small>Format of the secrets file</small>
            </label>
            <div class="form-actions">
              <button type="submit">Save Configuration</button>
              <button type="button" class="cancel" on:click={() => {
                showFileForm = false;
                newFileSecret = { name: '', file_path: '', format: 'key-value' };
              }}>Cancel</button>
            </div>
          </form>
        </div>
      {/if}

    {:else if activeTab === 'groups'}
      <h4>Unix Group Permissions ({unixGroups.length})</h4>
      <p class="help-text">
        Configure Unix group permissions to control which groups can read secret files. This is useful for managing file-system level access control.
      </p>

      {#if unixGroups.length === 0}
        <p class="empty-state">No Unix group configurations.</p>
      {:else}
        <table>
          <thead>
            <tr>
              <th>Group Name</th>
              <th>File Path</th>
              <th>Permissions</th>
              <th>Actions</th>
            </tr>
          </thead>
          <tbody>
            {#each unixGroups as group}
              <tr>
                <td class="key">
                  <span class="secret-icon">👥</span>
                  {group.group_name}
                </td>
                <td class="path">{group.file_path}</td>
                <td class="permissions">{group.permissions}</td>
                <td>
                  <button class="delete-button" on:click={() => deleteUnixGroup(group.group_name)}>
                    Delete
                  </button>
                </td>
              </tr>
            {/each}
          </tbody>
        </table>
      {/if}

      {#if !showGroupForm}
        <button class="add-button" on:click={() => showGroupForm = true}>
          + Add Unix Group Configuration
        </button>
      {/if}

      {#if showGroupForm}
        <div class="add-form">
          <h4>Add Unix Group Configuration</h4>
          <div class="info-banner">
            <strong>ℹ️ About Unix Groups:</strong> Unix groups control file access at the operating system level. 
            By assigning a group to a secret file, only users in that group can read it.
          </div>
          <form on:submit|preventDefault={saveUnixGroup}>
            <label>
              Group Name:
              <span class="tooltip">ⓘ
                <span class="tooltip-text">The Unix group name that should have read access (e.g., "secrets-readers", "db-admins"). Must exist on the system.</span>
              </span>
              <input 
                type="text" 
                bind:value={newUnixGroup.group_name} 
                required 
                placeholder="secrets-readers" 
              />
              <small>Use an existing Unix group name from your system</small>
            </label>
            <label>
              Secret File Path:
              <span class="tooltip">ⓘ
                <span class="tooltip-text">The absolute path to the secret file whose permissions you want to configure.</span>
              </span>
              <input 
                type="text" 
                bind:value={newUnixGroup.file_path} 
                required 
                placeholder="/etc/secrets/database.env" 
              />
              <small>Absolute path to the file (e.g., /etc/secrets/api-keys.env)</small>
            </label>
            <label>
              File Permissions:
              <span class="tooltip">ⓘ
                <span class="tooltip-text">Unix file permissions in octal notation. 640 means owner can read/write, group can read, others have no access.</span>
              </span>
              <select bind:value={newUnixGroup.permissions}>
                <option value="640">640 (Owner: RW, Group: R, Others: -)</option>
                <option value="644">644 (Owner: RW, Group: R, Others: R)</option>
                <option value="600">600 (Owner: RW, Group: -, Others: -)</option>
                <option value="660">660 (Owner: RW, Group: RW, Others: -)</option>
                <option value="440">440 (Owner: R, Group: R, Others: -)</option>
              </select>
              <small>
                Recommended: <strong>640</strong> (owner can read/write, group can read)
              </small>
            </label>
            <div class="warning-banner">
              <strong>⚠️ Important:</strong> Changing file permissions requires appropriate system privileges. 
              Make sure the application has permission to modify the file ownership and permissions.
            </div>
            <div class="form-actions">
              <button type="submit">Save Configuration</button>
              <button type="button" class="cancel" on:click={() => {
                showGroupForm = false;
                newUnixGroup = { group_name: '', file_path: '', permissions: '640' };
              }}>Cancel</button>
            </div>
          </form>
        </div>
      {/if}

    {:else if activeTab === 'managers'}
      <div class="tba-notice">
        <div class="tba-icon">🔧</div>
        <h4>Secret Managers Integration</h4>
        <p>
          Integration with external secret management services is coming soon.
          This will support:
        </p>
        <ul>
          <li>AWS Secrets Manager</li>
          <li>HashiCorp Vault</li>
          <li>Azure Key Vault</li>
          <li>Google Cloud Secret Manager</li>
        </ul>
        <p class="status">Status: <strong>To Be Announced (TBA)</strong></p>
      </div>
    {/if}
  </div>
</div>

<style>
  .secrets {
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

  .help-text {
    color: #6c757d;
    font-size: 0.9rem;
    margin-bottom: 1.5rem;
    line-height: 1.5;
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

  .key {
    font-family: 'Courier New', monospace;
    font-weight: 600;
    color: #007bff;
  }

  .secret-icon {
    margin-right: 0.5rem;
  }

  .source {
    color: #6c757d;
    font-size: 0.9rem;
  }

  .path {
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

  button[type="submit"] {
    padding: 0.6rem 1.2rem;
    background-color: #28a745;
    color: white;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    font-weight: 500;
  }

  button[type="submit"]:hover {
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

  .tba-notice {
    text-align: center;
    padding: 3rem 2rem;
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    border-radius: 8px;
    color: white;
  }

  .tba-icon {
    font-size: 4rem;
    margin-bottom: 1rem;
  }

  .tba-notice h4 {
    color: white;
    font-size: 1.5rem;
    margin-bottom: 1rem;
  }

  .tba-notice p {
    font-size: 1rem;
    line-height: 1.6;
    margin-bottom: 1rem;
    opacity: 0.95;
  }

  .tba-notice ul {
    list-style: none;
    padding: 0;
    margin: 1.5rem auto;
    max-width: 400px;
  }

  .tba-notice li {
    padding: 0.5rem;
    margin: 0.5rem 0;
    background: rgba(255, 255, 255, 0.15);
    border-radius: 4px;
    font-weight: 500;
  }

  .tba-notice .status {
    margin-top: 1.5rem;
    font-size: 1.1rem;
  }

  .tba-notice strong {
    color: #ffd700;
  }

  .permissions {
    font-family: 'Courier New', monospace;
    font-size: 0.9rem;
    color: #28a745;
    font-weight: 600;
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

  .warning-banner {
    background: #fff3cd;
    border-left: 4px solid #ffc107;
    padding: 1rem;
    margin-top: 1rem;
    border-radius: 4px;
    color: #856404;
    font-size: 0.9rem;
    line-height: 1.5;
  }
</style>
