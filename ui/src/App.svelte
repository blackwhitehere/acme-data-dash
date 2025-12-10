<script>
  import { onMount } from 'svelte';
  import CheckList from './lib/CheckList.svelte';
  import CheckRunner from './lib/CheckRunner.svelte';
  import History from './lib/History.svelte';
  import DataSources from './lib/DataSources.svelte';

  let view = 'dashboard'; // dashboard, history, data-sources
  let selectedCheck = null;
  let checks = [];

  onMount(async () => {
    try {
      const res = await fetch('/api/checks');
      checks = await res.json();
    } catch (e) {
      console.error("Failed to fetch checks", e);
    }
  });

  function goHome() {
    view = 'dashboard';
    selectedCheck = null;
  }

  function selectCheck(event) {
    selectedCheck = event.detail;
  }
</script>

<nav>
  <h1>Acme Data Dash</h1>
  <button class:active={view === 'dashboard'} on:click={goHome}>Dashboard</button>
  <button class:active={view === 'history'} on:click={() => { view = 'history'; selectedCheck = null; }}>History</button>
  <button class:active={view === 'data-sources'} on:click={() => { view = 'data-sources'; selectedCheck = null; }}>Data Sources</button>
</nav>

<main>
  {#if view === 'dashboard'}
    {#if selectedCheck}
      <CheckRunner check={selectedCheck} onBack={() => selectedCheck = null} />
    {:else}
      <CheckList {checks} on:select={selectCheck} />
    {/if}
  {:else if view === 'history'}
    <History />
  {:else if view === 'data-sources'}
    <DataSources />
  {/if}
</main>
