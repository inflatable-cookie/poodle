# VideoPlayer

Status: seed contract
Updated: 2026-03-22

## 1. Purpose

- Component name: `VideoPlayer`
- Layer: `composites`
- Summary: a video playback component with custom overlay controls, seek bar, volume, fullscreen, and optional captions
- In scope: play/pause, seek slider, time display, volume slider, mute toggle, fullscreen toggle, poster image, captions track, auto-hiding controls, big play button overlay
- Out of scope: playlist management, picture-in-picture, playback speed, quality selector, streaming protocol (HLS/DASH), download actions, theatre mode

## 2. Anatomy

```text
[Root]  <div> with aspect-ratio
  ├── [Video]  <video> element
  │     └── [Captions Track]  (optional, when showCaptions && captionsSrc)
  ├── [Big Play Button]  (only when paused at start)
  └── [Controls]  auto-hiding overlay at bottom
        ├── [Progress Bar]
        │     ├── [Progress Fill]  visual fill
        │     └── [Seek Slider]  <input type="range">, transparent overlay
        └── [Bar]
              ├── [Bar Left]
              │     ├── [Play/Pause Button]
              │     ├── [Mute Button]
              │     ├── [Volume Slider]
              │     └── [Time Display]  current / duration
              └── [Bar Right]
                    └── [Fullscreen Button]
```

### Parts

| Part | Element | Notes |
|------|---------|-------|
| root | `<div>` | Positioned relative, overflow hidden, `radius-surface`, black background, cursor pointer |
| video | `<video>` | Full-size, `object-fit: contain`, `preload="metadata"`, `playsinline` |
| captions-track | `<track>` | `kind="captions"`, optional |
| big-play | `<button>` | Centered overlay play icon, only visible when paused at `currentTime=0` |
| controls | `<div>` | Gradient overlay at bottom, auto-hides after 3s during playback |
| progress-bar | `<div>` | `0.25rem` tall track with fill and transparent range input overlay |
| progress-fill | `<div>` | Fills proportionally to playback progress, accent color |
| seek-slider | `<input type="range">` | Transparent overlay on progress bar, `aria-label="Seek"` |
| bar | `<div>` | Flex row, space-between, holding control buttons |
| play-button | `<button>` | Toggles play/pause, icon swaps |
| mute-button | `<button>` | Toggles mute, icon reflects muted state |
| volume-slider | `<input type="range">` | `0` to `1`, step `0.01` |
| time-display | `<span>` | `m:ss / m:ss` format, monospace |
| fullscreen-button | `<button>` | Toggles fullscreen, icon swaps |

## 3. Props And Inputs

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `src` | `string` | -- | yes | Video source URL |
| `poster` | `string \| null` | `null` | no | Poster image URL shown before playback |
| `aspectRatio` | `number` | `16 / 9` | no | CSS aspect-ratio for the container |
| `ariaLabel` | `string` | `"Video player"` | no | Accessible label for the root container |
| `showCaptions` | `boolean` | `false` | no | Enable captions track |
| `captionsSrc` | `string \| null` | `null` | no | URL to captions/subtitles file |

### Slots

None.

### Controlled / Uncontrolled

All playback state (isPlaying, currentTime, volume, isMuted, isFullscreen) is managed internally. Props control initial configuration only.

## 4. States

### Visual States

| State | Trigger | Visual Effect |
|-------|---------|---------------|
| initial | Video loaded, not yet played | Big play button centered, controls visible |
| playing | Video playing | Controls auto-hide after 3s; mouse movement re-shows them |
| paused | Video paused | Controls remain visible |
| ended | Video ended | Controls visible, play button shows play icon |
| controls-visible | Mouse movement or paused | Controls overlay opacity 1 |
| controls-hidden | 3s after last mouse move during playback | Controls overlay opacity 0 |
| fullscreen | Fullscreen active | Fullscreen button icon changes to exit-fullscreen |
| muted | Muted or volume=0 | Mute button icon shows muted variant |

### Component States

| State | Description |
|-------|-------------|
| idle | Initial state, video loaded but not started |
| playing | Video is actively playing |
| paused | Video paused mid-playback |
| ended | Playback completed |

## 5. Events

No custom events dispatched. All interaction is internal (play/pause/seek/volume/fullscreen).

## 6. Accessibility

### Semantics

- Root carries `aria-label` from prop
- Play/pause button: `aria-label` toggles between `"Pause"` and `"Play"`
- Big play button: `aria-label="Play video"`
- Mute button: `aria-label` toggles between `"Unmute"` and `"Mute"`
- Seek slider: `aria-label="Seek"`
- Volume slider: `aria-label="Volume"`
- Fullscreen button: `aria-label` toggles between `"Exit fullscreen"` and `"Fullscreen"`
- All SVG icons are `aria-hidden="true"`

### Keyboard

- Clicking root toggles play/pause
- Control buttons are standard button keyboard interaction
- Range inputs accept arrow key adjustments
- Controls bar click events use `stopPropagation` to avoid triggering play/pause

### Focus

- Control buttons show hover state (white background at 15% opacity)
- Focus styling inherits from button defaults

## 7. Layout

### Sizing

- Root: full width, `aspect-ratio` from prop, `radius-surface`, overflow hidden, black background
- Video: `width: 100%`, `height: 100%`, `object-fit: contain`
- Big play button: `4rem x 4rem`, centered absolutely, scale 1.1 on hover
- Controls: absolute bottom, gradient from transparent to `rgba(0,0,0,0.7)`, padding `1.5rem 0.5rem 0.375rem`
- Progress bar: `0.25rem` tall, `0.375rem` margin-bottom
- Control buttons: `1.75rem x 1.75rem`, icon `0.875rem x 0.875rem`
- Volume slider: `3.5rem` width
- Time display: `0.6875rem` font-size

### Composition

Standalone component. Does not compose other Flint primitives.

## 8. Token Usage And Precise CSS

Note: The video player renders against a black background regardless of theme.
Most colors use hardcoded `rgba(255,255,255,...)` values intentionally.

### Root

| Property | Value |
|----------|-------|
| position | `relative` |
| overflow | `hidden` |
| border-radius | `var(--flint-radius-surface)` |
| background | `#000` |
| cursor | `pointer` |
| aspect-ratio | set via inline style from `aspectRatio` prop |

### Video Element

| Property | Value |
|----------|-------|
| display | `block` |
| width | `100%` |
| height | `100%` |
| object-fit | `contain` |

### Big Play Button

| Property | Value |
|----------|-------|
| position | `absolute` |
| top | `50%` |
| left | `50%` |
| transform | `translate(-50%, -50%)` |
| width | `4rem` |
| height | `4rem` |
| padding | `0` |
| border | `0` |
| background | `transparent` |
| color | `rgba(255, 255, 255, 0.9)` |
| transition | `transform 0.2s ease` |
| `:hover` transform | `translate(-50%, -50%) scale(1.1)` |
| SVG circle | cx=24, cy=24, r=22, stroke-width=2, opacity=0.6 |
| SVG play path | `M18 14l16 10-16 10V14z` |

### Controls Overlay

| Property | Value |
|----------|-------|
| position | `absolute` |
| bottom | `0` |
| left | `0` |
| right | `0` |
| background | `linear-gradient(transparent, rgba(0, 0, 0, 0.7))` |
| padding | `1.5rem 0.5rem 0.375rem` |
| opacity | `0` (default), `1` (`.visible`) |
| transition | `opacity 0.3s ease` |
| cursor | `default` |

### Progress Bar

| Property | Value |
|----------|-------|
| position | `relative` |
| height | `0.25rem` |
| margin-bottom | `0.375rem` |
| background | `rgba(255, 255, 255, 0.2)` |
| border-radius | `999rem` |
| overflow | `hidden` |

### Progress Fill

| Property | Value |
|----------|-------|
| height | `100%` |
| background | `var(--flint-color-accent-base, #6366f1)` |
| border-radius | `999rem` |
| transition | `width 0.1s linear` |

### Seek Slider (transparent overlay)

| Property | Value |
|----------|-------|
| position | `absolute` |
| top | `-0.375rem` |
| left | `0` |
| width | `100%` |
| height | `1rem` |
| opacity | `0` |
| margin | `0` |

### Control Bar

| Property | Value |
|----------|-------|
| display | `flex` |
| align-items | `center` |
| justify-content | `space-between` |

### Bar Left / Bar Right

| Property | Value |
|----------|-------|
| display | `flex` |
| align-items | `center` |
| gap | `0.375rem` |

### Control Button

| Property | Value |
|----------|-------|
| display | `inline-flex` |
| align-items | `center` |
| justify-content | `center` |
| width | `1.75rem` |
| height | `1.75rem` |
| padding | `0` |
| border | `0` |
| border-radius | `var(--flint-radius-control)` |
| background | `transparent` |
| color | `rgba(255, 255, 255, 0.9)` |
| transition | `background 0.15s ease` |
| `:hover` background | `rgba(255, 255, 255, 0.15)` |
| SVG icon size | `0.875rem x 0.875rem` |

### Volume Slider

| Property | Value |
|----------|-------|
| width | `3.5rem` |
| height | `0.25rem` |
| accent-color | `white` |

### Time Display

| Property | Value |
|----------|-------|
| font-family | `var(--flint-typography-code-family)` |
| font-size | `0.6875rem` |
| color | `rgba(255, 255, 255, 0.8)` |
| white-space | `nowrap` |

### Light Theme Overrides

None (video player uses hardcoded dark-on-black colors).

## 9. Svelte Notes

- Uses `onMount` for video event listeners (`play`, `pause`, `ended`, `loadedmetadata`)
- `requestAnimationFrame` loop updates `currentTime` during playback for smooth seek bar
- Controls auto-hide via `setTimeout` (3s) reset on `mousemove`
- Fullscreen uses native `requestFullscreen` / `exitFullscreen` API
- `onDestroy` cleans up animation frame and timeout
- Click on root toggles play; control bar uses `stopPropagation` to prevent double-toggle

## 10. GPUI Notes

Not yet implemented.

## 11. Parity Checklist

| Feature | Svelte | GPUI | Jetstream |
|---------|--------|------|-----------|
| Play/pause toggle | Yes | -- | -- |
| Seek slider | Yes | -- | -- |
| Time display | Yes | -- | -- |
| Volume slider | Yes | -- | -- |
| Mute toggle | Yes | -- | -- |
| Fullscreen toggle | Yes | -- | -- |
| Big play button | Yes | -- | -- |
| Auto-hiding controls | Yes | -- | -- |
| Poster image | Yes | -- | -- |
| Captions track | Yes | -- | -- |

## 12. Known Deltas

None yet (single implementation).

## 13. Specimen Definitions

### Video Player

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Video player | `src` set to sample video, `ariaLabel="Sample video"` | Player with controls; click big play to start |

### Custom Aspect Ratio (4:3)

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Custom aspect ratio | `src` set to sample video, `aspectRatio={4/3}`, `ariaLabel="4:3 aspect video"` | Player with 4:3 aspect ratio container |

## 14. Approval And Adoption Notes

Use `VideoPlayer` for single-video playback embedded in content pages or media detail views. The component uses native HTML5 `<video>` with custom overlay controls. For streaming video with adaptive bitrate, the consuming application should handle protocol selection and provide a compatible `src`. The hardcoded white-on-black control colors are intentional for video player UI contrast and do not follow the theme token system.
