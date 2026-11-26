# Component Contracts

Status: active
Updated: 2026-09-01

Component contracts define every reusable surface in Poodle, from low-level
primitives and layout surfaces through higher-order application components.

## Current Contracts

Hand-kept and grouped by family. `docs:lint` fails when a contract file is missing from this list.

### Foundations

- `avatar.md`
- `box.md`
- `code.md`
- `dock-region.md`
- `error-boundary.md`
- `eyebrow.md`
- `format-display-date.md`
- `format-file-size.md`
- `grid.md`
- `icon.md`
- `icon-button.md`
- `icon-provider.md`
- `keyboard.md`
- `motion-policy-provider.md`
- `pill.md`
- `progress.md`
- `region.md`
- `resize-handle.md`
- `scroll-shell.md`
- `separator.md`
- `size-and-density.md`
- `skeleton.md`
- `spacer.md`
- `spinner.md`
- `split-view.md`
- `stack.md`
- `status-indicator.md`
- `surface.md`
- `surface-elevation.md`
- `text.md`
- `text-link.md`
- `time-ago.md`
- `ui-presentation-provider.md`
- `value-readout.md`

### Actions

- `bulk-action-bar.md`
- `button.md`
- `collapse-toggle.md`
- `confirm-action.md`
- `form-actions.md`
- `segmented-control.md`
- `split-button.md`
- `toggle-group.md`
- `toolbar.md`

### Inputs and selection

- `block-editor.md`
- `card-radio-group.md`
- `card-toggle-group.md`
- `checkbox.md`
- `code-editor.md`
- `code-input.md`
- `color-picker.md`
- `drag-number-field.md`
- `duration-input.md`
- `editable-label.md`
- `editable-list.md`
- `embed-input.md`
- `envelope-editor.md`
- `field.md`
- `field-set.md`
- `file-upload.md`
- `filter-builder.md`
- `filter-toolbar.md`
- `listbox.md`
- `markdown-editor.md`
- `number-input.md`
- `order-by.md`
- `password-requirements.md`
- `picker-shell.md`
- `radio.md`
- `radio-group.md`
- `range-slider.md`
- `rating.md`
- `ref-select.md`
- `relation-picker.md`
- `rich-text-editor.md`
- `select.md`
- `slider.md`
- `stepper.md`
- `switch.md`
- `text-input.md`
- `theme-select.md`
- `time-input.md`
- `time-zone-select.md`
- `token-input.md`
- `tri-state-switch.md`
- `validation-summary.md`

### Dates

- `calendar.md`
- `date-picker.md`
- `date-range-picker.md`
- `date-time-picker.md`
- `date-time-range-picker.md`
- `date-time-zone-picker.md`

### Forms

- `form-dialog.md`
- `form-layout.md`
- `form-shell.md`

### Overlays and menus

- `alert-dialog.md`
- `command-palette.md`
- `context-menu.md`
- `debug-dialog.md`
- `dialog.md`
- `drawer.md`
- `hover-card.md`
- `menu.md`
- `menubar.md`
- `navigation-menu.md`
- `popover.md`
- `toast-host.md`
- `toast-stack.md`
- `tooltip.md`

### Navigation and shells

- `accordion.md`
- `action-discovery-panel.md`
- `app-header.md`
- `breadcrumbs.md`
- `collapsible.md`
- `detail-shell.md`
- `history-center.md`
- `nav-card.md`
- `page-header.md`
- `pagination.md`
- `pagination-summary.md`
- `settings-shell.md`
- `sidebar-nav.md`
- `tab-strip.md`
- `tabs.md`
- `tree.md`

### Data and lists

- `card.md`
- `changed-files.md`
- `data-table.md`
- `detail-item.md`
- `detail-section.md`
- `detail-section-group.md`
- `empty-state.md`
- `inline-list-section.md`
- `list-card.md`
- `list-card-counter.md`
- `list-container.md`
- `list-grid.md`
- `log-list.md`
- `meta-bar.md`
- `meta-item.md`
- `metric-tile.md`
- `mod-matrix-grid.md`
- `page-loading.md`
- `selection-summary.md`
- `state-tile.md`
- `table.md`

### Feedback and status

- `callout.md`
- `inline-remediation.md`
- `message-center.md`
- `remediation-banner.md`
- `status-bar.md`
- `update-center.md`
- `update-status.md`

### Media and audio

- `audio-meter.md`
- `audio-player.md`
- `audio-switch.md`
- `embed-preview.md`
- `fader.md`
- `gain-reduction-meter.md`
- `knob.md`
- `media-browse-panel.md`
- `media-picker.md`
- `media-preview.md`
- `media-thumbnail.md`
- `meter.md`
- `meter-surface.md`
- `video-player.md`
- `waveform-display.md`
- `xy-pad.md`

### Agents

- `agent-chat-input.md`
- `agent-message.md`
- `agent-plan.md`
- `agent-plan-record.md`
- `agent-question.md`
- `agent-question-record.md`
- `agent-subagent.md`
- `agent-transcript.md`
- `tool-call.md`
- `tool-call-group.md`

### Models and licences

- `licence-activation.md`
- `licence-seats.md`
- `licence-status.md`
- `model-catalogue-editor.md`
- `model-connection-card.md`
- `model-connection-picker.md`
- `model-connection-setup.md`
- `model-picker.md`

## Accessibility Rule

All components need explicit accessibility treatment. In particular:

- non-interactive layout primitives should stay accessibility-neutral by
  default
- semantic surfaces should opt into region or landmark semantics explicitly
- scroll containers must define keyboard reachability and focus behavior
- GPUI implementations must preserve equivalent semantics through native
  accessibility APIs where ARIA is not available

## Composition Rule

Higher-order components should:

- compose documented primitives rather than redefining them
- stay generic enough for Underlay-style product apps and Loophole-adjacent
  settings, library, and inspector surfaces
- keep data fetching, command wiring, persistence, and domain-specific row or
  card content outside the component contract itself
- keep accessibility explicit for heading hierarchy, region labeling, empty
  states, and collection-browse shells in both Svelte and GPUI
