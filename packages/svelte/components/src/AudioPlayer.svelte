<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";
  import type {
    ControlDensity,
    ControlSize,
    SemanticControlSizeRole,
  } from "./types";

  export let src: string;
  export let ariaLabel = "Audio player";
  export let showSpeedControl = false;
  export let size: ControlSize | null = null;
  export let sizeRole: SemanticControlSizeRole = "control";
  export let density: ControlDensity | null = null;

  let audioEl: HTMLAudioElement | null = null;
  let isPlaying = false;
  let currentTime = 0;
  let duration = 0;
  let volume = 1;
  let playbackRate = 1;
  let isMuted = false;
  let animFrame: number | null = null;
  const uiPresentation = getUiPresentation();

  const speedOptions = [0.5, 0.75, 1, 1.25, 1.5, 2];

  $: formattedCurrent = formatTime(currentTime);
  $: formattedDuration = formatTime(duration);
  $: resolvedSize = size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole);
  $: resolvedDensity = density ?? $uiPresentation.density;

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

<div class="poodle-audio-player" aria-label={ariaLabel} data-size={resolvedSize} data-density={resolvedDensity}>
  <audio bind:this={audioEl} {src} preload="metadata"></audio>

  <button
    type="button"
    class="poodle-audio-player__play"
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

  <span class="poodle-audio-player__time">{formattedCurrent}</span>

  <input
    type="range"
    class="poodle-audio-player__seek"
    min="0"
    max={duration || 0}
    step="0.1"
    value={currentTime}
    on:input={handleSeek}
    aria-label="Seek"
  />

  <span class="poodle-audio-player__time">{formattedDuration}</span>

  <button
    type="button"
    class="poodle-audio-player__mute"
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
    class="poodle-audio-player__volume"
    min="0"
    max="1"
    step="0.01"
    value={isMuted ? 0 : volume}
    on:input={handleVolume}
    aria-label="Volume"
  />

  {#if showSpeedControl}
    <select
      class="poodle-audio-player__speed"
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
  .poodle-audio-player {
    --poodle-audio-player-gap: 0.5rem;
    --poodle-audio-player-pad-y: 0.5rem;
    --poodle-audio-player-pad-x: 0.75rem;
    --poodle-audio-player-button-size: 2rem;
    --poodle-audio-player-icon-size: 1rem;
    --poodle-audio-player-time-width: 2.5rem;
    --poodle-audio-player-volume-width: 4rem;
    --poodle-audio-player-speed-x: 0.25rem;
    --poodle-audio-player-speed-y: 0.125rem;
    display: flex;
    align-items: center;
    gap: var(--poodle-audio-player-gap);
    padding: var(--poodle-audio-player-pad-y) var(--poodle-audio-player-pad-x);
    border: 0.0625rem solid var(--poodle-color-border-default);
    border-radius: var(--poodle-radius-surface);
    background: var(--poodle-color-background-surface);
  }

  .poodle-audio-player[data-size="xs"] {
    --poodle-audio-player-button-size: 1.5rem;
    --poodle-audio-player-icon-size: 0.875rem;
    --poodle-audio-player-time-width: 2rem;
    --poodle-audio-player-volume-width: 3rem;
  }

  .poodle-audio-player[data-size="sm"] {
    --poodle-audio-player-button-size: 1.75rem;
  }

  .poodle-audio-player[data-size="lg"] {
    --poodle-audio-player-button-size: 2.25rem;
    --poodle-audio-player-icon-size: 1.125rem;
    --poodle-audio-player-time-width: 2.75rem;
    --poodle-audio-player-volume-width: 4.5rem;
    --poodle-audio-player-speed-x: 0.375rem;
  }

  .poodle-audio-player[data-size="xl"] {
    --poodle-audio-player-button-size: 2.5rem;
    --poodle-audio-player-icon-size: 1.25rem;
    --poodle-audio-player-time-width: 3rem;
    --poodle-audio-player-volume-width: 5rem;
    --poodle-audio-player-speed-x: 0.5rem;
  }

  .poodle-audio-player[data-density="compact"] {
    --poodle-audio-player-gap: 0.375rem;
    --poodle-audio-player-pad-y: 0.375rem;
    --poodle-audio-player-pad-x: 0.5rem;
    --poodle-audio-player-speed-y: 0.0625rem;
  }

  .poodle-audio-player[data-density="comfortable"] {
    --poodle-audio-player-gap: 0.625rem;
    --poodle-audio-player-pad-y: 0.625rem;
    --poodle-audio-player-pad-x: 0.875rem;
    --poodle-audio-player-speed-y: 0.1875rem;
  }

  .poodle-audio-player__play,
  .poodle-audio-player__mute {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: var(--poodle-audio-player-button-size);
    height: var(--poodle-audio-player-button-size);
    padding: 0;
    border: 0;
    border-radius: 999rem;
    background: transparent;
    color: var(--poodle-color-text-primary);
    cursor: pointer;
    flex-shrink: 0;
    transition: background var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard);
  }

  .poodle-audio-player__play:hover,
  .poodle-audio-player__mute:hover {
    background: color-mix(in srgb, var(--poodle-color-accent-base) 12%, transparent);
  }

  .poodle-audio-player__play:focus-visible,
  .poodle-audio-player__mute:focus-visible {
    outline: var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing);
    outline-offset: 0.0625rem;
  }

  .poodle-audio-player__play svg,
  .poodle-audio-player__mute svg {
    width: var(--poodle-audio-player-icon-size);
    height: var(--poodle-audio-player-icon-size);
  }

  .poodle-audio-player__time {
    font-family: var(--poodle-typography-code-family);
    font-size: var(--poodle-typography-label-size);
    color: var(--poodle-color-text-secondary);
    flex-shrink: 0;
    min-width: var(--poodle-audio-player-time-width);
    text-align: center;
  }

  .poodle-audio-player__seek,
  .poodle-audio-player__volume {
    -webkit-appearance: none;
    appearance: none;
    background: transparent;
    cursor: pointer;
    height: 1rem;
  }

  .poodle-audio-player__seek {
    flex: 1;
    min-width: 4rem;
  }

  .poodle-audio-player__volume {
    width: var(--poodle-audio-player-volume-width);
    flex-shrink: 0;
  }

  /* Seek track */
  .poodle-audio-player__seek::-webkit-slider-runnable-track {
    height: 0.25rem;
    border-radius: 0.125rem;
    background: var(--poodle-color-text-primary);
  }

  .poodle-audio-player__seek::-moz-range-track {
    height: 0.25rem;
    border-radius: 0.125rem;
    background: var(--poodle-color-text-primary);
  }

  /* Volume track */
  .poodle-audio-player__volume::-webkit-slider-runnable-track {
    height: 0.25rem;
    border-radius: 0.125rem;
    background: var(--poodle-color-accent-base);
  }

  .poodle-audio-player__volume::-moz-range-track {
    height: 0.25rem;
    border-radius: 0.125rem;
    background: var(--poodle-color-accent-base);
  }

  /* Thumb */
  .poodle-audio-player__seek::-webkit-slider-thumb,
  .poodle-audio-player__volume::-webkit-slider-thumb {
    -webkit-appearance: none;
    width: 0.625rem;
    height: 0.625rem;
    border-radius: 50%;
    border: none;
    background: var(--poodle-color-text-primary);
    margin-top: -0.1875rem;
  }

  .poodle-audio-player__seek::-moz-range-thumb,
  .poodle-audio-player__volume::-moz-range-thumb {
    width: 0.625rem;
    height: 0.625rem;
    border-radius: 50%;
    border: none;
    background: var(--poodle-color-text-primary);
  }

  .poodle-audio-player__speed {
    min-height: var(--poodle-size-control-height);
    padding: var(--poodle-audio-player-speed-y) var(--poodle-audio-player-speed-x);
    border: 0.0625rem solid var(--poodle-color-border-default);
    border-radius: var(--poodle-radius-control);
    background: transparent;
    color: var(--poodle-color-text-secondary);
    font: inherit;
    font-size: var(--poodle-typography-label-size);
    font-family: var(--poodle-typography-code-family);
    cursor: pointer;
    appearance: none;
    flex-shrink: 0;
  }
</style>
