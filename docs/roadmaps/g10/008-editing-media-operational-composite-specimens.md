# g10.008 — Editing, Media, and Operational Composite Specimens

Status: planned
Owner: Pug Core
Updated: 2026-03-15
Depends on: g10.007
Primary repos: `pug`

## Goals

- [ ] create per-component specimens for editing, media, and operational
  composites
- [ ] document any components simplified or excluded for the game engine
  context

## Execution Checklist

- [ ] create `markdown_editor.rs` — MarkdownEditor (simplified: text area
  with basic formatting toolbar, no live preview in Jetstream)
- [ ] create `block_editor.rs` — BlockEditor (simplified or documented as
  intentional skip for game context)
- [ ] create `embed_input.rs` — EmbedInput with URL input and resolution
  indicator
- [ ] create `embed_preview.rs` — EmbedPreview with provider card
- [ ] create `embed_shell.rs` — EmbedShell managing input/preview states
- [ ] create `audio_player.rs` — AudioPlayer with playback controls and
  progress (Jetstream audio integration)
- [ ] create `video_player.rs` — VideoPlayer with overlay controls (may
  require native adaptation documentation)
- [ ] create `media_picker.rs` — MediaPicker with gallery and selection
- [ ] create `media_thumbnail.rs` — MediaThumbnail with aspect ratio and
  fallback
- [ ] create `media_preview.rs` — MediaPreview with metadata display
- [ ] create `log_list.rs` — LogList with severity icons and timestamps
- [ ] create `page_loading.rs` — PageLoading with skeleton or spinner
- [ ] create `empty_state.rs` — EmptyState with message and action button
- [ ] create `toast_stack.rs` — ToastStack with auto-dismiss toasts
- [ ] create `state_tile.rs` — StateTile with metric, label, and trend
- [ ] create `slug_field.rs` — SlugField with auto-slug generation
- [ ] create `inline_editable_field.rs` — InlineEditableField with
  display/edit toggle
- [ ] document components with reduced functionality in delta register notes
- [ ] register all modules and wire slug routing
- [ ] verify all specimens render without panic

## Acceptance Criteria

- [ ] all Jetstream-appropriate composites have specimens
- [ ] components with simplified behavior are documented
- [ ] intentionally excluded components are listed with justification
- [ ] `cargo check` passes

## Next Task

Open `g10.009` and build workstation surface specimens.
