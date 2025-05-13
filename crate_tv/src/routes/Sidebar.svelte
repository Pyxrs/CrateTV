<script lang="ts">
  import type { Streamer } from "../types/Streamer";

  export let followedStreamers: Streamer[] = []; // Viewer count is 0 for offline streamers

  let hoveredStreamer: string | null = null;
  let collapsed = false;

  function toggleSidebar() {
    collapsed = !collapsed;
  }
</script>

<aside class="sidebar {collapsed ? 'collapsed' : ''}">
  <button
    class="collapse-btn"
    on:click={toggleSidebar}
    aria-label={collapsed ? "Expand sidebar" : "Collapse sidebar"}
  >
    {#if collapsed}
      &gt;
    {:else}
      &lt; Following
    {/if}
  </button>
  <ul>
    {#each followedStreamers.filter((s) => s.viewers > 0) as streamer}
      <li
        on:mouseenter={() => (hoveredStreamer = streamer.name)}
        on:mouseleave={() => (hoveredStreamer = null)}
      >
        <a href={`/${streamer.name}`} class="streamer-entry">
          <img src={streamer.avatar} alt={streamer.name + " avatar"} />
          {#if !collapsed}
            <span class="streamer-info">
              <span class="streamer-name">{streamer.name}</span>
              <span class="streamer-game">{streamer.game}</span>
            </span>
            <div class="streamer-meta">
              <span class="live-dot" title="Live"></span>
              <span class="viewers">{streamer.viewers}</span>
            </div>
          {/if}
        </a>
        <!-- Preview {#if hoveredStreamer === streamer.name}{/if} -->
      </li>
    {/each}
    {#each followedStreamers.filter((s) => s.viewers === 0) as streamer}
      <li class="offline-row">
        <a href={`/${streamer.name}`} class="streamer-entry">
          <img src={streamer.avatar} alt={streamer.name + " avatar"} />
          {#if !collapsed}
            <span class="streamer-info">
              <span class="streamer-name">{streamer.name}</span>
              <span class="streamer-game">{streamer.game}</span>
            </span>
            <span class="offline">Offline</span>
          {/if}
        </a>
      </li>
    {/each}
    {#if followedStreamers.length === 0 && !collapsed}
      <li class="empty">You don't follow anybody yet</li>
    {/if}
  </ul>
</aside>

<style>
  .sidebar {
    width: 220px;
    background: var(--color-bg-2);
    border-radius: 12px;
    padding-top: 1rem;
    padding-bottom: 0.5rem;
    padding-left: 0.5rem;
    padding-right: 0.5rem;
    margin-left: 0.5rem;
    margin-top: 1rem;
    margin-right: 0rem;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.12);
    display: flex;
    flex-direction: column;
    transition: width 0.2s;
  }

  .sidebar.collapsed {
    width: 52px;
    min-width: 52px;
  }

  .collapse-btn {
    background: none;
    border: none;
    color: var(--color-theme-1);
    font-size: 1.2em;
    cursor: pointer;
    margin-bottom: 0.5em;
    margin-left: 0.5em;
    border-radius: 50%;
    padding: 0.25em 0.6em 0.25em 0.4em;
    gap: 0.5em;
    text-decoration: none;
    white-space: nowrap;
    align-self: flex-start;
  }

  .collapse-btn:hover {
    color: var(--color-theme-2);
  }

  .sidebar ul {
    list-style: none;
    padding: 0;
    margin: 0;
    width: 100%;
  }

  .sidebar li {
    display: flex;
    align-items: center;
    gap: 0.7em;
    margin-bottom: 0.5em;
    font-size: 1rem;
    color: var(--color-text-1);
    position: relative;
    padding: 0;
    transition: background 0.18s cubic-bezier(0.4, 0, 0.2, 1);
  }

  .sidebar li:not(.empty):not(.offline-row):hover,
  .sidebar li.offline-row:hover {
    background: var(--color-bg-3);
    border-radius: 6px;
  }

  .sidebar li.empty {
    color: var(--color-text-2);
    font-size: 0.95em;
    margin-top: 1em;
  }

  .sidebar img {
    width: 32px;
    height: 32px;
    border-radius: 50%;
    object-fit: cover;
    border: 2px solid var(--color-theme-2);
  }

  .viewers {
    color: var(--color-theme-3);
    font-weight: 500;
  }

  .live-dot {
    width: 10px;
    height: 10px;
    background: var(--color-theme-3);
    border-radius: 50%;
    box-shadow: 0 0 6px var(--color-theme-3);
    display: inline-block;
  }

  .offline {
    color: var(--color-text-2);
    font-size: 0.97em;
    border-radius: 50%;
    margin-left: auto;
  }

  .offline-row {
    opacity: 0.5;
    filter: grayscale(0.7);
    pointer-events: auto;
  }

  .offline-row .streamer-entry {
    pointer-events: auto;
  }

  .streamer-entry {
    display: flex;
    align-items: center;
    gap: 0.5em;
    text-decoration: none;
    color: inherit;
    white-space: nowrap;
    width: 100%;
    padding: 0.4em 0.5em;
    box-sizing: border-box;
    border-radius: 6px;
  }

  .streamer-entry:hover {
    background: none;
    text-decoration: none;
    color: inherit;
  }

  .streamer-meta {
    display: flex;
    align-items: center;
    gap: 0.4em;
    margin-left: auto;
  }

  .streamer-info {
    display: flex;
    flex-direction: column;
    justify-content: center;
    min-width: 0;
  }

  .streamer-name {
    font-weight: bold;
    text-overflow: ellipsis;
    overflow: hidden;
    white-space: nowrap;
  }

  .streamer-game {
    font-size: 0.92em;
    color: var(--color-text-2);
    text-overflow: ellipsis;
    overflow: hidden;
    white-space: nowrap;
  }
</style>
