<script lang="ts">
  import type { PageProps } from "./$types";
  let { data }: PageProps = $props();

  import HLSStreamer from "./HLSStreamer.svelte";
</script>

<HLSStreamer src="https://test-streams.mux.dev/x36xhzz/x36xhzz.m3u8" />

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
</style>
