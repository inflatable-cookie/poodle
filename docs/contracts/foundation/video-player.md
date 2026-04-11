# VideoPlayer

Status: detailed contract
Updated: 2026-03-30

## 1. Purpose

- Component name: `VideoPlayer`
- Layer: `primitives`
- Summary: a video playback component with custom overlay controls, seek bar,
  volume, fullscreen, and optional captions
- In scope: play/pause, seek slider, time display, volume slider, mute toggle,
  fullscreen toggle, poster image, captions track, auto-hiding controls, big
  play button overlay, size and density scaling, keyboard space/enter on wrapper
- Out of scope: playlist management, picture-in-picture, playback speed, quality
  selector, streaming protocol (HLS/DASH), download actions, theatre mode

## 2. Anatomy

```text
[Root <div>]  role="button", tabindex=0, aspect-ratio from prop
  ├── [Video <video>]
  │     └── [CaptionsTrack <track>]  (optional, when showCaptions && captionsSrc)
  ├── [BigPlay <button>]  (only when paused at currentTime=0)
  │     └── [BigPlaySVG]  48x48 circle with play triangle
  └── [Controls <div>]  auto-hiding overlay at bottom
        ├── [ProgressBar <div>]
        │     ├── [ProgressFill <div>]  visual fill
        │     └── [SeekSlider <input type="range">]  transparent overlay
        └── [Bar <div>]
              ├── [BarLeft <div>]
              │     ├── [PlayButton <button>]
              │     ├── [MuteButton <button>]
              │     ├── [VolumeSlider <input type="range">]
              │     └── [TimeDisplay <span>]
              └── [BarRight <div>]
                    └── [FullscreenButton <button>]
```

### Parts

| Part | Element | Notes |
|------|---------|-------|
| Root | `<div>` | Class `video-player`, `role="button"`, `tabindex="0"`, `aria-label` from prop, `aria-pressed` from play state, `data-size`, `data-density` |
| Video | `<video>` | Full-size, `object-fit: contain`, `preload="metadata"`, `playsinline` |
| CaptionsTrack | `<track>` | `kind="captions"`, `default`, rendered when `showCaptions && captionsSrc` |
| BigPlay | `<button>` | Centered overlay, `aria-label="Play video"`, uses `stopPropagation` |
| Controls | `<div>` | Gradient overlay, toggles `.visible` class, uses `stopPropagation` on click |
| ProgressBar | `<div>` | `0.25rem` tall track |
| ProgressFill | `<div>` | Width set via inline style from progress percentage |
| SeekSlider | `<input type="range">` | Transparent overlay, `aria-label="Seek"`, `step="0.1"` |
| Bar | `<div>` | Flex row, space-between |
| PlayButton | `<button>` | `aria-label` toggles "Pause" / "Play", SVG icon swaps |
| MuteButton | `<button>` | `aria-label` toggles "Unmute" / "Mute", SVG icon swaps |
| VolumeSlider | `<input type="range">` | `0` to `1`, `step="0.01"`, `aria-label="Volume"`, custom range input styling (track: 0.25rem height, white 50% opacity background, 0.125rem radius; thumb: 0.625rem white circle, no border) |
| TimeDisplay | `<span>` | `m:ss / m:ss` format, monospace font |
| FullscreenButton | `<button>` | `aria-label` toggles "Exit fullscreen" / "Fullscreen", SVG icon swaps |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `src` | `string` | -- | yes | Video source URL |
| `poster` | `string \| null` | `null` | no | Poster image URL shown before playback |
| `aspectRatio` | `number` | `16 / 9` | no | CSS aspect-ratio for the container, set via inline style |
| `ariaLabel` | `string` | `"Video player"` | no | Accessible label for the root container |
| `showCaptions` | `boolean` | `false` | no | Enable captions track |
| `captionsSrc` | `string \| null` | `null` | no | URL to captions/subtitles file |
| `size` | `ControlSize \| null` | `null` | no | Explicit size override for controls |
| `sizeRole` | `SemanticControlSizeRole` | `"control"` | no | Semantic size intent for presentation context resolution |
| `density` | `ControlDensity \| null` | `null` | no | Explicit density override for spacing |

### Slots

None.

### Controlled And Uncontrolled

All playback state (`isPlaying`, `currentTime`, `volume`, `isMuted`,
`isFullscreen`, `showControls`) is managed internally. Props control initial
configuration only.

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| initial | Video loaded, not yet played | Big play button centered, controls visible |
| playing | Video playing | Controls auto-hide after 3s; mouse movement re-shows them |
| paused | Video paused mid-playback | Controls remain visible |
| ended | Video ended | Controls visible, play button shows play icon |
| controls-visible | Mouse movement or paused/initial state | Controls overlay opacity 1 |
| controls-hidden | 3s after last mouse move during playback | Controls overlay opacity 0 |
| fullscreen | Fullscreen active | Fullscreen button icon changes to exit-fullscreen |
| muted | Muted or volume=0 | Mute button icon shows muted variant |

### Internal State

| State | Type | Description |
|-------|------|-------------|
| `isPlaying` | `boolean` | Whether video is actively playing |
| `currentTime` | `number` | Current playback position in seconds |
| `duration` | `number` | Total video duration in seconds |
| `volume` | `number` | Volume level 0-1 |
| `isMuted` | `boolean` | Whether audio is muted |
| `isFullscreen` | `boolean` | Whether fullscreen is active |
| `showControls` | `boolean` | Whether controls overlay is visible |

## 5. Events

No custom events dispatched. All interaction is internal (play/pause/seek/
volume/fullscreen).

## 6. Accessibility

- Root carries `role="button"`, `tabindex="0"`, `aria-label` from prop, `aria-pressed` reflecting play state
- Wrapper keydown handles Space and Enter to toggle play/pause (only when wrapper itself is focused)
- Play/pause button: `aria-label` toggles between `"Pause"` and `"Play"`
- Big play button: `aria-label="Play video"`
- Mute button: `aria-label` toggles between `"Unmute"` and `"Mute"`
- Seek slider: `aria-label="Seek"`
- Volume slider: `aria-label="Volume"`
- Fullscreen button: `aria-label` toggles between `"Exit fullscreen"` and `"Fullscreen"`
- All SVG icons are `aria-hidden="true"`
- Control bar click events use `stopPropagation` to avoid triggering play/pause on root

## 7. Layout

### Sizing

- Root: full width, `aspect-ratio` from prop (inline style), `border-radius: var(--poodle-radius-surface)`, overflow hidden, black background
- Video: `width: 100%`, `height: 100%`, `object-fit: contain`
- Big play button: `4rem x 4rem`, centered absolutely, scale 1.1 on hover
- Controls: absolute bottom, gradient overlay, padding `1.5rem 0.5rem 0.375rem`
- Progress bar: `0.25rem` tall, `0.375rem` margin-bottom
- Control buttons: `1.75rem x 1.75rem`, icon `0.875rem x 0.875rem`
- Volume slider: `3.5rem` width
- Time display: `0.6875rem` font-size

### Composition

Standalone component. Does not compose other Poodle primitives. Uses
`getUiPresentation` and `resolveSemanticControlSize` for size/density resolution.

## 8. Token Usage

Note: The video player renders against a black background regardless of theme.
Most colors use hardcoded `rgba(255,255,255,...)` values intentionally.

### Data Attributes

| Attribute | Element | Values |
|-----------|---------|--------|
| `data-size` | Root | `"xs"`, `"sm"`, `"md"`, `"lg"`, `"xl"` |
| `data-density` | Root | `"compact"`, `"default"`, `"comfortable"` |

### `.video-player` (Root)

| Property | Value |
|----------|-------|
| `position` | `relative` |
| `overflow` | `hidden` |
| `border-radius` | `var(--poodle-radius-surface)` |
| `background` | `#000` |
| `cursor` | `pointer` |
| `aspect-ratio` | set via inline style from `aspectRatio` prop |

### `.video-player video`

| Property | Value |
|----------|-------|
| `display` | `block` |
| `width` | `100%` |
| `height` | `100%` |
| `object-fit` | `contain` |

### `.video-player__big-play`

| Property | Value |
|----------|-------|
| `position` | `absolute` |
| `top` | `50%` |
| `left` | `50%` |
| `transform` | `translate(-50%, -50%)` |
| `width` | `4rem` |
| `height` | `4rem` |
| `padding` | `0` |
| `border` | `0` |
| `background` | `transparent` |
| `color` | `rgba(255, 255, 255, 0.9)` |
| `cursor` | `pointer` |
| `transition` | `transform 0.2s ease` |

### `.video-player__big-play:hover`

| Property | Value |
|----------|-------|
| `transform` | `translate(-50%, -50%) scale(1.1)` |

### `.video-player__controls`

| Property | Value |
|----------|-------|
| `position` | `absolute` |
| `bottom` | `0` |
| `left` | `0` |
| `right` | `0` |
| `background` | `linear-gradient(transparent, rgba(0, 0, 0, 0.7))` |
| `padding` | `1.5rem 0.5rem 0.375rem` |
| `opacity` | `0` (default), `1` (`.visible`) |
| `transition` | `opacity 0.3s ease` |
| `cursor` | `default` |

### `.video-player__progress-bar`

| Property | Value |
|----------|-------|
| `position` | `relative` |
| `height` | `0.25rem` |
| `margin-bottom` | `0.375rem` |
| `background` | `rgba(255, 255, 255, 0.2)` |
| `border-radius` | `999rem` |
| `overflow` | `hidden` |

### `.video-player__progress-fill`

| Property | Value |
|----------|-------|
| `height` | `100%` |
| `background` | `var(--poodle-color-accent-base, #6366f1)` |
| `border-radius` | `999rem` |
| `transition` | `width 0.1s linear` |

### `.video-player__seek`

| Property | Value |
|----------|-------|
| `position` | `absolute` |
| `top` | `-0.375rem` |
| `left` | `0` |
| `width` | `100%` |
| `height` | `1rem` |
| `opacity` | `0` |
| `cursor` | `pointer` |
| `margin` | `0` |

### `.video-player__bar`

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `align-items` | `center` |
| `justify-content` | `space-between` |

### `.video-player__bar-left`, `.video-player__bar-right`

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `align-items` | `center` |
| `gap` | `0.375rem` |

### `.video-player__btn`

| Property | Value |
|----------|-------|
| `display` | `inline-flex` |
| `align-items` | `center` |
| `justify-content` | `center` |
| `width` | `1.75rem` |
| `height` | `1.75rem` |
| `padding` | `0` |
| `border` | `0` |
| `border-radius` | `var(--poodle-radius-control)` |
| `background` | `transparent` |
| `color` | `rgba(255, 255, 255, 0.9)` |
| `cursor` | `pointer` |
| `transition` | `background 0.15s ease` |

### `.video-player__btn:hover`

| Property | Value |
|----------|-------|
| `background` | `rgba(255, 255, 255, 0.15)` |

### `.video-player__btn svg`

| Property | Value |
|----------|-------|
| `width` | `0.875rem` |
| `height` | `0.875rem` |

### `.video-player__volume`

| Property | Value |
|----------|-------|
| `-webkit-appearance` | `none` |
| `appearance` | `none` |
| `width` | `3.5rem` |
| `height` | `1rem` |
| `background` | `transparent` |
| `cursor` | `pointer` |
| `flex-shrink` | `0` |

### `.video-player__volume::-webkit-slider-runnable-track`

| Property | Value |
|----------|-------|
| `height` | `0.25rem` |
| `border-radius` | `0.125rem` |
| `background` | `rgba(255, 255, 255, 0.5)` |

### `.video-player__volume::-moz-range-track`

| Property | Value |
|----------|-------|
| `height` | `0.25rem` |
| `border-radius` | `0.125rem` |
| `background` | `rgba(255, 255, 255, 0.5)` |

### `.video-player__volume::-webkit-slider-thumb`

| Property | Value |
|----------|-------|
| `-webkit-appearance` | `none` |
| `width` | `0.625rem` |
| `height` | `0.625rem` |
| `border-radius` | `50%` |
| `border` | `none` |
| `background` | `white` |
| `margin-top` | `-0.1875rem` |

### `.video-player__volume::-moz-range-thumb`

| Property | Value |
|----------|-------|
| `width` | `0.625rem` |
| `height` | `0.625rem` |
| `border-radius` | `50%` |
| `border` | `none` |
| `background` | `white` |

### `.video-player__time`

| Property | Value |
|----------|-------|
| `font-family` | `var(--poodle-typography-code-family)` |
| `font-size` | `0.6875rem` |
| `color` | `rgba(255, 255, 255, 0.8)` |
| `white-space` | `nowrap` |

### Size Variants

#### `[data-size="xs"]`

| Part | Property | Value |
|------|----------|-------|
| Button | `width` / `height` | `1.25rem` |
| Button SVG | `width` / `height` | `0.75rem` |
| Volume | `width` | `2.5rem` |
| Time | `font-size` | `0.5625rem` |
| BigPlay | `width` / `height` | `3rem` |

#### `[data-size="sm"]`

| Part | Property | Value |
|------|----------|-------|
| Button | `width` / `height` | `1.5rem` |
| Volume | `width` | `3rem` |
| Time | `font-size` | `0.625rem` |
| BigPlay | `width` / `height` | `3.5rem` |

#### `[data-size="lg"]`

| Part | Property | Value |
|------|----------|-------|
| Button | `width` / `height` | `2.125rem` |
| Button SVG | `width` / `height` | `1rem` |
| Volume | `width` | `4rem` |
| Time | `font-size` | `0.75rem` |
| BigPlay | `width` / `height` | `4.5rem` |

#### `[data-size="xl"]`

| Part | Property | Value |
|------|----------|-------|
| Button | `width` / `height` | `2.25rem` |
| Button SVG | `width` / `height` | `1.125rem` |
| Volume | `width` | `4.5rem` |
| Time | `font-size` | `0.8125rem` |
| BigPlay | `width` / `height` | `5rem` |

### Density Variants

#### `[data-density="compact"]`

| Part | Property | Value |
|------|----------|-------|
| Controls | `padding` | `0.25rem 0.375rem` |
| Controls | `gap` | `0.25rem` |
| Bar Left/Right | `gap` | `0.25rem` |

#### `[data-density="comfortable"]`

| Part | Property | Value |
|------|----------|-------|
| Controls | `padding` | `0.5rem 0.75rem` |
| Controls | `gap` | `0.5rem` |
| Bar Left/Right | `gap` | `0.5rem` |

### Light Theme Overrides

None (video player uses hardcoded dark-on-black colors).

## 9. Svelte Notes

- `data-size` resolves via `resolveSemanticControlSize` from presentation context
- `data-density` resolves via presentation context with explicit override
- Uses `onMount` for video event listeners (`play`, `pause`, `ended`, `loadedmetadata`)
- `requestAnimationFrame` loop updates `currentTime` during playback for smooth seek bar
- Controls auto-hide via `setTimeout` (3s) reset on `mousemove`
- Fullscreen uses native `requestFullscreen` / `exitFullscreen` API
- `onDestroy` cleans up animation frame and timeout
- Click on root toggles play; control bar uses `stopPropagation` to prevent double-toggle
- Big play button uses `on:click|stopPropagation` to prevent root click handler
- Wrapper keydown handler only activates on Space/Enter when `event.target === wrapperEl`
- `document.fullscreenchange` listener updates `isFullscreen` state
- SVG icons are inline (not Icon primitive) -- play, pause, mute, unmute, fullscreen, exit-fullscreen

## 10. GPUI Notes

- Not yet implemented
- Video playback requires platform-specific media integration
- Fullscreen API differs per platform
- Controls overlay with auto-hide may need platform animation system

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] all props have the same meaning and defaults
- [ ] play/pause toggle behavior matches
- [ ] big play button visibility logic matches (paused at currentTime=0)
- [ ] auto-hide controls after 3s during playback matches
- [ ] mute/unmute toggle behavior matches
- [ ] seek and volume slider behavior matches
- [ ] fullscreen toggle behavior matches
- [ ] keyboard Space/Enter toggle matches

### Tier 2: Visual Parity

- [ ] big play button size and hover scale matches
- [ ] controls gradient overlay matches
- [ ] progress bar and fill styling matches
- [ ] control button size and hover background matches
- [ ] time display monospace formatting matches
- [ ] size variant scaling matches all sizes
- [ ] density variant padding and gap matches

### Tier 3: Implementation Freedom

- [ ] animation frame vs polling approach is platform-owned
- [ ] fullscreen API is platform-owned
- [ ] SVG icon rendering internals stay internal

## 12. Specimen Definitions

### Video Player

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Video player | `src` set to sample video, `ariaLabel="Sample video"` | Player with controls; click big play to start |

### Custom Aspect Ratio (4:3)

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Custom aspect ratio | `src` set to sample video, `aspectRatio={4/3}`, `ariaLabel="4:3 aspect video"` | Player with 4:3 aspect ratio container |
