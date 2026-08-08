import "@inflatable-cookie/poodle-styles/video-player.css";

import { useEffect, useRef, useState, type ChangeEvent, type KeyboardEvent as ReactKeyboardEvent } from "react";

import { resolveSemanticControlSize, useUiPresentation } from "./presentation";
import type { ControlDensity, ControlSize, SemanticControlSizeRole } from "./types";

export interface VideoPlayerProps {
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

function formatTime(sec: number): string {
  const m = Math.floor(sec / 60);
  const s = Math.floor(sec % 60);
  return `${m}:${s.toString().padStart(2, "0")}`;
}

export function VideoPlayer({
  src,
  poster = null,
  aspectRatio = 16 / 9,
  ariaLabel = "Video player",
  showCaptions = false,
  captionsSrc = null,
  size = null,
  sizeRole = "control",
  density = null,
}: VideoPlayerProps) {
  const uiPresentation = useUiPresentation();

  const videoRef = useRef<HTMLVideoElement | null>(null);
  const wrapperRef = useRef<HTMLDivElement | null>(null);
  const controlsTimeout = useRef<ReturnType<typeof setTimeout> | null>(null);
  const animFrame = useRef<number | null>(null);

  const [isPlaying, setIsPlaying] = useState(false);
  const [currentTime, setCurrentTime] = useState(0);
  const [duration, setDuration] = useState(0);
  const [volume, setVolume] = useState(1);
  const [isMuted, setIsMuted] = useState(false);
  const [isFullscreen, setIsFullscreen] = useState(false);
  const [showControls, setShowControls] = useState(true);

  const resolvedSize = size ?? resolveSemanticControlSize(uiPresentation.sizeScale, sizeRole);
  const resolvedDensity = density ?? uiPresentation.density;
  const progress = duration > 0 ? (currentTime / duration) * 100 : 0;

  function togglePlay(): void {
    const video = videoRef.current;
    if (!video) return;
    if (!video.paused) {
      video.pause();
    } else {
      void video.play();
    }
  }

  function handleSeek(event: ChangeEvent<HTMLInputElement>): void {
    const video = videoRef.current;
    if (!video) return;
    const value = Number(event.currentTarget.value);
    video.currentTime = value;
    setCurrentTime(value);
  }

  function handleVolume(event: ChangeEvent<HTMLInputElement>): void {
    const video = videoRef.current;
    if (!video) return;
    const nextVolume = Number(event.currentTarget.value);
    setVolume(nextVolume);
    video.volume = nextVolume;
    setIsMuted(nextVolume === 0);
  }

  function toggleMute(): void {
    const video = videoRef.current;
    if (!video) return;
    const next = !isMuted;
    setIsMuted(next);
    video.muted = next;
  }

  function toggleFullscreen(): void {
    const wrapper = wrapperRef.current;
    if (!wrapper) return;
    if (!document.fullscreenElement) {
      void wrapper.requestFullscreen();
    } else {
      void document.exitFullscreen();
    }
  }

  function resetControlsTimer(): void {
    setShowControls(true);
    if (controlsTimeout.current) clearTimeout(controlsTimeout.current);
    if (!videoRef.current?.paused) {
      controlsTimeout.current = setTimeout(() => {
        setShowControls(false);
      }, 3000);
    }
  }

  function updateTime(): void {
    if (videoRef.current) {
      setCurrentTime(videoRef.current.currentTime);
    }
    if (!videoRef.current?.paused) {
      animFrame.current = requestAnimationFrame(updateTime);
    }
  }

  function handleWrapperKeydown(event: ReactKeyboardEvent): void {
    if (event.target !== wrapperRef.current) return;

    if (event.key === " " || event.key === "Enter") {
      event.preventDefault();
      togglePlay();
    }
  }

  function handlePlay(): void {
    setIsPlaying(true);
    updateTime();
    resetControlsTimer();
  }

  function handlePause(): void {
    setIsPlaying(false);
    setShowControls(true);
    if (animFrame.current !== null) cancelAnimationFrame(animFrame.current);
  }

  function handleEnded(): void {
    setIsPlaying(false);
    setShowControls(true);
    if (animFrame.current !== null) cancelAnimationFrame(animFrame.current);
  }

  function handleLoadedMetadata(): void {
    setDuration(videoRef.current?.duration ?? 0);
  }

  useEffect(() => {
    function handleFullscreenChange(): void {
      setIsFullscreen(Boolean(document.fullscreenElement));
    }

    document.addEventListener("fullscreenchange", handleFullscreenChange);

    return () => {
      document.removeEventListener("fullscreenchange", handleFullscreenChange);
      if (animFrame.current !== null) cancelAnimationFrame(animFrame.current);
      if (controlsTimeout.current) clearTimeout(controlsTimeout.current);
    };
  }, []);

  return (
    <div
      className="poodle-video-player"
      ref={wrapperRef}
      style={{ aspectRatio: `${aspectRatio}` }}
      onMouseMove={resetControlsTimer}
      onClick={togglePlay}
      onKeyDown={handleWrapperKeydown}
      aria-label={ariaLabel}
      data-size={resolvedSize}
      data-density={resolvedDensity}
    >
      <video
        ref={videoRef}
        src={src}
        poster={poster ?? undefined}
        preload="metadata"
        playsInline
        onPlay={handlePlay}
        onPause={handlePause}
        onEnded={handleEnded}
        onLoadedMetadata={handleLoadedMetadata}
      >
        {showCaptions && captionsSrc ? <track kind="captions" src={captionsSrc} default /> : null}
      </video>

      {!isPlaying && currentTime === 0 ? (
        <button
          type="button"
          className="poodle-video-player__big-play"
          onClick={(event) => {
            event.stopPropagation();
            togglePlay();
          }}
          aria-label="Play video"
        >
          <svg viewBox="0 0 48 48" fill="currentColor" aria-hidden="true">
            <circle cx="24" cy="24" r="22" fill="none" stroke="currentColor" strokeWidth="2" opacity="0.6" />
            <path d="M18 14l16 10-16 10V14z" />
          </svg>
        </button>
      ) : null}

      <div
        className={showControls ? "poodle-video-player__controls poodle-visible" : "poodle-video-player__controls"}
        role="presentation"
        onClick={(event) => event.stopPropagation()}
      >
        <div className="poodle-video-player__progress-bar">
          <div className="poodle-video-player__progress-fill" style={{ width: `${progress}%` }} />
          <input
            type="range"
            className="poodle-video-player__seek"
            min="0"
            max={duration || 0}
            step="0.1"
            value={currentTime}
            onChange={handleSeek}
            aria-label="Seek"
          />
        </div>

        <div className="poodle-video-player__bar">
          <div className="poodle-video-player__bar-left">
            <button
              type="button"
              className="poodle-video-player__btn"
              onClick={(event) => {
                event.stopPropagation();
                togglePlay();
              }}
              aria-label={isPlaying ? "Pause" : "Play"}
            >
              {isPlaying ? (
                <svg viewBox="0 0 16 16" fill="currentColor" aria-hidden="true">
                  <rect x="3" y="2" width="3.5" height="12" rx="0.75" />
                  <rect x="9.5" y="2" width="3.5" height="12" rx="0.75" />
                </svg>
              ) : (
                <svg viewBox="0 0 16 16" fill="currentColor" aria-hidden="true">
                  <path d="M4 2.5l9 5.5-9 5.5V2.5z" />
                </svg>
              )}
            </button>

            <button
              type="button"
              className="poodle-video-player__btn"
              onClick={(event) => {
                event.stopPropagation();
                toggleMute();
              }}
              aria-label={isMuted ? "Unmute" : "Mute"}
            >
              {isMuted || volume === 0 ? (
                <svg viewBox="0 0 16 16" fill="currentColor" aria-hidden="true">
                  <path d="M8 3L4.5 6H2v4h2.5L8 13V3z" />
                </svg>
              ) : (
                <svg viewBox="0 0 16 16" fill="none" stroke="currentColor" strokeWidth="1.25" aria-hidden="true">
                  <path d="M8 3L4.5 6H2v4h2.5L8 13V3z" fill="currentColor" stroke="none" />
                  <path d="M11 5.5a4 4 0 010 5" strokeLinecap="round" />
                </svg>
              )}
            </button>

            <input
              type="range"
              className="poodle-video-player__volume"
              min="0"
              max="1"
              step="0.01"
              value={isMuted ? 0 : volume}
              onChange={handleVolume}
              aria-label="Volume"
            />

            <span className="poodle-video-player__time">
              {formatTime(currentTime)} / {formatTime(duration)}
            </span>
          </div>

          <div className="poodle-video-player__bar-right">
            <button
              type="button"
              className="poodle-video-player__btn"
              onClick={(event) => {
                event.stopPropagation();
                toggleFullscreen();
              }}
              aria-label={isFullscreen ? "Exit fullscreen" : "Fullscreen"}
            >
              <svg viewBox="0 0 16 16" fill="none" stroke="currentColor" strokeWidth="1.25" strokeLinecap="round" aria-hidden="true">
                {isFullscreen ? <path d="M5 1v3H2m9-3v3h3M5 15v-3H2m9 3v-3h3" /> : <path d="M2 5V2h3m6-0h3v3M2 11v3h3m6 0h3v-3" />}
              </svg>
            </button>
          </div>
        </div>
      </div>
    </div>
  );
}
