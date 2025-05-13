<script lang="ts">
  import type { PageProps } from "./$types";

  let { data }: PageProps = $props();
  let { followedStreamers, liveStreamers } = data;

  import Sidebar from "./Sidebar.svelte";
  import StreamsGrid from "./StreamsGrid.svelte";
</script>

<svelte:head>
  <title>Home</title>
  <meta name="description" content="CrateTV" />
</svelte:head>

<div class="layout">
  <Sidebar {followedStreamers} />
  <main class="main-content">
    <h1><strong>Welcome to CrateTV</strong></h1>
    <StreamsGrid {liveStreamers} />
  </main>
</div>

<style>
  .layout {
    display: flex;
    width: 100%;
    gap: 2rem;
    align-items: flex-start;
  }

  .main-content {
    flex: 1 1 0;
    display: flex;
    flex-direction: column;
    align-items: stretch;
    margin-top: 0;
    margin-right: 1em;
    min-width: 0;
  }

  .main-content h1 {
    text-align: left;
    margin-bottom: 1.5rem;
    font-size: 2rem;
    color: var(--color-theme-1);
  }

  @media (max-width: 900px) {
    .layout {
      flex-direction: column;
      gap: 0;
    }
    .main-content {
      margin-top: 0;
    }
  }
</style>
