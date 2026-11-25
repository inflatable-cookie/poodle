# Poodle typography declaration inventory

Paths below are relative to `packages/core/src/styles/`. Counts include direct declarations and local assignments as separate authored occurrences. CSS values are grouped by spelling after whitespace normalization.

The resolved column uses a 16px root. P is the parent font size and F is the element font size. For local variables it lists possible authored values, independent of selector matching or consumer overrides. Defined tokens show their values when Poodle tokens load. Inline fallback values remain visible in the authored column.

## font-size

| Authored | Resolved value | Direct | Local | Files (uses) |
|---|---|---:|---:|---|
| `.6875rem` | 11px | 2 | 0 | mod-matrix-grid.css ×2 |
| `0` | 0px | 1 | 0 | number-input.css |
| `0.5rem` | 8px | 2 | 3 | callout.css, duration-input.css ×2, message-center.css, sidebar-nav.css |
| `0.9em` | 0.9 × P (14.4px at P = 16px) | 1 | 0 | agent-message.css |
| `0.65rem` | 10.4px | 1 | 0 | history-center.css |
| `0.75rem` | 12px | 94 | 52 | accordion.css ×2, action-discovery-panel.css, agent-chat-input.css ×2, agent-message.css, agent-plan-record.css, agent-plan.css, agent-question-record.css, agent-question.css, agent-subagent.css, agent-transcript.css, app-header.css ×2, avatar.css, block-editor.css, bulk-action-bar.css, button.css, calendar.css, callout.css ×2, card-radio-group.css ×2, card-toggle-group.css ×3, changed-files.css, checkbox.css, code.css, collapsible.css ×2, color-picker.css ×3, command-palette.css, date-picker.css ×2, date-range-picker.css ×2, date-time-picker.css ×2, date-time-range-picker.css ×2, date-time-zone-picker.css ×2, detail-item.css ×3, detail-section.css, dialog.css, drag-number-field.css, duration-input.css, editable-label.css, editable-list.css, embed-input.css, empty-state.css, field.css ×5, file-upload.css, filter-builder.css ×3, list-card.css ×8, log-list.css ×2, markdown-editor.css ×3, media-preview.css ×2, menu-surface.css ×2, menubar.css, meter.css, metric-tile.css ×2, model-picker.css ×2, navigation-menu.css, number-input.css, order-by.css ×4, page-header.css ×4, pagination.css ×2, password-requirements.css, pill.css, rating.css, ref-select.css ×3, relation-picker.css ×2, rich-text.css ×2, segmented-control.css, select.css ×3, selection-summary.css ×7, sidebar-nav.css, slider-family.css ×2, split-button.css, status-bar.css, status-indicator.css, stepper.css ×3, switch.css ×2, table.css ×2, tabs.css, text-input.css ×2, text.css, time-input.css, toast-stack.css, token-input.css, tool-call-group.css, tool-call.css, tree.css, value-readout.css, video-player.css |
| `0.85rem` | 13.6px | 1 | 0 | eyebrow.css |
| `0.625rem` | 10px | 18 | 13 | action-discovery-panel.css, avatar.css, callout.css, card-toggle-group.css ×2, color-picker.css, date-picker.css, date-range-picker.css, date-time-picker.css, date-time-range-picker.css, date-time-zone-picker.css, field.css, filter-builder.css, list-card.css ×4, media-preview.css, menu-surface.css, nav-card.css, order-by.css, page-header.css, pill.css, relation-picker.css, sidebar-nav.css, status-indicator.css, stepper.css ×3, table.css, video-player.css |
| `0.875em` | 0.875 × P (14px at P = 16px) | 2 | 0 | model-picker.css, ref-select.css |
| `0.875rem` | 14px | 67 | 54 | accordion.css ×3, agent-chat-input.css, agent-message.css, agent-plan-record.css, agent-plan.css, agent-question-record.css, agent-question.css ×2, agent-subagent.css, agent-transcript.css, app-header.css ×2, block-editor.css, button.css ×2, calendar.css ×2, card-radio-group.css, card-toggle-group.css ×3, changed-files.css, checkbox.css, code-input.css ×2, code.css, collapsible.css ×2, color-picker.css, data-table.css ×2, date-picker.css, date-range-picker.css, date-time-picker.css, date-time-range-picker.css, date-time-zone-picker.css, dialog.css ×2, drag-number-field.css, editable-list.css ×3, field.css ×2, file-upload.css ×2, filter-builder.css ×3, filter-toolbar.css, icon-button.css, list-card.css ×6, log-list.css, markdown-editor.css ×4, media-picker.css, media-preview.css, media-thumbnail.css ×2, menu-surface.css, menubar.css, meta-item.css, meter.css, metric-tile.css, model-picker.css ×2, navigation-menu.css, order-by.css ×2, page-header.css ×8, pagination.css, password-requirements.css ×3, rating.css, ref-select.css ×2, relation-picker.css, rich-text.css ×6, selection-summary.css ×4, sidebar-nav.css, slider-family.css ×2, split-button.css, status-bar.css, status-indicator.css, stepper.css, switch.css ×2, table.css, tabs.css, text.css, toast-stack.css, tool-call-group.css, tool-call.css, tree.css, value-readout.css |
| `0.5625rem` | 9px | 8 | 7 | action-discovery-panel.css, callout.css, duration-input.css, filter-builder.css, list-card.css ×2, menu-surface.css, order-by.css, pill.css, relation-picker.css, sidebar-nav.css, stepper.css ×2, table.css, video-player.css |
| `0.5786em` | 0.5786 × P (9.2576px at P = 16px) | 0 | 1 | pill.css |
| `0.6429em` | 0.6429 × P (10.2864px at P = 16px) | 0 | 1 | pill.css |
| `0.6875em` | 0.6875 × P (11px at P = 16px) | 0 | 1 | meta-item.css |
| `0.6875rem` | 11px | 70 | 38 | accordion.css, action-discovery-panel.css, agent-plan-record.css, agent-plan.css, agent-question-record.css, agent-question.css, agent-subagent.css, agent-transcript.css, app-header.css, breadcrumbs.css, button.css, calendar.css ×3, callout.css ×2, card-radio-group.css, card-toggle-group.css ×3, changed-files.css, checkbox.css, code.css ×2, collapsible.css, color-picker.css, data-table.css ×2, date-picker.css, date-range-picker.css, date-time-picker.css ×2, date-time-range-picker.css ×2, date-time-zone-picker.css ×2, dialog.css, editable-list.css, eyebrow.css ×2, field-set.css, field.css ×2, file-upload.css, filter-builder.css ×3, filter-toolbar.css, history-center.css ×3, icon-button.css, list-card-counter.css, list-card.css ×7, media-preview.css ×2, media-thumbnail.css, menu-surface.css ×2, menubar.css ×2, meta-item.css, model-picker.css ×4, navigation-menu.css, order-by.css, page-header.css ×3, pagination.css, pill.css, ref-select.css ×2, relation-picker.css ×4, select.css ×2, selection-summary.css ×5, sidebar-nav.css, split-button.css, status-bar.css, status-indicator.css, stepper.css ×2, table.css ×2, tabs.css ×2, toast-stack.css, tool-call-group.css, tool-call.css, tooltip.css, tree.css, video-player.css |
| `0.7071em` | 0.7071 × P (11.3136px at P = 16px) | 0 | 2 | pill.css ×2 |
| `0.7143em` | 0.7143 × P (11.4288px at P = 16px) | 1 | 0 | status-indicator.css |
| `0.7714em` | 0.7714 × P (12.3424px at P = 16px) | 0 | 1 | pill.css |
| `0.7857em` | 0.7857 × P (12.5712px at P = 16px) | 1 | 0 | status-indicator.css |
| `0.8125rem` | 13px | 81 | 58 | accordion.css ×3, action-discovery-panel.css, agent-chat-input.css ×2, agent-plan-record.css ×2, agent-plan.css ×2, agent-question-record.css ×2, agent-question.css ×3, agent-subagent.css ×2, agent-transcript.css ×2, app-header.css ×2, bulk-action-bar.css, button.css, calendar.css ×2, callout.css ×2, card-radio-group.css ×2, card-toggle-group.css ×2, changed-files.css ×2, code-input.css ×2, code.css, collapsible.css ×2, color-picker.css, command-palette.css, data-table.css ×2, date-picker.css ×2, date-range-picker.css ×2, date-time-picker.css ×2, date-time-range-picker.css ×2, date-time-zone-picker.css ×2, detail-section.css, dialog.css ×2, editable-list.css, embed-preview.css ×3, empty-state.css, field.css ×2, file-upload.css, filter-builder.css ×4, filter-toolbar.css, list-card.css ×5, log-list.css ×3, markdown-editor.css ×2, media-browse-panel.css, media-preview.css ×2, media-thumbnail.css, menu-surface.css, model-picker.css ×2, nav-card.css, navigation-menu.css, number-input.css, order-by.css ×3, page-header.css ×6, password-requirements.css, picker-shell.css, pill.css, ref-select.css, relation-picker.css ×4, rich-text.css ×3, select.css ×2, selection-summary.css ×5, sidebar-nav.css, slider-family.css ×2, split-button.css, status-bar.css, status-indicator.css, stepper.css, switch.css, table.css ×3, tabs.css, text-input.css, text.css, theme-select.css, time-input.css, toast-stack.css, token-input.css, tool-call-group.css ×2, tool-call.css ×2, tree.css, video-player.css |
| `0.8357em` | 0.8357 × P (13.3712px at P = 16px) | 0 | 1 | pill.css |
| `0.8571em` | 0.8571 × P (13.7136px at P = 16px) | 2 | 1 | list-card-counter.css, status-indicator.css ×2 |
| `0.9286em` | 0.9286 × P (14.8576px at P = 16px) | 2 | 0 | markdown-editor.css, status-indicator.css |
| `0.9375rem` | 15px | 45 | 40 | accordion.css ×3, agent-chat-input.css, agent-message.css ×2, agent-plan-record.css, agent-plan.css, agent-question-record.css, agent-question.css ×3, agent-subagent.css, agent-transcript.css, app-header.css ×2, bulk-action-bar.css, button.css, calendar.css, callout.css ×2, card-radio-group.css ×2, card-toggle-group.css ×2, changed-files.css, checkbox.css, code-input.css, code.css, collapsible.css, color-picker.css, date-picker.css, date-range-picker.css, date-time-picker.css, date-time-range-picker.css, date-time-zone-picker.css, detail-item.css, dialog.css, duration-input.css, editable-label.css, editable-list.css, empty-state.css ×2, field.css, file-upload.css, filter-builder.css, list-card.css ×2, media-preview.css ×2, media-thumbnail.css, menu-surface.css, menubar.css ×2, model-picker.css, number-input.css, order-by.css, page-header.css ×6, pagination.css, password-requirements.css ×2, ref-select.css, relation-picker.css, rich-text.css, select.css ×2, selection-summary.css, sidebar-nav.css, split-button.css, status-bar.css, table.css, tabs.css, text-input.css, time-input.css, toast-stack.css ×2, token-input.css, tool-call-group.css, tool-call.css, tree.css |
| `0.46875rem` | 7.5px | 0 | 1 | sidebar-nav.css |
| `0.59375rem` | 9.5px | 0 | 1 | sidebar-nav.css |
| `0.65625rem` | 10.5px | 0 | 2 | media-preview.css, page-header.css |
| `0.71875rem` | 11.5px | 5 | 8 | app-header.css, card-toggle-group.css, data-table.css ×2, filter-builder.css, filter-toolbar.css, media-preview.css, page-header.css, selection-summary.css ×3, theme-select.css, toast-stack.css |
| `0.78125rem` | 12.5px | 1 | 2 | breadcrumbs.css, media-preview.css, page-header.css |
| `0.84375rem` | 13.5px | 0 | 1 | page-header.css |
| `1.3em` | 1.3 × P (20.8px at P = 16px) | 1 | 0 | agent-message.css |
| `1.05em` | 1.05 × P (16.8px at P = 16px) | 1 | 0 | agent-message.css |
| `1.5em` | 1.5 × P (24px at P = 16px) | 1 | 0 | agent-message.css |
| `1.5rem` | 24px | 0 | 3 | avatar.css, page-header.css ×2 |
| `1.15em` | 1.15 × P (18.4px at P = 16px) | 1 | 0 | agent-message.css |
| `1.25rem` | 20px | 7 | 4 | code-input.css ×2, drag-number-field.css, media-preview.css, page-header.css ×3, picker-shell.css, rating.css, rich-text.css, value-readout.css |
| `1.75rem` | 28px | 0 | 3 | meter.css, page-header.css ×2 |
| `1.125rem` | 18px | 17 | 15 | accordion.css, agent-message.css, agent-question.css, breadcrumbs.css, callout.css, card-radio-group.css, card-toggle-group.css, code-input.css ×2, collapsible.css, detail-section.css, dialog.css, drag-number-field.css, empty-state.css ×2, list-card.css, markdown-editor.css ×2, media-preview.css ×2, meter.css, page-header.css ×4, password-requirements.css, rating.css, rich-text.css ×2, slider-family.css ×2, value-readout.css |
| `1.375rem` | 22px | 1 | 4 | command-palette.css, meter.css ×2, page-header.css ×2 |
| `1.0625rem` | 17px | 7 | 6 | accordion.css ×2, app-header.css, callout.css ×2, card-radio-group.css, collapsible.css, dialog.css, list-card.css, page-header.css ×3, password-requirements.css |
| `1.0714em` | 1.0714 × P (17.1424px at P = 16px) | 1 | 0 | markdown-editor.css |
| `1.1875rem` | 19px | 0 | 1 | media-preview.css |
| `1.2143em` | 1.2143 × P (19.4288px at P = 16px) | 1 | 0 | markdown-editor.css |
| `1.4286em` | 1.4286 × P (22.8576px at P = 16px) | 1 | 0 | markdown-editor.css |
| `1em` | 1 × P (16px at P = 16px) | 3 | 1 | agent-message.css, menu-surface.css, meta-item.css, status-indicator.css |
| `1rem` | 16px | 35 | 26 | accordion.css ×2, agent-chat-input.css, agent-message.css, agent-question.css, app-header.css, avatar.css, breadcrumbs.css, bulk-action-bar.css, callout.css, card-toggle-group.css, code-input.css ×2, collapsible.css, date-picker.css, date-range-picker.css, date-time-picker.css, date-time-range-picker.css, date-time-zone-picker.css, detail-item.css, dialog.css, drawer.css, duration-input.css, editable-label.css, file-upload.css, filter-builder.css, list-card.css ×3, markdown-editor.css ×2, media-preview.css, menu-surface.css, menubar.css, metric-tile.css, model-picker.css, nav-card.css, number-input.css, order-by.css, page-header.css ×8, password-requirements.css ×3, ref-select.css, rich-text.css ×2, select.css ×2, slider-family.css ×2, text-input.css, time-input.css, toast-stack.css, token-input.css |
| `2.25rem` | 36px | 0 | 2 | meter.css, page-header.css |
| `2rem` | 32px | 0 | 2 | avatar.css, page-header.css |
| `calc(0.75em * var(--poodle-typography-code-adjustmentRatio))` | 0.609375 × P (9.75px at P = 16px) | 1 | 1 | code-editor.css, code.css |
| `calc(0.875em * var(--poodle-typography-code-adjustmentRatio))` | 0.7109375 × P (11.375px at P = 16px) | 1 | 1 | code-editor.css, code.css |
| `calc(0.6875em * var(--poodle-typography-code-adjustmentRatio))` | 0.55859375 × P (8.9375px at P = 16px) | 1 | 1 | code-editor.css, code.css |
| `calc(0.8125em * var(--poodle-typography-code-adjustmentRatio))` | 0.66015625 × P (10.5625px at P = 16px) | 3 | 2 | code-editor.css ×2, code.css, rich-text.css ×2 |
| `calc(0.9375em * var(--poodle-typography-code-adjustmentRatio))` | 0.76171875 × P (12.1875px at P = 16px) | 1 | 0 | code.css |
| `calc(1em * var(--poodle-typography-code-adjustmentRatio, 0.9))` | 0.8125 × P (13px at P = 16px) | 1 | 0 | agent-message.css |
| `calc(1em * var(--poodle-typography-code-adjustmentRatio))` | 0.8125 × P (13px at P = 16px) | 3 | 1 | code-editor.css, code.css, meta-bar.css, text-input.css |
| `calc(var(--poodle-agent-question-font-size) * 0.9)` | 11.7px, 9.9px, 10.8px, 12.6px, 13.5px | 1 | 0 | agent-question.css |
| `calc(var(--poodle-md-prose-font-size) * 0.9286)` | 13.0004px, 11.1432px, 12.0718px, 14.8576px, 16.7148px | 1 | 0 | markdown-editor.css |
| `calc(var(--poodle-meter-ring-size) * 0.34)` | 7.48px, 4.76px, 6.12px, 9.52px, 12.24px | 1 | 0 | meter.css |
| `calc(var(--poodle-sidebar-nav-item-font-size) * 0.85)` | 11.05px, 9.35px, 10.2px, 11.9px, 12.75px | 0 | 1 | sidebar-nav.css |
| `calc(var(--poodle-tree-row-font) * 0.85)` | 11.05px, 9.35px, 10.2px, 11.9px, 12.75px | 0 | 1 | tree.css |
| `calc(var(--poodle-typography-label-size) * 0.75)` | 9.75px | 0 | 1 | sidebar-nav.css |
| `inherit` | inherit | 2 | 0 | field.css, tree.css |
| `var(--poodle-accordion-panel-font-size)` | 15px, 13px, 14px, 16px, 17px | 1 | 0 | accordion.css |
| `var(--poodle-action-discovery-chip-font-size)` | 11px, 9px, 10px, 12px, 13px | 1 | 0 | action-discovery-panel.css |
| `var(--poodle-agent-chat-input-font)` | 14px, 12px, 13px, 15px, 16px | 1 | 0 | agent-chat-input.css |
| `var(--poodle-agent-message-font-size)` | 15px, 12px, 14px, 16px, 18px | 1 | 0 | agent-message.css |
| `var(--poodle-agent-plan-font-size)` | 13px, 11px, 12px, 14px, 15px | 2 | 0 | agent-plan.css ×2 |
| `var(--poodle-agent-plan-record-font-size)` | 13px, 11px, 12px, 14px, 15px | 2 | 0 | agent-plan-record.css ×2 |
| `var(--poodle-agent-question-font-size)` | 13px, 11px, 12px, 14px, 15px | 4 | 0 | agent-question.css ×4 |
| `var(--poodle-agent-question-prompt-size)` | 15px, 13px, 14px, 16px, 18px | 1 | 0 | agent-question.css |
| `var(--poodle-agent-question-record-font-size)` | 13px, 11px, 12px, 14px, 15px | 1 | 0 | agent-question-record.css |
| `var(--poodle-agent-subagent-font-size)` | 13px, 11px, 12px, 14px, 15px | 2 | 0 | agent-subagent.css ×2 |
| `var(--poodle-agent-transcript-font-size)` | 13px, 11px, 12px, 14px, 15px | 2 | 0 | agent-transcript.css ×2 |
| `var(--poodle-app-header-subtitle-size)` | 12px, 11px, 11.5px, 13px, 14px | 1 | 0 | app-header.css |
| `var(--poodle-app-header-title-size)` | 15px, 13px, 14px, 16px, 17px | 1 | 0 | app-header.css |
| `var(--poodle-avatar-font-size)` | 10px, 12px, 16px, 24px, 32px | 1 | 0 | avatar.css |
| `var(--poodle-card-toggle-group-count-font-size)` | 12px, 10px, 11px, 11.5px, 13px, 14px | 1 | 0 | card-toggle-group.css |
| `var(--poodle-changed-files-font-size)` | 13px, 11px, 12px, 14px, 15px | 1 | 0 | changed-files.css |
| `var(--poodle-code-editor-font-size)` | 0.66015625 × P (10.5625px at P = 16px), 0.55859375 × P (8.9375px at P = 16px), 0.609375 × P (9.75px at P = 16px), 0.7109375 × P (11.375px at P = 16px), 0.8125 × P (13px at P = 16px) | 1 | 0 | code-editor.css |
| `var(--poodle-detail-section-title-size)` | 18px | 1 | 0 | detail-section.css |
| `var(--poodle-duration-input-digit-size)` | 14px, 12px, 15px, 16px | 2 | 0 | duration-input.css ×2 |
| `var(--poodle-duration-input-label-size)` | 9px, 8px | 1 | 0 | duration-input.css |
| `var(--poodle-editable-list-font-size)` | 13px, 11px, 12px, 14px, 15px | 1 | 0 | editable-list.css |
| `var(--poodle-file-upload-hint-size)` | 13px, 11px | 1 | 0 | file-upload.css |
| `var(--poodle-file-upload-label-size)` | 14px, 12px, 15px, 16px | 1 | 0 | file-upload.css |
| `var(--poodle-list-card-counter-font-size)` | 11px, 0.8571 × P (13.7136px at P = 16px) | 1 | 0 | list-card-counter.css |
| `var(--poodle-list-card-leading-font-size)` | 14px, 11px, 12px, 16px, 18px, 10px, 13px, 15px | 2 | 0 | list-card.css ×2 |
| `var(--poodle-md-prose-font-size)` | 14px, 12px, 13px, 16px, 18px | 1 | 0 | markdown-editor.css |
| `var(--poodle-media-preview-body-size)` | 13px, 12px, 12.5px, 14px, 15px | 1 | 0 | media-preview.css |
| `var(--poodle-media-preview-eyebrow-size)` | 11px, 10px, 10.5px, 11.5px, 12px | 1 | 0 | media-preview.css |
| `var(--poodle-media-preview-title-size)` | 18px, 15px, 16px, 19px, 20px | 1 | 0 | media-preview.css |
| `var(--poodle-meta-item-label-font-size)` | 11px, 0.6875 × P (11px at P = 16px) | 1 | 0 | meta-item.css |
| `var(--poodle-meta-item-value-font-size)` | 14px, 1 × P (16px at P = 16px) | 1 | 0 | meta-item.css |
| `var(--poodle-page-header-back-size)` | 13px, 12px, 12.5px, 14px, 15px | 1 | 0 | page-header.css |
| `var(--poodle-page-header-body-size)` | 14px, 13px, 15px, 16px | 2 | 0 | page-header.css ×2 |
| `var(--poodle-page-header-eyebrow-size)` | 11px, 10px, 10.5px, 12px, 13px | 1 | 0 | page-header.css |
| `var(--poodle-page-header-section-size)` | 12px, 11px, 11.5px, 13px, 14px | 1 | 0 | page-header.css |
| `var(--poodle-page-header-subtitle-compact-size)` | 16px, 14px, 15px, 17px, 18px | 2 | 0 | page-header.css ×2 |
| `var(--poodle-page-header-subtitle-size)` | 16px, 14px, 15px, 17px, 18px | 1 | 0 | page-header.css |
| `var(--poodle-page-header-title-compact-size)` | 20px, 17px, 18px, 22px, 24px | 1 | 0 | page-header.css |
| `var(--poodle-page-header-title-size)` | 28px, 22px, 24px, 32px, 36px | 1 | 0 | page-header.css |
| `var(--poodle-password-requirements-body-size)` | 14px, 12px, 13px, 15px, 16px | 2 | 0 | password-requirements.css ×2 |
| `var(--poodle-password-requirements-title-size)` | 16px, 14px, 15px, 17px, 18px | 1 | 0 | password-requirements.css |
| `var(--poodle-pill-font-size)` | 11px, 10px, 9px, 12px, 13px, 0.7071 × P (11.3136px at P = 16px), 0.5786 × P (9.2576px at P = 16px), 0.6429 × P (10.2864px at P = 16px), 0.7714 × P (12.3424px at P = 16px), 0.8357 × P (13.3712px at P = 16px) | 1 | 0 | pill.css |
| `var(--poodle-relation-picker-desc-size)` | 11px, 9px, 10px, 12px, 13px | 2 | 0 | relation-picker.css ×2 |
| `var(--poodle-relation-picker-title-size)` | 13px, 11px, 12px, 14px, 15px | 1 | 0 | relation-picker.css |
| `var(--poodle-rich-text-font-size)` | 14px, 12px, 13px, 16px, 18px | 2 | 0 | rich-text.css ×2 |
| `var(--poodle-selection-summary-chip-font-size)` | 12px, 11px, 11.5px, 13px, 14px | 2 | 0 | filter-builder.css, selection-summary.css |
| `var(--poodle-selection-summary-clear-font-size)` | 12px, 11px, 11.5px, 13px, 14px | 1 | 0 | selection-summary.css |
| `var(--poodle-selection-summary-font-size)` | 12px, 11px, 11.5px, 13px, 14px | 1 | 0 | selection-summary.css |
| `var(--poodle-selection-summary-overflow-font-size)` | 13px, 11px, 12px, 14px, 15px | 1 | 0 | selection-summary.css |
| `var(--poodle-sidebar-nav-end-label-font-size)` | 11.05px, 9.35px, 10.2px, 11.9px, 12.75px | 1 | 0 | sidebar-nav.css |
| `var(--poodle-sidebar-nav-item-font-size)` | 13px, 11px, 12px, 14px, 15px | 1 | 0 | sidebar-nav.css |
| `var(--poodle-sidebar-nav-title-font-size)` | 9.75px, 7.5px, 8px, 9px, 9.5px, 10px | 1 | 0 | sidebar-nav.css |
| `var(--poodle-split-button-font-size)` | 13px, 12px, 11px, 14px, 15px | 1 | 0 | split-button.css |
| `var(--poodle-status-label-font-size)` | 12px, 10px, 11px, 13px, 14px | 1 | 0 | status-indicator.css |
| `var(--poodle-stepper-font-size)` | 12px, 10px, 11px, 13px, 14px | 2 | 0 | stepper.css ×2 |
| `var(--poodle-stepper-marker-font-size)` | 10px, 9px, 11px, 12px | 1 | 0 | stepper.css |
| `var(--poodle-tabs-label-size)` | 13px, 12px, 14px, 11px, 15px | 1 | 0 | tabs.css |
| `var(--poodle-text-input-font-size)` | 14px, 12px, 13px, 15px, 16px | 2 | 0 | text-input.css ×2 |
| `var(--poodle-token-input-font-size)` | 14px, 12px, 13px, 15px, 16px | 1 | 0 | token-input.css |
| `var(--poodle-tool-call-font-size)` | 13px, 11px, 12px, 14px, 15px | 1 | 0 | tool-call.css |
| `var(--poodle-tool-call-group-font-size)` | 13px, 11px, 12px, 14px, 15px | 1 | 0 | tool-call-group.css |
| `var(--poodle-tree-chevron-size)` | 11.05px, 9.35px, 10.2px, 11.9px, 12.75px | 1 | 0 | tree.css |
| `var(--poodle-tree-row-font)` | 13px, 11px, 12px, 14px, 15px | 1 | 0 | tree.css |
| `var(--poodle-typography-body-size, 0.875rem)` | 14px | 3 | 0 | field-set.css, form-dialog.css, form-layout.css |
| `var(--poodle-typography-body-size)` | 14px | 44 | 5 | breadcrumbs.css, bulk-action-bar.css, data-table.css ×2, date-picker.css, date-range-picker.css, date-time-picker.css, date-time-range-picker.css, date-time-zone-picker.css, detail-item.css, detail-section.css, detail-shell.css, duration-input.css, history-center.css, inline-list-section.css, licence.css ×3, list-card.css ×2, media-browse-panel.css, menu-surface.css, menubar.css, message-center.css ×2, model-connection.css ×4, number-input.css, page-header.css ×2, pagination.css ×3, relation-picker.css, remediation-banner.css, select.css ×5, text-input.css, time-ago.css, time-input.css, token-input.css, update-center.css ×2, validation-summary.css |
| `var(--poodle-typography-caption-size)` | 11px | 2 | 0 | licence.css ×2 |
| `var(--poodle-typography-font-size-sm, 0.8125rem)` | 13px | 1 | 0 | drag-drop.css |
| `var(--poodle-typography-heading-size)` | 16px | 1 | 0 | state-tile.css |
| `var(--poodle-typography-label-size, 0.75rem)` | 13px | 4 | 0 | filter-toolbar.css, form-layout.css, pagination-summary.css, region.css |
| `var(--poodle-typography-label-size, 0.8125rem)` | 13px | 2 | 0 | page-loading.css ×2 |
| `var(--poodle-typography-label-size)` | 13px | 63 | 4 | audio-player.css ×2, button.css, callout.css, checkbox.css, command-palette.css, data-table.css ×7, detail-item.css, editable-label.css, editable-list.css, field.css, history-center.css ×8, inline-list-section.css ×2, licence.css ×4, log-list.css ×2, media-picker.css, menubar.css, message-center.css ×2, model-connection.css ×9, nav-card.css, radio-group.css, radio.css, relation-picker.css ×4, remediation-banner.css, sidebar-nav.css, split-button.css ×2, state-tile.css ×2, switch.css, tabs.css, toggle-group.css, tree.css, tri-state-switch.css, update-center.css ×2, validation-summary.css |

## font-weight

| Authored | Resolved value | Direct | Local | Files (uses) |
|---|---|---:|---:|---|
| `400` | 400 | 3 | 0 | message-center.css, model-picker.css, ref-select.css |
| `500` | 500 | 13 | 0 | calendar.css, list-card.css, model-picker.css, ref-select.css, relation-picker.css ×3, sidebar-nav.css ×2, table.css, text.css, tree.css ×2 |
| `600` | 600 | 54 | 0 | action-discovery-panel.css, avatar.css, calendar.css ×2, card-radio-group.css, card-toggle-group.css, code-input.css, date-time-picker.css, date-time-range-picker.css, date-time-zone-picker.css, detail-item.css, duration-input.css, empty-state.css, eyebrow.css, field-set.css, filter-builder.css, form-layout.css, list-card.css ×2, log-list.css, markdown-editor.css ×3, media-browse-panel.css, media-preview.css, media-thumbnail.css, model-picker.css ×2, nav-card.css ×2, navigation-menu.css, page-header.css ×2, pagination.css ×2, password-requirements.css, pill.css, ref-select.css, region.css, relation-picker.css, rich-text.css ×5, segmented-control.css, sidebar-nav.css, status-indicator.css ×2, table.css ×2, text.css, tree.css, validation-summary.css |
| `650` | 650 | 1 | 0 | detail-section.css |
| `700` | 700 | 16 | 1 | accordion.css, callout.css, card-toggle-group.css, collapsible.css, detail-section.css, list-card.css, log-list.css ×2, markdown-editor.css ×2, message-center.css, page-header.css ×2, pill.css, sidebar-nav.css, stepper.css, text.css |
| `var(--poodle-detail-section-title-weight)` | 700 | 1 | 0 | detail-section.css |
| `var(--poodle-typography-body-weight)` | 400 | 4 | 0 | model-connection.css, select.css, text-input.css ×2 |
| `var(--poodle-typography-heading-weight)` | 600 | 3 | 0 | select.css ×2, toggle-group.css |
| `var(--poodle-typography-label-weight)` | 500 | 49 | 0 | agent-message.css, agent-plan-record.css, agent-plan.css ×2, agent-question.css, agent-subagent.css ×2, button.css, changed-files.css, checkbox.css, code.css, color-picker.css, data-table.css, editable-label.css, field.css, filter-builder.css ×2, history-center.css ×2, inline-list-section.css ×2, licence.css ×3, menubar.css, message-center.css ×2, meta-item.css, model-connection.css ×6, model-picker.css ×2, order-by.css ×2, radio-group.css, radio.css, ref-select.css, select.css, split-button.css, switch.css, tabs.css, tool-call.css, tri-state-switch.css, update-center.css ×2 |

## line-height

| Authored | Resolved value | Direct | Local | Files (uses) |
|---|---|---:|---:|---|
| `0` | 0 × F | 1 | 0 | model-connection.css |
| `1` | 1 × F | 39 | 0 | accordion.css, avatar.css, block-editor.css, button.css ×2, callout.css, checkbox.css, code-input.css ×2, collapse-toggle.css, collapsible.css, data-table.css, duration-input.css ×3, filter-builder.css, icon-button.css, inline-list-section.css, list-card.css, log-list.css, markdown-editor.css, menu-surface.css, menubar.css, message-center.css, meta-item.css, meter.css, metric-tile.css, model-picker.css, navigation-menu.css, pagination.css, pill.css ×2, ref-select.css, rich-text.css, select.css ×2, slider-family.css, split-button.css, stepper.css |
| `1.1` | 1.1 × F | 1 | 0 | page-header.css |
| `1.2` | 1.2 × F | 17 | 2 | accordion.css, app-header.css ×2, calendar.css, collapsible.css, command-palette.css, detail-section.css ×2, dialog.css, drawer.css, empty-state.css, media-preview.css, page-header.css ×2, picker-shell.css, rich-text.css ×2, sidebar-nav.css, theme-select.css |
| `1.3` | 1.3 × F | 6 | 1 | agent-message.css, model-picker.css, ref-select.css, sidebar-nav.css, status-indicator.css, tree.css ×2 |
| `1.4` | 1.4 × F | 13 | 1 | calendar.css, code-editor.css, code.css, command-palette.css, detail-item.css, history-center.css, message-center.css, meta-bar.css, meta-item.css, model-picker.css, nav-card.css, page-loading.css, password-requirements.css, relation-picker.css |
| `1.5` | 1.5 × F | 24 | 3 | agent-chat-input.css, alert-dialog.css, callout.css ×2, card-radio-group.css, card-toggle-group.css, code-editor.css ×2, code.css, command-palette.css, detail-item.css, dialog.css, empty-state.css, eyebrow.css, field-set.css, field.css, list-card.css, media-preview.css, media-thumbnail.css, password-requirements.css ×2, picker-shell.css, relation-picker.css, status-bar.css, table.css, text.css, toast-stack.css |
| `1.5rem` | 24px | 0 | 2 | segmented-control.css, selection-summary.css |
| `1.6` | 1.6 × F | 6 | 3 | block-editor.css, code-editor.css, markdown-editor.css ×2, password-requirements.css ×2, rich-text.css ×2, text.css |
| `1.7` | 1.7 × F | 0 | 1 | password-requirements.css |
| `1.15` | 1.15 × F | 2 | 0 | page-header.css ×2 |
| `1.25` | 1.25 × F | 1 | 0 | card-toggle-group.css |
| `1.25rem` | 20px | 1 | 0 | rich-text.css |
| `1.35` | 1.35 × F | 9 | 1 | code-editor.css, detail-item.css ×2, detail-section.css, icon-button.css, list-card.css, media-thumbnail.css ×2, model-picker.css, tooltip.css |
| `1.45` | 1.45 × F | 4 | 0 | accordion.css, collapsible.css, log-list.css, media-browse-panel.css |
| `1.55` | 1.55 × F | 0 | 2 | code-editor.css, password-requirements.css |
| `1.65` | 1.65 × F | 0 | 1 | password-requirements.css |
| `1.75rem` | 28px | 0 | 1 | segmented-control.css |
| `1.625rem` | 26px | 0 | 1 | selection-summary.css |
| `2.25rem` | 36px | 0 | 2 | segmented-control.css, selection-summary.css |
| `2.75rem` | 44px | 0 | 1 | segmented-control.css |
| `2.125rem` | 34px | 0 | 1 | selection-summary.css |
| `2rem` | 32px | 0 | 2 | selection-summary.css ×2 |
| `3.25rem` | 52px | 0 | 1 | segmented-control.css |
| `calc(var(--poodle-segmented-control-height) - 0.25rem)` | 32px, 20px, 24px, 40px, 48px | 1 | 0 | segmented-control.css |
| `inherit` | inherit | 5 | 0 | code.css, list-card-counter.css, meta-bar.css, meta-item.css, text-link.css |
| `var(--poodle-code-editor-line-height)` | 1.5 × F, 1.35 × F, 1.4 × F, 1.55 × F, 1.6 × F | 1 | 0 | code-editor.css |
| `var(--poodle-detail-section-title-line-height)` | 1.2 × F | 1 | 0 | detail-section.css |
| `var(--poodle-media-preview-title-line-height)` | 1.2 × F | 1 | 0 | media-preview.css |
| `var(--poodle-password-requirements-list-line-height)` | 1.6 × F, 1.5 × F, 1.55 × F, 1.65 × F, 1.7 × F | 1 | 0 | password-requirements.css |
| `var(--poodle-selection-summary-overflow-line-height)` | 32px, 24px, 26px, 34px, 36px | 1 | 0 | selection-summary.css |
| `var(--poodle-size-control-height)` | 36px, 24px, 28px, 44px, 52px | 0 | 1 | segmented-control.css |
| `var(--poodle-status-label-line-height)` | 1.3 × F | 2 | 0 | status-indicator.css ×2 |
| `var(--poodle-text-input-line-height)` | 20px | 2 | 0 | text-input.css ×2 |
| `var(--poodle-token-input-line-height)` | 20px | 1 | 0 | token-input.css |
| `var(--poodle-typography-body-lineHeight, 1.5)` | 20px | 3 | 0 | field-set.css, form-dialog.css, form-layout.css |
| `var(--poodle-typography-body-lineHeight)` | 20px | 41 | 2 | accordion.css, agent-message.css, agent-question.css ×2, block-editor.css, breadcrumbs.css, bulk-action-bar.css, data-table.css ×2, date-picker.css, date-range-picker.css, date-time-picker.css, date-time-range-picker.css, date-time-zone-picker.css, detail-item.css, detail-section.css, detail-shell.css, field.css ×2, history-center.css, licence.css ×3, message-center.css ×2, model-connection.css ×4, number-input.css, page-header.css ×2, pagination.css ×2, select.css ×3, text-input.css ×2, time-input.css, token-input.css, update-center.css ×2 |
| `var(--poodle-typography-caption-lineHeight)` | 16px | 2 | 0 | licence.css ×2 |
| `var(--poodle-typography-caption-lineHeightRatio)` | 1.35 × F | 1 | 0 | tabs.css |
| `var(--poodle-typography-heading-lineHeight)` | 24px | 1 | 0 | state-tile.css |
| `var(--poodle-typography-label-lineHeight, 1.4)` | 16px | 2 | 0 | filter-toolbar.css, pagination-summary.css |
| `var(--poodle-typography-label-lineHeight)` | 16px | 35 | 0 | callout.css, checkbox.css, data-table.css, detail-item.css, editable-label.css, field.css, history-center.css ×8, licence.css ×4, message-center.css, model-connection.css ×7, radio-group.css, radio.css, remediation-banner.css, state-tile.css, switch.css, tri-state-switch.css, update-center.css ×2, validation-summary.css |
| `var(--poodle-typography-label-lineHeightTight)` | 1 × F | 2 | 0 | tabs.css, toggle-group.css |

## letter-spacing

| Authored | Resolved value | Direct | Local | Files (uses) |
|---|---|---:|---:|---|
| `0` | 0px | 1 | 0 | action-discovery-panel.css |
| `0.01em` | 0.01 × F | 1 | 0 | split-button.css |
| `0.02em` | 0.02 × F | 2 | 0 | calendar.css, pill.css |
| `0.2em` | 0.2 × F | 0 | 1 | sidebar-nav.css |
| `0.03em` | 0.03 × F | 1 | 0 | action-discovery-panel.css |
| `0.04em` | 0.04 × F | 10 | 0 | calendar.css, color-picker.css, date-time-picker.css, date-time-range-picker.css, date-time-zone-picker.css, detail-section.css, eyebrow.css, media-thumbnail.css, pill.css, table.css |
| `0.05em` | 0.05 × F | 10 | 0 | code.css, duration-input.css, filter-builder.css, inline-list-section.css, model-picker.css ×2, nav-card.css, order-by.css, ref-select.css, region.css |
| `0.08em` | 0.08 × F | 3 | 0 | meta-item.css, page-header.css, relation-picker.css |
| `0.12em` | 0.12 × F | 5 | 0 | eyebrow.css, field-set.css, list-card.css, media-preview.css, page-header.css |
| `0.16em` | 0.16 × F | 0 | 1 | sidebar-nav.css |
| `0.18em` | 0.18 × F | 0 | 2 | sidebar-nav.css ×2 |
| `inherit` | inherit | 2 | 0 | calendar.css ×2 |
| `var(--poodle-sidebar-nav-title-letter-spacing)` | 0.18 × F, 0.2 × F, 0.16 × F | 1 | 0 | sidebar-nav.css |
| `var(--poodle-typography-label-letterSpacing)` | 0.01 × F | 1 | 0 | button.css |
| `var(--poodle-typography-overline-letterSpacing)` | 0.04 × F | 1 | 0 | select.css |

## text-transform

| Authored | Resolved value | Direct | Local | Files (uses) |
|---|---|---:|---:|---|
| `none` | none | 1 | 0 | action-discovery-panel.css |
| `uppercase` | uppercase | 31 | 0 | action-discovery-panel.css, calendar.css, code.css, color-picker.css, date-time-picker.css, date-time-range-picker.css, date-time-zone-picker.css, detail-section.css, duration-input.css, eyebrow.css, field-set.css, filter-builder.css, inline-list-section.css, list-card.css ×2, media-preview.css, media-thumbnail.css, meta-item.css, model-picker.css ×2, nav-card.css, order-by.css, page-header.css ×2, pill.css, ref-select.css, region.css, relation-picker.css, select.css, sidebar-nav.css, table.css |

## font-family

| Authored | Resolved value | Direct | Local | Files (uses) |
|---|---|---:|---:|---|
| `inherit` | inherit | 6 | 0 | model-picker.css ×3, ref-select.css ×2, tree.css |
| `var(--poodle-typography-body-family)` | "Inter", "SF Pro Display", system-ui, sans-serif | 44 | 0 | agent-chat-input.css, block-editor.css, bulk-action-bar.css, data-table.css ×2, date-picker.css, date-range-picker.css, date-time-picker.css, date-time-range-picker.css, date-time-zone-picker.css, detail-item.css ×2, field.css ×3, history-center.css, licence.css ×3, list-card.css, markdown-editor.css, message-center.css, meta-item.css, model-connection.css ×4, number-input.css, pagination-summary.css, pagination.css ×3, rich-text.css ×2, select.css ×3, table.css, text-input.css ×3, time-ago.css, time-input.css, update-center.css |
| `var(--poodle-typography-code-family)` | "IBM Plex Mono", "SFMono-Regular", monospace | 44 | 0 | accordion.css, action-discovery-panel.css, agent-message.css, agent-question.css, audio-player.css ×2, button.css, callout.css, changed-files.css ×4, code-editor.css, code-input.css ×2, code.css ×2, collapsible.css, color-picker.css ×3, command-palette.css, duration-input.css ×2, icon-button.css, list-card.css, log-list.css ×3, markdown-editor.css ×3, menu-surface.css, menubar.css, metric-tile.css ×2, pill.css, ref-select.css, rich-text.css ×3, text-input.css, tool-call.css, video-player.css |
| `var(--poodle-typography-heading-family)` | "Inter", "SF Pro Display", system-ui, sans-serif | 18 | 0 | accordion.css, agent-message.css, collapsible.css, detail-section.css, dialog.css, drawer.css, history-center.css, licence.css ×3, message-center.css, model-connection.css ×3, page-header.css ×2, state-tile.css, update-center.css |
| `var(--poodle-typography-label-family, inherit)` | "Inter", "SF Pro Display", system-ui, sans-serif | 1 | 0 | drag-drop.css |
| `var(--poodle-typography-label-family, sans-serif)` | "Inter", "SF Pro Display", system-ui, sans-serif | 1 | 0 | region.css |
| `var(--poodle-typography-label-family)` | "Inter", "SF Pro Display", system-ui, sans-serif | 52 | 0 | action-discovery-panel.css, button.css, calendar.css ×3, callout.css, checkbox.css, code.css, color-picker.css, data-table.css, date-time-picker.css, date-time-range-picker.css, date-time-zone-picker.css, detail-item.css, editable-label.css, eyebrow.css, field-set.css, field.css, filter-builder.css, list-card.css, log-list.css, menubar.css, message-center.css, meta-item.css, model-picker.css ×2, nav-card.css, navigation-menu.css, order-by.css ×2, pagination.css ×2, pill.css, radio-group.css, radio.css, ref-select.css, remediation-banner.css, segmented-control.css, select.css, sidebar-nav.css ×2, split-button.css, state-tile.css, status-indicator.css, switch.css, table.css ×2, tabs.css, toggle-group.css, tree.css, tri-state-switch.css, validation-summary.css |
