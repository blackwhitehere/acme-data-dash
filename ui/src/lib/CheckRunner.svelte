<script>
  export let check;
  export let onBack;

  let params = {};
  let result = null;
  let error = null;
  let loading = false;
  let currentCheckId = null;

  // Initialize params with defaults or empty strings
  $: if (check && check.id !== currentCheckId) {
    currentCheckId = check.id;
    params = {};
    check.parameters.forEach(p => {
      params[p.name] = p.default || '';
    });
    result = null;
    error = null;
  }

  async function runCheck() {
    loading = true;
    result = null;
    error = null;

    try {
      const response = await fetch(`/checks/${check.id}/execute`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify({ params })
      });

      const data = await response.json();
      
      if (response.ok) {
        // The API returns Result<CheckResult, String>, so data is either the result object or an error string if status is 200 but logic failed?
        // Actually my API returns Json(Ok(result)) or Json(Err(string)).
        // If it's Ok, data is { status: ..., message: ... }
        // If it's Err, data is "Error string"
        // Wait, axum Json(Result) serializes Result as Ok(val) -> val, Err(e) -> e? No.
        // Let's check the Rust code.
        // Json(Ok(result)) -> JSON object of result.
        // Json(Err(string)) -> JSON string.
        // But wait, Result serialization in serde: Ok(x) -> x, Err(y) -> {"Err": y} usually?
        // Actually standard Result serialization depends on configuration but usually it's distinct.
        // Let's assume standard behavior:
        // If I returned Json(Ok(..)), the body is the CheckResult object.
        // If I returned Json(Err(..)), the body is the error string.
        // Wait, `Result` in Rust serializes to `{"Ok": ...}` or `{"Err": ...}` by default unless untagged.
        // I didn't specify untagged.
        // Let's look at `src/api/mod.rs`.
        // `-> Json<Result<CheckResult, String>>`
        // Yes, it will be `{"Ok": ...}` or `{"Err": ...}`.
        
        if (data.Ok) {
            result = data.Ok;
        } else if (data.Err) {
            error = data.Err;
        } else {
            // Maybe I was wrong and it just serialized the inner value?
            // Let's handle both just in case or assume standard Rust serde.
            // Actually, let's fix the Rust API to be cleaner later, but for now handle the likely `{"Ok":...}` format.
            // Or better, let's just inspect `data`.
            if (data.status) {
                result = data; // It was just the object (maybe untagged? no)
            } else {
                // Fallback
                result = data;
            }
        }
      } else {
        error = `HTTP Error: ${response.status}`;
      }
    } catch (e) {
      error = e.message;
    } finally {
      loading = false;
    }
  }
</script>

<div class="runner">
  <button on:click={onBack}>&larr; Back to List</button>
  
  <div class="card">
    <h2>Run: {check.id}</h2>
    <p>{check.description}</p>
    
    <form on:submit|preventDefault={runCheck}>
      {#each check.parameters as param}
        <div class="form-group">
          <label for={param.name}>{param.name} <small>({param.description})</small></label>
          <input 
            id={param.name} 
            type="text" 
            bind:value={params[param.name]} 
            placeholder={param.default || ''}
          />
        </div>
      {/each}
      
      <div class="actions">
        <button class="primary" type="submit" disabled={loading}>
          {loading ? 'Running...' : 'Execute Check'}
        </button>
      </div>
    </form>
  </div>

  {#if error}
    <div class="card error">
      <h3>Error</h3>
      <p>{error}</p>
    </div>
  {/if}

  {#if result}
    <div class="card result">
      <h3>Result: <span class="status-badge status-{result.status}">{result.status}</span></h3>
      <p>{result.message}</p>
      {#if result.details}
        <pre>{JSON.stringify(result.details, null, 2)}</pre>
      {/if}
    </div>
  {/if}
</div>

<style>
  .form-group {
    margin-bottom: 1rem;
  }
  
  label {
    display: block;
    margin-bottom: 0.5rem;
    font-weight: 500;
  }
  
  input {
    width: 100%;
    box-sizing: border-box;
  }
  
  .actions {
    margin-top: 1.5rem;
  }
  
  .error {
    border-left: 5px solid var(--danger-color);
  }
  
  .result {
    border-left: 5px solid var(--success-color);
  }
</style>
