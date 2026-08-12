# Contracts

Status: active
Updated: 2026-04-11

Contracts are the docs-first source of truth for Poodle components.

Each component contract describes one semantic surface that Svelte, React,
GPUI, and Jetstream implementations can satisfy against the same review
checklist.

## Structure

```text
docs/contracts/
  README.md
  template/
    component-contract-template.md
  components/
    ...
```

## Grouping Rules

- `components/` for all reusable component contracts, from low-level primitives
  through higher-order application composites
- workstation semantics are currently expressed through the shared contract
  crates plus the component docs above, rather than a separate
  `docs/contracts/workstation/` subtree

App-specific DAW widgets do not belong in this contract surface. They build
above it in downstream repos such as Loophole.

## Cross-Cutting Rules

Rules that bind more than one contract live beside them, and a component
contract references the rule rather than restating it:

- `001-working-rules.md` — roadmap/spec/log authority chain
- `002-anchored-overlays.md` — every anchored surface portals to the theme root
  and is positioned in viewport coordinates
- `003-native-accessibility.md` — neither native runtime exposes an
  accessibility API, so `aria_label` is carried but inert there
- `004-shared-control-types.md` — types shared by more than one component
  contract are defined once and referenced, never restated with fewer members

## Current Contracts

- `template/component-contract-template.md`
- `components/README.md`
- `components/accordion.md`
- `components/action-discovery-panel.md`
- `components/agent-chat-input.md`
- `components/agent-message.md`
- `components/agent-plan-record.md`
- `components/agent-plan.md`
- `components/agent-question-record.md`
- `components/agent-question.md`
- `components/agent-subagent.md`
- `components/agent-transcript.md`
- `components/alert-dialog.md`
- `components/app-header.md`
- `components/audio-player.md`
- `components/audio-meter.md`
- `components/audio-switch.md`
- `components/avatar.md`
- `components/block-editor.md`
- `components/box.md`
- `components/breadcrumbs.md`
- `components/bulk-action-bar.md`
- `components/button.md`
- `components/calendar.md`
- `components/callout.md`
- `components/card-radio-group.md`
- `components/card-toggle-group.md`
- `components/card.md`
- `components/changed-files.md`
- `components/checkbox.md`
- `components/code-input.md`
- `components/code.md`
- `components/collapse-toggle.md`
- `components/collapsible.md`
- `components/color-picker.md`
- `components/command-palette.md`
- `components/confirm-action.md`
- `components/context-menu.md`
- `components/data-table.md`
- `components/date-picker.md`
- `components/date-range-picker.md`
- `components/date-time-picker.md`
- `components/date-time-range-picker.md`
- `components/date-time-zone-picker.md`
- `components/debug-dialog.md`
- `components/detail-item.md`
- `components/detail-section.md`
- `components/detail-section-group.md`
- `components/detail-shell.md`
- `components/dialog.md`
- `components/dock-region.md`
- `components/drawer.md`
- `components/drag-number-field.md`
- `components/envelope-editor.md`
- `components/duration-input.md`
- `components/editable-label.md`
- `components/editable-list.md`
- `components/embed-input.md`
- `components/embed-preview.md`
- `components/empty-state.md`
- `components/error-boundary.md`
- `components/eyebrow.md`
- `components/field-set.md`
- `components/field.md`
- `components/fader.md`
- `components/gain-reduction-meter.md`
- `components/file-upload.md`
- `components/filter-builder.md`
- `components/filter-toolbar.md`
- `components/form-actions.md`
- `components/form-dialog.md`
- `components/form-layout.md`
- `components/form-shell.md`
- `components/format-display-date.md`
- `components/format-file-size.md`
- `components/grid.md`
- `components/hover-card.md`
- `components/history-center.md`
- `components/icon-button.md`
- `components/icon-provider.md`
- `components/icon.md`
- `components/inline-list-section.md`
- `components/inline-remediation.md`
- `components/keyboard.md`
- `components/knob.md`
- `components/list-card-counter.md`
- `components/list-card.md`
- `components/list-container.md`
- `components/list-grid.md`
- `components/log-list.md`
- `components/markdown-editor.md`
- `components/message-center.md`
- `components/media-browse-panel.md`
- `components/media-picker.md`
- `components/media-preview.md`
- `components/media-thumbnail.md`
- `components/menu.md`
- `components/menubar.md`
- `components/meta-bar.md`
- `components/meta-item.md`
- `components/meter.md`
- `components/metric-tile.md`
- `components/mod-matrix-grid.md`
- `components/model-picker.md`
- `components/nav-card.md`
- `components/navigation-menu.md`
- `components/number-input.md`
- `components/order-by.md`
- `components/page-header.md`
- `components/page-loading.md`
- `components/pagination-summary.md`
- `components/pagination.md`
- `components/password-requirements.md`
- `components/picker-shell.md`
- `components/pill.md`
- `components/popover.md`
- `components/progress.md`
- `components/radio.md`
- `components/radio-group.md`
- `components/range-slider.md`
- `components/rating.md`
- `components/ref-select.md`
- `components/region.md`
- `components/relation-picker.md`
- `components/remediation-banner.md`
- `components/resize-handle.md`
- `components/scroll-shell.md`
- `components/segmented-control.md`
- `components/select.md`
- `components/selection-summary.md`
- `components/separator.md`
- `components/settings-shell.md`
- `components/sidebar-nav.md`
- `components/size-and-density.md`
- `components/skeleton.md`
- `components/slider.md`
- `components/spacer.md`
- `components/spinner.md`
- `components/split-button.md`
- `components/split-view.md`
- `components/stack.md`
- `components/state-tile.md`
- `components/status-bar.md`
- `components/status-indicator.md`
- `components/stepper.md`
- `components/surface-elevation.md`
- `components/surface.md`
- `components/switch.md`
- `components/tab-strip.md`
- `components/table.md`
- `components/tabs.md`
- `components/text.md`
- `components/text-link.md`
- `components/text-input.md`
- `components/theme-select.md`
- `components/token-input.md`
- `components/time-ago.md`
- `components/time-input.md`
- `components/time-zone-select.md`
- `components/toast-host.md`
- `components/toast-stack.md`
- `components/toggle-group.md`
- `components/tool-call-group.md`
- `components/tool-call.md`
- `components/toolbar.md`
- `components/tooltip.md`
- `components/tree.md`
- `components/tri-state-switch.md`
- `components/ui-presentation-provider.md`
- `components/validation-summary.md`
- `components/value-readout.md`
- `components/video-player.md`
- `components/waveform-display.md`
- `components/xy-pad.md`

This index is generated manually and must stay aligned when contract files are
added, removed, or regrouped.
