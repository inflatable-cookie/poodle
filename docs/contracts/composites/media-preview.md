# MediaPreview

Status: contract
Updated: 2026-03-25

## 1. Purpose

- Component name: `MediaPreview`
- Layer: `composites`
- Summary: a rich asset-preview surface that composes `Card` with
  `MediaThumbnail` to combine framed media, textual identity, metadata chips,
  and body content
- In scope: title, eyebrow, description, media frame, metadata chips, caption,
  badge, thumbnail meta, state posture, aspect ratio, card variant
- Out of scope: transport controls, editing tools, waveform/video rendering
  engines, or app-specific asset workflows

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

### CardVariant

```ts
type CardVariant = "default" | "elevated" | "outlined";
```

## 3. Anatomy

```text
[Card]  ariaLabel, hasMedia=true
  ├── slot:media
  │     └── [MediaThumbnail]
  │           ├── frame with aspect ratio
  │           │     ├── slot:media content  (ready)
  │           │     └── state display       (loading/error/empty)
  │           └── badge overlay  (optional)
  ├── slot:header
  │     └── [Header]
  │           ├── [Heading]
  │           │     ├── <p> eyebrow       (optional)
  │           │     ├── <h3> title
  │           │     └── <p> description   (optional)
  │           └── [Meta List]  (optional)
  │                 ├── <li> thumbnailMeta (optional)
  │                 └── <li> meta[...]
  └── [Body]
        ├── <p> caption  (optional)
        └── slot:default
```

| Part | Required | Description |
|------|----------|-------------|
| Card | yes | outer card container with variant and media flag |
| MediaThumbnail | yes | framed preview area in card media slot |
| Header | yes | title block with optional eyebrow, description, metadata |
| Eyebrow | no | uppercase secondary label above title |
| Title | yes | primary heading |
| Description | no | secondary text below title |
| Meta List | no | pill-styled metadata items |
| Body | no | caption and default slot content below header |

## 4. Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `title` | `string` | none | yes | primary heading text |
| `description` | `string \| null` | `null` | no | secondary text below title |
| `eyebrow` | `string \| null` | `null` | no | uppercase label above title |
| `caption` | `string \| null` | `null` | no | text paragraph below header |
| `meta` | `string[]` | `[]` | no | metadata items rendered as pills |
| `badge` | `string \| null` | `null` | no | overlay badge on thumbnail |
| `thumbnailMeta` | `string \| null` | `null` | no | extra metadata item prepended to meta list |
| `kind` | `MediaKind` | `"image"` | no | media type passed to MediaThumbnail |
| `state` | `MediaState` | `"ready"` | no | state posture passed through to `MediaThumbnail` |
| `aspectRatio` | `AspectRatio` | `"landscape"` | no | aspect ratio passed through to `MediaThumbnail` |
| `variant` | `CardVariant` | `"default"` | no | card visual variant |
| `ariaLabel` | `string \| null` | `null` | no | accessible label; falls back to `title` |
| `stateTitle` | `string \| null` | `null` | no | heading for non-ready state in thumbnail |
| `stateMessage` | `string \| null` | `null` | no | body text for non-ready state in thumbnail |

## 5. Slots

| Slot | Purpose | Fallback |
|------|---------|----------|
| `media` | custom media content inside MediaThumbnail frame | MediaThumbnail placeholder |
| default | additional body content below caption | none |

## 6. Events

No component-owned events.

## 7. States

State posture is delegated to the `MediaThumbnail` component in both runtimes:

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| ready | `state="ready"` | media slot or placeholder visible in frame |
| loading | `state="loading"` | `MediaThumbnail` loading posture rendered in frame |
| error | `state="error"` | `MediaThumbnail` error posture rendered in frame |
| empty | `state="empty"` | `MediaThumbnail` empty posture rendered in frame |

Card header and body always render regardless of media state.

## 8. Accessibility

- Card receives `ariaLabel` (falls back to `title`)
- MediaThumbnail receives `ariaLabel={title}` for frame description
- Meta list has `aria-label="preview metadata"`
- Title and context remain textual and visible outside the preview frame
- State messaging remains readable when the renderer fails

## 9. Visual Rules And Precise CSS

### Header, Heading, Body

| Property | Value |
|----------|-------|
| display | `grid` |
| gap | `var(--poodle-space-stack-sm)` |

### Header (override)

| Property | Value |
|----------|-------|
| gap | `var(--poodle-space-stack-md)` |

### Eyebrow

| Property | Value |
|----------|-------|
| margin | `0` |
| color | `var(--poodle-color-text-secondary)` |
| font-size | `0.6875rem` |
| font-weight | `600` |
| letter-spacing | `0.12em` |
| text-transform | `uppercase` |

### Title (h3)

| Property | Value |
|----------|-------|
| margin | `0` |
| font-size | `1.25rem` |
| line-height | `1.2` |

### Description, Caption, Meta

| Property | Value |
|----------|-------|
| margin | `0` |
| color | `var(--poodle-color-text-secondary)` |
| font-size | `0.8125rem` |
| line-height | `1.5` |

### Meta List

| Property | Value |
|----------|-------|
| display | `flex` |
| flex-wrap | `wrap` |
| gap | `var(--poodle-space-inline-sm)` |
| margin | `0` |
| padding | `0` |
| list-style | `none` |

### Meta List Item

| Property | Value |
|----------|-------|
| padding | `0.375rem 0.625rem` |
| border-radius | `var(--poodle-radius-control)` |
| background | `color-mix(in srgb, var(--poodle-color-background-surface) 70%, transparent)` |

### Light Theme Overrides

| Selector | Property | Value |
|----------|----------|-------|
| `:global([data-theme="light"]) .media-preview__meta li` | box-shadow | `inset 0 0 0 0.0625rem color-mix(in srgb, var(--poodle-color-border-subtle) 32%, transparent)` |

## 10. Composition

- Composes: `Card` (from primitives), `MediaThumbnail`
- MediaThumbnail is configured with `title={null}` and `meta={null}` since
  the preview handles its own header section
- `badge`, `aspectRatio`, `state`, `stateTitle`, and `stateMessage` are
  pass-through inputs to the nested `MediaThumbnail`
- `thumbnailMeta` is prepended to the preview metadata list rather than
  rendered inside the frame caption

## 11. Specimen Definitions

### Image Preview

| Label | Props/Config | Expected Visual |
|-------|-------------|-----------------|
| Image preview | `title="Hero banner"`, `description="Main landing page banner image for the product launch."`, `eyebrow="Image"`, `meta=["1920 x 1080", "245 KB", "PNG"]`, `kind="image"`, `aspectRatio="landscape"`, media slot with placeholder | framed landscape media area with title, description, eyebrow label, and metadata chips below |

### Video Preview

| Label | Props/Config | Expected Visual |
|-------|-------------|-----------------|
| Video preview | `title="Onboarding walkthrough"`, `eyebrow="Video"`, `meta=["3:42", "48 MB"]`, `kind="video"`, `aspectRatio="video"`, media slot with placeholder | framed video-ratio media area with title, eyebrow, and duration/size metadata |

### Error State

| Label | Props/Config | Expected Visual |
|-------|-------------|-----------------|
| Error state | `title="Corrupted file"`, `kind="document"`, `state="error"`, `stateTitle="Preview unavailable"`, `stateMessage="This file cannot be previewed."`, `aspectRatio="landscape"` | framed area with error messaging replacing media content; title and error details visible |
