# AudioPlayer

Status: seed contract
Updated: 2026-03-19

## 1. Purpose

- Component name: `AudioPlayer`
- Layer: `composites`
- Summary: audio playback bar with transport controls, seek, volume, and optional speed control
- In scope: play/pause, seek slider, current/total time display, mute toggle, volume slider, playback speed selector
- Out of scope: playlist management, waveform visualization, streaming protocol selection, download actions

## 2. Anatomy

```
┌─[▶]─[0:00]─[━━━━━━━━━━━━━━━━━━━━━━━]─[3:24]─[🔊]─[━━━━]─[1×]─┐
│ play  time         seek track          total  mute  volume speed │
└─────────────────────────────────────────────────────────────────────┘
```

### Parts

| Part | Element | Notes |
|------|---------|-------|
| root | `<div>` | Flex row container with `aria-label` |
| play-button | `<button>` | Toggles play/pause, icon swaps between ▶ and ⏸ |
| current-time | `<span>` | Monospace `m:ss` format |
| seek-slider | `<input type="range">` | Tracks `0` to `duration`, step `0.1` |
| total-time | `<span>` | Monospace `m:ss` format |
| mute-button | `<button>` | Toggles mute, icon reflects muted state |
| volume-slider | `<input type="range">` | Tracks `0` to `1`, step `0.01` |
| speed-select | `<select>` | Optional. Options: 0.5×, 0.75×, 1×, 1.25×, 1.5×, 2× |

## 3. Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `src` | `string` | (required) | Audio source URL |
| `ariaLabel` | `string` | `"Audio player"` | Accessible label for the root container |
| `showSpeedControl` | `boolean` | `false` | Show playback speed selector |

## 4. Visual Rules

### Container
- Background: `background-surface`
- Border: `1px solid border-default`, `radius-surface`
- Padding: `0.5rem 0.75rem`
- Layout: flex row, `0.5rem` gap, vertically centered

### Play / Mute Buttons
- Size: `2rem × 2rem`, icon `1rem × 1rem`
- Background: transparent; hover: `accent-base` at 12% opacity
- Color: `text-primary`
- Focus: `border-width-focus` solid `accent-focusRing`, offset `0.0625rem`

### Time Labels
- Font: `typography-code-family`, `0.6875rem`
- Color: `text-secondary`
- Min-width: `2.5rem`, centered

### Seek Slider
- Track: `0.25rem` tall, `0.125rem` radius, color `text-primary`
- Thumb: `0.625rem` diameter solid circle, color `text-primary`, no border

### Volume Slider
- Track: `0.25rem` tall, `0.125rem` radius, color `accent-base`
- Thumb: `0.625rem` diameter solid circle, color `text-primary`, no border

### Speed Select
- Border: `1px solid border-default`, `radius-control`
- Background: transparent
- Font: `typography-code-family`, `0.6875rem`
- Color: `text-secondary`

## 5. Accessibility

- Root element carries `aria-label`
- Play button: `aria-label` toggles between "Pause" and "Play"
- Mute button: `aria-label` toggles between "Unmute" and "Mute"
- Seek slider: `aria-label="Seek"`
- Volume slider: `aria-label="Volume"`
- Speed select: `aria-label="Playback speed"`

## 6. Specimen Definitions

### Basic Audio Player

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Basic | `src` only | Play button, seek track, time labels, mute toggle, volume slider |

### With Speed Control

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Speed control | `showSpeedControl=true` | Same as basic plus speed selector at trailing edge |

## 7. Next Task

Use `AudioPlayer` for inline audio playback where the content is a single audio resource. For playlist or queue scenarios, compose multiple AudioPlayers or build a dedicated playlist composite.
