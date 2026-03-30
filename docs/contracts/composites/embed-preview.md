# EmbedPreview

Status: detailed contract
Updated: 2026-03-30

## 1. Purpose

- Component name: `EmbedPreview`
- Layer: `composites`
- Summary: renders a preview of a parsed embed — displays an iframe for
  known providers (YouTube, Vimeo), renders raw embed code for iframe embeds,
  shows a fallback link for generic URLs, and handles loading, error, and
  empty states with appropriate visual treatments
- In scope: iframe rendering with configurable aspect ratio, provider-specific
  embed URLs (YouTube privacy-enhanced, Vimeo player), loading skeleton,
  error display with icon, empty state placeholder with icon, raw embed code
  rendering, fallback link display, sandbox security attributes
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
  ├── [Container .embed-preview__container]  (when parsed && originalEmbed, no embedUrl)
  │     └── [RawEmbed]  {@html parsed.originalEmbed}
  └── [Fallback .embed-preview__fallback]  (when parsed && no embedUrl && no originalEmbed)
        └── [FallbackLink]  <a>
```

| Part | Required | Description | Token Targets |
|------|----------|-------------|---------------|
| Root | yes | container with border-radius and overflow hidden | radius-surface |
| Loading | conditional | centered column with skeleton and loading text | background-panel, radius-surface |
| Skeleton | conditional | Skeleton primitive (block shape) | delegates to Skeleton contract |
| LoadingText | conditional | "Loading preview..." text | text-secondary color |
| Error | conditional | centered column with alert icon and error message | background-panel, radius-surface |
| ErrorIcon | conditional | alert circle SVG (decorative) | text-danger color |
| ErrorText | conditional | error message text | text-secondary color |
| Empty | conditional | centered column with play icon and empty message | background-panel, radius-surface |
| EmptyIcon | conditional | play rectangle SVG (decorative) | text-tertiary color |
| EmptyText | conditional | empty message text | text-secondary color |
| Container | conditional | aspect-ratio wrapper for iframe or raw embed | background-panel |
| Iframe | conditional | sandboxed iframe loading the embed URL | full-size absolute positioning |
| RawEmbed | conditional | raw HTML from `parsed.originalEmbed` | contained within Container |
| Fallback | conditional | link to the original URL | background-panel, radius-surface, accent color |

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

Fully controlled display component; all state is driven by props.

## 4. States

### Visual States

| State | Trigger | Expected Result |
|-------|---------|-----------------|
| loading | `loading` is true | centered skeleton with "Loading preview..." text |
| error | `error` is set (takes priority over parsed) | alert circle icon with error message |
| empty | `parsed` is null, not loading, no error | play rectangle icon with empty message text |
| iframe preview | `parsed` is set and `embedUrl` is derived | iframe in aspect-ratio container |
| raw embed | `parsed` is set with `originalEmbed` but no `embedUrl` | raw HTML rendered in aspect-ratio container |
| fallback | `parsed` is set but no `embedUrl` and no `originalEmbed` | link to original URL |

### Render Priority

States are evaluated in this order: loading > error > empty > iframe >
raw embed > fallback.

### Component States (Derived)

- `embedUrl` (derived): provider-specific embed URL computed from `parsed`
- `isAudio` (derived): true when `parsed.provider === "audioboom"`
- `effectiveAspectRatio` (derived): `"auto"` for audio providers, otherwise
  the `aspectRatio` prop

## 5. Events

None. EmbedPreview is a pure display component.

## 6. Accessibility

### Semantics

- Iframe: `title="{provider} embed"` for screen reader identification
- Iframe: `loading="lazy"` for performance
- Iframe: `sandbox="allow-scripts allow-same-origin allow-popups"` for security
- Iframe: `allowfullscreen` attribute
- Iframe: `frameborder="0"` attribute
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
- Container: `position: relative`, full width, aspect-ratio set via inline
  style
- Iframe: absolute positioned, 100% width and height, no border
- When aspect-ratio is `"auto"` (no inline style applied): iframe is static
  with `height: 10rem`
- Loading/Error/Empty: centered flex column, min-height `8rem`, padding
  `1.5rem`, gap `0.5rem`
- Error/Empty icons: `2rem` square
- Text: `0.8125rem` font-size
- Fallback: padding `0.75rem 1rem`

### Composition

- Composes: `Skeleton` from `@poodle/svelte-primitives`
- Parent expectations: media embed forms, content editors, paired with
  EmbedInput
- Child expectations: none (self-contained display)
- Resizing rules: fills parent width; height determined by aspect ratio or
  content

## 8. Token Usage — Exact Values

### Root `.embed-preview`

| Property | Value |
|----------|-------|
| border-radius | `var(--poodle-radius-surface, 0.5rem)` |
| overflow | `hidden` |

### Container `.embed-preview__container`

| Property | Value |
|----------|-------|
| position | `relative` |
| width | `100%` |
| background | `var(--poodle-color-background-panel, #1a1a1a)` |

Aspect ratio is applied via inline `style` attribute when
`effectiveAspectRatio !== "auto"`.

### Iframe `.embed-preview__iframe`

| Property | Value |
|----------|-------|
| position | `absolute` |
| top | `0` |
| left | `0` |
| width | `100%` |
| height | `100%` |
| border | `0` |

### Iframe Without Aspect Ratio (`.embed-preview__container:not([style*="aspect-ratio"]) .embed-preview__iframe`)

| Property | Value |
|----------|-------|
| position | `static` |
| height | `10rem` |

### Loading/Error/Empty `.embed-preview__loading`, `.embed-preview__error`, `.embed-preview__empty`

| Property | Value |
|----------|-------|
| display | `flex` |
| flex-direction | `column` |
| align-items | `center` |
| justify-content | `center` |
| gap | `0.5rem` |
| min-height | `8rem` |
| padding | `1.5rem` |
| background | `var(--poodle-color-background-panel, #1a1a1a)` |
| border-radius | `var(--poodle-radius-surface, 0.5rem)` |

### Error/Empty Icons `.embed-preview__error svg`, `.embed-preview__empty svg`

| Property | Value |
|----------|-------|
| width | `2rem` |
| height | `2rem` |
| color | `var(--poodle-color-text-tertiary, #666)` |

### Error Icon Override `.embed-preview__error svg`

| Property | Value |
|----------|-------|
| color | `var(--poodle-color-text-danger, #ef4444)` |

### Loading Text `.embed-preview__loading-text`

| Property | Value |
|----------|-------|
| font-size | `0.8125rem` |
| color | `var(--poodle-color-text-secondary, #999)` |

### Error/Empty Text `.embed-preview__error span`, `.embed-preview__empty span`

| Property | Value |
|----------|-------|
| font-size | `0.8125rem` |
| color | `var(--poodle-color-text-secondary, #999)` |

### Fallback `.embed-preview__fallback`

| Property | Value |
|----------|-------|
| padding | `0.75rem 1rem` |
| background | `var(--poodle-color-background-panel, #1a1a1a)` |
| border-radius | `var(--poodle-radius-surface, 0.5rem)` |

### Fallback Link `.embed-preview__fallback a`

| Property | Value |
|----------|-------|
| color | `var(--poodle-color-accent-default, #6366f1)` |
| font-size | `0.8125rem` |
| word-break | `break-all` |

### Composed Primitives

| Part | Delegates To |
|------|-------------|
| Skeleton | Skeleton contract (foundation), `shape="block"` |

### Embed URL Derivation

| Provider | Generated URL |
|----------|--------------|
| `youtube` | `https://www.youtube-nocookie.com/embed/{id}` |
| `vimeo` | `https://player.vimeo.com/video/{id}` |
| default/generic | `parsed.originalUrl` (may be null) |

### Light Theme Overrides

None.

## 9. Svelte Notes

- Uses `Skeleton` from `@poodle/svelte-primitives` for the loading state
- Raw embed code rendered via `{@html parsed.originalEmbed}` — consumers must
  ensure embed code is trusted
- Aspect ratio applied via inline `style` attribute on the container
- `embedUrl` is a reactive derived value from `parsed` via `getEmbedUrl()`
- `isAudio` is derived from `parsed?.provider === "audioboom"`
- `effectiveAspectRatio` is derived: `"auto"` for audio, otherwise
  `aspectRatio` prop
- When `effectiveAspectRatio` is `"auto"`, no inline style is set on
  container, and the CSS selector `.embed-preview__container:not([style*="aspect-ratio"])`
  makes the iframe static with `height: 10rem`
- SVG icons are inline (alert circle for error, play rectangle for empty)

## 10. GPUI Notes

- Expected crate/module surface: `poodle_gpui::composites::embed_preview`
- Consumes the same `parsed` / `aspectRatio` / `loading` / `error` /
  `emptyMessage` contract as Svelte
- Iframe rendering remains platform-specific; GPUI currently renders a
  contract-aligned placeholder panel for derived `embedUrl` states rather
  than embedding a live web view
- Raw embed and fallback states still follow the same priority order as Svelte

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
- [ ] fallback link styling matches

### Tier 3: Implementation Freedom

- [ ] iframe rendering may differ per platform
- [ ] rendering internals stay internal

## 12. Specimen Definitions

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
| Error state | `error="Failed to load embed. The URL may be invalid or the provider is unavailable."` | alert circle icon with error message |

### Empty State

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Empty state | `emptyMessage="Paste a URL above to see a preview"` | play rectangle icon with custom empty message |

### Raw Embed

| Label | Props / Config | Expected Visual |
|-------|---------------|-----------------|
| Raw embed | `parsed.originalEmbed` set, no provider-specific embed URL | raw embed container with aspect-ratio wrapper |
