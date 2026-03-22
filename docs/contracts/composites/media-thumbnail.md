# MediaThumbnail

Status: contract
Updated: 2026-03-22

## 1. Purpose

- Component name: `MediaThumbnail`
- Layer: `composites`
- Summary: a framed preview surface for image, audio, video, document, or
  embed-oriented assets with aspect-ratio control and state posture
- In scope: aspect-ratio framing, preview placeholder with fallback icon,
  loading/error/empty posture, badge overlay, play indicator, optional
  title/meta caption, compact presentation mode
- Out of scope: real playback engines, zooming, annotation tools, file
  fetching, or app-specific asset actions

## 2. Types

### MediaKind

```ts
type MediaKind = "image" | "audio" | "video" | "document" | "embed";
```

### MediaState

```ts
type MediaState = "ready" | "loading" | "error" | "empty";
```

### AspectRatio

```ts
type AspectRatio = "square" | "landscape" | "portrait" | "video";
```

## 3. Anatomy

```text
[Root]  <figure> data-kind, data-state, data-aspect-ratio, data-presentation
  ├── [Frame]
  │     ├── [Content]  (state="ready")
  │     │     ├── slot:default  OR  [Placeholder]  fallback icon
  │     │     └── [Play Indicator]  (audio/video only)
  │     ├── [State Display]  (state != "ready")
  │     │     ├── <strong> stateTitle
  │     │     └── <p> stateMessage  (non-compact only)
  │     └── [Badge]  (optional, all states)
  └── [Caption]  (non-compact, when title or meta present)
        ├── <strong> title
        └── <span> meta
```

| Part | Required | Description |
|------|----------|-------------|
| Root | yes | `<figure>` element with data attributes |
| Frame | yes | aspect-ratio constrained preview area |
| Placeholder | no | fallback icon when no slot content and state is ready |
| Play Indicator | no | play/music icon overlay for audio/video kinds |
| State Display | yes (when not ready) | title and optional message for state |
| Badge | no | positioned overlay label (top-right) |
| Caption | no | title and meta below frame (hidden in compact mode) |

## 4. Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `kind` | `MediaKind` | `"image"` | no | media type; determines fallback icon and play indicator |
| `state` | `MediaState` | `"ready"` | no | content posture |
| `aspectRatio` | `AspectRatio` | `"landscape"` | no | frame aspect ratio |
| `title` | `string \| null` | `null` | no | caption heading below frame |
| `badge` | `string \| null` | `null` | no | overlay badge text |
| `meta` | `string \| null` | `null` | no | caption secondary text below title |
| `ariaLabel` | `string \| null` | `null` | no | accessible label; falls back to `title` |
| `stateTitle` | `string \| null` | `null` | no | heading for non-ready states; auto-defaults per state |
| `stateMessage` | `string \| null` | `null` | no | body text for non-ready states; hidden in compact mode |
| `presentation` | `"default" \| "compact"` | `"default"` | no | compact hides caption and state message |

## 5. Slots

| Slot | Purpose | Fallback |
|------|---------|----------|
| default | media content inside the frame (image, video element, etc.) | placeholder icon based on `kind` |

## 6. Events

No component-owned events.

## 7. States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| ready | `state="ready"` | slot content or placeholder icon visible |
| loading | `state="loading"` | state display with "Loading preview" title |
| error | `state="error"` | state display with "Preview unavailable" title |
| empty | `state="empty"` | state display with "No preview" title |

### State Title Defaults

When `stateTitle` is null, the component uses these defaults:

- loading: "Loading preview"
- error: "Preview unavailable"
- empty/other: "No preview"

## 8. Accessibility

- Element: `<figure>` with `aria-label` (falls back to `title`)
- `aria-busy="true"` when state is loading
- Play indicator: `aria-hidden="true"` (decorative)
- Placeholder icon: `aria-hidden="true"` (decorative)
- Textual identity must remain available when visual preview cannot render

## 9. Visual Rules

### Fallback Icons By Kind

| Kind | Icon |
|------|------|
| image | `image` |
| audio | `music` |
| video | `play` |
| document | `file-text` |
| embed | `external-link` |

### Aspect Ratios

| Value | Ratio |
|-------|-------|
| square | 1:1 |
| landscape | 16:10 |
| portrait | 3:4 |
| video | 16:9 |

### Frame

- Border: `0.0625rem solid --pug-color-border-subtle`
- Border-radius: `calc(--pug-radius-surface - 0.125rem)`
- Background: radial gradient with accent-base at 18% from top-left, over
  panel background at 94%

### Badge

- Position: absolute, top-right (0.625rem inset; 0.5rem in compact)
- Min-height: 1.5rem
- Uppercase, 0.6875rem, weight 600, letter-spacing 0.04em
- Backdrop-filter: blur(1rem)

### Play Indicator

- Position: absolute, bottom-left (0.625rem inset)
- 2rem x 2rem circle with backdrop blur
- Shows for audio (music icon) and video (play icon) kinds

### Caption (non-compact only)

- Title: 0.875rem, weight bold, line-height 1.35
- Meta: secondary color, 0.8125rem, line-height 1.5

### Compact Mode

- Caption hidden entirely
- State message (`<p>`) hidden
- State content vertically centered with smaller padding
- State title: 0.875rem, line-height 1.35

## 10. Composition

- Composes: `Icon` (from primitives)
- Used by: `MediaPreview`

## 11. Specimen Definitions

### Image Thumbnails

| Label | Props/Config | Expected Visual |
|-------|-------------|-----------------|
| Photo 1 | `kind="image"`, `title="Photo 1"`, `badge="New"`, `aspectRatio="square"` | square thumbnail frame with image placeholder and New badge |
| Photo 2 | `kind="image"`, `title="Photo 2"`, `meta="2.4 MB"`, `aspectRatio="square"` | square thumbnail frame with image placeholder and size metadata |
| Clip | `kind="video"`, `title="Clip"`, `badge="HD"`, `meta="1:24"`, `aspectRatio="square"` | square thumbnail frame with video placeholder, HD badge, and duration metadata |

### Compact Presentation

| Label | Props/Config | Expected Visual |
|-------|-------------|-----------------|
| Report.pdf | `kind="document"`, `title="Report.pdf"`, `presentation="compact"`, `aspectRatio="landscape"` | compact landscape thumbnail with document placeholder |
| Interview.mp3 | `kind="audio"`, `title="Interview.mp3"`, `presentation="compact"`, `aspectRatio="landscape"` | compact landscape thumbnail with audio placeholder |

### Loading State

| Label | Props/Config | Expected Visual |
|-------|-------------|-----------------|
| Loading | `kind="image"`, `state="loading"`, `aspectRatio="square"` | square thumbnail frame showing loading indicator |
