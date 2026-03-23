# g11.002 Implement Missing Composites

Status: planned
Owner: Poodle Core
Depends on: g11.001

## Context

16 Svelte composites have no GPUI counterpart. Each needs a real implementation
backed by its contract spec, with token resolution and proper anatomy.

## Missing Components

### Form/Interaction
- [ ] card_radio_group — radio group rendered as selectable cards
- [ ] confirm_action — confirmation dialog with action buttons
- [ ] form_dialog — dialog wrapping a form with validation

### Lists
- [ ] editable_list — list with add/remove/reorder capabilities
- [ ] reorderable_list — drag-to-reorder list
- [ ] log_list — timestamped log entry list

### Fields
- [ ] inline_editable_field — click-to-edit field with save/cancel
- [ ] slug_field — URL slug input with auto-generation

### Embeds
- [ ] embed_input — URL input for embedding external content
- [ ] embed_preview — preview renderer for embedded content

### Media
- [ ] audio_player — audio playback controls
- [ ] video_player — video playback controls
- [ ] media_picker — media selection/upload interface

### Utility
- [ ] page_loading — full-page loading state

### Rich Text
- [ ] markdown_editor — markdown editing with preview
- [ ] block_editor — block-based content editor

## Notes

Some of these (audio_player, video_player, markdown_editor, block_editor)
are complex components that may need GPUI-specific adaptation. Implementation
should follow the contract and Svelte reference. Where GPUI lacks a capability
(e.g., audio/video playback), implement the UI chrome (controls, layout) and
document the playback gap.
