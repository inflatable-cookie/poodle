# MediaPreview

Status: contract
Updated: 2026-07-10

## 1. Purpose

- Component name: `MediaPreview`
- Layer: `composites`
- Summary: a rich asset-preview surface that composes `Card` with
  `MediaThumbnail` to combine framed media, textual identity, metadata chips,
  and body content
- In scope: title, eyebrow, description, media frame, metadata chips, caption,
  badge, thumbnail meta, state posture, aspect ratio, card variant, size,
  density
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
type AspectRatio = "auto" | "square" | "landscape" | "portrait" | "video";
```

### CardVariant

```ts
type CardVariant = "default" | "elevated" | "outlined";
```

## 3. Anatomy

```text
[Card]  ariaLabel, hasMedia=true
  ├── mediaContent()
  │     └── [MediaThumbnail]
  │           ├── frame with aspect ratio
  │           │     ├── rendered media content  (ready)
  │           │     └── state display       (loading/error/empty)
  │           └── badge overlay  (optional)
  ├── header()
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
        └── children()
```

| Part | Required | Description |
|------|----------|-------------|
| Card | yes | outer card container with variant and media flag |
| MediaThumbnail | yes | framed preview area in the card media region |
| Header | yes | title block with optional eyebrow, description, metadata |
| Eyebrow | no | uppercase secondary label above title |
| Title | yes | primary heading |
| Description | no | secondary text below title |
| Meta List | no | pill-styled metadata items |
| Body | no | caption and children snippet content below header |

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
| `size` | `"xs" \| "sm" \| "md" \| "lg" \| "xl" \| null` | `null` | no | preview scale; resolves from presentation context when omitted |
| `sizeRole` | `SemanticControlSizeRole` | `"control"` | no | semantic size role for inherited scale resolution |
| `density` | `"compact" \| "default" \| "comfortable" \| null` | `null` | no | spacing density; resolves from presentation context when omitted |

### Behavior Machine

Behavior classification: styled-only (no machine)

Rendering and composition only, or interaction fully delegated to composed
Poodle primitives / native elements; no component-owned behavioral state
beyond plain props. Classified in the g11.004 long-tail sweep.

## 5. Snippets

| Snippet | Purpose | Fallback |
|---------|---------|----------|
| `mediaContent()` | custom media content inside the MediaThumbnail frame | MediaThumbnail placeholder |
| `children()` | additional body content below the caption | none |

## 6. Events

No component-owned events.

## 7. States

State posture is delegated to the `MediaThumbnail` component in both runtimes:

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| ready | `state="ready"` | media content or placeholder visible in frame |
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

### Presentation

- Root publishes `data-size` and `data-density`
- Composition is wrapped in `UiPresentationProvider`, so nested `MediaThumbnail`
  inherits the same presentation values
- `Card` receives the resolved density directly

### Header, Heading, Body

| Property | Value |
|----------|-------|
| display | `grid` |
| gap | `var(--poodle-media-preview-section-gap)` |

### Header (override)

| Property | Value |
|----------|-------|
| gap | `var(--poodle-media-preview-header-gap)` |

### Eyebrow

| Property | Value |
|----------|-------|
| margin | `0` |
| color | `var(--poodle-color-text-secondary)` |
| font-size | `var(--poodle-media-preview-eyebrow-size)` |
| font-weight | `600` |
| letter-spacing | `0.12em` |
| text-transform | `uppercase` |

### Title (h3)

| Property | Value |
|----------|-------|
| margin | `0` |
| font-size | `var(--poodle-media-preview-title-size)` |
| line-height | `var(--poodle-media-preview-title-line-height)` |

### Description, Caption, Meta

| Property | Value |
|----------|-------|
| margin | `0` |
| color | `var(--poodle-color-text-secondary)` |
| font-size | `var(--poodle-media-preview-body-size)` |
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
| padding | `var(--poodle-media-preview-meta-padding-y) var(--poodle-media-preview-meta-padding-x)` |
| border-radius | `var(--poodle-radius-control)` |
| background | `color-mix(in srgb, var(--poodle-color-background-surface) 70%, transparent)` |

### Size Variants

| Size | Eyebrow | Title | Body | Meta Padding |
|------|---------|-------|------|--------------|
| `xs` | `0.625rem` | `0.9375rem` | `0.75rem` | `0.25rem 0.5rem` |
| `sm` | `0.65625rem` | `1rem` | `0.78125rem` | `0.3125rem 0.5625rem` |
| `md` | `0.6875rem` | `1.125rem` | `0.8125rem` | `0.375rem 0.625rem` |
| `lg` | `0.71875rem` | `1.1875rem` | `0.875rem` | `0.4375rem 0.6875rem` |
| `xl` | `0.75rem` | `1.25rem` | `0.9375rem` | `0.5rem 0.75rem` |

### Density Variants

| Density | Header Gap | Section Gap |
|---------|------------|-------------|
| `compact` | `0.625rem` | `0.375rem` |
| `default` | `var(--poodle-space-stack-md)` | `var(--poodle-space-stack-sm)` |
| `comfortable` | `1rem` | `0.625rem` |

### Light Theme Overrides

| Selector | Property | Value |
|----------|----------|-------|
| `:global([data-theme="light"]) .media-preview__meta li` | box-shadow | `inset 0 0 0 0.0625rem color-mix(in srgb, var(--poodle-color-border-subtle) 32%, transparent)` |

## 10. Composition

- Composes: `Card` (from primitives), `MediaThumbnail`
- Wraps composition in `UiPresentationProvider` so nested media frame follows
  the same size and density
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
| Image preview | `title="Hero banner"`, `description="Main landing page banner image for the product launch."`, `eyebrow="Image"`, `meta=["1920 x 1080", "245 KB", "PNG"]`, `kind="image"`, `aspectRatio="landscape"`, mediaContent snippet with placeholder | framed landscape media area with title, description, eyebrow label, and metadata chips below |

### Video Preview

| Label | Props/Config | Expected Visual |
|-------|-------------|-----------------|
| Video preview | `title="Onboarding walkthrough"`, `eyebrow="Video"`, `meta=["3:42", "48 MB"]`, `kind="video"`, `aspectRatio="video"`, mediaContent snippet with placeholder | framed video-ratio media area with title, eyebrow, and duration/size metadata |

### Error State

| Label | Props/Config | Expected Visual |
|-------|-------------|-----------------|
| Error state | `title="Corrupted file"`, `kind="document"`, `state="error"`, `stateTitle="Preview unavailable"`, `stateMessage="This file cannot be previewed."`, `aspectRatio="landscape"` | framed area with error messaging replacing media content; title and error details visible |
