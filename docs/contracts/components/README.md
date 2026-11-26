# Component Contracts

Status: active

Component contracts define every reusable surface in Poodle, from low-level
primitives and layout surfaces through higher-order application components.

## Current Contracts

Every component contract, grouped by family. Hand-kept: `docs:lint` fails when a contract file is missing from this table.

| Family | Contracts |
| --- | --- |
| Foundations | [avatar](avatar.md) · [box](box.md) · [code](code.md) · [dock-region](dock-region.md) · [error-boundary](error-boundary.md) · [eyebrow](eyebrow.md) · [format-display-date](format-display-date.md) · [format-file-size](format-file-size.md) · [grid](grid.md) · [icon](icon.md) · [icon-button](icon-button.md) · [icon-provider](icon-provider.md) · [keyboard](keyboard.md) · [motion-policy-provider](motion-policy-provider.md) · [pill](pill.md) · [progress](progress.md) · [region](region.md) · [resize-handle](resize-handle.md) · [scroll-shell](scroll-shell.md) · [separator](separator.md) · [size-and-density](size-and-density.md) · [skeleton](skeleton.md) · [spacer](spacer.md) · [spinner](spinner.md) · [split-view](split-view.md) · [stack](stack.md) · [status-indicator](status-indicator.md) · [surface](surface.md) · [surface-elevation](surface-elevation.md) · [text](text.md) · [text-link](text-link.md) · [time-ago](time-ago.md) · [ui-presentation-provider](ui-presentation-provider.md) · [value-readout](value-readout.md) |
| Actions | [bulk-action-bar](bulk-action-bar.md) · [button](button.md) · [collapse-toggle](collapse-toggle.md) · [confirm-action](confirm-action.md) · [form-actions](form-actions.md) · [segmented-control](segmented-control.md) · [split-button](split-button.md) · [toggle-group](toggle-group.md) · [toolbar](toolbar.md) |
| Inputs and selection | [block-editor](block-editor.md) · [card-radio-group](card-radio-group.md) · [card-toggle-group](card-toggle-group.md) · [checkbox](checkbox.md) · [code-editor](code-editor.md) · [code-input](code-input.md) · [color-picker](color-picker.md) · [drag-number-field](drag-number-field.md) · [duration-input](duration-input.md) · [editable-label](editable-label.md) · [editable-list](editable-list.md) · [embed-input](embed-input.md) · [envelope-editor](envelope-editor.md) · [field](field.md) · [field-set](field-set.md) · [file-upload](file-upload.md) · [filter-builder](filter-builder.md) · [filter-toolbar](filter-toolbar.md) · [listbox](listbox.md) · [markdown-editor](markdown-editor.md) · [number-input](number-input.md) · [order-by](order-by.md) · [password-requirements](password-requirements.md) · [picker-shell](picker-shell.md) · [radio](radio.md) · [radio-group](radio-group.md) · [range-slider](range-slider.md) · [rating](rating.md) · [ref-select](ref-select.md) · [relation-picker](relation-picker.md) · [rich-text-editor](rich-text-editor.md) · [select](select.md) · [slider](slider.md) · [stepper](stepper.md) · [switch](switch.md) · [text-input](text-input.md) · [theme-select](theme-select.md) · [time-input](time-input.md) · [time-zone-select](time-zone-select.md) · [token-input](token-input.md) · [tri-state-switch](tri-state-switch.md) · [validation-summary](validation-summary.md) |
| Dates | [calendar](calendar.md) · [date-picker](date-picker.md) · [date-range-picker](date-range-picker.md) · [date-time-picker](date-time-picker.md) · [date-time-range-picker](date-time-range-picker.md) · [date-time-zone-picker](date-time-zone-picker.md) |
| Forms | [form-dialog](form-dialog.md) · [form-layout](form-layout.md) · [form-shell](form-shell.md) |
| Overlays and menus | [alert-dialog](alert-dialog.md) · [command-palette](command-palette.md) · [context-menu](context-menu.md) · [debug-dialog](debug-dialog.md) · [dialog](dialog.md) · [drawer](drawer.md) · [hover-card](hover-card.md) · [menu](menu.md) · [menubar](menubar.md) · [navigation-menu](navigation-menu.md) · [popover](popover.md) · [toast-host](toast-host.md) · [toast-stack](toast-stack.md) · [tooltip](tooltip.md) |
| Navigation and shells | [accordion](accordion.md) · [action-discovery-panel](action-discovery-panel.md) · [app-header](app-header.md) · [breadcrumbs](breadcrumbs.md) · [collapsible](collapsible.md) · [detail-shell](detail-shell.md) · [history-center](history-center.md) · [nav-card](nav-card.md) · [page-header](page-header.md) · [pagination](pagination.md) · [pagination-summary](pagination-summary.md) · [settings-shell](settings-shell.md) · [sidebar-nav](sidebar-nav.md) · [tab-strip](tab-strip.md) · [tabs](tabs.md) · [tree](tree.md) |
| Data and lists | [card](card.md) · [changed-files](changed-files.md) · [data-table](data-table.md) · [detail-item](detail-item.md) · [detail-section](detail-section.md) · [detail-section-group](detail-section-group.md) · [empty-state](empty-state.md) · [inline-list-section](inline-list-section.md) · [list-card](list-card.md) · [list-card-counter](list-card-counter.md) · [list-container](list-container.md) · [list-grid](list-grid.md) · [log-list](log-list.md) · [meta-bar](meta-bar.md) · [meta-item](meta-item.md) · [metric-tile](metric-tile.md) · [mod-matrix-grid](mod-matrix-grid.md) · [page-loading](page-loading.md) · [selection-summary](selection-summary.md) · [state-tile](state-tile.md) · [table](table.md) |
| Feedback and status | [callout](callout.md) · [inline-remediation](inline-remediation.md) · [message-center](message-center.md) · [remediation-banner](remediation-banner.md) · [status-bar](status-bar.md) · [update-center](update-center.md) · [update-status](update-status.md) |
| Media and audio | [audio-meter](audio-meter.md) · [audio-player](audio-player.md) · [audio-switch](audio-switch.md) · [embed-preview](embed-preview.md) · [fader](fader.md) · [gain-reduction-meter](gain-reduction-meter.md) · [knob](knob.md) · [media-browse-panel](media-browse-panel.md) · [media-picker](media-picker.md) · [media-preview](media-preview.md) · [media-thumbnail](media-thumbnail.md) · [meter](meter.md) · [meter-surface](meter-surface.md) · [video-player](video-player.md) · [waveform-display](waveform-display.md) · [xy-pad](xy-pad.md) |
| Agents | [agent-chat-input](agent-chat-input.md) · [agent-message](agent-message.md) · [agent-plan](agent-plan.md) · [agent-plan-record](agent-plan-record.md) · [agent-question](agent-question.md) · [agent-question-record](agent-question-record.md) · [agent-subagent](agent-subagent.md) · [agent-transcript](agent-transcript.md) · [tool-call](tool-call.md) · [tool-call-group](tool-call-group.md) |
| Models and licences | [licence-activation](licence-activation.md) · [licence-seats](licence-seats.md) · [licence-status](licence-status.md) · [model-catalogue-editor](model-catalogue-editor.md) · [model-connection-card](model-connection-card.md) · [model-connection-picker](model-connection-picker.md) · [model-connection-setup](model-connection-setup.md) · [model-picker](model-picker.md) |

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
