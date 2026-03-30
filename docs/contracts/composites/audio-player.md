# AudioPlayer

Status: detailed contract
Updated: 2026-03-30

## 1. Purpose

- Component name: `AudioPlayer`
- Layer: `composites`
- Summary: audio playback bar with transport controls, seek, volume, and optional speed control
- In scope: play/pause, seek slider, current/total time display, mute toggle, volume slider, playback speed selector, size and density variants
- Out of scope: playlist management, waveform visualization, streaming protocol selection, download actions

## 2. Anatomy

```text
[Root .audio-player]  <div> aria-label
  ├── [Audio]  <audio> hidden, preload="metadata"
  ├── [PlayButton .audio-player__play]  <button> toggle play/pause
  │     └── [PlayIcon]  <svg> play or pause icon
  ├── [CurrentTime .audio-player__time]  <span> m:ss format
  ├── [SeekSlider .audio-player__seek]  <input type="range"> 0..duration, step 0.1
  ├── [TotalTime .audio-player__time]  <span> m:ss format
  ├── [MuteButton .audio-player__mute]  <button> toggle mute
  │     └── [MuteIcon]  <svg> speaker or muted icon
  ├── [VolumeSlider .audio-player__volume]  <input type="range"> 0..1, step 0.01
  └── [SpeedSelect .audio-player__speed]  <select> (optional, when showSpeedControl)
        └── [SpeedOption]  <option> (repeated: 0.5x, 0.75x, 1x, 1.25x, 1.5x, 2x)
```

### Parts

| Part | Element | Required | Notes |
|------|---------|----------|-------|
| Root | `<div>` | yes | Flex row container with `aria-label` |
| Audio | `<audio>` | yes | Hidden audio element bound to `src`, `preload="metadata"` |
| PlayButton | `<button>` | yes | Toggles play/pause; icon swaps between play and pause SVG |
| CurrentTime | `<span>` | yes | Monospace `m:ss` format, semantic label size |
| SeekSlider | `<input type="range">` | yes | Tracks `0` to `duration`, step `0.1` |
| TotalTime | `<span>` | yes | Monospace `m:ss` format, semantic label size |
| MuteButton | `<button>` | yes | Toggles mute; icon reflects muted state |
| VolumeSlider | `<input type="range">` | yes | Tracks `0` to `1`, step `0.01` |
| SpeedSelect | `<select>` | no | Shown when `showSpeedControl` is true; options: 0.5x, 0.75x, 1x, 1.25x, 1.5x, 2x |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `src` | `string` | (required) | yes | Audio source URL |
| `ariaLabel` | `string` | `"Audio player"` | no | Accessible label for the root container |
| `showSpeedControl` | `boolean` | `false` | no | Show playback speed selector |
| `size` | `ControlSize \| null` | `null` | no | Explicit semantic size override for transport controls |
| `sizeRole` | `SemanticControlSizeRole` | `"control"` | no | Semantic role used to resolve inherited size scale |
| `density` | `ControlDensity \| null` | `null` | no | Explicit density override for shell spacing |

### Slots

None.

### Controlled And Uncontrolled

- Playback state (playing, currentTime, volume, playbackRate) is fully internal
- The `src` prop is controlled externally

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| idle | initial or after load | Play button shows play icon, time reads `0:00` |
| playing | play button clicked | Play button shows pause icon, seek slider progresses, time updates via `requestAnimationFrame` |
| paused | pause button clicked | Play button shows play icon, seek slider stops |
| ended | audio reaches end | Play button reverts to play icon |
| muted | mute button clicked | Mute icon shows crossed-out speaker, volume slider shows 0 |
| speed-active | speed control present and changed | Selected speed reflected in audio element |

### Component States

| State | Description |
|-------|-------------|
| `isPlaying` | Whether audio is currently playing |
| `currentTime` | Current playback position in seconds |
| `duration` | Total duration in seconds |
| `volume` | Volume level 0..1 |
| `isMuted` | Whether audio is muted |
| `playbackRate` | Current playback speed |

## 5. Events

No component-owned events are dispatched. All interaction is internal (transport controls directly manipulate the `<audio>` element).

## 6. Accessibility

### Semantics

- Root element carries `aria-label` from prop (default `"Audio player"`)
- Play button: `aria-label` toggles between `"Pause"` and `"Play"`
- Mute button: `aria-label` toggles between `"Unmute"` and `"Mute"`
- Seek slider: `aria-label="Seek"`
- Volume slider: `aria-label="Volume"`
- Speed select: `aria-label="Playback speed"`
- All SVG icons: `aria-hidden="true"`

### Keyboard

| Key | Behavior |
|-----|----------|
| `Tab` | Moves between play button, seek slider, mute button, volume slider, and speed select |
| `Space` / `Enter` | Activates focused button |
| `Arrow Left/Right` | Adjusts focused range slider value |

### Focus

- Play and mute buttons: `border-width-focus` solid `accent-focusRing`, offset `0.0625rem`
- Range sliders: browser-native focus styling
- Speed select: browser-native focus styling

## 7. Layout

### Sizing

- Root: flex row, centered vertically, fills parent width
- Seek slider: `flex: 1`, `min-width: 4rem`
- Volume slider: fixed width via custom property
- Time labels: fixed min-width via custom property, centered text
- Buttons: square, size driven by custom property

### Composition

- Parent expectations: media detail views, content editors, inline audio embeds
- Child expectations: none (self-contained)
- Resizing rules: seek slider stretches to fill available width; all other parts are fixed or flex-shrink: 0

## 8. Token Usage -- Exact Values

### Recipe Custom Properties

| Property | Default |
|----------|---------|
| `--poodle-audio-player-gap` | `0.5rem` |
| `--poodle-audio-player-pad-y` | `0.5rem` |
| `--poodle-audio-player-pad-x` | `0.75rem` |
| `--poodle-audio-player-button-size` | `2rem` |
| `--poodle-audio-player-icon-size` | `1rem` |
| `--poodle-audio-player-time-width` | `2.5rem` |
| `--poodle-audio-player-volume-width` | `4rem` |
| `--poodle-audio-player-speed-x` | `0.25rem` |
| `--poodle-audio-player-speed-y` | `0.125rem` |

#### `.audio-player` (Root)

| Property | Value |
|----------|-------|
| `display` | `flex` |
| `align-items` | `center` |
| `gap` | `var(--poodle-audio-player-gap)` |
| `padding` | `var(--poodle-audio-player-pad-y) var(--poodle-audio-player-pad-x)` |
| `border` | `0.0625rem solid var(--poodle-color-border-default)` |
| `border-radius` | `var(--poodle-radius-surface)` |
| `background` | `var(--poodle-color-background-surface)` |

#### `.audio-player__play`, `.audio-player__mute`

| Property | Value |
|----------|-------|
| `display` | `inline-flex` |
| `align-items` | `center` |
| `justify-content` | `center` |
| `width` | `var(--poodle-audio-player-button-size)` |
| `height` | `var(--poodle-audio-player-button-size)` |
| `padding` | `0` |
| `border` | `0` |
| `border-radius` | `999rem` |
| `background` | `transparent` |
| `color` | `var(--poodle-color-text-primary)` |
| `cursor` | `pointer` |
| `flex-shrink` | `0` |
| `transition` | `background var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard)` |

#### `.audio-player__play:hover`, `.audio-player__mute:hover`

| Property | Value |
|----------|-------|
| `background` | `color-mix(in srgb, var(--poodle-color-accent-base) 12%, transparent)` |

#### `.audio-player__play:focus-visible`, `.audio-player__mute:focus-visible`

| Property | Value |
|----------|-------|
| `outline` | `var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing)` |
| `outline-offset` | `0.0625rem` |

#### `.audio-player__play svg`, `.audio-player__mute svg`

| Property | Value |
|----------|-------|
| `width` | `var(--poodle-audio-player-icon-size)` |
| `height` | `var(--poodle-audio-player-icon-size)` |

#### `.audio-player__time`

| Property | Value |
|----------|-------|
| `font-family` | `var(--poodle-typography-code-family)` |
| `font-size` | `var(--poodle-typography-label-size)` |
| `color` | `var(--poodle-color-text-secondary)` |
| `flex-shrink` | `0` |
| `min-width` | `var(--poodle-audio-player-time-width)` |
| `text-align` | `center` |

#### `.audio-player__seek`, `.audio-player__volume`

| Property | Value |
|----------|-------|
| `-webkit-appearance` | `none` |
| `appearance` | `none` |
| `background` | `transparent` |
| `cursor` | `pointer` |
| `height` | `1rem` |

#### `.audio-player__seek`

| Property | Value |
|----------|-------|
| `flex` | `1` |
| `min-width` | `4rem` |

#### `.audio-player__volume`

| Property | Value |
|----------|-------|
| `width` | `var(--poodle-audio-player-volume-width)` |
| `flex-shrink` | `0` |

#### Seek track (`::-webkit-slider-runnable-track` / `::-moz-range-track`)

| Property | Value |
|----------|-------|
| `height` | `0.25rem` |
| `border-radius` | `0.125rem` |
| `background` | `var(--poodle-color-text-primary)` |

#### Volume track (`::-webkit-slider-runnable-track` / `::-moz-range-track`)

| Property | Value |
|----------|-------|
| `height` | `0.25rem` |
| `border-radius` | `0.125rem` |
| `background` | `var(--poodle-color-accent-base)` |

#### Slider thumb (`::-webkit-slider-thumb` / `::-moz-range-thumb`)

| Property | Value |
|----------|-------|
| `width` | `0.625rem` |
| `height` | `0.625rem` |
| `border-radius` | `50%` |
| `border` | `none` |
| `background` | `var(--poodle-color-text-primary)` |

#### `.audio-player__speed`

| Property | Value |
|----------|-------|
| `min-height` | `var(--poodle-size-control-height)` |
| `padding` | `var(--poodle-audio-player-speed-y) var(--poodle-audio-player-speed-x)` |
| `border` | `0.0625rem solid var(--poodle-color-border-default)` |
| `border-radius` | `var(--poodle-radius-control)` |
| `background` | `transparent` |
| `color` | `var(--poodle-color-text-secondary)` |
| `font` | `inherit` |
| `font-size` | `var(--poodle-typography-label-size)` |
| `font-family` | `var(--poodle-typography-code-family)` |
| `cursor` | `pointer` |
| `appearance` | `none` |
| `flex-shrink` | `0` |

### Size Adjustments

| Size | `button-size` | `icon-size` | `time-width` | `volume-width` | `speed-x` |
|------|--------------|-------------|-------------|----------------|-----------|
| `xs` | `1.5rem` | `0.875rem` | `2rem` | `3rem` | (default) |
| `sm` | `1.75rem` | (default) | (default) | (default) | (default) |
| `md` | (default `2rem`) | (default `1rem`) | (default `2.5rem`) | (default `4rem`) | (default `0.25rem`) |
| `lg` | `2.25rem` | `1.125rem` | `2.75rem` | `4.5rem` | `0.375rem` |
| `xl` | `2.5rem` | `1.25rem` | `3rem` | `5rem` | `0.5rem` |

### Density Adjustments

| Density | `gap` | `pad-y` | `pad-x` | `speed-y` |
|---------|-------|---------|---------|-----------|
| `compact` | `0.375rem` | `0.375rem` | `0.5rem` | `0.0625rem` |
| `default` | `0.5rem` | `0.5rem` | `0.75rem` | `0.125rem` |
| `comfortable` | `0.625rem` | `0.625rem` | `0.875rem` | `0.1875rem` |

### Data Attributes Used for CSS Selectors

| Attribute | Element | Purpose |
|-----------|---------|---------|
| `data-size` | `.audio-player` root | drives size variant custom properties |
| `data-density` | `.audio-player` root | drives density variant custom properties |

## 9. Svelte Notes

- Uses `onMount` to attach event listeners (`play`, `pause`, `ended`, `loadedmetadata`, `durationchange`) to the `<audio>` element
- `requestAnimationFrame` loop for smooth time updates during playback; cancelled `onDestroy`
- Resolves size via `resolveSemanticControlSize` from `getUiPresentation()`
- Resolves density via `getUiPresentation().density`
- No event dispatcher -- all state is internal

## 10. GPUI Notes

- Expected crate/module surface: `poodle_gpui::composites::audio_player`
- Audio playback requires platform-specific audio API integration
- Slider controls will need custom range input implementations

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] all props have the same meaning and defaults
- [ ] play/pause, mute/unmute toggle behavior matches
- [ ] seek and volume slider ranges and steps match
- [ ] speed options match (0.5, 0.75, 1, 1.25, 1.5, 2)
- [ ] time formatting matches (`m:ss`)

### Tier 2: Visual Parity

- [ ] button size and icon scaling per size variant
- [ ] track and thumb styling matches
- [ ] density spacing matches
- [ ] focus ring styling matches

### Tier 3: Implementation Freedom

- [ ] audio API and animation frame timing stay internal

## 12. Specimen Definitions

### Basic Audio Player

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Basic | `src` only | Play button, seek track, time labels, mute toggle, volume slider |

### With Speed Control

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Speed control | `showSpeedControl=true` | Same as basic plus speed selector at trailing edge |
