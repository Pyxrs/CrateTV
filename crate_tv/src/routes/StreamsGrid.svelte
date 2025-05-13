<script lang="ts">
  import type { Streamer } from "../types/Streamer";

  export let liveStreamers: Streamer[] = [];
</script>

<div class="streams-grid">
  {#each liveStreamers as streamer}
    <div class="stream-card">
      <a href="/{streamer.name}" aria-label="Account">
        <div class="preview-container">
          <img
            class="preview"
            src={streamer.preview}
            alt={streamer.name + " preview"}
          />
        </div>
      </a>
      <div class="stream-info">
        <div class="stream-title">{streamer.title}</div>
        <div class="stream-meta">
          <a href="/{streamer.name}" aria-label="Account" class="streamer-name">
            {streamer.name}
          </a>
          <span class="stream-game">{streamer.game}</span>

          <div>
            <span class="live-dot" title="Live"></span>
            <span class="viewers">{streamer.viewers} watching</span>
          </div>
        </div>
      </div>
    </div>
  {/each}
</div>

<style>
  .streams-grid {
    display: flex;
    flex-wrap: wrap;
    gap: 2rem;
    justify-content: flex-start;
  }

  .stream-card {
    flex: 1 1 240px;
    max-width: 320px;
    background: var(--color-bg-2);
    border-radius: 10px;
    overflow: hidden;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.13);
    display: flex;
    flex-direction: column;
    transition: transform 0.15s;
  }

  .stream-card:hover {
    transform: translateY(-4px) scale(1.03);
    box-shadow: 0 4px 16px rgba(0, 0, 0, 0.18);
  }

  .preview-container {
    aspect-ratio: 16 / 9;
    width: 100%;
    background: #222;
    overflow: hidden;
    display: block;
  }

  .preview {
    width: 100%;
    height: 100%;
    object-fit: cover;
    display: block;
  }

  .stream-info {
    padding: 0.8em 1em 1em 1em;
    display: flex;
    flex-direction: column;
    gap: 0.5em;
  }

  .stream-title {
    font-size: 1.1em;
    font-weight: 500;
    color: var(--color-text-1);
    margin-bottom: 0.2em;
  }

  .stream-meta {
    display: flex;
    justify-content: space-between;
    font-size: 0.97em;
    color: var(--color-text-2);
    white-space: nowrap;
    gap: 0.5em;
  }

  .streamer-name {
    color: var(--color-theme-1);
    text-decoration: none;
    transition: color 0.15s;
  }
  .streamer-name:hover {
    color: var(--color-theme-2);
    text-decoration: none;
  }

  .stream-game {
    text-overflow: ellipsis;
    overflow: hidden;
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
    margin-left: auto;
    box-shadow: 0 0 6px var(--color-theme-3);
    display: inline-block;
  }
</style>
