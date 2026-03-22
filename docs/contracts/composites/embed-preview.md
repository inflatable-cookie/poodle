# EmbedPreview

Status: seed contract
Updated: 2026-03-22

## 1. Purpose

- Component name: `EmbedPreview`
- Layer: `composites`
- Summary: renders a preview of a parsed embed — displays an iframe for
  known providers (YouTube, Vimeo), renders raw embed code for iframe embeds,
  shows a fallback link for generic URLs, and handles loading, error, and
  empty states
- In scope: iframe rendering with configurable aspect ratio, provider-specific
  embed URLs (YouTube privacy-enhanced, Vimeo player), loading skeleton,
  error display, empty state placeholder, raw embed code rendering, fallback
  link display, sandbox security attributes
- Out of scope: embed parsing (see EmbedInput), player controls, playback
  state management, embed editing

## 2. Anatomy

```text
[Root .embed-preview]  <div>
  ├── [Loading .embed-preview__loading]  (when loading)
  │     ├── [Skeleton]  Skeleton primitive (shape="block")
  │     └── [LoadingText .embed-preview__loading-text]  <span>
  ├── [Error .embed-preview__error]  (when error)
  │     ├── [ErrorIcon]  <svg> (alert circle)
  │     └── [ErrorText]  <span>
  ├── [Empty .embed-preview__empty]  (when !parsed && !loading && !error)
  │     ├── [EmptyIcon]  <svg> (play rectangle)
  │     └── [EmptyText]  <span>
  ├── [Container .embed-preview__container]  (when parsed && embedUrl)
  │     └── [Iframe .embed-preview__iframe]  <iframe>
  ├── [Container .embed-preview__container]  (when parsed && originalEmbed)
  │     └── [RawEmbed]  {@html parsed.originalEmbed}
  └── [Fallback .embed-preview__fallback]  (when parsed && no embedUrl && no originalEmbed)
        └── [FallbackLink]  <a> to originalUrl
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | container with border-radius and overflow hidden | radius-surface |
| Loading | conditional | centered column with skeleton and loading text | background-panel, radius-surface, min-height, padding |
| Skeleton | conditional | Skeleton primitive (block shape) | delegates to Skeleton contract |
| Error | conditional | centered column with error icon and message | background-panel, text-danger (icon), text-secondary (message) |
| Empty | conditional | centered column with play icon and empty message | background-panel, text-tertiary (icon), text-secondary (message) |
| Container | conditional | aspect-ratio wrapper for iframe or raw embed | background-panel, aspect-ratio |
| Iframe | conditional | sandboxed iframe loading the embed URL | full-size absolute positioning |
| Fallback | conditional | link to the original URL | background-panel, accent color for link |

## 3. Props And Inputs

### Public Props

| Prop | Type | Default | Required | Notes |
|------|------|---------|----------|-------|
| `parsed` | `ParsedEmbed \| null` | `null` | no | parsed embed data from EmbedInput |
| `aspectRatio` | `number \| "auto"` | `16 / 9` | no | aspect ratio for the embed container; `"auto"` disables fixed ratio |
| `loading` | `boolean` | `false` | no | shows loading skeleton state |
| `error` | `string \| null` | `null` | no | error message to display |
| `emptyMessage` | `string` | `"No embed to preview"` | no | message shown when no embed is parsed |

### Types

```ts
type ParsedEmbed = {
  provider: string;
  id: string;
  originalUrl?: string;
  originalEmbed?: string;
  width?: number;
  height?: number;
};
```

### Slots

None.

### Controlled And Uncontrolled

- Fully controlled display component; all state is driven by props.

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| loading | `loading` is true | centered skeleton with "Loading preview..." text |
| error | `error` is set (takes priority over parsed) | alert icon with error message |
| empty | `parsed` is null, not loading, no error | play icon with empty message text |
| iframe preview | `parsed` is set and embedUrl is derived | iframe in aspect-ratio container |
| raw embed | `parsed` is set with `originalEmbed` but no embedUrl | raw HTML rendered in container |
| fallback | `parsed` is set but no embedUrl and no originalEmbed | link to original URL |

### Component States

- `embedUrl` (derived): provider-specific embed URL computed from `parsed`
- `isAudio` (derived): true when `parsed.provider === "audioboom"`
- `effectiveAspectRatio` (derived): `"auto"` for audio, otherwise the
  `aspectRatio` prop

### Render Priority

States are evaluated in this order: loading > error > empty > iframe >
raw embed > fallback.

## 5. Events

None. EmbedPreview is a pure display component.

## 6. Accessibility

### Semantics

- Iframe: `title="{provider} embed"` for screen reader identification
- Iframe: `loading="lazy"` for performance
- Iframe: `sandbox="allow-scripts allow-same-origin allow-popups"` for security
- Error and empty icons: decorative SVGs (no aria attributes needed as
  accompanying text provides meaning)
- Fallback link: `target="_blank"` with `rel="noopener noreferrer"`

### Keyboard

| Key | Behavior |
|-----|----------|
| `Tab` | focuses the iframe or fallback link (native behavior) |

### Focus And Announcement

- No custom focus management
- Iframe receives focus via native tab behavior

## 7. Layout

### Sizing

- Root: `border-radius` from token, `overflow: hidden`
- Container: `position: relative`, full width, aspect-ratio set via inline style
- Iframe: absolute positioned, 100% width and height, no border
- When aspect-ratio is `"auto"`: iframe is static with `height: 10rem`
- Loading/Error/Empty: centered flex column, min-height `8rem`, padding `1.5rem`,
  gap `0.5rem`
- Error/Empty icons: `2rem` square
- Text: `0.8125rem` font-size
- Fallback: padding `0.75rem 1rem`

### Composition

- Parent expectations: media embed forms, content editors, paired with EmbedInput
- Child expectations: Skeleton primitive (loading state)
- Resizing rules: fills parent width; height determined by aspect ratio or content

## 8. Token Usage

| Part | Token | Purpose |
|------|-------|---------|
| Root | `--pug-radius-surface` | container border-radius (fallback 0.5rem) |
| Container | `--pug-color-background-panel` | container background (fallback #1a1a1a) |
| Loading/Error/Empty | `--pug-color-background-panel` | state container background |
| Loading/Error/Empty | `--pug-radius-surface` | state container border-radius |
| LoadingText | `--pug-color-text-secondary` | loading text color (fallback #999) |
| ErrorIcon | `--pug-color-text-danger` | error icon color (fallback #ef4444) |
| ErrorText | `--pug-color-text-secondary` | error message color (fallback #999) |
| EmptyIcon | `--pug-color-text-tertiary` | empty icon color (fallback #666) |
| EmptyText | `--pug-color-text-secondary` | empty message color (fallback #999) |
| Fallback | `--pug-color-background-panel` | fallback container background |
| FallbackLink | `--pug-color-accent-default` | link color (fallback #6366f1) |

## 9. Svelte Notes

- Uses `Skeleton` from `@pug/svelte-primitives` for the loading state
- Provider-specific embed URL generation:
  - YouTube: `https://www.youtube-nocookie.com/embed/{id}` (privacy-enhanced)
  - Vimeo: `https://player.vimeo.com/video/{id}`
  - Default: `parsed.originalUrl`
- Raw embed code rendered via `{@html parsed.originalEmbed}` — consumers must
  ensure embed code is trusted
- Aspect ratio applied via inline `style` attribute

## 10. GPUI Notes

- Expected crate/module surface: `pug_gpui::composites::embed_preview`
- Iframe rendering is web-specific; GPUI may need a WebView or placeholder approach
- Loading and error states can be implemented with GPUI primitives

## 11. Parity Checklist

### Tier 1: Strict Parity

- [ ] all props have the same meaning and defaults
- [ ] ParsedEmbed type is identical
- [ ] state priority order matches (loading > error > empty > iframe > raw > fallback)
- [ ] embed URL generation matches per provider

### Tier 2: Visual Parity

- [ ] aspect-ratio behavior matches
- [ ] loading, error, and empty state layouts match
- [ ] icon and text styling matches

### Tier 3: Implementation Freedom

- [ ] iframe rendering may differ per platform
- [ ] rendering internals stay internal

## 12. Known Deltas

| Delta | Why Allowed | Approval Status | Follow-Up |
|-------|-------------|-----------------|-----------|
| GPUI iframe support | GPUI may use WebView or placeholder instead of iframe | pending | investigate WebView integration |

## 13. Specimen Definitions

### YouTube Embed

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| YouTube embed | `parsed` with `provider="youtube"`, `id="dQw4w9WgXcQ"` | 16:9 iframe showing YouTube embed |

### Vimeo Embed

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Vimeo embed | `parsed` with `provider="vimeo"`, `id="76979871"` | 16:9 iframe showing Vimeo embed |

### Loading State

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Loading state | `loading={true}` | skeleton block with "Loading preview..." text |

### Error State

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Error state | `error="Failed to load embed. The URL may be invalid or the provider is unavailable."` | alert icon with error message |

### Empty State

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Empty state | `emptyMessage="Paste a URL above to see a preview"` | play icon with custom empty message |

## 14. Approval And Adoption Notes

- Contract status: `seed contract`
- Approvers: pending
- Downstream adopters: content editors, media management UIs, paired with EmbedInput
- Future follow-up: consider adding `onLoad` and `onError` events for iframe
  loading feedback; consider thumbnail preview mode before activating iframe;
  consider explicit allow-list for iframe sandbox attributes
