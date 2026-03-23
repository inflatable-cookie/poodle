<script lang="ts">
  import { onMount, onDestroy } from "svelte";

  export let src: string;
  export let ariaLabel = "Audio player";
  export let showSpeedControl = false;

  let audioEl: HTMLAudioElement | null = null;
  let isPlaying = false;
  let currentTime = 0;
  let duration = 0;
  let volume = 1;
  let playbackRate = 1;
  let isMuted = false;
  let animFrame: number | null = null;

  const speedOptions = [0.5, 0.75, 1, 1.25, 1.5, 2];

  $: formattedCurrent = formatTime(currentTime);
  $: formattedDuration = formatTime(duration);

  function formatTime(sec: number): string {
    const m = Math.floor(sec / 60);
    const s = Math.floor(sec % 60);
    return `${m}:${s.toString().padStart(2, "0")}`;
  }

  function togglePlay(): void {
    if (!audioEl) return;
    if (isPlaying) {
      audioEl.pause();
    } else {
      audioEl.play();
    }
  }

  function handleSeek(event: Event): void {
    if (!audioEl) return;
    const value = Number((event.currentTarget as HTMLInputElement).value);
    audioEl.currentTime = value;
    currentTime = value;
  }

  function handleVolume(event: Event): void {
    if (!audioEl) return;
    volume = Number((event.currentTarget as HTMLInputElement).value);
    audioEl.volume = volume;
    isMuted = volume === 0;
  }

  function toggleMute(): void {
    if (!audioEl) return;
    isMuted = !isMuted;
    audioEl.muted = isMuted;
  }

  function setSpeed(rate: number): void {
    if (!audioEl) return;
    playbackRate = rate;
    audioEl.playbackRate = rate;
  }

  function updateTime(): void {
    if (audioEl) {
      currentTime = audioEl.currentTime;
    }
    if (isPlaying) {
      animFrame = requestAnimationFrame(updateTime);
    }
  }

  onMount(() => {
    if (!audioEl) return;

    audioEl.addEventListener("play", () => {
      isPlaying = true;
      updateTime();
    });
    audioEl.addEventListener("pause", () => {
      isPlaying = false;
      if (animFrame !== null) cancelAnimationFrame(animFrame);
    });
    audioEl.addEventListener("ended", () => {
      isPlaying = false;
      if (animFrame !== null) cancelAnimationFrame(animFrame);
    });
    audioEl.addEventListener("loadedmetadata", () => {
      duration = audioEl?.duration ?? 0;
    });
    audioEl.addEventListener("durationchange", () => {
      duration = audioEl?.duration ?? 0;
    });
  });

  onDestroy(() => {
    if (animFrame !== null) cancelAnimationFrame(animFrame);
  });
</script>

<div class="audio-player" aria-label={ariaLabel}>
  <audio bind:this={audioEl} {src} preload="metadata"></audio>

  <button
    type="button"
    class="audio-player__play"
    on:click={togglePlay}
    aria-label={isPlaying ? "Pause" : "Play"}
  >
    {#if isPlaying}
      <svg viewBox="0 0 16 16" fill="currentColor" aria-hidden="true">
        <rect x="3" y="2" width="3.5" height="12" rx="0.75" />
        <rect x="9.5" y="2" width="3.5" height="12" rx="0.75" />
      </svg>
    {:else}
      <svg viewBox="0 0 16 16" fill="currentColor" aria-hidden="true">
        <path d="M4 2.5l9 5.5-9 5.5V2.5z" />
      </svg>
    {/if}
  </button>

  <span class="audio-player__time">{formattedCurrent}</span>

  <input
    type="range"
    class="audio-player__seek"
    min="0"
    max={duration || 0}
    step="0.1"
    value={currentTime}
    on:input={handleSeek}
    aria-label="Seek"
  />

  <span class="audio-player__time">{formattedDuration}</span>

  <button
    type="button"
    class="audio-player__mute"
    on:click={toggleMute}
    aria-label={isMuted ? "Unmute" : "Mute"}
  >
    {#if isMuted || volume === 0}
      <svg viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.25" aria-hidden="true">
        <path d="M8 3L4.5 6H2v4h2.5L8 13V3z" fill="currentColor" stroke="none" />
        <path d="M12 5l-4 6m0-6l4 6" stroke-linecap="round" />
      </svg>
    {:else}
      <svg viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.25" aria-hidden="true">
        <path d="M8 3L4.5 6H2v4h2.5L8 13V3z" fill="currentColor" stroke="none" />
        <path d="M11 5.5a4 4 0 010 5" stroke-linecap="round" />
      </svg>
    {/if}
  </button>

  <input
    type="range"
    class="audio-player__volume"
    min="0"
    max="1"
    step="0.01"
    value={isMuted ? 0 : volume}
    on:input={handleVolume}
    aria-label="Volume"
  />

  {#if showSpeedControl}
    <select
      class="audio-player__speed"
      value={playbackRate}
      on:change={(e) => setSpeed(Number((e.currentTarget as HTMLSelectElement).value))}
      aria-label="Playback speed"
    >
      {#each speedOptions as speed}
        <option value={speed}>{speed}×</option>
      {/each}
    </select>
  {/if}
</div>

<style>
  .audio-player {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.5rem 0.75rem;
    border: 0.0625rem solid var(--flint-color-border-default);
    border-radius: var(--flint-radius-surface);
    background: var(--flint-color-background-surface);
  }

  .audio-player__play,
  .audio-player__mute {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 2rem;
    height: 2rem;
    padding: 0;
    border: 0;
    border-radius: 999rem;
    background: transparent;
    color: var(--flint-color-text-primary);
    cursor: pointer;
    flex-shrink: 0;
    transition: background var(--flint-motion-duration-interaction) var(--flint-motion-easing-standard);
  }

  .audio-player__play:hover,
  .audio-player__mute:hover {
    background: color-mix(in srgb, var(--flint-color-accent-base) 12%, transparent);
  }

  .audio-player__play:focus-visible,
  .audio-player__mute:focus-visible {
    outline: var(--flint-border-width-focus) solid var(--flint-color-accent-focusRing);
    outline-offset: 0.0625rem;
  }

  .audio-player__play svg,
  .audio-player__mute svg {
    width: 1rem;
    height: 1rem;
  }

  .audio-player__time {
    font-family: var(--flint-typography-code-family);
    font-size: 0.6875rem;
    color: var(--flint-color-text-secondary);
    flex-shrink: 0;
    min-width: 2.5rem;
    text-align: center;
  }

  .audio-player__seek,
  .audio-player__volume {
    -webkit-appearance: none;
    appearance: none;
    background: transparent;
    cursor: pointer;
    height: 1rem;
  }

  .audio-player__seek {
    flex: 1;
    min-width: 4rem;
  }

  .audio-player__volume {
    width: 4rem;
    flex-shrink: 0;
  }

  /* Seek track */
  .audio-player__seek::-webkit-slider-runnable-track {
    height: 0.25rem;
    border-radius: 0.125rem;
    background: var(--flint-color-text-primary);
  }

  .audio-player__seek::-moz-range-track {
    height: 0.25rem;
    border-radius: 0.125rem;
    background: var(--flint-color-text-primary);
  }

  /* Volume track */
  .audio-player__volume::-webkit-slider-runnable-track {
    height: 0.25rem;
    border-radius: 0.125rem;
    background: var(--flint-color-accent-base);
  }

  .audio-player__volume::-moz-range-track {
    height: 0.25rem;
    border-radius: 0.125rem;
    background: var(--flint-color-accent-base);
  }

  /* Thumb */
  .audio-player__seek::-webkit-slider-thumb,
  .audio-player__volume::-webkit-slider-thumb {
    -webkit-appearance: none;
    width: 0.625rem;
    height: 0.625rem;
    border-radius: 50%;
    border: none;
    background: var(--flint-color-text-primary);
    margin-top: -0.1875rem;
  }

  .audio-player__seek::-moz-range-thumb,
  .audio-player__volume::-moz-range-thumb {
    width: 0.625rem;
    height: 0.625rem;
    border-radius: 50%;
    border: none;
    background: var(--flint-color-text-primary);
  }

  .audio-player__speed {
    padding: 0.125rem 0.25rem;
    border: 0.0625rem solid var(--flint-color-border-default);
    border-radius: var(--flint-radius-control);
    background: transparent;
    color: var(--flint-color-text-secondary);
    font: inherit;
    font-size: 0.6875rem;
    font-family: var(--flint-typography-code-family);
    cursor: pointer;
    appearance: none;
    flex-shrink: 0;
  }
</style>
