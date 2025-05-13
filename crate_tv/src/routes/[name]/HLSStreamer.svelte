<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import Hls from "hls.js";

  export let src: string;
  let videoElement: HTMLVideoElement;
  let hls: Hls;

  // Store available quality levels and the selected level
  let levels: { height: number; name: string; index: number }[] = [];
  let selectedLevel: number = -1; // -1 is auto

  function onQualityChange(event: Event) {
    const value = +(event.target as HTMLSelectElement).value;
    selectedLevel = value;
    if (hls) {
      hls.currentLevel = value;
    }
  }

  onMount(() => {
    if (Hls.isSupported()) {
      hls = new Hls();
      hls.loadSource(src);
      hls.attachMedia(videoElement);

      hls.on(Hls.Events.MANIFEST_PARSED, function (_, data) {
        levels = data.levels.map((level: any, i: number) => ({
          height: level.height,
          name: level.name || `${level.height}p`,
          index: i,
        }));
      });

      hls.on(Hls.Events.LEVEL_SWITCHED, function (_, data) {
        selectedLevel = hls.currentLevel;
      });
    } else if (videoElement.canPlayType("application/vnd.apple.mpegurl")) {
      // Safari native HLS
      videoElement.src = src;
    }

    return () => {
      hls?.destroy();
    };
  });

  onDestroy(() => {
    hls?.destroy();
  });
</script>

<div class="video-container">
  <!-- captions are out of the scope of this project. -->
  <!-- svelte-ignore a11y_media_has_caption -->
  <video
    bind:this={videoElement}
    controls
    controlslist="nodownload noplaybackrate"
    autoplay
    playsinline
  >
  </video>
  {#if levels.length > 1}
    <label>
      Quality:
      <select on:change={onQualityChange} bind:value={selectedLevel}>
        <option value={-1}>Auto</option>
        {#each levels as level}
          <option value={level.index}>{level.name}</option>
        {/each}
      </select>
    </label>
  {/if}
</div>

<style>
  .video-container {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
  }
  video {
    height: 70vh;
    max-width: 100%;
    object-fit: contain;
    border-radius: 0.5rem;
    box-shadow:
      0 1px 3px 0 rgba(0, 0, 0, 0.1),
      0 1px 2px 0 rgba(0, 0, 0, 0.06);
  }
  label {
    margin-top: 0.5rem;
  }
</style>
