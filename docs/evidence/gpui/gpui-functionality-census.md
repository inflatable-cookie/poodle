# g18.001 — Contract-bound GPUI functionality census

Evidence commit (execution identity from the execution record, not the checkout): `a720b95b3982748ccce9b1a7924be6aacacfde62`
Denominator: **176** public / **175** portable; `MeterSurface` is the single contract-approved non-portable row.

<!-- g18-census-method -->
## Method

Capability axes are closed: `semantic`, `events`, `pointer`, `keyboard_focus`, `accessibility`, `visual`.
Each portable row requires the axes its contract declares; `not-applicable` needs an exact contract section and can never cite platform state.
Admitted capabilities trace to validated Nucleus M1/A1/V1 receipts or to retained expected tests that ran green on the recorded source and dependency identity, mount the production renderer plus GPUI node backend, and show the claimed axis signals in their bodies.
Construction is not functional completion. A passing route, a test name, or one passing test never marks a component complete.

<!-- g18-census-summary -->
## Summary

Rows with at least one admitted capability: **73**/175.
Fully admitted rows: **24**/175.
Missing by axis: semantic 102; events 101; pointer 112; keyboard_focus 97; accessibility 123; visual 144.
Accessibility platform holds (A2, narrow): **175**. Refusals recorded: **224**.

## Rows

| Component | Required | Admitted | Missing | Holds | Receipts |
| --- | --- | --- | --- | --- | --- |
| Accordion | semantic; events; pointer; keyboard_focus; accessibility; visual | semantic (expected-test); events (expected-test); pointer (expected-test); keyboard_focus (expected-test); accessibility (expected-test) | visual | accessibility (A2-platform-hold) | `docs/evidence/gpui/mounted-receipts/Accordion--accordion-result-disclosure-focus-identity-and-disabled-paths.json` |
| AgentChatInput | semantic; events; pointer; keyboard_focus; accessibility; visual | semantic (nucleus-m1); events (nucleus-m1); pointer (nucleus-m1); keyboard_focus (nucleus-m1); accessibility (nucleus-a1); visual (nucleus-m1) | — | accessibility (A2-platform-hold) | — |
| AudioPlayer | semantic; events; pointer; keyboard_focus; accessibility; visual | — | semantic; events; pointer; keyboard_focus; accessibility; visual | accessibility (A2-platform-hold) | — |
| AlertDialog | semantic; events; pointer; keyboard_focus; accessibility; visual | — | semantic; events; pointer; keyboard_focus; accessibility; visual | accessibility (A2-platform-hold) | — |
| Avatar | semantic; events; pointer; keyboard_focus; accessibility; visual | — | semantic; events; pointer; keyboard_focus; accessibility; visual | accessibility (A2-platform-hold) | — |
| Box | semantic; accessibility; visual | — | semantic; accessibility; visual | accessibility (A2-platform-hold) | — |
| Breadcrumbs | semantic; events; pointer; keyboard_focus; accessibility; visual | semantic (expected-test); pointer (expected-test); keyboard_focus (expected-test); accessibility (expected-test) | events; visual | accessibility (A2-platform-hold) | `docs/evidence/gpui/mounted-receipts/Breadcrumbs--breadcrumbs-callback-navigation-through-mounted-pointer-and-keyboard.json` |
| BulkActionBar | semantic; events; pointer; keyboard_focus; accessibility; visual | — | semantic; events; pointer; keyboard_focus; accessibility; visual | accessibility (A2-platform-hold) | — |
| Button | semantic; events; pointer; keyboard_focus; accessibility; visual | semantic (nucleus-m1); pointer (nucleus-m1); accessibility (nucleus-a1); visual (nucleus-v1) | events; keyboard_focus | accessibility (A2-platform-hold) | `docs/evidence/gpui/mounted-receipts/Button--a-mounted-button-carries-its-controls-target.json` |
| Callout | semantic; events; pointer; keyboard_focus; accessibility; visual | semantic (nucleus-m1); events (nucleus-m1); pointer (nucleus-m1); keyboard_focus (nucleus-m1); accessibility (nucleus-m1); visual (nucleus-m1) | — | accessibility (A2-platform-hold) | `docs/evidence/gpui/mounted-receipts/Callout--callout-dismiss-rebuilds-the-host-spec-through-mounted-input.json` |
| RemediationBanner | semantic; events; pointer; keyboard_focus; accessibility; visual | semantic (expected-test); keyboard_focus (expected-test) | events; pointer; accessibility; visual | accessibility (A2-platform-hold) | `docs/evidence/gpui/mounted-receipts/RemediationBanner--remediation-banner-action-and-dismiss-rebuild-the-host-spec.json` |
| Card | semantic; pointer; keyboard_focus; accessibility; visual | — | semantic; pointer; keyboard_focus; accessibility; visual | accessibility (A2-platform-hold) | — |
| Code | semantic; pointer; keyboard_focus; accessibility; visual | — | semantic; pointer; keyboard_focus; accessibility; visual | accessibility (A2-platform-hold) | — |
| ColorPicker | semantic; events; pointer; keyboard_focus; accessibility; visual | — | semantic; events; pointer; keyboard_focus; accessibility; visual | accessibility (A2-platform-hold) | — |
| Checkbox | semantic; events; pointer; keyboard_focus; accessibility; visual | semantic (expected-test); events (expected-test); pointer (expected-test); keyboard_focus (expected-test); accessibility (expected-test) | visual | accessibility (A2-platform-hold) | `docs/evidence/gpui/mounted-receipts/Checkbox--checkbox-toggle-readonly-and-disabled-rebuild-the-host-spec.json` |
| Calendar | semantic; events; pointer; keyboard_focus; accessibility; visual | — | semantic; events; pointer; keyboard_focus; accessibility; visual | accessibility (A2-platform-hold) | — |
| ContextMenu | semantic; events; pointer; keyboard_focus; accessibility; visual | — | semantic; events; pointer; keyboard_focus; accessibility; visual | accessibility (A2-platform-hold) | — |
| CollapseToggle | semantic; events; pointer; keyboard_focus; accessibility; visual | semantic (expected-test); pointer (expected-test); keyboard_focus (expected-test); accessibility (expected-test) | events; visual | accessibility (A2-platform-hold) | `docs/evidence/gpui/mounted-receipts/CollapseToggle--collapse-toggle-disclosure-focus-and-disabled-through-mounted-pointer-and-keyboard.json` |
| Collapsible | semantic; events; pointer; keyboard_focus; accessibility; visual | semantic (expected-test); pointer (expected-test); keyboard_focus (expected-test); accessibility (expected-test) | events; visual | accessibility (A2-platform-hold) | `docs/evidence/gpui/mounted-receipts/Collapsible--collapsible-disclosure-and-identity-through-mounted-pointer-and-keyboard.json` |
| DetailItem | semantic; events; pointer; keyboard_focus; accessibility; visual | semantic (nucleus-m1); events (nucleus-m1); pointer (nucleus-m1); keyboard_focus (nucleus-m1); accessibility (nucleus-m1); visual (nucleus-m1) | — | accessibility (A2-platform-hold) | — |
| DatePicker | semantic; events; pointer; keyboard_focus; accessibility; visual | — | semantic; events; pointer; keyboard_focus; accessibility; visual | accessibility (A2-platform-hold) | — |
| DateRangePicker | semantic; events; pointer; keyboard_focus; accessibility; visual | — | semantic; events; pointer; keyboard_focus; accessibility; visual | accessibility (A2-platform-hold) | — |
| DateTimePicker | semantic; events; pointer; keyboard_focus; accessibility; visual | — | semantic; events; pointer; keyboard_focus; accessibility; visual | accessibility (A2-platform-hold) | — |
| DateTimeRangePicker | semantic; events; pointer; keyboard_focus; accessibility; visual | — | semantic; events; pointer; keyboard_focus; accessibility; visual | accessibility (A2-platform-hold) | — |
| Dialog | semantic; events; pointer; keyboard_focus; accessibility; visual | semantic (nucleus-m1); events (nucleus-m1); pointer (nucleus-m1); keyboard_focus (nucleus-m1); accessibility (nucleus-m1); visual (nucleus-m1) | — | accessibility (A2-platform-hold) | — |
| Drawer | semantic; events; pointer; keyboard_focus; accessibility; visual | — | semantic; events; pointer; keyboard_focus; accessibility; visual | accessibility (A2-platform-hold) | — |
| DurationInput | semantic; events; pointer; keyboard_focus; accessibility; visual | semantic (expected-test); events (expected-test); keyboard_focus (expected-test) | pointer; accessibility; visual | accessibility (A2-platform-hold) | `docs/evidence/gpui/mounted-receipts/DurationInput--duration-input-segments-edit-and-rebuild-the-host-spec.json` |
| EditableLabel | semantic; events; pointer; keyboard_focus; accessibility; visual | semantic (nucleus-m1); events (nucleus-m1); pointer (nucleus-m1); keyboard_focus (nucleus-m1); accessibility (nucleus-m1); visual (nucleus-m1) | — | accessibility (A2-platform-hold) | `docs/evidence/gpui/mounted-receipts/EditableLabel--editable-label-commits-on-enter-and-once-through-the-blur-tab-causes.json`; `docs/evidence/gpui/mounted-receipts/EditableLabel--editable-label-live-draft-stays-off-the-committed-value.json` |
| Eyebrow | semantic; accessibility; visual | — | semantic; accessibility; visual | accessibility (A2-platform-hold) | — |
| Field | semantic; pointer; keyboard_focus; accessibility; visual | — | semantic; pointer; keyboard_focus; accessibility; visual | accessibility (A2-platform-hold) | — |
| FieldSet | semantic; events; pointer; keyboard_focus; accessibility; visual | — | semantic; events; pointer; keyboard_focus; accessibility; visual | accessibility (A2-platform-hold) | — |
| FileUpload | semantic; events; pointer; keyboard_focus; accessibility; visual | semantic (expected-test); events (expected-test); pointer (expected-test) | keyboard_focus; accessibility; visual | accessibility (A2-platform-hold) | `docs/evidence/gpui/mounted-receipts/FileUpload--a-dropzone-browse-flows-fixture-bytes-through-the-generic-seam.json` |
| FilterBuilder | semantic; events; pointer; keyboard_focus; accessibility; visual | — | semantic; events; pointer; keyboard_focus; accessibility; visual | accessibility (A2-platform-hold) | — |
| FormActions | semantic; pointer; keyboard_focus; accessibility; visual | — | semantic; pointer; keyboard_focus; accessibility; visual | accessibility (A2-platform-hold) | — |
| Grid | semantic; accessibility; visual | — | semantic; accessibility; visual | accessibility (A2-platform-hold) | — |
| HoverCard | semantic; events; pointer; keyboard_focus; accessibility; visual | — | semantic; events; pointer; keyboard_focus; accessibility; visual | accessibility (A2-platform-hold) | — |
| Icon | semantic; pointer; keyboard_focus; accessibility; visual | semantic (nucleus-m1); pointer (nucleus-m1); accessibility (nucleus-m1); visual (nucleus-m1) | keyboard_focus | accessibility (A2-platform-hold) | — |
| IconButton | semantic; events; pointer; keyboard_focus; accessibility; visual | semantic (nucleus-m1); events (nucleus-m1); pointer (nucleus-m1); keyboard_focus (nucleus-m1); accessibility (nucleus-a1); visual (nucleus-m1) | — | accessibility (A2-platform-hold) | `docs/evidence/gpui/mounted-receipts/IconButton--icon-button-activation-toggle-and-tooltip-through-mounted-pointer-and-keyboard.json` |
| IconProvider | semantic; pointer; keyboard_focus; accessibility; visual | — | semantic; pointer; keyboard_focus; accessibility; visual | accessibility (A2-platform-hold) | — |
| Meter | semantic; accessibility; visual | — | semantic; accessibility; visual | accessibility (A2-platform-hold) | — |
| ListCard | semantic; events; pointer; keyboard_focus; accessibility; visual | — | semantic; events; pointer; keyboard_focus; accessibility; visual | accessibility (A2-platform-hold) | — |
| ListCardCounter | semantic; events; pointer; keyboard_focus; accessibility; visual | — | semantic; events; pointer; keyboard_focus; accessibility; visual | accessibility (A2-platform-hold) | — |
| ListGrid | semantic; events; pointer; keyboard_focus; accessibility; visual | — | semantic; events; pointer; keyboard_focus; accessibility; visual | accessibility (A2-platform-hold) | — |
| Menu | semantic; events; pointer; keyboard_focus; accessibility; visual | semantic (nucleus-m1); events (nucleus-m1); pointer (nucleus-m1); keyboard_focus (nucleus-m1); accessibility (nucleus-m1); visual (nucleus-m1) | — | accessibility (A2-platform-hold) | — |
| MetaBar | semantic; events; pointer; keyboard_focus; accessibility; visual | — | semantic; events; pointer; keyboard_focus; accessibility; visual | accessibility (A2-platform-hold) | — |
| MetaItem | semantic; events; pointer; keyboard_focus; accessibility; visual | — | semantic; events; pointer; keyboard_focus; accessibility; visual | accessibility (A2-platform-hold) | — |
| NumberInput | semantic; events; pointer; keyboard_focus; accessibility; visual | semantic (expected-test); events (expected-test); keyboard_focus (expected-test); accessibility (expected-test) | pointer; visual | accessibility (A2-platform-hold) | `docs/evidence/gpui/mounted-receipts/NumberInput--number-input-mounted-valid-direct-editing-rebuilds-host-draft-and-value.json`; `docs/evidence/gpui/mounted-receipts/NumberInput--number-input-mounted-accessibility-projects-spin-button-surface.json` |
| OrderBy | semantic; events; pointer; keyboard_focus; accessibility; visual | semantic (expected-test); events (expected-test); pointer (expected-test); keyboard_focus (expected-test); accessibility (expected-test) | visual | accessibility (A2-platform-hold) | `docs/evidence/gpui/mounted-receipts/OrderBy--order-by-substrate-reorder-and-alt-arrow-rebuild-the-host-spec.json` |
| NavCard | semantic; events; pointer; keyboard_focus; accessibility; visual | — | semantic; events; pointer; keyboard_focus; accessibility; visual | accessibility (A2-platform-hold) | — |
| NavigationMenu | semantic; events; pointer; keyboard_focus; accessibility; visual | — | semantic; events; pointer; keyboard_focus; accessibility; visual | accessibility (A2-platform-hold) | — |
| Pill | semantic; accessibility; visual | — | semantic; accessibility; visual | accessibility (A2-platform-hold) | — |
| CodeInput | semantic; events; pointer; keyboard_focus; accessibility; visual | semantic (expected-test); events (expected-test); pointer (expected-test); keyboard_focus (expected-test) | accessibility; visual | accessibility (A2-platform-hold) | `docs/evidence/gpui/mounted-receipts/CodeInput--a-grouped-code-input-types-and-completes-through-the-real-tree.json` |
| Popover | semantic; events; pointer; keyboard_focus; accessibility; visual | semantic (nucleus-m1); events (nucleus-m1); pointer (nucleus-m1); keyboard_focus (nucleus-m1); accessibility (nucleus-m1); visual (nucleus-m1) | — | accessibility (A2-platform-hold) | `docs/evidence/gpui/mounted-receipts/Popover--a-nested-popover-paints-without-nesting-deferred-draws.json` |
| Pagination | semantic; events; pointer; keyboard_focus; accessibility; visual | semantic (expected-test); events (expected-test); pointer (expected-test); keyboard_focus (expected-test); accessibility (expected-test) | visual | accessibility (A2-platform-hold) | `docs/evidence/gpui/mounted-receipts/Pagination--pagination-navigation-limit-and-loading-through-mounted-pointer-and-keyboard.json` |
| PaginationSummary | semantic; events; pointer; keyboard_focus; accessibility; visual | — | semantic; events; pointer; keyboard_focus; accessibility; visual | accessibility (A2-platform-hold) | — |
| PasswordRequirements | semantic; events; pointer; accessibility; visual | — | semantic; events; pointer; accessibility; visual | accessibility (A2-platform-hold) | — |
| Progress | semantic; accessibility; visual | — | semantic; accessibility; visual | accessibility (A2-platform-hold) | — |
| Radio | semantic; events; pointer; keyboard_focus; accessibility; visual | semantic (expected-test); keyboard_focus (expected-test) | events; pointer; accessibility; visual | accessibility (A2-platform-hold) | `docs/evidence/gpui/mounted-receipts/Radio--radio-selects-on-activate-and-does-not-uncheck-itself.json` |
| RefSelect | semantic; events; pointer; keyboard_focus; accessibility; visual | — | semantic; events; pointer; keyboard_focus; accessibility; visual | accessibility (A2-platform-hold) | — |
| RadioGroup | semantic; events; pointer; keyboard_focus; accessibility; visual | semantic (nucleus-m1); events (nucleus-m1); pointer (nucleus-m1); keyboard_focus (nucleus-m1); accessibility (nucleus-m1); visual (nucleus-m1) | — | accessibility (A2-platform-hold) | `docs/evidence/gpui/mounted-receipts/RadioGroup--radio-group-exclusive-focus-identity-and-disabled-paths.json` |
| Rating | semantic; events; pointer; keyboard_focus; accessibility; visual | semantic (expected-test); events (expected-test); pointer (expected-test); keyboard_focus (expected-test); accessibility (expected-test) | visual | accessibility (A2-platform-hold) | `docs/evidence/gpui/mounted-receipts/Rating--rating-nullable-fractional-and-whole-step-through-mounted-pointer-and-keyboard.json` |
| Region | semantic; events; pointer; keyboard_focus; accessibility; visual | — | semantic; events; pointer; keyboard_focus; accessibility; visual | accessibility (A2-platform-hold) | — |
| ResizeHandle | semantic; events; pointer; keyboard_focus; accessibility; visual | semantic (expected-test); keyboard_focus (expected-test); accessibility (expected-test) | events; pointer; visual | accessibility (A2-platform-hold) | `docs/evidence/gpui/mounted-receipts/ResizeHandle--a-focused-resize-handle-steps-the-pane-and-its-declared-value.json`; `docs/evidence/gpui/mounted-receipts/ResizeHandle--a-disabled-resize-handle-takes-no-focus-and-answers-no-key.json` |
| RangeSlider | semantic; events; pointer; keyboard_focus; accessibility; visual | semantic (expected-test); events (expected-test) | pointer; keyboard_focus; accessibility; visual | accessibility (A2-platform-hold) | `docs/evidence/gpui/mounted-receipts/RangeSlider--a-scrub-reports-change-while-dragging-and-commits-once-at-release.json` |
| SegmentedControl | semantic; events; pointer; keyboard_focus; accessibility; visual | semantic (nucleus-m1); events (nucleus-m1); pointer (nucleus-m1); keyboard_focus (nucleus-m1); accessibility (nucleus-m1); visual (nucleus-m1) | — | accessibility (A2-platform-hold) | `docs/evidence/gpui/mounted-receipts/SegmentedControl--segmented-control-exclusive-focus-identity-and-disabled-paths.json` |
| Select | semantic; events; pointer; keyboard_focus; accessibility; visual | semantic (nucleus-m1); events (nucleus-m1); pointer (nucleus-m1); keyboard_focus (nucleus-m1); accessibility (nucleus-m1); visual (nucleus-m1) | — | accessibility (A2-platform-hold) | `docs/evidence/gpui/mounted-receipts/Select--select-two-instances-search-pointer-and-dismiss-through-mounted-rebuilds.json` |
| ScrollShell | semantic; events; pointer; keyboard_focus; accessibility; visual | — | semantic; events; pointer; keyboard_focus; accessibility; visual | accessibility (A2-platform-hold) | — |
| Separator | semantic; keyboard_focus; accessibility; visual | — | semantic; keyboard_focus; accessibility; visual | accessibility (A2-platform-hold) | — |
| SplitButton | semantic; events; pointer; keyboard_focus; accessibility; visual | — | semantic; events; pointer; keyboard_focus; accessibility; visual | accessibility (A2-platform-hold) | — |
| Skeleton | semantic; keyboard_focus; accessibility; visual | — | semantic; keyboard_focus; accessibility; visual | accessibility (A2-platform-hold) | — |
| Slider | semantic; events; pointer; keyboard_focus; accessibility; visual | semantic (expected-test); events (expected-test); keyboard_focus (expected-test); accessibility (expected-test); visual (expected-test) | pointer | accessibility (A2-platform-hold) | `docs/evidence/gpui/mounted-receipts/Slider--slider-axis-keyboard-and-disabled-rebuild-the-host-spec.json` |
| Spinner | semantic; accessibility; visual | — | semantic; accessibility; visual | accessibility (A2-platform-hold) | — |
| Spacer | semantic; keyboard_focus; accessibility; visual | — | semantic; keyboard_focus; accessibility; visual | accessibility (A2-platform-hold) | — |
| Stack | semantic; accessibility; visual | — | semantic; accessibility; visual | accessibility (A2-platform-hold) | — |
| Stepper | semantic; events; pointer; keyboard_focus; accessibility; visual | semantic (expected-test); events (expected-test); pointer (expected-test); keyboard_focus (expected-test); accessibility (expected-test) | visual | accessibility (A2-platform-hold) | `docs/evidence/gpui/mounted-receipts/Stepper--stepper-selection-and-rerun-reach-separate-mounted-controls.json`; `docs/evidence/gpui/mounted-receipts/Stepper--stepper-collapse-stays-independent-in-a-mounted-window.json`; `docs/evidence/gpui/mounted-receipts/Stepper--stepper-keyboard-entry-focuses-and-activates-without-a-pointer-press.json`; `docs/evidence/gpui/mounted-receipts/Stepper--stepper-summary-takes-keyboard-entry-and-paints-the-inset-ring.json` |
| AgentQuestion | semantic; events; pointer; keyboard_focus; accessibility; visual | semantic (nucleus-m1); events (nucleus-m1); pointer (nucleus-m1); keyboard_focus (nucleus-m1); accessibility (nucleus-a1); visual (nucleus-m1) | — | accessibility (A2-platform-hold) | — |
| AgentQuestionRecord | semantic; events; pointer; keyboard_focus; accessibility; visual | — | semantic; events; pointer; keyboard_focus; accessibility; visual | accessibility (A2-platform-hold) | — |
| AgentSubagent | semantic; events; pointer; keyboard_focus; accessibility; visual | semantic (expected-test); keyboard_focus (expected-test) | events; pointer; accessibility; visual | accessibility (A2-platform-hold) | `docs/evidence/gpui/mounted-receipts/AgentSubagent--agent-subagent-disclosure-rebuilds-the-host-spec-through-mounted-input.json` |
| ChangedFiles | semantic; events; pointer; keyboard_focus; accessibility; visual | semantic (expected-test); keyboard_focus (expected-test) | events; pointer; accessibility; visual | accessibility (A2-platform-hold) | `docs/evidence/gpui/mounted-receipts/ChangedFiles--changed-files-disclosure-and-selection-rebuild-the-host-spec.json` |
| ToolCall | semantic; events; pointer; keyboard_focus; accessibility; visual | semantic (expected-test); keyboard_focus (expected-test) | events; pointer; accessibility; visual | accessibility (A2-platform-hold) | `docs/evidence/gpui/mounted-receipts/ToolCall--tool-call-disclosure-rebuilds-the-host-spec-through-mounted-input.json` |
| ToolCallGroup | semantic; events; pointer; keyboard_focus; accessibility; visual | semantic (expected-test); keyboard_focus (expected-test) | events; pointer; accessibility; visual | accessibility (A2-platform-hold) | `docs/evidence/gpui/mounted-receipts/ToolCallGroup--tool-call-group-disclosure-rebuilds-the-host-spec-through-mounted-input.json` |
| StatusBar | semantic; pointer; keyboard_focus; accessibility; visual | — | semantic; pointer; keyboard_focus; accessibility; visual | accessibility (A2-platform-hold) | — |
| StatusIndicator | semantic; accessibility; visual | semantic (nucleus-m1); accessibility (nucleus-m1); visual (nucleus-m1) | — | accessibility (A2-platform-hold) | — |
| Surface | semantic; accessibility; visual | semantic (nucleus-m1); accessibility (nucleus-a1); visual (nucleus-m1) | — | accessibility (A2-platform-hold) | — |
| Switch | semantic; events; pointer; keyboard_focus; accessibility; visual | semantic (nucleus-m1); events (nucleus-m1); pointer (nucleus-m1); keyboard_focus (nucleus-m1); accessibility (nucleus-m1); visual (nucleus-m1) | — | accessibility (A2-platform-hold) | `docs/evidence/gpui/mounted-receipts/Switch--switch-toggle-readonly-and-disabled-rebuild-the-host-spec.json` |
| Text | semantic; events; pointer; keyboard_focus; accessibility; visual | semantic (nucleus-m1); events (nucleus-m1); pointer (nucleus-m1); keyboard_focus (nucleus-m1); accessibility (nucleus-a1); visual (nucleus-m1) | — | accessibility (A2-platform-hold) | — |
| TextLink | semantic; events; pointer; keyboard_focus; accessibility; visual | — | semantic; events; pointer; keyboard_focus; accessibility; visual | accessibility (A2-platform-hold) | — |
| Tabs | semantic; events; pointer; keyboard_focus; accessibility; visual | semantic (nucleus-m1); events (nucleus-m1); pointer (nucleus-m1); keyboard_focus (nucleus-m1); accessibility (nucleus-m1); visual (nucleus-m1) | — | accessibility (A2-platform-hold) | `docs/evidence/gpui/mounted-receipts/Tabs--tabs-drag-keyboard-and-identity-rebuild-the-host-spec.json` |
| Table | semantic; pointer; keyboard_focus; accessibility; visual | — | semantic; pointer; keyboard_focus; accessibility; visual | accessibility (A2-platform-hold) | — |
| TimeAgo | semantic; accessibility; visual | — | semantic; accessibility; visual | accessibility (A2-platform-hold) | — |
| TextInput | semantic; events; pointer; keyboard_focus; accessibility; visual | semantic (nucleus-m1); events (nucleus-m1); pointer (nucleus-m1); keyboard_focus (nucleus-m1); accessibility (nucleus-m1); visual (nucleus-m1) | — | accessibility (A2-platform-hold) | `docs/evidence/gpui/mounted-receipts/TextInput--text-input-controlled-editing-and-identity-rebuild-the-host-spec.json` |
| TokenInput | semantic; events; pointer; keyboard_focus; accessibility; visual | — | semantic; events; pointer; keyboard_focus; accessibility; visual | accessibility (A2-platform-hold) | — |
| TimeInput | semantic; events; pointer; keyboard_focus; accessibility; visual | semantic (expected-test); events (expected-test); keyboard_focus (expected-test) | pointer; accessibility; visual | accessibility (A2-platform-hold) | `docs/evidence/gpui/mounted-receipts/TimeInput--time-input-segmented-editor-commits-drafts-and-bounds.json` |
| TimeZoneSelect | semantic; events; pointer; keyboard_focus; accessibility; visual | — | semantic; events; pointer; keyboard_focus; accessibility; visual | accessibility (A2-platform-hold) | — |
| ThemeSelect | semantic; events; pointer; keyboard_focus; accessibility; visual | — | semantic; events; pointer; keyboard_focus; accessibility; visual | accessibility (A2-platform-hold) | — |
| ToggleGroup | semantic; events; pointer; keyboard_focus; accessibility; visual | semantic (expected-test); events (expected-test); pointer (expected-test); keyboard_focus (expected-test) | accessibility; visual | accessibility (A2-platform-hold) | `docs/evidence/gpui/mounted-receipts/ToggleGroup--toggle-group-result-focus-identity-and-disabled-paths.json` |
| Toolbar | semantic; pointer; keyboard_focus; accessibility; visual | — | semantic; pointer; keyboard_focus; accessibility; visual | accessibility (A2-platform-hold) | — |
| Tooltip | semantic; events; pointer; keyboard_focus; accessibility; visual | — | semantic; events; pointer; keyboard_focus; accessibility; visual | accessibility (A2-platform-hold) | — |
| TriStateSwitch | semantic; events; pointer; keyboard_focus; accessibility; visual | semantic (expected-test); events (expected-test); pointer (expected-test); keyboard_focus (expected-test); accessibility (expected-test) | visual | accessibility (A2-platform-hold) | `docs/evidence/gpui/mounted-receipts/TriStateSwitch--tri-state-switch-value-focus-identity-and-disabled-paths.json` |
| Menubar | semantic; events; pointer; keyboard_focus; accessibility; visual | — | semantic; events; pointer; keyboard_focus; accessibility; visual | accessibility (A2-platform-hold) | — |
| UiPresentationProvider | semantic; pointer; keyboard_focus; accessibility; visual | — | semantic; pointer; keyboard_focus; accessibility; visual | accessibility (A2-platform-hold) | — |
| MotionPolicyProvider | semantic; events; pointer; keyboard_focus; accessibility; visual | — | semantic; events; pointer; keyboard_focus; accessibility; visual | accessibility (A2-platform-hold) | — |
| VideoPlayer | semantic; events; pointer; keyboard_focus; accessibility; visual | — | semantic; events; pointer; keyboard_focus; accessibility; visual | accessibility (A2-platform-hold) | — |
| DateTimeZonePicker | semantic; events; pointer; keyboard_focus; accessibility; visual | — | semantic; events; pointer; keyboard_focus; accessibility; visual | accessibility (A2-platform-hold) | — |
| ActionDiscoveryPanel | semantic; events; pointer; keyboard_focus; accessibility; visual | semantic (expected-test); keyboard_focus (expected-test) | events; pointer; accessibility; visual | accessibility (A2-platform-hold) | `docs/evidence/gpui/mounted-receipts/ActionDiscoveryPanel--action-discovery-selection-rebuilds-the-host-spec-through-mounted-input.json` |
| AppHeader | semantic; events; pointer; keyboard_focus; accessibility; visual | semantic (nucleus-m1); pointer (nucleus-m1); keyboard_focus (nucleus-m1); accessibility (nucleus-m1); visual (nucleus-m1) | events | accessibility (A2-platform-hold) | — |
| EditableList | semantic; events; pointer; keyboard_focus; accessibility; visual | semantic (expected-test); events (expected-test); pointer (expected-test); keyboard_focus (expected-test); accessibility (expected-test) | visual | accessibility (A2-platform-hold) | `docs/evidence/gpui/mounted-receipts/EditableList--editable-list-substrate-reorder-rebuilds-the-host-spec.json` |
| ErrorBoundary | semantic; events; pointer; keyboard_focus; accessibility; visual | — | semantic; events; pointer; keyboard_focus; accessibility; visual | accessibility (A2-platform-hold) | — |
| BlockEditor | semantic; events; pointer; keyboard_focus; accessibility; visual | semantic (expected-test); events (expected-test); pointer (expected-test); keyboard_focus (expected-test); accessibility (expected-test) | visual | accessibility (A2-platform-hold) | `docs/evidence/gpui/mounted-receipts/BlockEditor--block-editor-grip-drag-and-move-controls-rebuild-the-host-spec.json` |
| CardRadioGroup | semantic; events; pointer; keyboard_focus; accessibility; visual | — | semantic; events; pointer; keyboard_focus; accessibility; visual | accessibility (A2-platform-hold) | — |
| CardToggleGroup | semantic; events; pointer; keyboard_focus; accessibility; visual | — | semantic; events; pointer; keyboard_focus; accessibility; visual | accessibility (A2-platform-hold) | — |
| CommandPalette | semantic; events; pointer; keyboard_focus; accessibility; visual | semantic (nucleus-m1); events (nucleus-m1); pointer (nucleus-m1); keyboard_focus (nucleus-m1); accessibility (nucleus-m1); visual (nucleus-m1) | — | accessibility (A2-platform-hold) | — |
| ConfirmAction | semantic; events; pointer; keyboard_focus; accessibility; visual | semantic (nucleus-m1); events (nucleus-m1); pointer (nucleus-m1); keyboard_focus (nucleus-m1); accessibility (nucleus-m1); visual (nucleus-m1) | — | accessibility (A2-platform-hold) | — |
| DataTable | semantic; events; pointer; keyboard_focus; accessibility; visual | — | semantic; events; pointer; keyboard_focus; accessibility; visual | accessibility (A2-platform-hold) | — |
| DetailSectionGroup | semantic; events; pointer; keyboard_focus; accessibility; visual | — | semantic; events; pointer; keyboard_focus; accessibility; visual | accessibility (A2-platform-hold) | — |
| DetailSection | semantic; events; pointer; keyboard_focus; accessibility; visual | — | semantic; events; pointer; keyboard_focus; accessibility; visual | accessibility (A2-platform-hold) | — |
| DockRegion | semantic; events; pointer; keyboard_focus; accessibility; visual | semantic (expected-test); events (expected-test); keyboard_focus (expected-test) | pointer; accessibility; visual | accessibility (A2-platform-hold) | `docs/evidence/gpui/mounted-receipts/DockRegion--dock-region-tab-and-collapse-rebuild-the-host-spec-through-mounted-input.json` |
| DetailShell | semantic; events; pointer; keyboard_focus; accessibility; visual | — | semantic; events; pointer; keyboard_focus; accessibility; visual | accessibility (A2-platform-hold) | — |
| EmbedInput | semantic; events; pointer; keyboard_focus; accessibility; visual | — | semantic; events; pointer; keyboard_focus; accessibility; visual | accessibility (A2-platform-hold) | — |
| EmbedPreview | semantic; events; pointer; keyboard_focus; accessibility; visual | — | semantic; events; pointer; keyboard_focus; accessibility; visual | accessibility (A2-platform-hold) | — |
| EmptyState | semantic; events; pointer; keyboard_focus; accessibility; visual | — | semantic; events; pointer; keyboard_focus; accessibility; visual | accessibility (A2-platform-hold) | — |
| FilterToolbar | semantic; events; pointer; keyboard_focus; accessibility; visual | — | semantic; events; pointer; keyboard_focus; accessibility; visual | accessibility (A2-platform-hold) | — |
| FormDialog | semantic; events; pointer; keyboard_focus; accessibility; visual | — | semantic; events; pointer; keyboard_focus; accessibility; visual | accessibility (A2-platform-hold) | — |
| FormLayout | semantic; events; pointer; keyboard_focus; accessibility; visual | — | semantic; events; pointer; keyboard_focus; accessibility; visual | accessibility (A2-platform-hold) | — |
| InlineListSection | semantic; events; pointer; keyboard_focus; accessibility; visual | — | semantic; events; pointer; keyboard_focus; accessibility; visual | accessibility (A2-platform-hold) | — |
| DebugDialog | semantic; events; pointer; keyboard_focus; accessibility; visual | — | semantic; events; pointer; keyboard_focus; accessibility; visual | accessibility (A2-platform-hold) | — |
| LicenceActivation | semantic; events; pointer; keyboard_focus; accessibility; visual | semantic (expected-test); events (expected-test); pointer (expected-test); keyboard_focus (expected-test); accessibility (expected-test) | visual | accessibility (A2-platform-hold) | `docs/evidence/gpui/mounted-receipts/LicenceActivation--licence-activation-key-entry-types-and-emits-through-the-real-tree.json`; `docs/evidence/gpui/mounted-receipts/LicenceActivation--licence-activation-machine-name-enter-and-escape-restore-display-focus.json` |
| LicenceSeats | semantic; events; pointer; keyboard_focus; accessibility; visual | semantic (expected-test); events (expected-test); pointer (expected-test); keyboard_focus (expected-test); accessibility (expected-test) | visual | accessibility (A2-platform-hold) | `docs/evidence/gpui/mounted-receipts/LicenceSeats--licence-seats-release-flows-through-confirm-in-a-mounted-window.json`; `docs/evidence/gpui/mounted-receipts/LicenceSeats--licence-seats-seat-row-enter-and-escape-restore-display-focus.json` |
| LicenceStatus | semantic; events; pointer; keyboard_focus; accessibility; visual | semantic (expected-test); accessibility (expected-test) | events; pointer; keyboard_focus; visual | accessibility (A2-platform-hold) | `docs/evidence/gpui/mounted-receipts/LicenceStatus--licence-status-renders-state-and-authority-reads-in-a-mounted-window.json` |
| LogList | semantic; events; pointer; keyboard_focus; accessibility; visual | — | semantic; events; pointer; keyboard_focus; accessibility; visual | accessibility (A2-platform-hold) | — |
| ListContainer | semantic; events; pointer; keyboard_focus; accessibility; visual | — | semantic; events; pointer; keyboard_focus; accessibility; visual | accessibility (A2-platform-hold) | — |
| PageLoading | semantic; events; pointer; keyboard_focus; accessibility; visual | — | semantic; events; pointer; keyboard_focus; accessibility; visual | accessibility (A2-platform-hold) | — |
| MediaPicker | semantic; events; pointer; keyboard_focus; accessibility; visual | — | semantic; events; pointer; keyboard_focus; accessibility; visual | accessibility (A2-platform-hold) | — |
| MediaBrowsePanel | semantic; events; pointer; keyboard_focus; accessibility; visual | — | semantic; events; pointer; keyboard_focus; accessibility; visual | accessibility (A2-platform-hold) | — |
| MediaPreview | semantic; events; pointer; keyboard_focus; accessibility; visual | — | semantic; events; pointer; keyboard_focus; accessibility; visual | accessibility (A2-platform-hold) | — |
| MediaThumbnail | semantic; events; pointer; keyboard_focus; accessibility; visual | — | semantic; events; pointer; keyboard_focus; accessibility; visual | accessibility (A2-platform-hold) | — |
| PageHeader | semantic; events; pointer; keyboard_focus; accessibility; visual | — | semantic; events; pointer; keyboard_focus; accessibility; visual | accessibility (A2-platform-hold) | — |
| PickerShell | semantic; pointer; keyboard_focus; accessibility; visual | — | semantic; pointer; keyboard_focus; accessibility; visual | accessibility (A2-platform-hold) | — |
| RelationPicker | semantic; events; pointer; keyboard_focus; accessibility; visual | — | semantic; events; pointer; keyboard_focus; accessibility; visual | accessibility (A2-platform-hold) | — |
| SelectionSummary | semantic; events; pointer; keyboard_focus; accessibility; visual | — | semantic; events; pointer; keyboard_focus; accessibility; visual | accessibility (A2-platform-hold) | — |
| SettingsShell | semantic; events; pointer; keyboard_focus; accessibility; visual | semantic (expected-test); events (expected-test); keyboard_focus (expected-test) | pointer; accessibility; visual | accessibility (A2-platform-hold) | `docs/evidence/gpui/mounted-receipts/SettingsShell--settings-shell-navigates-and-refused-close-stays-open.json` |
| SidebarNav | semantic; events; pointer; keyboard_focus; accessibility; visual | — | semantic; events; pointer; keyboard_focus; accessibility; visual | accessibility (A2-platform-hold) | — |
| Tree | semantic; events; pointer; keyboard_focus; accessibility; visual | semantic (expected-test); events (expected-test); pointer (expected-test); keyboard_focus (expected-test) | accessibility; visual | accessibility (A2-platform-hold) | `docs/evidence/gpui/mounted-receipts/Tree--tree-selection-expand-and-substrate-reorder-rebuild-the-host-spec.json` |
| SplitView | semantic; events; pointer; keyboard_focus; accessibility; visual | semantic (nucleus-m1); events (nucleus-m1); keyboard_focus (nucleus-m1); accessibility (nucleus-m1); visual (nucleus-m1) | pointer | accessibility (A2-platform-hold) | `docs/evidence/gpui/mounted-receipts/SplitView--two-composed-split-views-do-not-share-a-divider-focus-handle.json` |
| MetricTile | semantic; events; pointer; accessibility; visual | — | semantic; events; pointer; accessibility; visual | accessibility (A2-platform-hold) | — |
| StateTile | semantic; events; pointer; keyboard_focus; accessibility; visual | — | semantic; events; pointer; keyboard_focus; accessibility; visual | accessibility (A2-platform-hold) | — |
| ValidationSummary | semantic; events; pointer; keyboard_focus; accessibility; visual | — | semantic; events; pointer; keyboard_focus; accessibility; visual | accessibility (A2-platform-hold) | — |
| ModelPicker | semantic; events; pointer; keyboard_focus; accessibility; visual | semantic (nucleus-m1); events (nucleus-m1); pointer (nucleus-m1); keyboard_focus (nucleus-m1); accessibility (nucleus-m1); visual (nucleus-m1) | — | accessibility (A2-platform-hold) | — |
| ModelConnectionPicker | semantic; events; pointer; keyboard_focus; accessibility; visual | semantic (expected-test); keyboard_focus (expected-test) | events; pointer; accessibility; visual | accessibility (A2-platform-hold) | `docs/evidence/gpui/mounted-receipts/ModelConnectionPicker--model-connection-picker-roving-focus-moves-real-backend-focus.json` |
| ModelConnectionSetup | semantic; events; pointer; keyboard_focus; accessibility; visual | semantic (expected-test); pointer (expected-test); visual (expected-test) | events; keyboard_focus; accessibility | accessibility (A2-platform-hold) | `docs/evidence/gpui/mounted-receipts/ModelConnectionSetup--model-connection-setup-direct-add-submits-from-choose-in-a-mounted-window.json` |
| ModelConnectionCard | semantic; events; pointer; keyboard_focus; accessibility; visual | semantic (expected-test); pointer (expected-test); keyboard_focus (expected-test); accessibility (expected-test) | events; visual | accessibility (A2-platform-hold) | `docs/evidence/gpui/mounted-receipts/ModelConnectionCard--model-connection-card-closes-and-returns-real-focus-to-the-disclosure.json` |
| ModelCatalogueEditor | semantic; events; pointer; keyboard_focus; accessibility; visual | semantic (expected-test); events (expected-test); keyboard_focus (expected-test); accessibility (expected-test) | pointer; visual | accessibility (A2-platform-hold) | `docs/evidence/gpui/mounted-receipts/ModelCatalogueEditor--model-catalogue-editor-grabs-moves-and-cancels-in-a-mounted-window.json` |
| MessageCenter | semantic; events; pointer; keyboard_focus; accessibility; visual | semantic (nucleus-m1); events (nucleus-m1); pointer (nucleus-m1); keyboard_focus (nucleus-m1); accessibility (nucleus-m1); visual (nucleus-m1) | — | accessibility (A2-platform-hold) | — |
| HistoryCenter | semantic; events; pointer; keyboard_focus; accessibility; visual | — | semantic; events; pointer; keyboard_focus; accessibility; visual | accessibility (A2-platform-hold) | — |
| UpdateStatus | semantic; events; pointer; keyboard_focus; accessibility; visual | semantic (expected-test); keyboard_focus (expected-test) | events; pointer; accessibility; visual | accessibility (A2-platform-hold) | `docs/evidence/gpui/mounted-receipts/UpdateStatus--update-status-confirm-then-install-through-the-real-tree.json` |
| UpdateCenter | semantic; events; pointer; keyboard_focus; accessibility; visual | semantic (expected-test); keyboard_focus (expected-test); accessibility (expected-test) | events; pointer; visual | accessibility (A2-platform-hold) | `docs/evidence/gpui/mounted-receipts/UpdateCenter--update-center-hidden-presence-mounts-nothing-and-open-shows-status.json` |
| ToastStack | semantic; events; pointer; keyboard_focus; accessibility; visual | — | semantic; events; pointer; keyboard_focus; accessibility; visual | accessibility (A2-platform-hold) | — |
| ToastHost | semantic; events; pointer; keyboard_focus; accessibility; visual | semantic (nucleus-m1); events (nucleus-m1); pointer (nucleus-m1); keyboard_focus (nucleus-m1); accessibility (nucleus-m1); visual (nucleus-m1) | — | accessibility (A2-platform-hold) | — |
| AudioMeter | semantic; events; pointer; keyboard_focus; accessibility; visual | — | semantic; events; pointer; keyboard_focus; accessibility; visual | accessibility (A2-platform-hold) | — |
| MeterSurface | not-applicable (web-only) | — | — | — | — |
| AudioSwitch | semantic; events; pointer; keyboard_focus; accessibility; visual | — | semantic; events; pointer; keyboard_focus; accessibility; visual | accessibility (A2-platform-hold) | — |
| DragNumberField | semantic; events; pointer; keyboard_focus; accessibility; visual | — | semantic; events; pointer; keyboard_focus; accessibility; visual | accessibility (A2-platform-hold) | — |
| EnvelopeEditor | semantic; events; pointer; keyboard_focus; accessibility; visual | — | semantic; events; pointer; keyboard_focus; accessibility; visual | accessibility (A2-platform-hold) | — |
| Fader | semantic; events; pointer; keyboard_focus; accessibility; visual | semantic (expected-test); events (expected-test); pointer (expected-test); keyboard_focus (expected-test); accessibility (expected-test) | visual | accessibility (A2-platform-hold) | `docs/evidence/gpui/mounted-receipts/Fader--fader-mounted-parity-through-production-dispatch.json` |
| GainReductionMeter | semantic; events; pointer; keyboard_focus; accessibility; visual | — | semantic; events; pointer; keyboard_focus; accessibility; visual | accessibility (A2-platform-hold) | — |
| Keyboard | semantic; events; pointer; keyboard_focus; accessibility; visual | — | semantic; events; pointer; keyboard_focus; accessibility; visual | accessibility (A2-platform-hold) | — |
| Knob | semantic; events; pointer; keyboard_focus; accessibility; visual | semantic (expected-test); events (expected-test); pointer (expected-test); keyboard_focus (expected-test) | accessibility; visual | accessibility (A2-platform-hold) | `docs/evidence/gpui/mounted-receipts/Knob--knob-mounted-parity-through-production-dispatch.json` |
| ModMatrixGrid | semantic; events; pointer; keyboard_focus; accessibility; visual | — | semantic; events; pointer; keyboard_focus; accessibility; visual | accessibility (A2-platform-hold) | — |
| ValueReadout | semantic; events; pointer; keyboard_focus; accessibility; visual | — | semantic; events; pointer; keyboard_focus; accessibility; visual | accessibility (A2-platform-hold) | — |
| WaveformDisplay | semantic; events; pointer; keyboard_focus; accessibility; visual | — | semantic; events; pointer; keyboard_focus; accessibility; visual | accessibility (A2-platform-hold) | — |
| XYPad | semantic; events; pointer; keyboard_focus; accessibility; visual | semantic (expected-test); events (expected-test); pointer (expected-test); keyboard_focus (expected-test); accessibility (expected-test) | visual | accessibility (A2-platform-hold) | `docs/evidence/gpui/mounted-receipts/XYPad--xy-pad-mounted-parity-through-production-dispatch.json` |
| AgentMessage | semantic; events; pointer; keyboard_focus; accessibility; visual | — | semantic; events; pointer; keyboard_focus; accessibility; visual | accessibility (A2-platform-hold) | — |
| AgentPlan | semantic; events; pointer; keyboard_focus; accessibility; visual | semantic (nucleus-m1); events (nucleus-m1); pointer (nucleus-m1); keyboard_focus (nucleus-m1); accessibility (nucleus-a1); visual (nucleus-m1) | — | accessibility (A2-platform-hold) | `docs/evidence/gpui/mounted-receipts/AgentPlan--agent-plan-decisions-rebuild-the-host-spec-through-mounted-input.json` |
| AgentPlanRecord | semantic; events; pointer; keyboard_focus; accessibility; visual | semantic (expected-test); keyboard_focus (expected-test) | events; pointer; accessibility; visual | accessibility (A2-platform-hold) | `docs/evidence/gpui/mounted-receipts/AgentPlanRecord--agent-plan-record-disclosure-rebuilds-the-host-spec-through-mounted-input.json` |
| AgentTranscript | semantic; events; pointer; keyboard_focus; accessibility; visual | semantic (nucleus-m1); pointer (nucleus-m1); keyboard_focus (nucleus-m1); accessibility (nucleus-m1); visual (nucleus-m1) | events | accessibility (A2-platform-hold) | — |
| MarkdownEditor | semantic; events; pointer; keyboard_focus; accessibility; visual | — | semantic; events; pointer; keyboard_focus; accessibility; visual | accessibility (A2-platform-hold) | — |

<!-- g18-census-refusals -->
## Refusals

- Accordion: Expected test accordion_result_disclosure_focus_identity_and_disabled_paths proves no visual claim; visual stays missing.
- AudioPlayer: No validated receipt and no retained expected test for AudioPlayer; every required capability stays missing.
- AlertDialog: No validated receipt and no retained expected test for AlertDialog; every required capability stays missing.
- Avatar: No validated receipt and no retained expected test for Avatar; every required capability stays missing.
- Box: No validated receipt and no retained expected test for Box; every required capability stays missing.
- Breadcrumbs: Expected test breadcrumbs_callback_navigation_through_mounted_pointer_and_keyboard proves no events claim; events stays missing.
- Breadcrumbs: Expected test breadcrumbs_callback_navigation_through_mounted_pointer_and_keyboard proves no visual claim; visual stays missing.
- BulkActionBar: No validated receipt and no retained expected test for BulkActionBar; every required capability stays missing.
- Button: Expected test a_mounted_button_carries_its_controls_target proves no events claim; events stays missing.
- Button: Expected test a_mounted_button_carries_its_controls_target proves no keyboard_focus claim; keyboard_focus stays missing.
- RemediationBanner: Expected test remediation_banner_action_and_dismiss_rebuild_the_host_spec proves no events claim; events stays missing.
- RemediationBanner: Expected test remediation_banner_action_and_dismiss_rebuild_the_host_spec proves no pointer claim; pointer stays missing.
- RemediationBanner: Expected test remediation_banner_action_and_dismiss_rebuild_the_host_spec proves no accessibility claim; accessibility stays missing.
- RemediationBanner: Expected test remediation_banner_action_and_dismiss_rebuild_the_host_spec proves no visual claim; visual stays missing.
- Card: No validated receipt and no retained expected test for Card; every required capability stays missing.
- Code: No validated receipt and no retained expected test for Code; every required capability stays missing.
- ColorPicker: No validated receipt and no retained expected test for ColorPicker; every required capability stays missing.
- Checkbox: Expected test checkbox_toggle_readonly_and_disabled_rebuild_the_host_spec proves no visual claim; visual stays missing.
- Calendar: No validated receipt and no retained expected test for Calendar; every required capability stays missing.
- ContextMenu: No validated receipt and no retained expected test for ContextMenu; every required capability stays missing.
- CollapseToggle: Expected test collapse_toggle_disclosure_focus_and_disabled_through_mounted_pointer_and_keyboard proves no events claim; events stays missing.
- CollapseToggle: Expected test collapse_toggle_disclosure_focus_and_disabled_through_mounted_pointer_and_keyboard proves no visual claim; visual stays missing.
- Collapsible: Expected test collapsible_disclosure_and_identity_through_mounted_pointer_and_keyboard proves no events claim; events stays missing.
- Collapsible: Expected test collapsible_disclosure_and_identity_through_mounted_pointer_and_keyboard proves no visual claim; visual stays missing.
- DatePicker: No validated receipt and no retained expected test for DatePicker; every required capability stays missing.
- DateRangePicker: No validated receipt and no retained expected test for DateRangePicker; every required capability stays missing.
- DateTimePicker: No validated receipt and no retained expected test for DateTimePicker; every required capability stays missing.
- DateTimeRangePicker: No validated receipt and no retained expected test for DateTimeRangePicker; every required capability stays missing.
- Drawer: No validated receipt and no retained expected test for Drawer; every required capability stays missing.
- DurationInput: Expected test duration_input_segments_edit_and_rebuild_the_host_spec proves no pointer claim; pointer stays missing.
- DurationInput: Expected test duration_input_segments_edit_and_rebuild_the_host_spec proves no accessibility claim; accessibility stays missing.
- DurationInput: Expected test duration_input_segments_edit_and_rebuild_the_host_spec proves no visual claim; visual stays missing.
- Eyebrow: No validated receipt and no retained expected test for Eyebrow; every required capability stays missing.
- Field: No validated receipt and no retained expected test for Field; every required capability stays missing.
- FieldSet: No validated receipt and no retained expected test for FieldSet; every required capability stays missing.
- FileUpload: Expected test a_dropzone_browse_flows_fixture_bytes_through_the_generic_seam proves no keyboard_focus claim; keyboard_focus stays missing.
- FileUpload: Expected test a_dropzone_browse_flows_fixture_bytes_through_the_generic_seam proves no accessibility claim; accessibility stays missing.
- FileUpload: Expected test a_dropzone_browse_flows_fixture_bytes_through_the_generic_seam proves no visual claim; visual stays missing.
- FilterBuilder: No validated receipt and no retained expected test for FilterBuilder; every required capability stays missing.
- FormActions: No validated receipt and no retained expected test for FormActions; every required capability stays missing.
- Grid: No validated receipt and no retained expected test for Grid; every required capability stays missing.
- HoverCard: No validated receipt and no retained expected test for HoverCard; every required capability stays missing.
- IconProvider: No validated receipt and no retained expected test for IconProvider; every required capability stays missing.
- Meter: No validated receipt and no retained expected test for Meter; every required capability stays missing.
- ListCard: No validated receipt and no retained expected test for ListCard; every required capability stays missing.
- ListCardCounter: No validated receipt and no retained expected test for ListCardCounter; every required capability stays missing.
- ListGrid: No validated receipt and no retained expected test for ListGrid; every required capability stays missing.
- MetaBar: No validated receipt and no retained expected test for MetaBar; every required capability stays missing.
- MetaItem: No validated receipt and no retained expected test for MetaItem; every required capability stays missing.
- NumberInput: Expected test number_input_mounted_valid_direct_editing_rebuilds_host_draft_and_value proves no pointer claim; pointer stays missing.
- NumberInput: Expected test number_input_mounted_valid_direct_editing_rebuilds_host_draft_and_value proves no visual claim; visual stays missing.
- NumberInput: Expected test number_input_mounted_accessibility_projects_spin_button_surface proves no pointer claim; pointer stays missing.
- NumberInput: Expected test number_input_mounted_accessibility_projects_spin_button_surface proves no visual claim; visual stays missing.
- OrderBy: Expected test order_by_substrate_reorder_and_alt_arrow_rebuild_the_host_spec proves no visual claim; visual stays missing.
- NavCard: No validated receipt and no retained expected test for NavCard; every required capability stays missing.
- NavigationMenu: No validated receipt and no retained expected test for NavigationMenu; every required capability stays missing.
- Pill: No validated receipt and no retained expected test for Pill; every required capability stays missing.
- CodeInput: Expected test a_grouped_code_input_types_and_completes_through_the_real_tree proves no accessibility claim; accessibility stays missing.
- CodeInput: Expected test a_grouped_code_input_types_and_completes_through_the_real_tree proves no visual claim; visual stays missing.
- Pagination: Expected test pagination_navigation_limit_and_loading_through_mounted_pointer_and_keyboard proves no visual claim; visual stays missing.
- PaginationSummary: No validated receipt and no retained expected test for PaginationSummary; every required capability stays missing.
- PasswordRequirements: No validated receipt and no retained expected test for PasswordRequirements; every required capability stays missing.
- Progress: No validated receipt and no retained expected test for Progress; every required capability stays missing.
- Radio: Expected test radio_selects_on_activate_and_does_not_uncheck_itself proves no events claim; events stays missing.
- Radio: Expected test radio_selects_on_activate_and_does_not_uncheck_itself proves no pointer claim; pointer stays missing.
- Radio: Expected test radio_selects_on_activate_and_does_not_uncheck_itself proves no accessibility claim; accessibility stays missing.
- Radio: Expected test radio_selects_on_activate_and_does_not_uncheck_itself proves no visual claim; visual stays missing.
- RefSelect: No validated receipt and no retained expected test for RefSelect; every required capability stays missing.
- Rating: Expected test rating_nullable_fractional_and_whole_step_through_mounted_pointer_and_keyboard proves no visual claim; visual stays missing.
- Region: No validated receipt and no retained expected test for Region; every required capability stays missing.
- ResizeHandle: Expected test a_focused_resize_handle_steps_the_pane_and_its_declared_value proves no events claim; events stays missing.
- ResizeHandle: Expected test a_focused_resize_handle_steps_the_pane_and_its_declared_value proves no pointer claim; pointer stays missing.
- ResizeHandle: Expected test a_focused_resize_handle_steps_the_pane_and_its_declared_value proves no visual claim; visual stays missing.
- ResizeHandle: Expected test a_disabled_resize_handle_takes_no_focus_and_answers_no_key proves no events claim; events stays missing.
- ResizeHandle: Expected test a_disabled_resize_handle_takes_no_focus_and_answers_no_key proves no pointer claim; pointer stays missing.
- ResizeHandle: Expected test a_disabled_resize_handle_takes_no_focus_and_answers_no_key proves no visual claim; visual stays missing.
- RangeSlider: Expected test a_scrub_reports_change_while_dragging_and_commits_once_at_release proves no pointer claim; pointer stays missing.
- RangeSlider: Expected test a_scrub_reports_change_while_dragging_and_commits_once_at_release proves no keyboard_focus claim; keyboard_focus stays missing.
- RangeSlider: Expected test a_scrub_reports_change_while_dragging_and_commits_once_at_release proves no accessibility claim; accessibility stays missing.
- RangeSlider: Expected test a_scrub_reports_change_while_dragging_and_commits_once_at_release proves no visual claim; visual stays missing.
- ScrollShell: No validated receipt and no retained expected test for ScrollShell; every required capability stays missing.
- Separator: No validated receipt and no retained expected test for Separator; every required capability stays missing.
- SplitButton: No validated receipt and no retained expected test for SplitButton; every required capability stays missing.
- Skeleton: No validated receipt and no retained expected test for Skeleton; every required capability stays missing.
- Slider: Expected test slider_axis_keyboard_and_disabled_rebuild_the_host_spec proves no pointer claim; pointer stays missing.
- Spinner: No validated receipt and no retained expected test for Spinner; every required capability stays missing.
- Spacer: No validated receipt and no retained expected test for Spacer; every required capability stays missing.
- Stack: No validated receipt and no retained expected test for Stack; every required capability stays missing.
- Stepper: Expected test stepper_selection_and_rerun_reach_separate_mounted_controls proves no accessibility claim; accessibility stays missing.
- Stepper: Expected test stepper_selection_and_rerun_reach_separate_mounted_controls proves no visual claim; visual stays missing.
- Stepper: Expected test stepper_collapse_stays_independent_in_a_mounted_window proves no visual claim; visual stays missing.
- Stepper: Expected test stepper_keyboard_entry_focuses_and_activates_without_a_pointer_press proves no visual claim; visual stays missing.
- Stepper: Expected test stepper_summary_takes_keyboard_entry_and_paints_the_inset_ring proves no visual claim; visual stays missing.
- AgentQuestionRecord: No validated receipt and no retained expected test for AgentQuestionRecord; every required capability stays missing.
- AgentSubagent: Expected test agent_subagent_disclosure_rebuilds_the_host_spec_through_mounted_input proves no events claim; events stays missing.
- AgentSubagent: Expected test agent_subagent_disclosure_rebuilds_the_host_spec_through_mounted_input proves no pointer claim; pointer stays missing.
- AgentSubagent: Expected test agent_subagent_disclosure_rebuilds_the_host_spec_through_mounted_input proves no accessibility claim; accessibility stays missing.
- AgentSubagent: Expected test agent_subagent_disclosure_rebuilds_the_host_spec_through_mounted_input proves no visual claim; visual stays missing.
- ChangedFiles: Expected test changed_files_disclosure_and_selection_rebuild_the_host_spec proves no events claim; events stays missing.
- ChangedFiles: Expected test changed_files_disclosure_and_selection_rebuild_the_host_spec proves no pointer claim; pointer stays missing.
- ChangedFiles: Expected test changed_files_disclosure_and_selection_rebuild_the_host_spec proves no accessibility claim; accessibility stays missing.
- ChangedFiles: Expected test changed_files_disclosure_and_selection_rebuild_the_host_spec proves no visual claim; visual stays missing.
- ToolCall: Expected test tool_call_disclosure_rebuilds_the_host_spec_through_mounted_input proves no events claim; events stays missing.
- ToolCall: Expected test tool_call_disclosure_rebuilds_the_host_spec_through_mounted_input proves no pointer claim; pointer stays missing.
- ToolCall: Expected test tool_call_disclosure_rebuilds_the_host_spec_through_mounted_input proves no accessibility claim; accessibility stays missing.
- ToolCall: Expected test tool_call_disclosure_rebuilds_the_host_spec_through_mounted_input proves no visual claim; visual stays missing.
- ToolCallGroup: Expected test tool_call_group_disclosure_rebuilds_the_host_spec_through_mounted_input proves no events claim; events stays missing.
- ToolCallGroup: Expected test tool_call_group_disclosure_rebuilds_the_host_spec_through_mounted_input proves no pointer claim; pointer stays missing.
- ToolCallGroup: Expected test tool_call_group_disclosure_rebuilds_the_host_spec_through_mounted_input proves no accessibility claim; accessibility stays missing.
- ToolCallGroup: Expected test tool_call_group_disclosure_rebuilds_the_host_spec_through_mounted_input proves no visual claim; visual stays missing.
- StatusBar: No validated receipt and no retained expected test for StatusBar; every required capability stays missing.
- TextLink: No validated receipt and no retained expected test for TextLink; every required capability stays missing.
- Table: No validated receipt and no retained expected test for Table; every required capability stays missing.
- TimeAgo: No validated receipt and no retained expected test for TimeAgo; every required capability stays missing.
- TokenInput: No validated receipt and no retained expected test for TokenInput; every required capability stays missing.
- TimeInput: Expected test time_input_segmented_editor_commits_drafts_and_bounds proves no pointer claim; pointer stays missing.
- TimeInput: Expected test time_input_segmented_editor_commits_drafts_and_bounds proves no accessibility claim; accessibility stays missing.
- TimeInput: Expected test time_input_segmented_editor_commits_drafts_and_bounds proves no visual claim; visual stays missing.
- TimeZoneSelect: No validated receipt and no retained expected test for TimeZoneSelect; every required capability stays missing.
- ThemeSelect: No validated receipt and no retained expected test for ThemeSelect; every required capability stays missing.
- ToggleGroup: Expected test toggle_group_result_focus_identity_and_disabled_paths proves no accessibility claim; accessibility stays missing.
- ToggleGroup: Expected test toggle_group_result_focus_identity_and_disabled_paths proves no visual claim; visual stays missing.
- Toolbar: No validated receipt and no retained expected test for Toolbar; every required capability stays missing.
- Tooltip: No validated receipt and no retained expected test for Tooltip; every required capability stays missing.
- TriStateSwitch: Expected test tri_state_switch_value_focus_identity_and_disabled_paths proves no visual claim; visual stays missing.
- Menubar: No validated receipt and no retained expected test for Menubar; every required capability stays missing.
- UiPresentationProvider: No validated receipt and no retained expected test for UiPresentationProvider; every required capability stays missing.
- MotionPolicyProvider: No validated receipt and no retained expected test for MotionPolicyProvider; every required capability stays missing.
- VideoPlayer: No validated receipt and no retained expected test for VideoPlayer; every required capability stays missing.
- DateTimeZonePicker: No validated receipt and no retained expected test for DateTimeZonePicker; every required capability stays missing.
- ActionDiscoveryPanel: Expected test action_discovery_selection_rebuilds_the_host_spec_through_mounted_input proves no events claim; events stays missing.
- ActionDiscoveryPanel: Expected test action_discovery_selection_rebuilds_the_host_spec_through_mounted_input proves no pointer claim; pointer stays missing.
- ActionDiscoveryPanel: Expected test action_discovery_selection_rebuilds_the_host_spec_through_mounted_input proves no accessibility claim; accessibility stays missing.
- ActionDiscoveryPanel: Expected test action_discovery_selection_rebuilds_the_host_spec_through_mounted_input proves no visual claim; visual stays missing.
- EditableList: Expected test editable_list_substrate_reorder_rebuilds_the_host_spec proves no visual claim; visual stays missing.
- ErrorBoundary: No validated receipt and no retained expected test for ErrorBoundary; every required capability stays missing.
- BlockEditor: Expected test block_editor_grip_drag_and_move_controls_rebuild_the_host_spec proves no visual claim; visual stays missing.
- CardRadioGroup: No validated receipt and no retained expected test for CardRadioGroup; every required capability stays missing.
- CardToggleGroup: No validated receipt and no retained expected test for CardToggleGroup; every required capability stays missing.
- DataTable: No validated receipt and no retained expected test for DataTable; every required capability stays missing.
- DetailSectionGroup: No validated receipt and no retained expected test for DetailSectionGroup; every required capability stays missing.
- DetailSection: No validated receipt and no retained expected test for DetailSection; every required capability stays missing.
- DockRegion: Expected test dock_region_tab_and_collapse_rebuild_the_host_spec_through_mounted_input proves no pointer claim; pointer stays missing.
- DockRegion: Expected test dock_region_tab_and_collapse_rebuild_the_host_spec_through_mounted_input proves no accessibility claim; accessibility stays missing.
- DockRegion: Expected test dock_region_tab_and_collapse_rebuild_the_host_spec_through_mounted_input proves no visual claim; visual stays missing.
- DetailShell: No validated receipt and no retained expected test for DetailShell; every required capability stays missing.
- EmbedInput: No validated receipt and no retained expected test for EmbedInput; every required capability stays missing.
- EmbedPreview: No validated receipt and no retained expected test for EmbedPreview; every required capability stays missing.
- EmptyState: No validated receipt and no retained expected test for EmptyState; every required capability stays missing.
- FilterToolbar: No validated receipt and no retained expected test for FilterToolbar; every required capability stays missing.
- FormDialog: No validated receipt and no retained expected test for FormDialog; every required capability stays missing.
- FormLayout: No validated receipt and no retained expected test for FormLayout; every required capability stays missing.
- InlineListSection: No validated receipt and no retained expected test for InlineListSection; every required capability stays missing.
- DebugDialog: No validated receipt and no retained expected test for DebugDialog; every required capability stays missing.
- LicenceActivation: Expected test licence_activation_key_entry_types_and_emits_through_the_real_tree proves no visual claim; visual stays missing.
- LicenceActivation: Expected test licence_activation_machine_name_enter_and_escape_restore_display_focus proves no visual claim; visual stays missing.
- LicenceSeats: Expected test licence_seats_release_flows_through_confirm_in_a_mounted_window proves no events claim; events stays missing.
- LicenceSeats: Expected test licence_seats_release_flows_through_confirm_in_a_mounted_window proves no keyboard_focus claim; keyboard_focus stays missing.
- LicenceSeats: Expected test licence_seats_release_flows_through_confirm_in_a_mounted_window proves no visual claim; visual stays missing.
- LicenceSeats: Expected test licence_seats_seat_row_enter_and_escape_restore_display_focus proves no visual claim; visual stays missing.
- LicenceStatus: Expected test licence_status_renders_state_and_authority_reads_in_a_mounted_window proves no events claim; events stays missing.
- LicenceStatus: Expected test licence_status_renders_state_and_authority_reads_in_a_mounted_window proves no pointer claim; pointer stays missing.
- LicenceStatus: Expected test licence_status_renders_state_and_authority_reads_in_a_mounted_window proves no keyboard_focus claim; keyboard_focus stays missing.
- LicenceStatus: Expected test licence_status_renders_state_and_authority_reads_in_a_mounted_window proves no visual claim; visual stays missing.
- LogList: No validated receipt and no retained expected test for LogList; every required capability stays missing.
- ListContainer: No validated receipt and no retained expected test for ListContainer; every required capability stays missing.
- PageLoading: No validated receipt and no retained expected test for PageLoading; every required capability stays missing.
- MediaPicker: No validated receipt and no retained expected test for MediaPicker; every required capability stays missing.
- MediaBrowsePanel: No validated receipt and no retained expected test for MediaBrowsePanel; every required capability stays missing.
- MediaPreview: No validated receipt and no retained expected test for MediaPreview; every required capability stays missing.
- MediaThumbnail: No validated receipt and no retained expected test for MediaThumbnail; every required capability stays missing.
- PageHeader: No validated receipt and no retained expected test for PageHeader; every required capability stays missing.
- PickerShell: No validated receipt and no retained expected test for PickerShell; every required capability stays missing.
- RelationPicker: No validated receipt and no retained expected test for RelationPicker; every required capability stays missing.
- SelectionSummary: No validated receipt and no retained expected test for SelectionSummary; every required capability stays missing.
- SettingsShell: Expected test settings_shell_navigates_and_refused_close_stays_open proves no pointer claim; pointer stays missing.
- SettingsShell: Expected test settings_shell_navigates_and_refused_close_stays_open proves no accessibility claim; accessibility stays missing.
- SettingsShell: Expected test settings_shell_navigates_and_refused_close_stays_open proves no visual claim; visual stays missing.
- SidebarNav: No validated receipt and no retained expected test for SidebarNav; every required capability stays missing.
- Tree: Expected test tree_selection_expand_and_substrate_reorder_rebuild_the_host_spec proves no accessibility claim; accessibility stays missing.
- Tree: Expected test tree_selection_expand_and_substrate_reorder_rebuild_the_host_spec proves no visual claim; visual stays missing.
- SplitView: Expected test two_composed_split_views_do_not_share_a_divider_focus_handle proves no pointer claim; pointer stays missing.
- MetricTile: No validated receipt and no retained expected test for MetricTile; every required capability stays missing.
- StateTile: No validated receipt and no retained expected test for StateTile; every required capability stays missing.
- ValidationSummary: No validated receipt and no retained expected test for ValidationSummary; every required capability stays missing.
- ModelConnectionPicker: Expected test model_connection_picker_roving_focus_moves_real_backend_focus proves no events claim; events stays missing.
- ModelConnectionPicker: Expected test model_connection_picker_roving_focus_moves_real_backend_focus proves no pointer claim; pointer stays missing.
- ModelConnectionPicker: Expected test model_connection_picker_roving_focus_moves_real_backend_focus proves no accessibility claim; accessibility stays missing.
- ModelConnectionPicker: Expected test model_connection_picker_roving_focus_moves_real_backend_focus proves no visual claim; visual stays missing.
- ModelConnectionSetup: Expected test model_connection_setup_direct_add_submits_from_choose_in_a_mounted_window proves no events claim; events stays missing.
- ModelConnectionSetup: Expected test model_connection_setup_direct_add_submits_from_choose_in_a_mounted_window proves no keyboard_focus claim; keyboard_focus stays missing.
- ModelConnectionSetup: Expected test model_connection_setup_direct_add_submits_from_choose_in_a_mounted_window proves no accessibility claim; accessibility stays missing.
- ModelConnectionCard: Expected test model_connection_card_closes_and_returns_real_focus_to_the_disclosure proves no events claim; events stays missing.
- ModelConnectionCard: Expected test model_connection_card_closes_and_returns_real_focus_to_the_disclosure proves no visual claim; visual stays missing.
- ModelCatalogueEditor: Expected test model_catalogue_editor_grabs_moves_and_cancels_in_a_mounted_window proves no pointer claim; pointer stays missing.
- ModelCatalogueEditor: Expected test model_catalogue_editor_grabs_moves_and_cancels_in_a_mounted_window proves no visual claim; visual stays missing.
- HistoryCenter: No validated receipt and no retained expected test for HistoryCenter; every required capability stays missing.
- UpdateStatus: Expected test update_status_confirm_then_install_through_the_real_tree proves no events claim; events stays missing.
- UpdateStatus: Expected test update_status_confirm_then_install_through_the_real_tree proves no pointer claim; pointer stays missing.
- UpdateStatus: Expected test update_status_confirm_then_install_through_the_real_tree proves no accessibility claim; accessibility stays missing.
- UpdateStatus: Expected test update_status_confirm_then_install_through_the_real_tree proves no visual claim; visual stays missing.
- UpdateCenter: Expected test update_center_hidden_presence_mounts_nothing_and_open_shows_status proves no events claim; events stays missing.
- UpdateCenter: Expected test update_center_hidden_presence_mounts_nothing_and_open_shows_status proves no pointer claim; pointer stays missing.
- UpdateCenter: Expected test update_center_hidden_presence_mounts_nothing_and_open_shows_status proves no visual claim; visual stays missing.
- ToastStack: No validated receipt and no retained expected test for ToastStack; every required capability stays missing.
- AudioMeter: No validated receipt and no retained expected test for AudioMeter; every required capability stays missing.
- AudioSwitch: No validated receipt and no retained expected test for AudioSwitch; every required capability stays missing.
- DragNumberField: No validated receipt and no retained expected test for DragNumberField; every required capability stays missing.
- EnvelopeEditor: No validated receipt and no retained expected test for EnvelopeEditor; every required capability stays missing.
- Fader: Expected test fader_mounted_parity_through_production_dispatch proves no visual claim; visual stays missing.
- GainReductionMeter: No validated receipt and no retained expected test for GainReductionMeter; every required capability stays missing.
- Keyboard: No validated receipt and no retained expected test for Keyboard; every required capability stays missing.
- Knob: Expected test knob_mounted_parity_through_production_dispatch proves no accessibility claim; accessibility stays missing.
- Knob: Expected test knob_mounted_parity_through_production_dispatch proves no visual claim; visual stays missing.
- ModMatrixGrid: No validated receipt and no retained expected test for ModMatrixGrid; every required capability stays missing.
- ValueReadout: No validated receipt and no retained expected test for ValueReadout; every required capability stays missing.
- WaveformDisplay: No validated receipt and no retained expected test for WaveformDisplay; every required capability stays missing.
- XYPad: Expected test xy_pad_mounted_parity_through_production_dispatch proves no visual claim; visual stays missing.
- AgentMessage: No validated receipt and no retained expected test for AgentMessage; every required capability stays missing.
- AgentPlanRecord: Expected test agent_plan_record_disclosure_rebuilds_the_host_spec_through_mounted_input proves no events claim; events stays missing.
- AgentPlanRecord: Expected test agent_plan_record_disclosure_rebuilds_the_host_spec_through_mounted_input proves no pointer claim; pointer stays missing.
- AgentPlanRecord: Expected test agent_plan_record_disclosure_rebuilds_the_host_spec_through_mounted_input proves no accessibility claim; accessibility stays missing.
- AgentPlanRecord: Expected test agent_plan_record_disclosure_rebuilds_the_host_spec_through_mounted_input proves no visual claim; visual stays missing.
- MarkdownEditor: No validated receipt and no retained expected test for MarkdownEditor; every required capability stays missing.

## Missing-capability groups

Shared-substrate groupings for repair-tranche compilation. Grouping only; no tranche is planned here.

- overlay-dismissal: 44 components (AgentPlanRecord, AlertDialog, Breadcrumbs, ColorPicker, ContextMenu, DataTable, DatePicker, DateRangePicker, DateTimePicker, DateTimeRangePicker, DateTimeZonePicker, DebugDialog, Drawer, Field, FilterBuilder, FilterToolbar, FormDialog, HistoryCenter, HoverCard, ListCardCounter, MediaBrowsePanel, MediaPicker, MediaThumbnail, Menubar, ModelConnectionSetup, NavigationMenu, OrderBy, PageLoading, PickerShell, RangeSlider, RefSelect, RelationPicker, RemediationBanner, ResizeHandle, SettingsShell, Slider, SplitButton, ThemeSelect, ToastStack, Toolbar, Tree, UpdateCenter, UpdateStatus, VideoPlayer); missing semantic 32; events 36; pointer 40; keyboard_focus 34; accessibility 39; visual 42
- selection-navigation: 24 components (Accordion, AudioPlayer, BulkActionBar, Button, ChangedFiles, Checkbox, CodeInput, CollapseToggle, DurationInput, ListContainer, LogList, ModelConnectionPicker, NavCard, Pagination, PaginationSummary, Pill, Radio, SelectionSummary, Table, TimeInput, TimeZoneSelect, ToggleGroup, ToolCallGroup, TriStateSwitch); missing semantic 10; events 14; pointer 15; keyboard_focus 10; accessibility 18; visual 23
- feedback-status: 21 components (ActionDiscoveryPanel, AgentSubagent, AgentTranscript, Avatar, DetailSectionGroup, DetailShell, EmbedPreview, EmptyState, ErrorBoundary, InlineListSection, LicenceSeats, MetaBar, MetaItem, Meter, PageHeader, PasswordRequirements, Progress, Region, Spinner, StatusBar, ToolCall); missing semantic 16; events 16; pointer 16; keyboard_focus 12; accessibility 19; visual 20
- drag-resize-reorder: 16 components (AppHeader, BlockEditor, Card, CardRadioGroup, CardToggleGroup, DockRegion, DragNumberField, EditableList, FileUpload, ListCard, MarkdownEditor, ModelCatalogueEditor, Separator, SidebarNav, SplitView, TokenInput); missing semantic 9; events 8; pointer 11; keyboard_focus 10; accessibility 11; visual 14
- form-editing: 13 components (AgentQuestionRecord, Calendar, EmbedInput, Fader, FormActions, FormLayout, Keyboard, Knob, LicenceActivation, NumberInput, Rating, Stepper, ValidationSummary); missing semantic 7; events 6; pointer 8; keyboard_focus 7; accessibility 8; visual 13
- general-composite: 9 components (Collapsible, EnvelopeEditor, FieldSet, LicenceStatus, ModMatrixGrid, ModelConnectionCard, Spacer, Stack, WaveformDisplay); missing semantic 6; events 7; pointer 5; keyboard_focus 6; accessibility 6; visual 9
- text-display: 8 components (Code, Eyebrow, Icon, IconProvider, MediaPreview, MetricTile, StateTile, TextLink); missing semantic 7; events 4; pointer 6; keyboard_focus 6; accessibility 7; visual 7
- layout-primitive: 6 components (Box, Grid, ListGrid, Skeleton, TimeAgo, Tooltip); missing semantic 6; events 2; pointer 2; keyboard_focus 3; accessibility 6; visual 6
- media-data: 5 components (AudioMeter, AudioSwitch, GainReductionMeter, ValueReadout, XYPad); missing semantic 4; events 4; pointer 4; keyboard_focus 4; accessibility 4; visual 5
- workstation-shell: 3 components (DetailSection, ScrollShell, UiPresentationProvider); missing semantic 3; events 2; pointer 3; keyboard_focus 3; accessibility 3; visual 3
- agent-composites: 2 components (AgentMessage, MotionPolicyProvider); missing semantic 2; events 2; pointer 2; keyboard_focus 2; accessibility 2; visual 2
