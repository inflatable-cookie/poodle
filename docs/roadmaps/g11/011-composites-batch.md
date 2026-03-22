# g11.011 Composites Batch

Status: planned
Owner: Pug Core
Depends on: contract audit

## Components

### Existing composites (audit + fix)
data_table, detail_section, detail_shell, page_header, filter_toolbar,
empty_state, picker_shell, relation_picker, selection_summary, split_view,
toast_stack, metric_tile, dock_region, app_header, command_palette,
action_discovery_panel, media_preview, media_thumbnail

### Newly implemented composites (audit + fix)
audio_player, block_editor, card_radio_group, confirm_action, editable_list,
embed_input, embed_preview, form_dialog, form_layout, inline_editable_field,
log_list, markdown_editor, media_picker, page_loading, reorderable_list,
slug_field, video_player

### Primitives that are composites in nature (audit)
breadcrumbs, list_card, nav_card, nav_card_grid, order_by, pagination_summary

## Structural Issues

- [ ] `editable_list` — contract exists (`editable-list.md`) but **no Rust spec**.
      GPUI has `editable_list.rs` implemented without a spec struct. Need to
      create `EditableListSpec` in `pug-composites`.
- [ ] `form_dialog` — contract exists (`form-dialog.md`) but **no Rust spec**.
      GPUI has `form_dialog.rs` implemented without a spec struct. Need to
      create `FormDialogSpec` in `pug-composites`.
- [ ] `form_layout` — contract exists (`form-layout.md`) but **no Rust spec**.
      GPUI has `form_layout.rs` implemented without a spec struct. Need to
      create `FormLayoutSpec` in `pug-composites`.
- [ ] `inline_editable_field` — GPUI component exists in composites, Rust spec
      `InlineEditableFieldSpec` exists, but **no contract markdown**
      `inline-editable-field.md` in composites dir. Verify it exists or create it.
- [ ] `pagination_summary` — contract is in `foundation/` but Rust spec is in
      `pug-composites`. GPUI has it in `primitives/`. Align: either move the
      Rust spec to `pug-primitives` or reclassify.

### Orphaned Rust specs (no contract, no Svelte)

These Rust spec files in `pug-composites` have no corresponding contract
markdown and may be deprecated. Verify and remove if so:

- [ ] `autonomous_list.rs` — no contract
- [ ] `form_shell.rs` — no contract (replaced by `form-layout`/`form-dialog`?)
- [ ] `inline_remediation.rs` — no contract
- [ ] `remediation_banner.rs` — no contract
- [ ] `shell_status_bar.rs` — no contract (replaced by `status-bar` in foundation?)
- [ ] `state_tile.rs` — no contract
- [ ] `validation_summary.rs` — no contract

## Per-Component Compliance

### Existing composites
- [ ] data_table — audit against `docs/contracts/composites/data-table.md`
- [ ] detail_section — audit against `docs/contracts/composites/detail-section.md`
- [ ] detail_shell — audit against `docs/contracts/composites/detail-shell.md`
- [ ] page_header — audit against `docs/contracts/composites/page-header.md`
- [ ] filter_toolbar — audit against `docs/contracts/composites/filter-toolbar.md`
- [ ] empty_state — audit against `docs/contracts/composites/empty-state.md`
- [ ] picker_shell — audit against `docs/contracts/composites/picker-shell.md`
- [ ] relation_picker — audit against `docs/contracts/composites/relation-picker.md`
- [ ] selection_summary — audit against `docs/contracts/composites/selection-summary.md`
- [ ] split_view — audit against `docs/contracts/composites/split-view.md`
- [ ] toast_stack — audit against `docs/contracts/composites/toast-stack.md`
- [ ] metric_tile — audit against `docs/contracts/composites/metric-tile.md`
- [ ] dock_region — audit against `docs/contracts/composites/dock-region.md`
- [ ] app_header — audit against `docs/contracts/composites/app-header.md`
- [ ] command_palette — audit against `docs/contracts/composites/command-palette.md`
- [ ] action_discovery_panel — audit against `docs/contracts/composites/action-discovery-panel.md`
- [ ] media_preview — audit against `docs/contracts/composites/media-preview.md`
- [ ] media_thumbnail — audit against `docs/contracts/composites/media-thumbnail.md`

### New composites
- [ ] audio_player — audit against `docs/contracts/composites/audio-player.md`
- [ ] block_editor — audit against `docs/contracts/composites/block-editor.md`
- [ ] card_radio_group — audit against `docs/contracts/composites/card-radio-group.md`
- [ ] confirm_action — audit against `docs/contracts/composites/confirm-action.md`
- [ ] editable_list — audit against `docs/contracts/composites/editable-list.md`
- [ ] embed_input — audit against `docs/contracts/composites/embed-input.md`
- [ ] embed_preview — audit against `docs/contracts/composites/embed-preview.md`
- [ ] form_dialog — audit against `docs/contracts/composites/form-dialog.md`
- [ ] form_layout — audit against `docs/contracts/composites/form-layout.md`
- [ ] log_list — audit against `docs/contracts/composites/log-list.md`
- [ ] markdown_editor — audit against `docs/contracts/composites/markdown-editor.md`
- [ ] media_picker — audit against `docs/contracts/composites/media-picker.md`
- [ ] page_loading — audit against `docs/contracts/composites/page-loading.md`
- [ ] reorderable_list — audit against `docs/contracts/composites/reorderable-list.md`
- [ ] slug_field — audit against `docs/contracts/composites/slug-field.md`
- [ ] video_player — audit against `docs/contracts/composites/video-player.md`

### Primitives (composites in nature)
- [ ] breadcrumbs — audit against `docs/contracts/foundation/breadcrumbs.md`
- [ ] list_card — audit against `docs/contracts/foundation/list-card.md`
- [ ] nav_card — audit against `docs/contracts/foundation/nav-card.md`
- [ ] nav_card_grid — audit against `docs/contracts/foundation/nav-card-grid.md`
- [ ] order_by — audit against `docs/contracts/foundation/order-by.md`
- [ ] pagination_summary — audit against `docs/contracts/foundation/pagination-summary.md`
