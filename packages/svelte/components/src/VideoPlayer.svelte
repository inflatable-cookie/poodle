<script lang="ts">
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";
  import type { ControlDensity, ControlSize, SemanticControlSizeRole } from "./types";

  interface Props {
    src: string;
    poster?: string | null;
    aspectRatio?: number;
    ariaLabel?: string;
    showCaptions?: boolean;
    captionsSrc?: string | null;
    size?: ControlSize | null;
    sizeRole?: SemanticControlSizeRole;
    density?: ControlDensity | null;
  }

  let {
    src,
    poster = null,
    aspectRatio = 16 / 9,
    ariaLabel = "Video player",
    showCaptions = false,
    captionsSrc = null,
    size = null,
    sizeRole = "control",
    density = null,
  }: Props = $props();

  let videoEl = $state<HTMLVideoElement | null>(null);
  let wrapperEl = $state<HTMLDivElement | null>(null);
  let isPlaying = $state(false);
  let currentTime = $state(0);
  let duration = $state(0);
  let volume = $state(1);
  let isMuted = $state(false);
  let isFullscreen = $state(false);
  let showControls = $state(true);
  let controlsTimeout = $state<ReturnType<typeof setTimeout> | null>(null);
  let animFrame = $state<number | null>(null);

  const uiPresentation = getUiPresentation();

  const resolvedSize = $derived(size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole));
  const resolvedDensity = $derived(density ?? $uiPresentation.density);
  const formattedCurrent = $derived(formatTime(currentTime));
  const formattedDuration = $derived(formatTime(duration));
  const progress = $derived(duration > 0 ? (currentTime / duration) * 100 : 0);

  function formatTime(sec: number): string {
    const m = Math.floor(sec / 60);
    const s = Math.floor(sec % 60);
    return `${m}:${s.toString().padStart(2, "0")}`;
  }

  function togglePlay(): void {
    if (!videoEl) return;
    if (isPlaying) {
      videoEl.pause();
    } else {
      videoEl.play();
    }
  }

  function handleSeek(event: Event): void {
    if (!videoEl) return;
    const value = Number((event.currentTarget as HTMLInputElement).value);
    videoEl.currentTime = value;
    currentTime = value;
  }

  function handleVolume(event: Event): void {
    if (!videoEl) return;
    volume = Number((event.currentTarget as HTMLInputElement).value);
    videoEl.volume = volume;
    isMuted = volume === 0;
  }

  function toggleMute(): void {
    if (!videoEl) return;
    isMuted = !isMuted;
    videoEl.muted = isMuted;
  }

  function toggleFullscreen(): void {
    if (!wrapperEl) return;
    if (!document.fullscreenElement) {
      wrapperEl.requestFullscreen();
    } else {
      document.exitFullscreen();
    }
  }

  function resetControlsTimer(): void {
    showControls = true;
    if (controlsTimeout) clearTimeout(controlsTimeout);
    if (isPlaying) {
      controlsTimeout = setTimeout(() => {
        showControls = false;
      }, 3000);
    }
  }

  function updateTime(): void {
    if (videoEl) {
      currentTime = videoEl.currentTime;
    }
    if (isPlaying) {
      animFrame = requestAnimationFrame(updateTime);
    }
  }

  function handleWrapperKeydown(event: KeyboardEvent): void {
    if (event.target !== wrapperEl) return;

    if (event.key === " " || event.key === "Enter") {
      event.preventDefault();
      togglePlay();
    }
  }

  function handlePlay(): void {
    isPlaying = true;
    updateTime();
    resetControlsTimer();
  }

  function handlePause(): void {
    isPlaying = false;
    showControls = true;
    if (animFrame !== null) cancelAnimationFrame(animFrame);
  }

  function handleEnded(): void {
    isPlaying = false;
    showControls = true;
    if (animFrame !== null) cancelAnimationFrame(animFrame);
  }

  function handleLoadedMetadata(): void {
    duration = videoEl?.duration ?? 0;
  }

  function handleFullscreenChange(): void {
    isFullscreen = !!document.fullscreenElement;
  }

  $effect(() => {
    document.addEventListener("fullscreenchange", handleFullscreenChange);

    return () => {
      document.removeEventListener("fullscreenchange", handleFullscreenChange);
      if (animFrame !== null) cancelAnimationFrame(animFrame);
      if (controlsTimeout) clearTimeout(controlsTimeout);
    };
  });
</script>

<div
  class="poodle-video-player"
  bind:this={wrapperEl}
  style="aspect-ratio: {aspectRatio}"
  role="button"
  tabindex="0"
  onmousemove={resetControlsTimer}
  onclick={togglePlay}
  onkeydown={handleWrapperKeydown}
  aria-label={ariaLabel}
  aria-pressed={isPlaying}
  data-size={resolvedSize}
  data-density={resolvedDensity}
>
  <!-- svelte-ignore a11y_media_has_caption -->
  <video
    bind:this={videoEl}
    {src}
    poster={poster ?? undefined}
    preload="metadata"
    playsinline
    onplay={handlePlay}
    onpause={handlePause}
    onended={handleEnded}
    onloadedmetadata={handleLoadedMetadata}
  >
    {#if showCaptions && captionsSrc}
      <track kind="captions" src={captionsSrc} default />
    {/if}
  </video>

  {#if !isPlaying && currentTime === 0}
    <button
      type="button"
      class="poodle-video-player__big-play"
      onclick={(event) => {
        event.stopPropagation();
        togglePlay();
      }}
      aria-label="Play video"
    >
      <svg viewBox="0 0 48 48" fill="currentColor" aria-hidden="true">
        <circle cx="24" cy="24" r="22" fill="none" stroke="currentColor" stroke-width="2" opacity="0.6" />
        <path d="M18 14l16 10-16 10V14z" />
      </svg>
    </button>
  {/if}

  <!-- svelte-ignore a11y_click_events_have_key_events a11y_no_static_element_interactions -->
  <div
    class="poodle-video-player__controls"
    class:poodle-visible={showControls}
    role="presentation"
    onclick={(event) => event.stopPropagation()}
  >
    <div class="poodle-video-player__progress-bar">
      <div class="poodle-video-player__progress-fill" style="width: {progress}%"></div>
      <input
        type="range"
        class="poodle-video-player__seek"
        min="0"
        max={duration || 0}
        step="0.1"
        value={currentTime}
        oninput={handleSeek}
        aria-label="Seek"
      />
    </div>

    <div class="poodle-video-player__bar">
      <div class="poodle-video-player__bar-left">
        <button
          type="button"
          class="poodle-video-player__btn"
          onclick={(event) => {
            event.stopPropagation();
            togglePlay();
          }}
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

        <button
          type="button"
          class="poodle-video-player__btn"
          onclick={(event) => {
            event.stopPropagation();
            toggleMute();
          }}
          aria-label={isMuted ? "Unmute" : "Mute"}
        >
          {#if isMuted || volume === 0}
            <svg viewBox="0 0 16 16" fill="currentColor" aria-hidden="true">
              <path d="M8 3L4.5 6H2v4h2.5L8 13V3z" />
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
          class="poodle-video-player__volume"
          min="0"
          max="1"
          step="0.01"
          value={isMuted ? 0 : volume}
          oninput={handleVolume}
          aria-label="Volume"
        />

        <span class="poodle-video-player__time">{formattedCurrent} / {formattedDuration}</span>
      </div>

      <div class="poodle-video-player__bar-right">
        <button
          type="button"
          class="poodle-video-player__btn"
          onclick={(event) => {
            event.stopPropagation();
            toggleFullscreen();
          }}
          aria-label={isFullscreen ? "Exit fullscreen" : "Fullscreen"}
        >
          <svg viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.25" stroke-linecap="round" aria-hidden="true">
            {#if isFullscreen}
              <path d="M5 1v3H2m9-3v3h3M5 15v-3H2m9 3v-3h3" />
            {:else}
              <path d="M2 5V2h3m6-0h3v3M2 11v3h3m6 0h3v-3" />
            {/if}
          </svg>
        </button>
      </div>
    </div>
  </div>
</div>

<style>
  .poodle-video-player {
    position: relative;
    overflow: hidden;
    border-radius: var(--poodle-radius-surface);
    background: #000;
    cursor: pointer;
  }

  .poodle-video-player video {
    display: block;
    width: 100%;
    height: 100%;
    object-fit: contain;
  }

  .poodle-video-player__big-play {
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    width: 4rem;
    height: 4rem;
    padding: 0;
    border: 0;
    background: transparent;
    color: rgba(255, 255, 255, 0.9);
    cursor: pointer;
    transition: transform 0.2s ease;
  }

  .poodle-video-player__big-play:hover {
    transform: translate(-50%, -50%) scale(1.1);
  }

  .poodle-video-player__big-play svg {
    width: 100%;
    height: 100%;
  }

  .poodle-video-player__controls {
    position: absolute;
    bottom: 0;
    left: 0;
    right: 0;
    background: linear-gradient(transparent, rgba(0, 0, 0, 0.7));
    padding: 1.5rem 0.5rem 0.375rem;
    opacity: 0;
    transition: opacity 0.3s ease;
    cursor: default;
  }

  .poodle-video-player__controls.poodle-visible {
    opacity: 1;
  }

  .poodle-video-player__progress-bar {
    position: relative;
    height: 0.25rem;
    margin-bottom: 0.375rem;
    background: rgba(255, 255, 255, 0.2);
    border-radius: 999rem;
    overflow: hidden;
  }

  .poodle-video-player__progress-fill {
    height: 100%;
    background: var(--poodle-color-accent-base, #6366f1);
    border-radius: 999rem;
    transition: width 0.1s linear;
  }

  .poodle-video-player__seek {
    position: absolute;
    top: -0.375rem;
    left: 0;
    width: 100%;
    height: 1rem;
    opacity: 0;
    cursor: pointer;
    margin: 0;
  }

  .poodle-video-player__bar {
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  .poodle-video-player__bar-left,
  .poodle-video-player__bar-right {
    display: flex;
    align-items: center;
    gap: 0.375rem;
  }

  .poodle-video-player__btn {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 1.75rem;
    height: 1.75rem;
    padding: 0;
    border: 0;
    border-radius: var(--poodle-radius-control);
    background: transparent;
    color: rgba(255, 255, 255, 0.9);
    cursor: pointer;
    transition: background 0.15s ease;
  }

  .poodle-video-player__btn:hover {
    background: rgba(255, 255, 255, 0.15);
  }

  .poodle-video-player__btn svg {
    width: 0.875rem;
    height: 0.875rem;
  }

  .poodle-video-player__volume {
    -webkit-appearance: none;
    appearance: none;
    width: 3.5rem;
    height: 1rem;
    background: transparent;
    cursor: pointer;
    flex-shrink: 0;
  }

  .poodle-video-player__volume::-webkit-slider-runnable-track {
    height: 0.25rem;
    border-radius: 0.125rem;
    background: rgba(255, 255, 255, 0.5);
  }

  .poodle-video-player__volume::-moz-range-track {
    height: 0.25rem;
    border-radius: 0.125rem;
    background: rgba(255, 255, 255, 0.5);
  }

  .poodle-video-player__volume::-webkit-slider-thumb {
    -webkit-appearance: none;
    width: 0.625rem;
    height: 0.625rem;
    border-radius: 50%;
    border: none;
    background: white;
    margin-top: -0.1875rem;
  }

  .poodle-video-player__volume::-moz-range-thumb {
    width: 0.625rem;
    height: 0.625rem;
    border-radius: 50%;
    border: none;
    background: white;
  }

  .poodle-video-player__time {
    font-family: var(--poodle-typography-code-family);
    font-size: 0.6875rem;
    color: rgba(255, 255, 255, 0.8);
    white-space: nowrap;
  }

  /* ── Size variants ──────────────────────────────────────────── */

  .poodle-video-player[data-size="xs"] .poodle-video-player__btn {
    width: 1.25rem;
    height: 1.25rem;
  }

  .poodle-video-player[data-size="xs"] .poodle-video-player__btn svg {
    width: 0.75rem;
    height: 0.75rem;
  }

  .poodle-video-player[data-size="xs"] .poodle-video-player__volume {
    width: 2.5rem;
  }

  .poodle-video-player[data-size="xs"] .poodle-video-player__time {
    font-size: 0.5625rem;
  }

  .poodle-video-player[data-size="xs"] .poodle-video-player__big-play {
    width: 3rem;
    height: 3rem;
  }

  .poodle-video-player[data-size="sm"] .poodle-video-player__btn {
    width: 1.5rem;
    height: 1.5rem;
  }

  .poodle-video-player[data-size="sm"] .poodle-video-player__volume {
    width: 3rem;
  }

  .poodle-video-player[data-size="sm"] .poodle-video-player__time {
    font-size: 0.625rem;
  }

  .poodle-video-player[data-size="sm"] .poodle-video-player__big-play {
    width: 3.5rem;
    height: 3.5rem;
  }

  .poodle-video-player[data-size="lg"] .poodle-video-player__btn {
    width: 2.125rem;
    height: 2.125rem;
  }

  .poodle-video-player[data-size="lg"] .poodle-video-player__btn svg {
    width: 1rem;
    height: 1rem;
  }

  .poodle-video-player[data-size="lg"] .poodle-video-player__volume {
    width: 4rem;
  }

  .poodle-video-player[data-size="lg"] .poodle-video-player__time {
    font-size: 0.75rem;
  }

  .poodle-video-player[data-size="lg"] .poodle-video-player__big-play {
    width: 4.5rem;
    height: 4.5rem;
  }

  .poodle-video-player[data-size="xl"] .poodle-video-player__btn {
    width: 2.25rem;
    height: 2.25rem;
  }

  .poodle-video-player[data-size="xl"] .poodle-video-player__btn svg {
    width: 1.125rem;
    height: 1.125rem;
  }

  .poodle-video-player[data-size="xl"] .poodle-video-player__volume {
    width: 4.5rem;
  }

  .poodle-video-player[data-size="xl"] .poodle-video-player__time {
    font-size: 0.8125rem;
  }

  .poodle-video-player[data-size="xl"] .poodle-video-player__big-play {
    width: 5rem;
    height: 5rem;
  }

  /* Density variants */
  .poodle-video-player[data-density="compact"] .poodle-video-player__controls { padding-inline: 0.375rem; gap: 0.25rem; }
  .poodle-video-player[data-density="comfortable"] .poodle-video-player__controls { padding-inline: 0.75rem; gap: 0.5rem; }

  .poodle-video-player[data-density="compact"] .poodle-video-player__bar-left,
  .poodle-video-player[data-density="compact"] .poodle-video-player__bar-right {
    gap: 0.25rem;
  }

  .poodle-video-player[data-density="comfortable"] .poodle-video-player__bar-left,
  .poodle-video-player[data-density="comfortable"] .poodle-video-player__bar-right {
    gap: 0.5rem;
  }
</style>
