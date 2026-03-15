# g09.012 — Specimen Upgrade: Composites Batch 2 (Editing, Media, Workstation)

Status: planned
Owner: Pug Core
Updated: 2026-03-15
Depends on: g09.005, g09.006
Primary repos: `pug`

## Goals

- [ ] break grouped media, editor, and workstation specimens into per-component
  files using real Pug component instances
- [ ] complete the specimen-per-component model so every slug maps to a
  dedicated file

## Execution Checklist

- [ ] split `media.rs` into individual specimens:
  - [ ] `audio_player.rs` → `PugAudioPlayer` with playback controls
  - [ ] `video_player.rs` → `PugVideoPlayer` with overlay controls
  - [ ] `media_picker.rs` → `PugMediaPicker` with gallery and selection
  - [ ] `media_preview.rs` → `PugMediaPreview` with metadata
  - [ ] `media_thumbnail.rs` → `PugMediaThumbnail` with aspect ratio
- [ ] split `editors.rs` into individual specimens:
  - [ ] `markdown_editor.rs` → `PugMarkdownEditor` with preview toggle
  - [ ] `block_editor.rs` → `PugBlockEditor` with block manipulation
- [ ] create `embed_input.rs` → `PugEmbedInput` with URL paste
- [ ] create `embed_preview.rs` → `PugEmbedPreview` with provider card
- [ ] create `embed_shell.rs` → `PugEmbedShell` managing input/preview
- [ ] create `log_list.rs` → `PugLogList` with severity filtering
- [ ] create `page_loading.rs` → `PugPageLoading` with skeleton variant
- [ ] create `empty_state.rs` → `PugEmptyState` with illustration and CTA
- [ ] create `toast_stack.rs` → `PugToastStack` with multiple severity toasts
- [ ] create `state_tile.rs` → `PugStateTile` with metric and trend
- [ ] create `slug_field.rs` → `PugSlugField` with auto-slug generation
- [ ] create `inline_editable_field.rs` → `PugInlineEditableField` with
  display/edit mode
- [ ] split `workspace.rs` into:
  - [ ] `workspace_shell.rs` → `PugWorkspaceShell`
  - [ ] `app_header.rs` → `PugAppHeader`
- [ ] split `panel.rs` into:
  - [ ] `panel_header.rs` → `PugPanelHeader`
  - [ ] `panel_surface.rs` → `PugPanelSurface`
  - [ ] `panel_tabs.rs` → `PugPanelTabs`
- [ ] split `dock_split.rs` into:
  - [ ] `dock_region.rs` → `PugDockRegion`
  - [ ] `split_view.rs` → `PugSplitView`
- [ ] split `status_bar.rs` into:
  - [ ] `shell_status_bar.rs` → `PugShellStatusBar`
  - [ ] `surface_tabs.rs` → `PugSurfaceTabs`
- [ ] update `command_palette.rs` to use `PugCommandPalette`
- [ ] update `action_discovery.rs` to use `PugActionDiscoveryPanel`
- [ ] update `project_header.rs` to use real workstation components
- [ ] remove archived grouped files (media.rs, editors.rs, workspace.rs,
  panel.rs, dock_split.rs, status_bar.rs)
- [ ] update `mod.rs` with all new modules and slug routing
- [ ] verify all slugs render without panic

## Acceptance Criteria

- [ ] zero grouped specimen files remain — every slug has a dedicated file
- [ ] every specimen uses real `Pug*` component constructors
- [ ] all media, editor, and workstation slugs render correctly
- [ ] `cargo check` passes for the preview crate

## Next Task

Open `g09.013` and build the preview app shell.
