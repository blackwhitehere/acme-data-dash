<script>
  import { createEventDispatcher } from 'svelte';
  export let checks = [];
  
  const dispatch = createEventDispatcher();

  function selectCheck(check) {
    dispatch('select', check);
  }
</script>

<div class="check-list">
  <h2>Available Checks</h2>
  {#if checks.length === 0}
    <p>No checks available.</p>
  {:else}
    <div class="grid">
      {#each checks as check}
        <div class="card">
          <h3>{check.id}</h3>
          <p>{check.description}</p>
          <button class="primary" on:click={() => selectCheck(check)}>Run Check</button>
        </div>
      {/each}
    </div>
  {/if}
</div>

<style>
  .grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
    gap: 1rem;
  }
  
  h3 {
    margin-top: 0;
  }
</style>
