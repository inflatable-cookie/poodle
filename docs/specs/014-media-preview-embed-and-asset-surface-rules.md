# 014 Media Preview, Embed, And Asset-Surface Rules

Status: active
Updated: 2026-03-11
Depends on: `012-detail-display-card-header-and-navigation-rules.md`, `013-picker-relation-and-selection-workflow-rules.md`

## Purpose

Freeze the first shared rules for media previews, asset surfaces, and embed shells so Svelte and GPUI expose the same framing, fallback, and accessibility posture.

## Preview Family Rule

Media preview surfaces need explicit framing.

At minimum they must expose:

- stable aspect-ratio or framed media region
- textual identity outside the visual preview itself
- explicit media kind or surrounding context when it matters
- explicit loading, error, or empty posture when no preview is ready

## Asset Surface Rule

Asset-oriented previews may vary in content.

They may represent:

- still imagery
- audio-like preview surfaces
- video-like preview surfaces
- document-like preview surfaces

They may not hide asset identity, fallback text, or available actions inside the renderer alone.

## Embed Shell Rule

Embeds require a shell, not just a viewport.

The shell must preserve:

- local title and supporting context
- framed destination region
- provider/source context when relevant
- recovery or alternate actions when the embedded destination is unavailable

## Optional Capability Rule

Not every runtime or host can render every preview.

That is allowed.
What is not allowed is silently degrading from a meaningful preview surface into an unlabeled blank block.

## Accessibility Rule

Both runtimes must preserve:

- textual identity for the asset or embed
- explicit preview state posture
- footer or alternate actions that do not disappear with the renderer
- stable framing so screen-reader, keyboard, and focus behavior stay predictable

Svelte should use native text, button, and region semantics first.
GPUI must recreate equivalent meaning in the native accessibility tree even where rendering engines differ.

## Seed Evidence

- `docs/contracts/components/media-thumbnail.md`
- `docs/contracts/components/media-preview.md`
- `docs/contracts/components/embed-shell.md`
- `packages/svelte/composites/src/MediaThumbnail.svelte`
- `packages/svelte/composites/src/MediaPreview.svelte`
- `packages/svelte/composites/src/EmbedShell.svelte`
- `packages/svelte/preview/src/App.svelte`

## Next Task

Carry this media and embed baseline into `g02.007` and later milestones so loading, remediation, and notification depth build on one explicit fallback posture.
