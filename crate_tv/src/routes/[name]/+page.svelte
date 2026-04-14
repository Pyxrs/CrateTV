<script lang="ts">
  import type { PageProps } from "./$types";
  let { data }: PageProps = $props();

  import HLSStreamer from "./HLSStreamer.svelte";
</script>

{#if data.stream_url}
  <HLSStreamer src={data.stream_url} />
{:else}
  <p class="offline">Stream is offline.</p>
{/if}

<h1>{data.info.title}</h1>
<p>{data.info.description}</p>

<style>
  h1 {
    text-align: center;
    margin-top: 1rem;
    font-size: 2rem;
    color: var(--color-theme-1);
  }

  p {
    text-align: center;
    margin-top: 0.5rem;
    font-size: 1.2rem;
    color: var(--color-text-1-1);
  }

  p.offline {
    margin-top: 2rem;
    font-size: 1.5rem;
    color: var(--color-text-1-1);
    opacity: 0.6;
  }
</style>
