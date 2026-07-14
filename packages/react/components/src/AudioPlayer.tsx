import "@poodle/styles/audio-player.css";

import { useEffect, useRef, useState, type ChangeEvent } from "react";

import { resolveSemanticControlSize, useUiPresentation } from "./presentation";
import type { ControlDensity, ControlSize, SemanticControlSizeRole } from "./types";

export interface AudioPlayerProps {
  src: string;
  ariaLabel?: string;
  showSpeedControl?: boolean;
  size?: ControlSize | null;
  sizeRole?: SemanticControlSizeRole;
  density?: ControlDensity | null;
}

const speedOptions = [0.5, 0.75, 1, 1.25, 1.5, 2];

function formatTime(sec: number): string {
  const m = Math.floor(sec / 60);
  const s = Math.floor(sec % 60);
  return `${m}:${s.toString().padStart(2, "0")}`;
}

export function AudioPlayer({
  src,
  ariaLabel = "Audio player",
  showSpeedControl = false,
  size = null,
  sizeRole = "control",
  density = null,
}: AudioPlayerProps) {
  const uiPresentation = useUiPresentation();

  const audioRef = useRef<HTMLAudioElement | null>(null);
  const animFrame = useRef<number | null>(null);
  const [isPlaying, setIsPlaying] = useState(false);
  const [currentTime, setCurrentTime] = useState(0);
  const [duration, setDuration] = useState(0);
  const [volume, setVolume] = useState(1);
  const [playbackRate, setPlaybackRate] = useState(1);
  const [isMuted, setIsMuted] = useState(false);

  const resolvedSize = size ?? resolveSemanticControlSize(uiPresentation.sizeScale, sizeRole);
  const resolvedDensity = density ?? uiPresentation.density;

  function togglePlay(): void {
    const audio = audioRef.current;
    if (!audio) return;
    if (isPlaying) {
      audio.pause();
    } else {
      void audio.play();
    }
  }

  function handleSeek(event: ChangeEvent<HTMLInputElement>): void {
    const audio = audioRef.current;
    if (!audio) return;
    const value = Number(event.currentTarget.value);
    audio.currentTime = value;
    setCurrentTime(value);
  }

  function handleVolume(event: ChangeEvent<HTMLInputElement>): void {
    const audio = audioRef.current;
    if (!audio) return;
    const nextVolume = Number(event.currentTarget.value);
    setVolume(nextVolume);
    audio.volume = nextVolume;
    setIsMuted(nextVolume === 0);
  }

  function toggleMute(): void {
    const audio = audioRef.current;
    if (!audio) return;
    const next = !isMuted;
    setIsMuted(next);
    audio.muted = next;
  }

  function handleSpeedChange(event: ChangeEvent<HTMLSelectElement>): void {
    const audio = audioRef.current;
    if (!audio) return;
    const rate = Number(event.currentTarget.value);
    setPlaybackRate(rate);
    audio.playbackRate = rate;
  }

  function updateTime(): void {
    if (audioRef.current) {
      setCurrentTime(audioRef.current.currentTime);
    }
    if (!audioRef.current?.paused) {
      animFrame.current = requestAnimationFrame(updateTime);
    }
  }

  function handlePlay(): void {
    setIsPlaying(true);
    updateTime();
  }

  function handlePause(): void {
    setIsPlaying(false);
    if (animFrame.current !== null) cancelAnimationFrame(animFrame.current);
  }

  function handleEnded(): void {
    setIsPlaying(false);
    if (animFrame.current !== null) cancelAnimationFrame(animFrame.current);
  }

  function updateDuration(): void {
    setDuration(audioRef.current?.duration ?? 0);
  }

  useEffect(() => {
    return () => {
      if (animFrame.current !== null) cancelAnimationFrame(animFrame.current);
    };
  }, []);

  return (
    <div className="poodle-audio-player" aria-label={ariaLabel} data-size={resolvedSize} data-density={resolvedDensity}>
      <audio
        ref={audioRef}
        src={src}
        preload="metadata"
        onPlay={handlePlay}
        onPause={handlePause}
        onEnded={handleEnded}
        onLoadedMetadata={updateDuration}
        onDurationChange={updateDuration}
      />

      <button type="button" className="poodle-audio-player__play" onClick={togglePlay} aria-label={isPlaying ? "Pause" : "Play"}>
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

      <span className="poodle-audio-player__time">{formatTime(currentTime)}</span>

      <input
        type="range"
        className="poodle-audio-player__seek"
        min="0"
        max={duration || 0}
        step="0.1"
        value={currentTime}
        onChange={handleSeek}
        aria-label="Seek"
      />

      <span className="poodle-audio-player__time">{formatTime(duration)}</span>

      <button type="button" className="poodle-audio-player__mute" onClick={toggleMute} aria-label={isMuted ? "Unmute" : "Mute"}>
        {isMuted || volume === 0 ? (
          <svg viewBox="0 0 16 16" fill="none" stroke="currentColor" strokeWidth="1.25" aria-hidden="true">
            <path d="M8 3L4.5 6H2v4h2.5L8 13V3z" fill="currentColor" stroke="none" />
            <path d="M12 5l-4 6m0-6l4 6" strokeLinecap="round" />
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
        className="poodle-audio-player__volume"
        min="0"
        max="1"
        step="0.01"
        value={isMuted ? 0 : volume}
        onChange={handleVolume}
        aria-label="Volume"
      />

      {showSpeedControl ? (
        <select className="poodle-audio-player__speed" value={playbackRate} onChange={handleSpeedChange} aria-label="Playback speed">
          {speedOptions.map((speed) => (
            <option key={speed} value={speed}>
              {speed}×
            </option>
          ))}
        </select>
      ) : null}
    </div>
  );
}
