<script lang="ts">
  import "./video-player.css";
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

