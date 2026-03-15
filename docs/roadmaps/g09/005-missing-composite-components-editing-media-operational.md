# g09.005 — Missing Composite Components: Editing, Media, and Operational

Status: planned
Owner: Pug Core
Updated: 2026-03-15
Depends on: g09.004
Primary repos: `pug`

## Goals

- [ ] implement composite components for editing, media, and operational
  patterns covering the remaining Svelte composites
- [ ] these are the most complex composites and may require native GPUI
  adaptations documented in the delta register

## Execution Checklist

- [ ] create `PugMarkdownEditor` composite — text area with markdown preview
  toggle, toolbar for formatting actions
- [ ] create `PugBlockEditor` composite — block-based content editor with
  block type selector and reordering (may be simplified for GPUI)
- [ ] create `PugEmbedInput` composite — URL input with paste detection and
  embed resolution indicator
- [ ] create `PugEmbedPreview` composite — rich embed card with provider
  icon, title, description, thumbnail
- [ ] create `PugEmbedShell` composite — container managing embed input and
  preview states
- [ ] create `PugAudioPlayer` composite — playback controls, progress bar,
  time display, volume
- [ ] create `PugVideoPlayer` composite — video display area with overlay
  controls and progress
- [ ] create `PugMediaPicker` composite — gallery browser with search,
  filtering, and selection
- [ ] create `PugMediaThumbnail` composite — image thumbnail with aspect
  ratio, loading placeholder, and fallback
- [ ] create `PugMediaPreview` composite — full preview with metadata
  and action buttons
- [ ] create `PugLogList` composite — scrollable log stream with severity
  icons, timestamps, and filtering
- [ ] create `PugPageLoading` composite — full-page loading state with
  skeleton or spinner
- [ ] create `PugEmptyState` composite — illustration area, title, message,
  and primary action button
- [ ] create `PugToastStack` composite — notification toast container with
  auto-dismiss, severity tones, and stacking
- [ ] create `PugStateTile` composite — metric card with label, value,
  trend indicator, and optional sparkline
- [ ] create `PugSlugField` composite — text input with auto-slug generation,
  prefix display, and validation
- [ ] create `PugInlineEditableField` composite — click-to-edit field with
  inline display/edit mode toggle
- [ ] register all composites in `lib.rs`
- [ ] verify compilation with `cargo check`

## Acceptance Criteria

- [ ] all 17 composite components compile and are exported
- [ ] media components gracefully handle missing media (placeholder display)
- [ ] ToastStack supports multiple simultaneous toasts with auto-dismiss
- [ ] BlockEditor documents any GPUI-specific limitations in the delta register
- [ ] `cargo check` passes with zero errors

## Next Task

Open `g09.006` and implement missing workstation components.
