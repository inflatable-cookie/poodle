<script lang="ts">
  import { onMount, onDestroy } from "svelte";

  export let src: string;
  export let poster: string | null = null;
  export let aspectRatio: number = 16 / 9;
  export let ariaLabel = "Video player";
  export let showCaptions = false;
  export let captionsSrc: string | null = null;

  let videoEl: HTMLVideoElement | null = null;
  let wrapperEl: HTMLDivElement | null = null;
  let isPlaying = false;
  let currentTime = 0;
  let duration = 0;
  let volume = 1;
  let isMuted = false;
  let isFullscreen = false;
  let showControls = true;
  let controlsTimeout: ReturnType<typeof setTimeout> | null = null;
  let animFrame: number | null = null;

  $: formattedCurrent = formatTime(currentTime);
  $: formattedDuration = formatTime(duration);
  $: progress = duration > 0 ? (currentTime / duration) * 100 : 0;

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

  onMount(() => {
    if (!videoEl) return;

    videoEl.addEventListener("play", () => {
      isPlaying = true;
      updateTime();
      resetControlsTimer();
    });
    videoEl.addEventListener("pause", () => {
      isPlaying = false;
      showControls = true;
      if (animFrame !== null) cancelAnimationFrame(animFrame);
    });
    videoEl.addEventListener("ended", () => {
      isPlaying = false;
      showControls = true;
      if (animFrame !== null) cancelAnimationFrame(animFrame);
    });
    videoEl.addEventListener("loadedmetadata", () => {
      duration = videoEl?.duration ?? 0;
    });

    document.addEventListener("fullscreenchange", () => {
      isFullscreen = !!document.fullscreenElement;
    });
  });

  onDestroy(() => {
    if (animFrame !== null) cancelAnimationFrame(animFrame);
    if (controlsTimeout) clearTimeout(controlsTimeout);
  });
</script>

<!-- svelte-ignore a11y-no-static-element-interactions -->
<div
  class="video-player"
  bind:this={wrapperEl}
  style="aspect-ratio: {aspectRatio}"
  on:mousemove={resetControlsTimer}
  on:click={togglePlay}
  aria-label={ariaLabel}
>
  <!-- svelte-ignore a11y-media-has-caption -->
  <video
    bind:this={videoEl}
    {src}
    poster={poster ?? undefined}
    preload="metadata"
    playsinline
  >
    {#if showCaptions && captionsSrc}
      <track kind="captions" src={captionsSrc} default />
    {/if}
  </video>

  {#if !isPlaying && currentTime === 0}
    <button
      type="button"
      class="video-player__big-play"
      on:click|stopPropagation={togglePlay}
      aria-label="Play video"
    >
      <svg viewBox="0 0 48 48" fill="currentColor" aria-hidden="true">
        <circle cx="24" cy="24" r="22" fill="none" stroke="currentColor" stroke-width="2" opacity="0.6" />
        <path d="M18 14l16 10-16 10V14z" />
      </svg>
    </button>
  {/if}

  <!-- svelte-ignore a11y-click-events-have-key-events a11y-no-static-element-interactions -->
  <div class="video-player__controls" class:visible={showControls} on:click|stopPropagation>
    <div class="video-player__progress-bar">
      <div class="video-player__progress-fill" style="width: {progress}%"></div>
      <input
        type="range"
        class="video-player__seek"
        min="0"
        max={duration || 0}
        step="0.1"
        value={currentTime}
        on:input={handleSeek}
        aria-label="Seek"
      />
    </div>

    <div class="video-player__bar">
      <div class="video-player__bar-left">
        <button
          type="button"
          class="video-player__btn"
          on:click|stopPropagation={togglePlay}
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
          class="video-player__btn"
          on:click|stopPropagation={toggleMute}
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
          class="video-player__volume"
          min="0"
          max="1"
          step="0.01"
          value={isMuted ? 0 : volume}
          on:input={handleVolume}
          aria-label="Volume"
        />

        <span class="video-player__time">{formattedCurrent} / {formattedDuration}</span>
      </div>

      <div class="video-player__bar-right">
        <button
          type="button"
          class="video-player__btn"
          on:click|stopPropagation={toggleFullscreen}
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
  .video-player {
    position: relative;
    overflow: hidden;
    border-radius: var(--pug-radius-surface);
    background: #000;
    cursor: pointer;
  }

  .video-player video {
    display: block;
    width: 100%;
    height: 100%;
    object-fit: contain;
  }

  .video-player__big-play {
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

  .video-player__big-play:hover {
    transform: translate(-50%, -50%) scale(1.1);
  }

  .video-player__big-play svg {
    width: 100%;
    height: 100%;
  }

  .video-player__controls {
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

  .video-player__controls.visible {
    opacity: 1;
  }

  .video-player__progress-bar {
    position: relative;
    height: 0.25rem;
    margin-bottom: 0.375rem;
    background: rgba(255, 255, 255, 0.2);
    border-radius: 999rem;
    overflow: hidden;
  }

  .video-player__progress-fill {
    height: 100%;
    background: var(--pug-color-accent-base, #6366f1);
    border-radius: 999rem;
    transition: width 0.1s linear;
  }

  .video-player__seek {
    position: absolute;
    top: -0.375rem;
    left: 0;
    width: 100%;
    height: 1rem;
    opacity: 0;
    cursor: pointer;
    margin: 0;
  }

  .video-player__bar {
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  .video-player__bar-left,
  .video-player__bar-right {
    display: flex;
    align-items: center;
    gap: 0.375rem;
  }

  .video-player__btn {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 1.75rem;
    height: 1.75rem;
    padding: 0;
    border: 0;
    border-radius: var(--pug-radius-control);
    background: transparent;
    color: rgba(255, 255, 255, 0.9);
    cursor: pointer;
    transition: background 0.15s ease;
  }

  .video-player__btn:hover {
    background: rgba(255, 255, 255, 0.15);
  }

  .video-player__btn svg {
    width: 0.875rem;
    height: 0.875rem;
  }

  .video-player__volume {
    width: 3.5rem;
    height: 0.25rem;
    accent-color: white;
  }

  .video-player__time {
    font-family: var(--pug-typography-code-family);
    font-size: 0.6875rem;
    color: rgba(255, 255, 255, 0.8);
    white-space: nowrap;
  }
</style>
