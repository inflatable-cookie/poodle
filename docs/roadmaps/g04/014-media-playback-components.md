# g04.014 Media Playback Components

Status: planned
Owner: Poodle Core
Updated: 2026-03-14
Depends on: g04.001
Primary repos: `poodle`

## Goals

- [ ] implement AudioPlayer as a primitive for audio file playback with
  transport controls
- [ ] implement VideoPlayer as a primitive for video playback with standard
  controls

## Execution Checklist

- [ ] write contract for AudioPlayer: src, play/pause, seek bar, volume,
  duration display, playback speed, waveform visualization (optional)
- [ ] implement AudioPlayer primitive in `@poodle/svelte`
- [ ] write contract for VideoPlayer: src, play/pause, seek bar, volume,
  fullscreen, aspect ratio, poster image, captions/subtitles
- [ ] implement VideoPlayer primitive in `@poodle/svelte`
- [ ] create specimens for AudioPlayer and VideoPlayer
- [ ] register in component-registry.ts and specimen registry

## Acceptance Criteria

- [ ] AudioPlayer renders transport controls (play/pause, seek, volume)
- [ ] AudioPlayer displays current time and duration
- [ ] AudioPlayer supports playback speed adjustment
- [ ] VideoPlayer renders video with overlay controls
- [ ] VideoPlayer supports fullscreen toggle
- [ ] VideoPlayer displays poster image before playback
- [ ] both components use Poodle tokens for consistent styling
- [ ] both components pass build and render in the preview catalogue

## Next Task

Open `g04.015` and explore the block editor baseline.
