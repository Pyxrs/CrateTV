<script lang="ts">
  // Auth state
  let username = $state<string | null>(null);
  let loading = $state(true);

  // Form state
  let tab = $state<"login" | "register">("login");
  let formUsername = $state("");
  let formPassword = $state("");
  let error = $state("");
  let submitting = $state(false);

  // Stream key state
  let streamKeyLoaded = $state(false);
  let streamKey = $state<string | null>(null);
  let streamKeyVisible = $state(false);
  let streamKeyWorking = $state(false);

  async function checkSession() {
    try {
      const res = await fetch("/api/me");
      if (res.ok) {
        const body = await res.json();
        username = body.username;
      }
    } catch {
      // Not logged in
    } finally {
      loading = false;
    }
  }

  checkSession();

  async function submit() {
    error = "";
    submitting = true;
    const endpoint = tab === "login" ? "/api/login" : "/api/register";
    try {
      const res = await fetch(endpoint, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ username: formUsername, password: formPassword }),
      });
      const text = await res.text();
      if (!res.ok) {
        error = text;
        return;
      }
      if (tab === "login") {
        const body = JSON.parse(text);
        username = body.username;
        formUsername = "";
        formPassword = "";
      } else {
        // Auto-login after registration
        tab = "login";
        error = "Account created! You can now log in.";
      }
    } catch (e) {
      error = "Network error. Is the server running?";
    } finally {
      submitting = false;
    }
  }

  async function logout() {
    await fetch("/api/logout", { method: "POST" });
    username = null;
    streamKey = null;
    streamKeyLoaded = false;
    streamKeyVisible = false;
  }

  async function fetchStreamKey() {
    streamKeyWorking = true;
    try {
      const res = await fetch("/api/stream-key");
      if (res.ok) {
        const body = await res.json();
        streamKey = body.stream_key;
        streamKeyLoaded = true;
      }
    } finally {
      streamKeyWorking = false;
    }
  }

  async function regenerateStreamKey() {
    if (!confirm("Regenerate your stream key? Your current key will stop working immediately.")) return;
    streamKeyWorking = true;
    try {
      const res = await fetch("/api/stream-key/regenerate", { method: "POST" });
      if (res.ok) {
        const body = await res.json();
        streamKey = body.stream_key;
        streamKeyLoaded = true;
        streamKeyVisible = true;
      }
    } finally {
      streamKeyWorking = false;
    }
  }

  function copyStreamKey() {
    if (streamKey) navigator.clipboard.writeText(streamKey);
  }
</script>

<svelte:head>
  <title>Account</title>
  <meta name="description" content="CrateTV" />
</svelte:head>

<div class="layout">
  {#if loading}
    <p class="hint">Loading…</p>
  {:else if username}
    <div class="card">
      <h2>Signed in as <strong>{username}</strong></h2>

      <div class="section">
        <h3>Stream Key</h3>
        {#if !streamKeyLoaded}
          <button class="btn-secondary" onclick={fetchStreamKey} disabled={streamKeyWorking}>
            {streamKeyWorking ? "Loading…" : "Reveal stream key"}
          </button>
        {:else if streamKey === null}
          <p class="hint">No stream key yet.</p>
          <button class="btn-secondary" onclick={regenerateStreamKey} disabled={streamKeyWorking}>
            {streamKeyWorking ? "Working…" : "Generate stream key"}
          </button>
        {:else}
          <div class="key-row">
            <input type={streamKeyVisible ? "text" : "password"} value={streamKey} readonly />
            <button class="btn-secondary" onclick={() => (streamKeyVisible = !streamKeyVisible)}>
              {streamKeyVisible ? "Hide" : "Show"}
            </button>
            <button class="btn-secondary" onclick={copyStreamKey}>Copy</button>
          </div>
          <button class="btn-danger" onclick={regenerateStreamKey} disabled={streamKeyWorking}>
            {streamKeyWorking ? "Working…" : "Regenerate"}
          </button>
        {/if}
      </div>

      <button class="btn-secondary" onclick={logout}>Log out</button>
    </div>
  {:else}
    <div class="card">
      <div class="tabs">
        <button
          class="tab"
          class:active={tab === "login"}
          onclick={() => { tab = "login"; error = ""; }}
        >Log in</button>
        <button
          class="tab"
          class:active={tab === "register"}
          onclick={() => { tab = "register"; error = ""; }}
        >Register</button>
      </div>

      <form onsubmit={(e) => { e.preventDefault(); submit(); }}>
        <label>
          Username
          <input
            type="text"
            bind:value={formUsername}
            autocomplete={tab === "login" ? "username" : "username"}
            required
          />
        </label>
        <label>
          Password
          <input
            type="password"
            bind:value={formPassword}
            autocomplete={tab === "login" ? "current-password" : "new-password"}
            required
          />
        </label>

        {#if error}
          <p class="error">{error}</p>
        {/if}

        <button type="submit" class="btn-primary" disabled={submitting}>
          {tab === "login" ? "Log in" : "Create account"}
        </button>
      </form>
    </div>
  {/if}
</div>

<style>
  .layout {
    display: flex;
    width: 100%;
    justify-content: center;
    padding-top: 2rem;
  }

  .card {
    background: var(--color-bg-2);
    border-radius: 10px;
    padding: 2rem;
    width: 100%;
    max-width: 380px;
    display: flex;
    flex-direction: column;
    gap: 1.25rem;
  }

  .tabs {
    display: flex;
    gap: 0.5rem;
    border-bottom: 1px solid #3a3d45;
    padding-bottom: 0.75rem;
  }

  .tab {
    background: none;
    border: none;
    color: var(--color-text-2);
    cursor: pointer;
    font-size: 0.95rem;
    padding: 0.25rem 0.5rem;
    border-radius: 4px;
    transition: color 0.15s;
  }

  .tab.active {
    color: var(--color-theme-1);
    font-weight: 600;
  }

  .tab:hover:not(.active) {
    color: var(--color-text-1);
  }

  form {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  label {
    display: flex;
    flex-direction: column;
    gap: 0.35rem;
    font-size: 0.9rem;
    color: var(--color-text-2);
  }

  input {
    background: var(--color-bg-3);
    border: 1px solid #3a3d45;
    border-radius: 6px;
    color: var(--color-text-1);
    padding: 0.5rem 0.75rem;
    font-size: 1rem;
    outline: none;
    transition: border-color 0.15s;
  }

  input:focus {
    border-color: var(--color-theme-1);
  }

  .btn-primary {
    background: var(--color-theme-1);
    border: none;
    border-radius: 6px;
    color: #fff;
    cursor: pointer;
    font-size: 1rem;
    padding: 0.6rem 1rem;
    transition: opacity 0.15s;
  }

  .btn-primary:hover:not(:disabled) {
    opacity: 0.85;
  }

  .btn-primary:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .btn-secondary {
    background: var(--color-bg-3);
    border: 1px solid #3a3d45;
    border-radius: 6px;
    color: var(--color-text-1);
    cursor: pointer;
    font-size: 0.95rem;
    padding: 0.5rem 1rem;
    transition: border-color 0.15s;
    align-self: flex-start;
  }

  .btn-secondary:hover:not(:disabled) {
    border-color: var(--color-theme-1);
  }

  .btn-secondary:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .btn-danger {
    background: none;
    border: 1px solid #6b2323;
    border-radius: 6px;
    color: var(--color-theme-3);
    cursor: pointer;
    font-size: 0.85rem;
    padding: 0.35rem 0.75rem;
    transition: border-color 0.15s;
    align-self: flex-start;
  }

  .btn-danger:hover:not(:disabled) {
    border-color: var(--color-theme-3);
  }

  .btn-danger:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .section {
    display: flex;
    flex-direction: column;
    gap: 0.6rem;
    border-top: 1px solid #3a3d45;
    padding-top: 1rem;
  }

  .section h3 {
    margin: 0;
    font-size: 0.85rem;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    color: var(--color-text-2);
  }

  .key-row {
    display: flex;
    gap: 0.5rem;
    align-items: center;
  }

  .key-row input {
    flex: 1;
    font-family: monospace;
    font-size: 0.85rem;
  }

  .error {
    color: var(--color-theme-3);
    font-size: 0.9rem;
    margin: 0;
  }

  .hint {
    color: var(--color-text-2);
    font-size: 0.9rem;
    margin: 0;
  }

  h2 {
    margin: 0;
    font-size: 1.1rem;
    color: var(--color-text-1);
  }
</style>
