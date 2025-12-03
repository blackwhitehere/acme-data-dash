<script>
  import { onMount } from 'svelte';
  import CheckList from './lib/CheckList.svelte';
  import CheckRunner from './lib/CheckRunner.svelte';
  import History from './lib/History.svelte';
  import Settings from './lib/Settings.svelte';

  let view = 'dashboard'; // dashboard, history, settings
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
  <a href="#" class:active={view === 'dashboard'} on:click|preventDefault={goHome}>Dashboard</a>
  <a href="#" class:active={view === 'history'} on:click|preventDefault={() => { view = 'history'; selectedCheck = null; }}>History</a>
  <a href="#" class:active={view === 'settings'} on:click|preventDefault={() => { view = 'settings'; selectedCheck = null; }}>Settings</a>
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
  {:else if view === 'settings'}
    <Settings />
  {/if}
</main>
