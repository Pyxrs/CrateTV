<script lang="ts">
  import type { PageProps } from "./$types";

  let { data }: PageProps = $props();
</script>

<svelte:head>
  <title>Account</title>
  <meta name="description" content="CrateTV" />
</svelte:head>

<div class="layout">
  <p>Unimplemented</p>
</div>

<style>
  .layout {
    display: flex;
    width: 100%;
    gap: 2rem;
    align-items: flex-start;
  }

  @media (max-width: 900px) {
    .layout {
      flex-direction: column;
      gap: 0;
    }
  }
</style>
