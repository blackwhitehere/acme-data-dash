<script>
  import { createEventDispatcher, onMount } from 'svelte';
  export let checks = [];
  
  const dispatch = createEventDispatcher();
  let checkStatuses = {};

  onMount(async () => {
    await loadCheckStatuses();
  });

  async function loadCheckStatuses() {
    try {
      const res = await fetch('/api/check-statuses');
      if (res.ok) {
        checkStatuses = await res.json();
      }
    } catch (e) {
      console.error('Failed to fetch check statuses', e);
    }
  }

  function selectCheck(check) {
    dispatch('select', check);
  }

  function getStatusClass(status) {
    if (!status) return 'unknown';
    const s = status.toLowerCase();
    if (s.includes('pass') || s.includes('success')) return 'success';
    if (s.includes('fail') || s.includes('error')) return 'error';
    if (s.includes('warn')) return 'warning';
    return 'unknown';
  }

  function formatTimestamp(timestamp) {
    if (!timestamp) return 'Never run';
    const date = new Date(timestamp);
    return date.toLocaleString();
  }
</script>

<div class="check-list">
  <h2>Data Quality Checks</h2>
  {#if checks.length === 0}
    <p class="empty-state">No checks available.</p>
  {:else}
    <div class="table-container">
      <table>
        <thead>
          <tr>
            <th>Check ID</th>
            <th>Description</th>
            <th>Last Status</th>
            <th>Last Run</th>
            <th>Action</th>
          </tr>
        </thead>
        <tbody>
          {#each checks as check}
            {@const status = checkStatuses[check.id]}
            <tr>
              <td class="check-id">{check.id}</td>
              <td class="description">{check.description}</td>
              <td>
                <span class="status-badge {getStatusClass(status?.status)}">
                  {status?.status || 'Not Run'}
                </span>
              </td>
              <td class="timestamp">{formatTimestamp(status?.executed_at)}</td>
              <td>
                <button class="run-button" on:click={() => selectCheck(check)}>
                  Run Check
                </button>
              </td>
            </tr>
          {/each}
        </tbody>
      </table>
    </div>
  {/if}
</div>

<style>
  .check-list {
    padding: 1rem;
  }

  h2 {
    margin-bottom: 1.5rem;
    color: #333;
  }

  .empty-state {
    text-align: center;
    color: #999;
    font-style: italic;
    padding: 2rem;
  }

  .table-container {
    overflow-x: auto;
    background: white;
    border-radius: 8px;
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
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
    padding: 1rem;
    text-align: left;
    font-weight: 600;
    color: #495057;
    font-size: 0.9rem;
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }

  tbody tr {
    border-bottom: 1px solid #e9ecef;
    transition: background-color 0.2s;
  }

  tbody tr:hover {
    background-color: #f8f9fa;
  }

  td {
    padding: 1rem;
    color: #333;
  }

  .check-id {
    font-family: 'Courier New', monospace;
    font-weight: 500;
    color: #007bff;
  }

  .description {
    max-width: 400px;
    line-height: 1.5;
  }

  .timestamp {
    color: #6c757d;
    font-size: 0.9rem;
    white-space: nowrap;
  }

  .status-badge {
    display: inline-block;
    padding: 0.35rem 0.75rem;
    border-radius: 12px;
    font-size: 0.85rem;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }

  .status-badge.success {
    background-color: #d4edda;
    color: #155724;
  }

  .status-badge.error {
    background-color: #f8d7da;
    color: #721c24;
  }

  .status-badge.warning {
    background-color: #fff3cd;
    color: #856404;
  }

  .status-badge.unknown {
    background-color: #e2e3e5;
    color: #6c757d;
  }

  .run-button {
    padding: 0.5rem 1rem;
    background-color: #007bff;
    color: white;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    font-weight: 500;
    font-size: 0.9rem;
    transition: background-color 0.2s;
  }

  .run-button:hover {
    background-color: #0056b3;
  }

  @media (max-width: 768px) {
    .table-container {
      overflow-x: scroll;
    }

    th, td {
      padding: 0.75rem 0.5rem;
      font-size: 0.85rem;
    }
  }
</style>
