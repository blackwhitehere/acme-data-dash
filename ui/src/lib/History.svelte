<script>
  import { onMount } from 'svelte';
  
  let history = [];
  let loading = true;

  onMount(async () => {
    try {
      const res = await fetch('/api/history');
      history = await res.json();
    } catch (e) {
      console.error(e);
    } finally {
      loading = false;
    }
  });
</script>

<div class="history">
  <h2>Execution History</h2>
  
  {#if loading}
    <p>Loading...</p>
  {:else if history.length === 0}
    <p>No history available.</p>
  {:else}
    <table>
      <thead>
        <tr>
          <th>Time</th>
          <th>Check ID</th>
          <th>Status</th>
        </tr>
      </thead>
      <tbody>
        {#each history as entry}
          <tr>
            <td>{new Date(entry.executed_at).toLocaleString()}</td>
            <td>{entry.check_id}</td>
            <td><span class="status-badge status-{entry.status}">{entry.status}</span></td>
          </tr>
        {/each}
      </tbody>
    </table>
  {/if}
</div>

<style>
  table {
    width: 100%;
    border-collapse: collapse;
    background: white;
    border-radius: 8px;
    overflow: hidden;
    box-shadow: 0 2px 4px rgba(0,0,0,0.1);
  }
  
  th, td {
    padding: 1rem;
    text-align: left;
    border-bottom: 1px solid #eee;
  }
  
  th {
    background-color: #f8f9fa;
    font-weight: 600;
  }
</style>
