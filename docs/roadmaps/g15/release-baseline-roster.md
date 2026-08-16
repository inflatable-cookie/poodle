# g15 — Release Baseline Roster (frozen v0.2.0 denominator)

Status: complete — measured by `g15.001`
Date: 2026-08-16
Card: `docs/roadmaps/g15/001-release-baseline-roster-inventory.md`
Governing refs: `docs/roadmaps/g15/001-release-baseline-roster-inventory.md`, `docs/roadmaps/g14/022-generation-closeout.md`, `docs/contracts/001-working-rules.md`, `docs/roadmaps/g14/conformance-estate.md`

## Denominator

The v0.2.0 release denominator is **175 public Svelte component exports**, enumerated mechanically from `export { default as <Name> } from "./<Name>.svelte"` in `packages/svelte/components/src/index.ts` (175 matches) and verified one-to-one against the component files (175/175 `.svelte` files present). Packed reachability: the package `exports` map exposes `.` (index), `./*.svelte` (per-file), and `./types`; the `files` array ships `src`, so every component is reachable from the packed tarball both through the index and its per-file subpath.

Public types and helpers are recorded separately and are **not** part of the denominator: the `types` block and the `file-upload`, `theme-controller`, `date`, `presentation`, `anchored`, `portal`, `embed-input`, `media-workflow`, `persistence`, and `icon-registry` exports. The canonical preview catalogue (174 portable slugs plus the web-only `meter-surface`) maps one-to-one onto this roster.

| Surface | present | missing | not-applicable |
| --- | ---: | ---: | ---: |
| Implementation file present (`src/<Name>.svelte`) | 175 | 0 | 0 |
| Export from index + packed `exports` map | 175 | 0 | 0 |
| Contract (`docs/contracts/components/<name>.md`) | 175 | 0 | 0 |
| Svelte preview specimen (dedicated file or scene-shared) | 175 | 0 | 0 |
| Focused Svelte test (named file/case, beyond anatomy smoke) | 116 | 59 | 0 |
| React implementation + export | 173 | 2 | 0 |
| React gallery specimen | 169 | 6 | 0 |
| Focused React test | 112 | 63 | 0 |
| Rust declaration (`<Name>Spec`, including documented aliases) | 163 | 11 | 1 |
| Rust render module (`poodle-render`) | 161 | 13 | 1 |
| GPUI specimen | 145 | 29 | 1 |
| `test:web-pack-install` Svelte mounted proof | 9 | 166 (not exercised) | 0 |
| Downstream consumer use (16 canonical consumers scanned) | 110 | 65 (no use found) | 0 |
| Jetstream | 0 (program-deferred) | — | — |

`not-applicable` is exactly one component on exactly one axis each: `MeterSurface` is web-only by fixed decision (spec 068) and has no Rust declaration, Rust render, or GPUI counterpart. It still counts as a member of the denominator (exported, contracted, implemented, specified in Svelte) and its `not-applicable` rows are recorded as such, not as missing or present.

## Count Method (reproducible)

- **Implementation / Export**: 175 `export { default as <Name> } from "./<Name>.svelte"` lines in `packages/svelte/components/src/index.ts`, each matched to a file of the same name; package `exports` map and `files` array checked once for packed reachability.
- **Contract**: one `docs/contracts/components/<kebab>.md` per component (kebab-case from the export name); 175 of 175 present, verified by direct file check.
- **Specimen**: keys of `specimenMap` in `packages/svelte/preview/src/specimens/registry.ts` against the canonical slugs (174 portable + web-only `meter-surface`); 175 entries. 168 map to a dedicated `*Specimen.svelte`; 7 map to a shared specimen (5 `SceneSpecimen`, 1 `ListCardSpecimen` for `ListCardCounter`, 1 `MetaBarSpecimen` for `MetaItem`).
- **Focused Svelte test**: component imports resolved across all files in `packages/svelte/components/test/` (`.test.ts` and harness `.svelte` files); a component counts when at least one named test file mounts and asserts it beyond the anatomy smoke. 116 count; 59 record `missing` (smoke-only).
- **React implementation/export**: named component exports in `packages/react/components/src/index.ts` (173); React gallery: `specimen-map.ts` keys against canonical slugs (169); focused React test: same import-resolution method over `packages/react/components/test/` (112; 63 missing).
- **Rust declaration**: `pub struct <Name>Spec` searched recursively in `packages/contracts/components/src` (163). Three documented naming discrepancies count as present: `CallOutSpec` (`Callout`), `ShellStatusBarSpec` (`StatusBar`), `TimeFieldSpec` (`TimeInput`). `MeterSurface` has no declaration and records not-applicable per spec 068.
- **Rust render**: module names in `packages/render/src/lib.rs` (161). Documented naming discrepancies count as present: `bx.rs` (`Box`), `shell_status_bar.rs` (`StatusBar`), `time_field.rs` (`TimeInput`), and the batched `audio.rs` covering the 12 audio widgets (13 audio components minus `MeterSurface`). `MeterSurface` records not-applicable.
- **GPUI specimen**: file presence in `packages/gpui/preview/src/specimens/` per component (145). The batched `audio_controls.rs` covers 12 audio widgets; the 12 audio widgets are those covered — `audio_controls.rs` has no `meter_surface` function and `MeterSurface` records not-applicable. Counts do not include the `mod.rs` dispatch fallback (`missing_specimen`).
- **Pack-install**: components listed in the `mountedProof.svelte.components` array of `test/package-install/web-preview.ts` (9).
- **Downstream use**: import statements of `@inflatable-cookie/poodle-svelte` / `-react` resolved (single- and multi-line) across source files of the 16 canonical consumers under `~/Dev/projects`: acowtancy, bovine-accelerator-desktop, compli-me, composer, contact-patch, figmatic, finch, longhorn, loophole, loophole-legacy, nucleus, songsprout, soundcheck, soundcheck-library, underlay, underlay-reference. Excluded: `poodle` itself (source), `jetstream` (program-deferred), worktree/absorbed duplicates (e.g. `soundcheck-wt`, `acowtancy/dairy-card011-worktree`), vendored/build/generated/fixture/example/archive paths, and test directories. No canonical consumer imports `poodle-react`; all component imports resolve to `poodle-svelte`.

## Posture Legend

Per-surface posture is `complete` / `partial` / `missing` / `not-applicable`, always with exact evidence. `missing` is recorded from direct inspection of the tree; no posture is inferred from another runtime's pass. The Svelte anatomy smoke (`packages/svelte/components/test/smoke.test.ts`) generates one named mount case per component through a module glob (all but `IconProvider`, excluded with a recorded reason); it is board-level health and is not counted as focused evidence.

Rust declarations use the documented naming discrepancies where they exist: `CallOutSpec` for `Callout`, `ShellStatusBarSpec` for `StatusBar`, `TimeFieldSpec` for `TimeInput` (each self-documented in the declaration file header).

## Svelte Denominator Surfaces (per component)

### Foundations (primitives)

| Component | Contract | Specimen | Focused Svelte test | Pack-install | Downstream use |
| --- | --- | --- | --- | --- | --- |
| `Accordion` | `docs/contracts/components/accordion.md` | `AccordionSpecimen.svelte` | `Accordion.test.ts` | `missing` | `finch` |
| `AgentChatInput` | `docs/contracts/components/agent-chat-input.md` | `AgentChatInputSpecimen.svelte` | `AgentChatInput.test.ts` | `missing` | no consumer use found (absence is not a release failure) |
| `AudioPlayer` | `docs/contracts/components/audio-player.md` | `AudioPlayerSpecimen.svelte` | `AudioPlayer.test.ts` | `missing` | `acowtancy` |
| `AlertDialog` | `docs/contracts/components/alert-dialog.md` | `AlertDialogSpecimen.svelte` | `AlertDialog.test.ts` | `missing` | `acowtancy`, `compli-me`, `contact-patch`, `underlay`, `underlay-reference` |
| `Avatar` | `docs/contracts/components/avatar.md` | shared `SceneSpecimen.svelte` (generated scene) | `Avatar.test.ts` | `missing` | `acowtancy` |
| `Box` | `docs/contracts/components/box.md` | `BoxSpecimen.svelte` | `Box.test.ts` | `missing` | `acowtancy` |
| `Breadcrumbs` | `docs/contracts/components/breadcrumbs.md` | `BreadcrumbsSpecimen.svelte` | `Breadcrumbs.test.ts` | `missing` | `acowtancy`, `underlay` |
| `BulkActionBar` | `docs/contracts/components/bulk-action-bar.md` | `BulkActionBarSpecimen.svelte` | `BulkActionBar.test.ts` | `missing` | `acowtancy`, `underlay` |
| `Button` | `docs/contracts/components/button.md` | `ButtonSpecimen.svelte` | `Button.test.ts` | `missing` | `acowtancy`, `bovine-accelerator-desktop`, `compli-me`, `composer`, `contact-patch`, `figmatic`, `finch`, `longhorn`, `loophole`, `loophole-legacy`, `nucleus`, `songsprout`, `soundcheck`, `soundcheck-library`, `underlay`, `underlay-reference` |
| `Callout` | `docs/contracts/components/callout.md` | shared `SceneSpecimen.svelte` (generated scene) | `Callout.test.ts` | `missing` | `acowtancy`, `compli-me`, `composer`, `contact-patch`, `longhorn`, `loophole-legacy`, `nucleus`, `songsprout`, `soundcheck`, `soundcheck-library`, `underlay`, `underlay-reference` |
| `RemediationBanner` | `docs/contracts/components/remediation-banner.md` | `RemediationBannerSpecimen.svelte` | `WebParityCloseout.test.ts` | `missing` | no consumer use found (absence is not a release failure) |
| `Card` | `docs/contracts/components/card.md` | `CardSpecimen.svelte` | `Card.test.ts` | `missing` | `acowtancy`, `compli-me`, `composer`, `contact-patch`, `songsprout`, `underlay`, `underlay-reference` |
| `Code` | `docs/contracts/components/code.md` | `CodeSpecimen.svelte` | `Code.test.ts` | `missing` | `acowtancy`, `compli-me`, `composer`, `contact-patch`, `underlay`, `underlay-reference` |
| `ColorPicker` | `docs/contracts/components/color-picker.md` | `ColorPickerSpecimen.svelte` | `ColorPicker.test.ts` | `missing` | `underlay-reference` |
| `Checkbox` | `docs/contracts/components/checkbox.md` | `CheckboxSpecimen.svelte` | `interactions.test.ts` | `missing` | `finch`, `soundcheck`, `soundcheck-library`, `underlay` |
| `Calendar` | `docs/contracts/components/calendar.md` | `CalendarSpecimen.svelte` | `Calendar.test.ts` | `missing` | no consumer use found (absence is not a release failure) |
| `ContextMenu` | `docs/contracts/components/context-menu.md` | `ContextMenuSpecimen.svelte` | `ContextMenu.test.ts` | `missing` | `figmatic`, `loophole-legacy`, `soundcheck-library` |
| `CollapseToggle` | `docs/contracts/components/collapse-toggle.md` | `CollapseToggleSpecimen.svelte` | `CollapseToggle.test.ts` | `missing` | no consumer use found (absence is not a release failure) |
| `Collapsible` | `docs/contracts/components/collapsible.md` | `CollapsibleSpecimen.svelte` | `Collapsible.test.ts` | `missing` | `acowtancy` |
| `DetailItem` | `docs/contracts/components/detail-item.md` | `DetailItemSpecimen.svelte` | `DetailItem.test.ts` | `missing` | `acowtancy`, `composer`, `contact-patch`, `longhorn`, `nucleus`, `underlay`, `underlay-reference` |
| `DatePicker` | `docs/contracts/components/date-picker.md` | `DatePickerSpecimen.svelte` | `DatePicker.test.ts` | `missing` | no consumer use found (absence is not a release failure) |
| `DateRangePicker` | `docs/contracts/components/date-range-picker.md` | `DateRangePickerSpecimen.svelte` | `DateRangePicker.test.ts` | `missing` | no consumer use found (absence is not a release failure) |
| `DateTimePicker` | `docs/contracts/components/date-time-picker.md` | `DateTimePickerSpecimen.svelte` | `DateTimePicker.test.ts` | `missing` | no consumer use found (absence is not a release failure) |
| `DateTimeRangePicker` | `docs/contracts/components/date-time-range-picker.md` | `DateTimeRangePickerSpecimen.svelte` | `DateTimeRangePicker.test.ts` | `missing` | no consumer use found (absence is not a release failure) |
| `Dialog` | `docs/contracts/components/dialog.md` | `DialogSpecimen.svelte` | `DialogControlled.svelte.test.ts`, `DialogDismissOutside.svelte.test.ts`, `DialogInitialFocus.svelte.test.ts`, `PopoverInDialog.svelte.test.ts` | `missing` | `acowtancy`, `finch`, `longhorn`, `loophole-legacy`, `nucleus`, `soundcheck-library`, `underlay`, `underlay-reference` |
| `Drawer` | `docs/contracts/components/drawer.md` | `DrawerSpecimen.svelte` | `DrawerDismissOutside.svelte.test.ts` | `missing` | `acowtancy`, `underlay`, `underlay-reference` |
| `DurationInput` | `docs/contracts/components/duration-input.md` | `DurationInputSpecimen.svelte` | `DurationInput.test.ts` | `missing` | `acowtancy` |
| `EditableLabel` | `docs/contracts/components/editable-label.md` | `EditableLabelSpecimen.svelte` | `EditableLabel.test.ts` | `missing` | `loophole`, `loophole-legacy`, `nucleus`, `soundcheck-library` |
| `Eyebrow` | `docs/contracts/components/eyebrow.md` | `EyebrowSpecimen.svelte` | `Eyebrow.test.ts` | `missing` | `acowtancy` |
| `Field` | `docs/contracts/components/field.md` | `FieldSpecimen.svelte` | `Field.test.ts` | `missing` | `acowtancy`, `compli-me`, `composer`, `contact-patch`, `longhorn`, `loophole-legacy`, `songsprout`, `underlay`, `underlay-reference` |
| `FieldSet` | `docs/contracts/components/field-set.md` | `FieldSetSpecimen.svelte` | `FieldSet.test.ts` | `missing` | `acowtancy`, `composer`, `contact-patch`, `underlay-reference` |
| `FileUpload` | `docs/contracts/components/file-upload.md` | `FileUploadSpecimen.svelte` | `FileUpload.test.ts` | `missing` | `underlay` |
| `FilterBuilder` | `docs/contracts/components/filter-builder.md` | `FilterBuilderSpecimen.svelte` | `FilterBuilder.test.ts` | `missing` | no consumer use found (absence is not a release failure) |
| `FormActions` | `docs/contracts/components/form-actions.md` | `FormActionsSpecimen.svelte` | `FormActions.test.ts` | `missing` | `acowtancy`, `compli-me`, `composer`, `contact-patch`, `longhorn`, `songsprout`, `underlay`, `underlay-reference` |
| `Grid` | `docs/contracts/components/grid.md` | `GridSpecimen.svelte` | `Grid.test.ts` | `missing` | `acowtancy`, `compli-me`, `loophole-legacy`, `songsprout`, `underlay`, `underlay-reference` |
| `HoverCard` | `docs/contracts/components/hover-card.md` | `HoverCardSpecimen.svelte` | `HoverCard.test.ts` | `missing` | no consumer use found (absence is not a release failure) |
| `Icon` | `docs/contracts/components/icon.md` | `IconSpecimen.svelte` | `IconProviderHarness.svelte` | `missing` | `acowtancy`, `loophole-legacy`, `nucleus`, `soundcheck`, `soundcheck-library`, `underlay` |
| `IconButton` | `docs/contracts/components/icon-button.md` | `IconButtonSpecimen.svelte` | `IconButton.test.ts` | `missing` | `acowtancy`, `compli-me`, `composer`, `figmatic`, `loophole-legacy`, `nucleus`, `soundcheck`, `soundcheck-library`, `underlay`, `underlay-reference` |
| `IconProvider` | `docs/contracts/components/icon-provider.md` | `IconProviderSpecimen.svelte` | `IconProviderHarness.svelte` | `missing` | `acowtancy`, `compli-me`, `composer`, `figmatic`, `loophole-legacy`, `nucleus`, `underlay-reference` |
| `Meter` | `docs/contracts/components/meter.md` | `MeterSpecimen.svelte` | `Meter.test.ts` | `missing` | `soundcheck-library` |
| `ListCard` | `docs/contracts/components/list-card.md` | `ListCardSpecimen.svelte` | `ListCard.test.ts` | `missing` | `figmatic`, `underlay`, `underlay-reference` |
| `ListCardCounter` | `docs/contracts/components/list-card-counter.md` | shared specimen (composed inside hosting specimen) | `ListCardCounter.test.ts` | `missing` | `underlay` |
| `ListGrid` | `docs/contracts/components/list-grid.md` | `ListGridSpecimen.svelte` | `ListGrid.test.ts` | `missing` | `acowtancy`, `underlay` |
| `Menu` | `docs/contracts/components/menu.md` | `MenuSpecimen.svelte` | `Menu.test.ts`, `OverlayGeometry.svelte.test.ts` | `missing` | `figmatic`, `loophole-legacy`, `nucleus`, `soundcheck-library`, `underlay` |
| `MetaBar` | `docs/contracts/components/meta-bar.md` | `MetaBarSpecimen.svelte` | `MetaBar.test.ts` | `missing` | `acowtancy`, `compli-me`, `composer`, `contact-patch`, `underlay`, `underlay-reference` |
| `MetaItem` | `docs/contracts/components/meta-item.md` | shared specimen (composed inside hosting specimen) | `MetaBar.test.ts` | `missing` | `acowtancy`, `compli-me`, `composer`, `contact-patch`, `underlay`, `underlay-reference` |
| `NumberInput` | `docs/contracts/components/number-input.md` | `NumberInputSpecimen.svelte` | `NumberInput.test.ts` | `missing` | `acowtancy`, `underlay` |
| `OrderBy` | `docs/contracts/components/order-by.md` | `OrderBySpecimen.svelte` | `OrderBy.test.ts` | `missing` | `acowtancy` |
| `NavCard` | `docs/contracts/components/nav-card.md` | `NavCardSpecimen.svelte` | `NavCard.test.ts` | `missing` | `acowtancy`, `compli-me`, `composer`, `songsprout`, `underlay` |
| `NavigationMenu` | `docs/contracts/components/navigation-menu.md` | `NavigationMenuSpecimen.svelte` | `NavigationMenu.test.ts` | `missing` | no consumer use found (absence is not a release failure) |
| `Pill` | `docs/contracts/components/pill.md` | shared `SceneSpecimen.svelte` (generated scene) | `Pill.test.ts` | `missing` | `acowtancy`, `compli-me`, `composer`, `contact-patch`, `loophole-legacy`, `songsprout`, `soundcheck`, `soundcheck-library`, `underlay`, `underlay-reference` |
| `CodeInput` | `docs/contracts/components/code-input.md` | `CodeInputSpecimen.svelte` | `CodeInput.test.ts` | `missing` | `acowtancy`, `compli-me`, `contact-patch`, `underlay`, `underlay-reference` |
| `Popover` | `docs/contracts/components/popover.md` | `PopoverSpecimen.svelte` | `OverlayGeometry.svelte.test.ts`, `PopoverInDialog.svelte.test.ts`, `PopoverRetained.svelte.test.ts` | `missing` | `acowtancy`, `loophole-legacy`, `nucleus`, `underlay` |
| `Pagination` | `docs/contracts/components/pagination.md` | `PaginationSpecimen.svelte` | `Pagination.test.ts` | `missing` | `underlay` |
| `PaginationSummary` | `docs/contracts/components/pagination-summary.md` | `PaginationSummarySpecimen.svelte` | `PaginationSummary.test.ts` | `missing` | no consumer use found (absence is not a release failure) |
| `PasswordRequirements` | `docs/contracts/components/password-requirements.md` | `PasswordRequirementsSpecimen.svelte` | `PasswordRequirements.test.ts` | `missing` | `underlay` |
| `Progress` | `docs/contracts/components/progress.md` | `ProgressSpecimen.svelte` | `Progress.test.ts` | `missing` | `acowtancy`, `soundcheck`, `underlay`, `underlay-reference` |
| `Radio` | `docs/contracts/components/radio.md` | `RadioSpecimen.svelte` | `Radio.test.ts` | `missing` | `loophole-legacy` |
| `RefSelect` | `docs/contracts/components/ref-select.md` | `RefSelectSpecimen.svelte` | `RefSelect.test.ts` | `missing` | no consumer use found (absence is not a release failure) |
| `RadioGroup` | `docs/contracts/components/radio-group.md` | `RadioGroupSpecimen.svelte` | `RadioGroup.test.ts` | `missing` | `nucleus` |
| `Rating` | `docs/contracts/components/rating.md` | `RatingSpecimen.svelte` | `Rating.test.ts` | `missing` | no consumer use found (absence is not a release failure) |
| `Region` | `docs/contracts/components/region.md` | `RegionSpecimen.svelte` | `Region.test.ts` | `missing` | no consumer use found (absence is not a release failure) |
| `ResizeHandle` | `docs/contracts/components/resize-handle.md` | `ResizeHandleSpecimen.svelte` | `ResizeHandle.test.ts` | `missing` | no consumer use found (absence is not a release failure) |
| `RangeSlider` | `docs/contracts/components/range-slider.md` | `RangeSliderSpecimen.svelte` | `RangeSlider.test.ts` | `missing` | no consumer use found (absence is not a release failure) |
| `SegmentedControl` | `docs/contracts/components/segmented-control.md` | `SegmentedControlSpecimen.svelte` | `SegmentedControl.test.ts` | `missing` | `acowtancy`, `loophole-legacy`, `nucleus` |
| `Select` | `docs/contracts/components/select.md` | `SelectSpecimen.svelte` | `Select.test.ts` | `missing` | `acowtancy`, `bovine-accelerator-desktop`, `composer`, `longhorn`, `loophole`, `loophole-legacy`, `nucleus`, `soundcheck`, `soundcheck-library`, `underlay`, `underlay-reference` |
| `ScrollShell` | `docs/contracts/components/scroll-shell.md` | `ScrollShellSpecimen.svelte` | `ScrollShell.test.ts` | `missing` | no consumer use found (absence is not a release failure) |
| `Separator` | `docs/contracts/components/separator.md` | `SeparatorSpecimen.svelte` | `Separator.test.ts` | `missing` | `acowtancy`, `finch`, `loophole-legacy` |
| `SplitButton` | `docs/contracts/components/split-button.md` | `SplitButtonSpecimen.svelte` | `SplitButton.test.ts` | `missing` | `acowtancy`, `contact-patch`, `underlay`, `underlay-reference` |
| `Skeleton` | `docs/contracts/components/skeleton.md` | `SkeletonSpecimen.svelte` | `Skeleton.test.ts` | `missing` | no consumer use found (absence is not a release failure) |
| `Slider` | `docs/contracts/components/slider.md` | `SliderSpecimen.svelte` | `Slider.test.ts` | `missing` | `acowtancy`, `loophole-legacy` |
| `Spinner` | `docs/contracts/components/spinner.md` | shared `SceneSpecimen.svelte` (generated scene) | `Spinner.test.ts` | `missing` | `acowtancy`, `figmatic`, `soundcheck-library` |
| `Spacer` | `docs/contracts/components/spacer.md` | `SpacerSpecimen.svelte` | `Spacer.test.ts` | `missing` | no consumer use found (absence is not a release failure) |
| `Stack` | `docs/contracts/components/stack.md` | `StackSpecimen.svelte` | `Stack.test.ts` | `missing` | `acowtancy`, `figmatic`, `longhorn`, `underlay` |
| `Stepper` | `docs/contracts/components/stepper.md` | `StepperSpecimen.svelte` | `Stepper.test.ts` | `missing` | no consumer use found (absence is not a release failure) |

### Agent surfaces

| Component | Contract | Specimen | Focused Svelte test | Pack-install | Downstream use |
| --- | --- | --- | --- | --- | --- |
| `AgentMessage` | `docs/contracts/components/agent-message.md` | `AgentMessageSpecimen.svelte` | `missing` — anatomy smoke only | `missing` | no consumer use found (absence is not a release failure) |
| `AgentPlan` | `docs/contracts/components/agent-plan.md` | `AgentPlanSpecimen.svelte` | `AgentChatInputPlan.svelte.test.ts` | `missing` | no consumer use found (absence is not a release failure) |
| `AgentPlanRecord` | `docs/contracts/components/agent-plan-record.md` | `AgentPlanRecordSpecimen.svelte` | `missing` — anatomy smoke only | `missing` | no consumer use found (absence is not a release failure) |
| `AgentQuestion` | `docs/contracts/components/agent-question.md` | `AgentQuestionSpecimen.svelte` | `missing` — anatomy smoke only | `missing` | no consumer use found (absence is not a release failure) |
| `AgentQuestionRecord` | `docs/contracts/components/agent-question-record.md` | `AgentQuestionRecordSpecimen.svelte` | `missing` — anatomy smoke only | `missing` | no consumer use found (absence is not a release failure) |
| `AgentSubagent` | `docs/contracts/components/agent-subagent.md` | `AgentSubagentSpecimen.svelte` | `missing` — anatomy smoke only | `missing` | no consumer use found (absence is not a release failure) |
| `AgentTranscript` | `docs/contracts/components/agent-transcript.md` | `AgentTranscriptSpecimen.svelte` | `AgentTranscriptSubagent.svelte.test.ts` | `missing` | no consumer use found (absence is not a release failure) |
| `ChangedFiles` | `docs/contracts/components/changed-files.md` | `ChangedFilesSpecimen.svelte` | `missing` — anatomy smoke only | `missing` | no consumer use found (absence is not a release failure) |
| `ToolCall` | `docs/contracts/components/tool-call.md` | `ToolCallSpecimen.svelte` | `missing` — anatomy smoke only | `missing` | no consumer use found (absence is not a release failure) |
| `ToolCallGroup` | `docs/contracts/components/tool-call-group.md` | `ToolCallGroupSpecimen.svelte` | `missing` — anatomy smoke only | `missing` | no consumer use found (absence is not a release failure) |

### Workstation systems

| Component | Contract | Specimen | Focused Svelte test | Pack-install | Downstream use |
| --- | --- | --- | --- | --- | --- |
| `StatusBar` | `docs/contracts/components/status-bar.md` | `StatusBarSpecimen.svelte` | `missing` — anatomy smoke only | `missing` | no consumer use found (absence is not a release failure) |
| `StatusIndicator` | `docs/contracts/components/status-indicator.md` | `StatusIndicatorSpecimen.svelte` | `missing` — anatomy smoke only | `missing` | `finch`, `longhorn`, `underlay` |
| `Surface` | `docs/contracts/components/surface.md` | `SurfaceSpecimen.svelte` | `missing` — anatomy smoke only | `missing` | `acowtancy`, `longhorn`, `loophole-legacy`, `nucleus` |
| `Switch` | `docs/contracts/components/switch.md` | `SwitchSpecimen.svelte` | `interactions.test.ts` | `missing` | `acowtancy`, `bovine-accelerator-desktop`, `loophole`, `loophole-legacy`, `nucleus` |
| `Text` | `docs/contracts/components/text.md` | `TextSpecimen.svelte` | `missing` — anatomy smoke only | `missing` | `acowtancy`, `figmatic`, `nucleus` |
| `TextLink` | `docs/contracts/components/text-link.md` | `TextLinkSpecimen.svelte` | `missing` — anatomy smoke only | `missing` | `acowtancy`, `underlay` |
| `Tabs` | `docs/contracts/components/tabs.md` | `TabsSpecimen.svelte` | `interactions.test.ts` | `missing` | `acowtancy`, `compli-me`, `contact-patch`, `figmatic`, `longhorn`, `loophole-legacy`, `nucleus`, `songsprout`, `soundcheck-library`, `underlay`, `underlay-reference` |
| `Table` | `docs/contracts/components/table.md` | `TableSpecimen.svelte` | `missing` — anatomy smoke only | `missing` | `longhorn` |
| `TimeAgo` | `docs/contracts/components/time-ago.md` | `TimeAgoSpecimen.svelte` | `TimeAgo.test.ts` | `missing` | `acowtancy`, `compli-me`, `composer`, `contact-patch`, `underlay`, `underlay-reference` |
| `TextInput` | `docs/contracts/components/text-input.md` | `TextInputSpecimen.svelte` | `TextInput.test.ts` | `missing` | `acowtancy`, `compli-me`, `composer`, `contact-patch`, `longhorn`, `loophole-legacy`, `nucleus`, `songsprout`, `soundcheck`, `soundcheck-library`, `underlay`, `underlay-reference` |
| `TokenInput` | `docs/contracts/components/token-input.md` | `TokenInputSpecimen.svelte` | `missing` — anatomy smoke only | `missing` | `acowtancy` |
| `TimeInput` | `docs/contracts/components/time-input.md` | `TimeInputSpecimen.svelte` | `missing` — anatomy smoke only | `missing` | no consumer use found (absence is not a release failure) |
| `TimeZoneSelect` | `docs/contracts/components/time-zone-select.md` | `TimeZoneSelectSpecimen.svelte` | `missing` — anatomy smoke only | `missing` | no consumer use found (absence is not a release failure) |
| `ThemeSelect` | `docs/contracts/components/theme-select.md` | `ThemeSelectSpecimen.svelte` | `ThemeSelect.test.ts` | `missing` | no consumer use found (absence is not a release failure) |
| `ToggleGroup` | `docs/contracts/components/toggle-group.md` | `ToggleGroupSpecimen.svelte` | `missing` — anatomy smoke only | `missing` | no consumer use found (absence is not a release failure) |
| `Toolbar` | `docs/contracts/components/toolbar.md` | `ToolbarSpecimen.svelte` | `missing` — anatomy smoke only | `missing` | `loophole-legacy`, `soundcheck-library` |
| `Tooltip` | `docs/contracts/components/tooltip.md` | `TooltipSpecimen.svelte` | `missing` — anatomy smoke only | `missing` | `acowtancy` |
| `TriStateSwitch` | `docs/contracts/components/tri-state-switch.md` | `TriStateSwitchSpecimen.svelte` | `missing` — anatomy smoke only | `missing` | no consumer use found (absence is not a release failure) |
| `Menubar` | `docs/contracts/components/menubar.md` | `MenubarSpecimen.svelte` | `Menubar.test.ts` | `missing` | no consumer use found (absence is not a release failure) |
| `UiPresentationProvider` | `docs/contracts/components/ui-presentation-provider.md` | `UiPresentationProviderSpecimen.svelte` | `missing` — anatomy smoke only | `missing` | `loophole`, `loophole-legacy` |
| `VideoPlayer` | `docs/contracts/components/video-player.md` | `VideoPlayerSpecimen.svelte` | `missing` — anatomy smoke only | `missing` | no consumer use found (absence is not a release failure) |
| `DateTimeZonePicker` | `docs/contracts/components/date-time-zone-picker.md` | `DateTimeZonePickerSpecimen.svelte` | `missing` — anatomy smoke only | `missing` | no consumer use found (absence is not a release failure) |

### Composites

| Component | Contract | Specimen | Focused Svelte test | Pack-install | Downstream use |
| --- | --- | --- | --- | --- | --- |
| `ActionDiscoveryPanel` | `docs/contracts/components/action-discovery-panel.md` | `ActionDiscoveryPanelSpecimen.svelte` | `missing` — anatomy smoke only | `missing` | no consumer use found (absence is not a release failure) |
| `AppHeader` | `docs/contracts/components/app-header.md` | `AppHeaderSpecimen.svelte` | `AppHeader.svelte.test.ts` | `missing` | `figmatic`, `nucleus` |
| `EditableList` | `docs/contracts/components/editable-list.md` | `EditableListSpecimen.svelte` | `missing` — anatomy smoke only | `missing` | `composer` |
| `ErrorBoundary` | `docs/contracts/components/error-boundary.md` | `ErrorBoundarySpecimen.svelte` | `missing` — anatomy smoke only | `missing` | `acowtancy`, `contact-patch`, `songsprout`, `underlay-reference` |
| `BlockEditor` | `docs/contracts/components/block-editor.md` | `BlockEditorSpecimen.svelte` | `missing` — anatomy smoke only | `missing` | `underlay` |
| `CardRadioGroup` | `docs/contracts/components/card-radio-group.md` | `CardRadioGroupSpecimen.svelte` | `missing` — anatomy smoke only | `missing` | `acowtancy`, `soundcheck-library` |
| `CardToggleGroup` | `docs/contracts/components/card-toggle-group.md` | `CardToggleGroupSpecimen.svelte` | `missing` — anatomy smoke only | `missing` | no consumer use found (absence is not a release failure) |
| `CommandPalette` | `docs/contracts/components/command-palette.md` | `CommandPaletteSpecimen.svelte` | `missing` — anatomy smoke only | `missing` | `longhorn`, `nucleus` |
| `ConfirmAction` | `docs/contracts/components/confirm-action.md` | `ConfirmActionSpecimen.svelte` | `missing` — anatomy smoke only | `missing` | `longhorn`, `nucleus` |
| `DataTable` | `docs/contracts/components/data-table.md` | `DataTableSpecimen.svelte` | `missing` — anatomy smoke only | `missing` | `acowtancy`, `composer`, `underlay` |
| `DetailSectionGroup` | `docs/contracts/components/detail-section-group.md` | `DetailSectionGroupSpecimen.svelte` | `missing` — anatomy smoke only | `missing` | `acowtancy`, `underlay` |
| `DetailSection` | `docs/contracts/components/detail-section.md` | `DetailSectionSpecimen.svelte` | `missing` — anatomy smoke only | `missing` | `acowtancy`, `composer`, `finch`, `underlay` |
| `DockRegion` | `docs/contracts/components/dock-region.md` | `DockRegionSpecimen.svelte` | `DockRegionDragOverGate.svelte.test.ts`, `DockRegionExternalDrag.svelte.test.ts`, `DockRegionTabPassThroughs.svelte.test.ts`, `DockRegionZoneDrop.svelte.test.ts` | `test:web-pack-install` | `longhorn`, `loophole-legacy` |
| `DetailShell` | `docs/contracts/components/detail-shell.md` | `DetailShellSpecimen.svelte` | `missing` — anatomy smoke only | `missing` | `finch` |
| `EmbedInput` | `docs/contracts/components/embed-input.md` | `EmbedInputSpecimen.svelte` | `missing` — anatomy smoke only | `missing` | no consumer use found (absence is not a release failure) |
| `EmbedPreview` | `docs/contracts/components/embed-preview.md` | `EmbedPreviewSpecimen.svelte` | `missing` — anatomy smoke only | `missing` | `acowtancy` |
| `EmptyState` | `docs/contracts/components/empty-state.md` | shared `SceneSpecimen.svelte` (generated scene) | `missing` — anatomy smoke only | `missing` | `acowtancy`, `compli-me`, `composer`, `contact-patch`, `figmatic`, `songsprout`, `soundcheck`, `soundcheck-library`, `underlay`, `underlay-reference` |
| `FilterToolbar` | `docs/contracts/components/filter-toolbar.md` | `FilterToolbarSpecimen.svelte` | `missing` — anatomy smoke only | `missing` | `acowtancy` |
| `FormDialog` | `docs/contracts/components/form-dialog.md` | `FormDialogSpecimen.svelte` | `FormDialogInitialFocusHarness.svelte` | `missing` | `acowtancy`, `contact-patch`, `underlay`, `underlay-reference` |
| `FormLayout` | `docs/contracts/components/form-layout.md` | `FormLayoutSpecimen.svelte` | `missing` — anatomy smoke only | `missing` | `acowtancy`, `composer` |
| `InlineListSection` | `docs/contracts/components/inline-list-section.md` | `InlineListSectionSpecimen.svelte` | `missing` — anatomy smoke only | `missing` | `acowtancy`, `longhorn`, `underlay` |
| `DebugDialog` | `docs/contracts/components/debug-dialog.md` | `DebugDialogSpecimen.svelte` | `missing` — anatomy smoke only | `missing` | no consumer use found (absence is not a release failure) |
| `LicenceActivation` | `docs/contracts/components/licence-activation.md` | `LicenceActivationSpecimen.svelte` | `LicenceActivation.test.ts` | `test:web-pack-install` | `longhorn` |
| `LicenceSeats` | `docs/contracts/components/licence-seats.md` | `LicenceSeatsSpecimen.svelte` | `LicenceSeats.test.ts` | `test:web-pack-install` | `longhorn` |
| `LicenceStatus` | `docs/contracts/components/licence-status.md` | `LicenceStatusSpecimen.svelte` | `LicenceStatus.test.ts` | `test:web-pack-install` | `longhorn` |
| `LogList` | `docs/contracts/components/log-list.md` | `LogListSpecimen.svelte` | `missing` — anatomy smoke only | `missing` | `contact-patch`, `underlay-reference` |
| `ListContainer` | `docs/contracts/components/list-container.md` | `ListContainerSpecimen.svelte` | `missing` — anatomy smoke only | `missing` | `underlay-reference` |
| `MarkdownEditor` | `docs/contracts/components/markdown-editor.md` | `MarkdownEditorSpecimen.svelte` | `missing` — anatomy smoke only | `missing` | `acowtancy`, `underlay`, `underlay-reference` |
| `PageLoading` | `docs/contracts/components/page-loading.md` | `PageLoadingSpecimen.svelte` | `missing` — anatomy smoke only | `missing` | `acowtancy`, `compli-me`, `composer`, `contact-patch`, `underlay`, `underlay-reference` |
| `MediaPicker` | `docs/contracts/components/media-picker.md` | `MediaPickerSpecimen.svelte` | `missing` — anatomy smoke only | `missing` | no consumer use found (absence is not a release failure) |
| `MediaBrowsePanel` | `docs/contracts/components/media-browse-panel.md` | `MediaBrowsePanelSpecimen.svelte` | `missing` — anatomy smoke only | `missing` | no consumer use found (absence is not a release failure) |
| `MediaPreview` | `docs/contracts/components/media-preview.md` | `MediaPreviewSpecimen.svelte` | `missing` — anatomy smoke only | `missing` | no consumer use found (absence is not a release failure) |
| `MediaThumbnail` | `docs/contracts/components/media-thumbnail.md` | `MediaThumbnailSpecimen.svelte` | `missing` — anatomy smoke only | `missing` | `acowtancy`, `underlay` |
| `PageHeader` | `docs/contracts/components/page-header.md` | `PageHeaderSpecimen.svelte` | `missing` — anatomy smoke only | `missing` | `acowtancy`, `compli-me`, `composer`, `contact-patch`, `figmatic`, `songsprout`, `soundcheck-library`, `underlay`, `underlay-reference` |
| `PickerShell` | `docs/contracts/components/picker-shell.md` | `PickerShellSpecimen.svelte` | `missing` — anatomy smoke only | `missing` | no consumer use found (absence is not a release failure) |
| `RelationPicker` | `docs/contracts/components/relation-picker.md` | `RelationPickerSpecimen.svelte` | `missing` — anatomy smoke only | `missing` | `underlay` |
| `SelectionSummary` | `docs/contracts/components/selection-summary.md` | `SelectionSummarySpecimen.svelte` | `missing` — anatomy smoke only | `missing` | no consumer use found (absence is not a release failure) |
| `SettingsShell` | `docs/contracts/components/settings-shell.md` | `SettingsShellSpecimen.svelte` | `SettingsShell.test.ts` | `missing` | `longhorn` |
| `SidebarNav` | `docs/contracts/components/sidebar-nav.md` | `SidebarNavSpecimen.svelte` | `missing` — anatomy smoke only | `missing` | no consumer use found (absence is not a release failure) |
| `Tree` | `docs/contracts/components/tree.md` | `TreeSpecimen.svelte` | `Tree.test.ts` | `missing` | `figmatic` |
| `SplitView` | `docs/contracts/components/split-view.md` | `SplitViewSpecimen.svelte` | `SplitView.svelte.test.ts` | `missing` | `longhorn`, `loophole-legacy`, `nucleus`, `soundcheck-library` |
| `MetricTile` | `docs/contracts/components/metric-tile.md` | `MetricTileSpecimen.svelte` | `missing` — anatomy smoke only | `missing` | `acowtancy`, `compli-me`, `composer`, `contact-patch`, `underlay-reference` |
| `StateTile` | `docs/contracts/components/state-tile.md` | `StateTileSpecimen.svelte` | `WebParityCloseout.test.ts` | `missing` | no consumer use found (absence is not a release failure) |
| `ValidationSummary` | `docs/contracts/components/validation-summary.md` | `ValidationSummarySpecimen.svelte` | `WebParityCloseout.test.ts` | `missing` | no consumer use found (absence is not a release failure) |
| `ModelPicker` | `docs/contracts/components/model-picker.md` | `ModelPickerSpecimen.svelte` | `ModelPicker.test.ts` | `missing` | no consumer use found (absence is not a release failure) |
| `ModelConnectionPicker` | `docs/contracts/components/model-connection-picker.md` | `ModelConnectionPickerSpecimen.svelte` | `ModelConnectionPicker.test.ts` | `test:web-pack-install` | no consumer use found (absence is not a release failure) |
| `ModelConnectionSetup` | `docs/contracts/components/model-connection-setup.md` | `ModelConnectionSetupSpecimen.svelte` | `ModelConnectionSetup.test.ts` | `test:web-pack-install` | no consumer use found (absence is not a release failure) |
| `ModelConnectionCard` | `docs/contracts/components/model-connection-card.md` | `ModelConnectionCardSpecimen.svelte` | `ModelConnectionCard.test.ts` | `test:web-pack-install` | no consumer use found (absence is not a release failure) |
| `ModelCatalogueEditor` | `docs/contracts/components/model-catalogue-editor.md` | `ModelCatalogueEditorSpecimen.svelte` | `ModelCatalogueEditor.test.ts` | `test:web-pack-install` | no consumer use found (absence is not a release failure) |
| `MessageCenter` | `docs/contracts/components/message-center.md` | `MessageCenterSpecimen.svelte` | `MessageCenter.test.ts` | `missing` | `bovine-accelerator-desktop`, `figmatic`, `nucleus`, `soundcheck` |
| `HistoryCenter` | `docs/contracts/components/history-center.md` | `HistoryCenterSpecimen.svelte` | `HistoryCenter.test.ts` | `missing` | `soundcheck` |
| `UpdateStatus` | `docs/contracts/components/update-status.md` | `UpdateStatusSpecimen.svelte` | `UpdateStatus.test.ts` | `missing` | `longhorn` |
| `UpdateCenter` | `docs/contracts/components/update-center.md` | `UpdateCenterSpecimen.svelte` | `UpdateCenter.test.ts` | `missing` | `longhorn` |
| `ToastStack` | `docs/contracts/components/toast-stack.md` | `ToastStackSpecimen.svelte` | `missing` — anatomy smoke only | `missing` | `longhorn` |
| `ToastHost` | `docs/contracts/components/toast-host.md` | `ToastHostSpecimen.svelte` | `missing` — anatomy smoke only | `missing` | `acowtancy`, `compli-me`, `contact-patch`, `longhorn`, `nucleus`, `songsprout`, `underlay`, `underlay-reference` |

### Audio controls

| Component | Contract | Specimen | Focused Svelte test | Pack-install | Downstream use |
| --- | --- | --- | --- | --- | --- |
| `AudioMeter` | `docs/contracts/components/audio-meter.md` | `AudioMeterSpecimen.svelte` | `AudioControls.svelte.test.ts`, `MeterSurface.svelte.test.ts` | `missing` | `loophole` |
| `MeterSurface` | `docs/contracts/components/meter-surface.md` | `MeterSurfaceSpecimen.svelte` | `MeterSurface.svelte.test.ts` | `test:web-pack-install` | no consumer use found (absence is not a release failure) |
| `AudioSwitch` | `docs/contracts/components/audio-switch.md` | `AudioSwitchSpecimen.svelte` | `AudioControls.svelte.test.ts` | `missing` | no consumer use found (absence is not a release failure) |
| `DragNumberField` | `docs/contracts/components/drag-number-field.md` | `DragNumberFieldSpecimen.svelte` | `AudioControls.svelte.test.ts` | `missing` | `loophole` |
| `EnvelopeEditor` | `docs/contracts/components/envelope-editor.md` | `EnvelopeEditorSpecimen.svelte` | `AudioControls.svelte.test.ts` | `missing` | no consumer use found (absence is not a release failure) |
| `Fader` | `docs/contracts/components/fader.md` | `FaderSpecimen.svelte` | `AudioControls.svelte.test.ts` | `missing` | `loophole` |
| `GainReductionMeter` | `docs/contracts/components/gain-reduction-meter.md` | `GainReductionMeterSpecimen.svelte` | `AudioControls.svelte.test.ts` | `missing` | no consumer use found (absence is not a release failure) |
| `Keyboard` | `docs/contracts/components/keyboard.md` | `KeyboardSpecimen.svelte` | `AudioControls.svelte.test.ts` | `missing` | no consumer use found (absence is not a release failure) |
| `Knob` | `docs/contracts/components/knob.md` | `KnobSpecimen.svelte` | `AudioControls.svelte.test.ts` | `missing` | `loophole` |
| `ModMatrixGrid` | `docs/contracts/components/mod-matrix-grid.md` | `ModMatrixGridSpecimen.svelte` | `AudioControls.svelte.test.ts` | `missing` | no consumer use found (absence is not a release failure) |
| `ValueReadout` | `docs/contracts/components/value-readout.md` | `ValueReadoutSpecimen.svelte` | `AudioControls.svelte.test.ts` | `missing` | `loophole` |
| `WaveformDisplay` | `docs/contracts/components/waveform-display.md` | `WaveformDisplaySpecimen.svelte` | `AudioControls.svelte.test.ts` | `missing` | no consumer use found (absence is not a release failure) |
| `XYPad` | `docs/contracts/components/xy-pad.md` | `XYPadSpecimen.svelte` | `AudioControls.svelte.test.ts` | `missing` | no consumer use found (absence is not a release failure) |

## Cross-Runtime Surfaces (per component)

One runtime never borrows another runtime's pass. React mirror posture names implementation+export, gallery specimen, and focused test. Rust declaration and render are recorded independently. GPUI posture names the specimen file; headless regression coverage is listed separately below the table.

### Foundations (primitives)

| Component | React impl/export | React gallery | Focused React test | Rust declaration | Rust render | GPUI specimen |
| --- | --- | --- | --- | --- | --- | --- |
| `Accordion` | complete | complete | `Accordion.test.tsx` | `AccordionSpec` (`packages/contracts/components/src/accordion.rs`) | `packages/render/src/accordion.rs` | `packages/gpui/preview/src/specimens/accordion.rs` |
| `AgentChatInput` | complete | complete | `AgentChatInput.test.tsx` | `AgentChatInputSpec` (`packages/contracts/components/src/agent_chat_input.rs`) | `packages/render/src/agent_chat_input.rs` | `packages/gpui/preview/src/specimens/agent_chat_input_specimen.rs` |
| `AudioPlayer` | complete | complete | `AudioPlayer.test.tsx` | `AudioPlayerSpec` (`packages/contracts/components/src/audio_player.rs`) | `packages/render/src/audio_player.rs` | `packages/gpui/preview/src/specimens/audio_player_specimen.rs` |
| `AlertDialog` | complete | complete | `AlertDialog.test.tsx` | `AlertDialogSpec` (`packages/contracts/components/src/alert_dialog.rs`) | `packages/render/src/alert_dialog.rs` | `packages/gpui/preview/src/specimens/alert_dialog.rs` |
| `Avatar` | complete | complete | `Avatar.test.tsx` | `AvatarSpec` (`packages/contracts/components/src/avatar.rs`) | `packages/render/src/avatar.rs` | `missing` |
| `Box` | complete | complete | `Box.test.tsx` | `BoxSpec` (`packages/contracts/components/src/box.rs`) | `packages/render/src/bx.rs` | `packages/gpui/preview/src/specimens/bx.rs` |
| `Breadcrumbs` | complete | complete | `Breadcrumbs.test.tsx` | `BreadcrumbsSpec` (`packages/contracts/components/src/breadcrumbs.rs`) | `packages/render/src/breadcrumbs.rs` | `packages/gpui/preview/src/specimens/breadcrumbs_specimen.rs` |
| `BulkActionBar` | complete | complete | `BulkActionBar.test.tsx` | `BulkActionBarSpec` (`packages/contracts/components/src/bulk_action_bar.rs`) | `packages/render/src/bulk_action_bar.rs` | `packages/gpui/preview/src/specimens/bulk_action_bar_specimen.rs` |
| `Button` | complete | complete | `Button.test.tsx` | `ButtonSpec` (`packages/contracts/components/src/button.rs`) | `packages/render/src/button.rs` | `packages/gpui/preview/src/specimens/button.rs` |
| `Callout` | complete | complete | `Callout.test.tsx` | `CallOutSpec` (`call_out.rs`, documented rename) | `packages/render/src/callout.rs` | `missing` |
| `RemediationBanner` | complete | complete | `WebParityCloseout.test.tsx` | `RemediationBannerSpec` (`packages/contracts/components/src/remediation_banner.rs`) | `packages/render/src/remediation_banner.rs` | `missing` |
| `Card` | complete | complete | `Card.test.tsx` | `CardSpec` (`packages/contracts/components/src/card.rs`) | `packages/render/src/card.rs` | `packages/gpui/preview/src/specimens/card_specimen.rs` |
| `Code` | complete | complete | `Code.test.tsx` | `CodeSpec` (`packages/contracts/components/src/code.rs`) | `packages/render/src/code.rs` | `packages/gpui/preview/src/specimens/code.rs` |
| `ColorPicker` | complete | complete | `ColorPicker.test.tsx` | `ColorPickerSpec` (`packages/contracts/components/src/color_picker.rs`) | `packages/render/src/color_picker.rs` | `packages/gpui/preview/src/specimens/color_picker.rs` |
| `Checkbox` | complete | complete | `interactions.test.tsx` | `CheckboxSpec` (`packages/contracts/components/src/checkbox.rs`) | `packages/render/src/checkbox.rs` | `packages/gpui/preview/src/specimens/checkbox.rs` |
| `Calendar` | complete | complete | `Calendar.test.tsx` | `CalendarSpec` (`packages/contracts/components/src/calendar.rs`) | `packages/render/src/calendar.rs` | `packages/gpui/preview/src/specimens/calendar.rs` |
| `ContextMenu` | complete | complete | `ContextMenu.test.tsx` | `ContextMenuSpec` (`packages/contracts/components/src/context_menu.rs`) | `packages/render/src/context_menu.rs` | `packages/gpui/preview/src/specimens/context_menu.rs` |
| `CollapseToggle` | complete | complete | `CollapseToggle.test.tsx` | `CollapseToggleSpec` (`packages/contracts/components/src/collapse_toggle.rs`) | `packages/render/src/collapse_toggle.rs` | `packages/gpui/preview/src/specimens/collapse_toggle.rs` |
| `Collapsible` | complete | complete | `Collapsible.test.tsx` | `CollapsibleSpec` (`packages/contracts/components/src/collapsible.rs`) | `packages/render/src/collapsible.rs` | `packages/gpui/preview/src/specimens/collapsible.rs` |
| `DetailItem` | complete | complete | `DetailItem.test.tsx` | `DetailItemSpec` (`packages/contracts/components/src/detail_item.rs`) | `packages/render/src/detail_item.rs` | `packages/gpui/preview/src/specimens/detail_item_specimen.rs` |
| `DatePicker` | complete | complete | `DatePicker.test.tsx` | `DatePickerSpec` (`packages/contracts/components/src/date_picker.rs`) | `packages/render/src/date_picker.rs` | `packages/gpui/preview/src/specimens/date_picker.rs` |
| `DateRangePicker` | complete | complete | `DateRangePicker.test.tsx` | `DateRangePickerSpec` (`packages/contracts/components/src/date_range_picker.rs`) | `packages/render/src/date_range_picker.rs` | `packages/gpui/preview/src/specimens/date_range_picker.rs` |
| `DateTimePicker` | complete | complete | `DateTimePicker.test.tsx` | `DateTimePickerSpec` (`packages/contracts/components/src/date_time_picker.rs`) | `packages/render/src/date_time_picker.rs` | `packages/gpui/preview/src/specimens/date_time_picker.rs` |
| `DateTimeRangePicker` | complete | complete | `DateTimeRangePicker.test.tsx` | `DateTimeRangePickerSpec` (`packages/contracts/components/src/date_time_range_picker.rs`) | `packages/render/src/date_time_range_picker.rs` | `packages/gpui/preview/src/specimens/date_time_range_picker.rs` |
| `Dialog` | complete | complete | `DialogDismissOutside.test.tsx`, `DialogInitialFocus.test.tsx` | `DialogSpec` (`packages/contracts/components/src/dialog.rs`) | `packages/render/src/dialog.rs` | `packages/gpui/preview/src/specimens/dialog.rs` |
| `Drawer` | complete | complete | `DrawerDismissOutside.test.tsx` | `DrawerSpec` (`packages/contracts/components/src/drawer.rs`) | `packages/render/src/drawer.rs` | `packages/gpui/preview/src/specimens/drawer.rs` |
| `DurationInput` | complete | complete | `DurationInput.test.tsx` | `DurationInputSpec` (`packages/contracts/components/src/duration_input.rs`) | `packages/render/src/duration_input.rs` | `packages/gpui/preview/src/specimens/duration_input_specimen.rs` |
| `EditableLabel` | complete | complete | `EditableLabel.test.tsx` | `EditableLabelSpec` (`packages/contracts/components/src/editable_label.rs`) | `packages/render/src/editable_label.rs` | `packages/gpui/preview/src/specimens/editable_label.rs` |
| `Eyebrow` | complete | complete | `Eyebrow.test.tsx` | `EyebrowSpec` (`packages/contracts/components/src/eyebrow.rs`) | `packages/render/src/eyebrow.rs` | `packages/gpui/preview/src/specimens/eyebrow.rs` |
| `Field` | complete | complete | `Field.test.tsx` | `FieldSpec` (`packages/contracts/components/src/field.rs`) | `packages/render/src/field.rs` | `packages/gpui/preview/src/specimens/field.rs` |
| `FieldSet` | complete | complete | `FieldSet.test.tsx` | `FieldSetSpec` (`packages/contracts/components/src/field_set.rs`) | `packages/render/src/field_set.rs` | `packages/gpui/preview/src/specimens/field_set_specimen.rs` |
| `FileUpload` | complete | complete | `FileUpload.test.tsx` | `FileUploadSpec` (`packages/contracts/components/src/file_upload.rs`) | `packages/render/src/file_upload.rs` | `packages/gpui/preview/src/specimens/file_upload.rs` |
| `FilterBuilder` | complete | complete | `FilterBuilder.test.tsx` | `FilterBuilderSpec` (`packages/contracts/components/src/filter_builder.rs`) | `packages/render/src/filter_builder.rs` | `packages/gpui/preview/src/specimens/filter_builder_specimen.rs` |
| `FormActions` | complete | complete | `FormActions.test.tsx` | `FormActionsSpec` (`packages/contracts/components/src/form_actions.rs`) | `packages/render/src/form_actions.rs` | `packages/gpui/preview/src/specimens/form_actions.rs` |
| `Grid` | complete | complete | `Grid.test.tsx` | `GridSpec` (`packages/contracts/components/src/grid.rs`) | `packages/render/src/grid.rs` | `packages/gpui/preview/src/specimens/grid.rs` |
| `HoverCard` | complete | complete | `HoverCard.test.tsx` | `HoverCardSpec` (`packages/contracts/components/src/hover_card.rs`) | `packages/render/src/hover_card.rs` | `packages/gpui/preview/src/specimens/hover_card.rs` |
| `Icon` | complete | complete | `missing` — smoke only | `IconSpec` (`packages/contracts/components/src/icon.rs`) | `packages/render/src/icon.rs` | `packages/gpui/preview/src/specimens/icon.rs` |
| `IconButton` | complete | complete | `IconButton.test.tsx` | `IconButtonSpec` (`packages/contracts/components/src/icon_button.rs`) | `packages/render/src/icon_button.rs` | `packages/gpui/preview/src/specimens/icon_button.rs` |
| `IconProvider` | complete | complete | `missing` — smoke only | `IconProviderSpec` (`packages/contracts/components/src/icon_provider.rs`) | `missing` | `packages/gpui/preview/src/specimens/icon_provider.rs` |
| `Meter` | complete | complete | `Meter.test.tsx` | `MeterSpec` (`packages/contracts/components/src/meter.rs`) | `packages/render/src/meter.rs` | `packages/gpui/preview/src/specimens/meter.rs` |
| `ListCard` | complete | complete | `ListCard.test.tsx` | `ListCardSpec` (`packages/contracts/components/src/list_card.rs`) | `packages/render/src/list_card.rs` | `packages/gpui/preview/src/specimens/list_card.rs` |
| `ListCardCounter` | complete | complete | `ListCardCounter.test.tsx` | `ListCardCounterSpec` (`packages/contracts/components/src/list_card_counter.rs`) | `packages/render/src/list_card_counter.rs` | `packages/gpui/preview/src/specimens/list_card_counter.rs` |
| `ListGrid` | complete | complete | `ListGrid.test.tsx` | `ListGridSpec` (`packages/contracts/components/src/list_grid.rs`) | `packages/render/src/list_grid.rs` | `packages/gpui/preview/src/specimens/list_grid.rs` |
| `Menu` | complete | complete | `Menu.test.tsx`, `OverlayGeometry.test.tsx` | `MenuSpec` (`packages/contracts/components/src/menu.rs`) | `packages/render/src/menu.rs` | `packages/gpui/preview/src/specimens/menu.rs` |
| `MetaBar` | complete | complete | `MetaBar.test.tsx` | `MetaBarSpec` (`packages/contracts/components/src/meta_bar.rs`) | `packages/render/src/meta_bar.rs` | `packages/gpui/preview/src/specimens/meta_bar.rs` |
| `MetaItem` | complete | complete | `MetaBar.test.tsx` | `MetaItemSpec` (`packages/contracts/components/src/meta_item.rs`) | `packages/render/src/meta_item.rs` | `missing` |
| `NumberInput` | complete | complete | `NumberInput.test.tsx` | `NumberInputSpec` (`packages/contracts/components/src/number_input.rs`) | `packages/render/src/number_input.rs` | `packages/gpui/preview/src/specimens/number_input.rs` |
| `OrderBy` | complete | complete | `OrderBy.test.tsx` | `OrderBySpec` (`packages/contracts/components/src/order_by.rs`) | `packages/render/src/order_by.rs` | `packages/gpui/preview/src/specimens/order_by_specimen.rs` |
| `NavCard` | complete | complete | `NavCard.test.tsx` | `NavCardSpec` (`packages/contracts/components/src/nav_card.rs`) | `packages/render/src/nav_card.rs` | `packages/gpui/preview/src/specimens/nav_card.rs` |
| `NavigationMenu` | complete | complete | `NavigationMenu.test.tsx` | `NavigationMenuSpec` (`packages/contracts/components/src/navigation_menu.rs`) | `packages/render/src/navigation_menu.rs` | `packages/gpui/preview/src/specimens/navigation_menu.rs` |
| `Pill` | complete | complete | `Pill.test.tsx` | `PillSpec` (`packages/contracts/components/src/pill.rs`) | `packages/render/src/pill.rs` | `missing` |
| `CodeInput` | complete | complete | `CodeInput.test.tsx` | `CodeInputSpec` (`packages/contracts/components/src/code_input.rs`) | `packages/render/src/code_input.rs` | `packages/gpui/preview/src/specimens/code_input.rs` |
| `Popover` | complete | complete | `OverlayGeometry.test.tsx`, `PopoverRetained.test.tsx` | `PopoverSpec` (`packages/contracts/components/src/popover.rs`) | `packages/render/src/popover.rs` | `packages/gpui/preview/src/specimens/popover.rs` |
| `Pagination` | complete | complete | `Pagination.test.tsx` | `PaginationSpec` (`packages/contracts/components/src/pagination.rs`) | `packages/render/src/pagination.rs` | `packages/gpui/preview/src/specimens/pagination.rs` |
| `PaginationSummary` | complete | complete | `PaginationSummary.test.tsx` | `PaginationSummarySpec` (`packages/contracts/components/src/pagination_summary.rs`) | `packages/render/src/pagination_summary.rs` | `packages/gpui/preview/src/specimens/pagination_summary_specimen.rs` |
| `PasswordRequirements` | complete | complete | `PasswordRequirements.test.tsx` | `PasswordRequirementsSpec` (`packages/contracts/components/src/password_requirements.rs`) | `packages/render/src/password_requirements.rs` | `packages/gpui/preview/src/specimens/password_requirements.rs` |
| `Progress` | complete | complete | `Progress.test.tsx` | `ProgressSpec` (`packages/contracts/components/src/progress.rs`) | `packages/render/src/progress.rs` | `packages/gpui/preview/src/specimens/progress.rs` |
| `Radio` | complete | complete | `Radio.test.tsx` | `missing` | `missing` | `missing` |
| `RefSelect` | complete | complete | `RefSelect.test.tsx` | `RefSelectSpec` (`packages/contracts/components/src/ref_select.rs`) | `packages/render/src/ref_select.rs` | `packages/gpui/preview/src/specimens/ref_select_specimen.rs` |
| `RadioGroup` | complete | complete | `RadioGroup.test.tsx` | `RadioGroupSpec` (`packages/contracts/components/src/radio_group.rs`) | `packages/render/src/radio_group.rs` | `packages/gpui/preview/src/specimens/radio_group.rs` |
| `Rating` | complete | complete | `Rating.test.tsx` | `RatingSpec` (`packages/contracts/components/src/rating.rs`) | `packages/render/src/rating.rs` | `packages/gpui/preview/src/specimens/rating.rs` |
| `Region` | complete | complete | `Region.test.tsx` | `RegionSpec` (`packages/contracts/components/src/region.rs`) | `packages/render/src/region.rs` | `packages/gpui/preview/src/specimens/region.rs` |
| `ResizeHandle` | complete | complete | `ResizeHandle.test.tsx` | `ResizeHandleSpec` (`packages/contracts/components/src/resize_handle.rs`) | `packages/render/src/resize_handle.rs` | `packages/gpui/preview/src/specimens/resize_handle.rs` |
| `RangeSlider` | complete | complete | `RangeSlider.test.tsx` | `RangeSliderSpec` (`packages/contracts/components/src/range_slider.rs`) | `packages/render/src/range_slider.rs` | `packages/gpui/preview/src/specimens/range_slider.rs` |
| `SegmentedControl` | complete | complete | `SegmentedControl.test.tsx` | `SegmentedControlSpec` (`packages/contracts/components/src/segmented_control.rs`) | `packages/render/src/segmented_control.rs` | `packages/gpui/preview/src/specimens/segmented_control.rs` |
| `Select` | complete | complete | `Select.test.tsx` | `SelectSpec` (`packages/contracts/components/src/select.rs`) | `packages/render/src/select.rs` | `packages/gpui/preview/src/specimens/select.rs` |
| `ScrollShell` | complete | complete | `ScrollShell.test.tsx` | `ScrollShellSpec` (`packages/contracts/components/src/scroll_shell.rs`) | `packages/render/src/scroll_shell.rs` | `packages/gpui/preview/src/specimens/scroll_shell.rs` |
| `Separator` | complete | complete | `Separator.test.tsx` | `SeparatorSpec` (`packages/contracts/components/src/separator.rs`) | `packages/render/src/separator.rs` | `packages/gpui/preview/src/specimens/separator.rs` |
| `SplitButton` | complete | complete | `SplitButton.test.tsx` | `SplitButtonSpec` (`packages/contracts/components/src/split_button.rs`) | `packages/render/src/split_button.rs` | `packages/gpui/preview/src/specimens/split_button.rs` |
| `Skeleton` | complete | complete | `Skeleton.test.tsx` | `SkeletonSpec` (`packages/contracts/components/src/skeleton.rs`) | `packages/render/src/skeleton.rs` | `packages/gpui/preview/src/specimens/skeleton.rs` |
| `Slider` | complete | complete | `Slider.test.tsx` | `SliderSpec` (`packages/contracts/components/src/slider.rs`) | `packages/render/src/slider.rs` | `packages/gpui/preview/src/specimens/slider.rs` |
| `Spinner` | complete | complete | `Spinner.test.tsx` | `SpinnerSpec` (`packages/contracts/components/src/spinner.rs`) | `packages/render/src/spinner.rs` | `missing` |
| `Spacer` | complete | complete | `Spacer.test.tsx` | `SpacerSpec` (`packages/contracts/components/src/spacer.rs`) | `packages/render/src/spacer.rs` | `packages/gpui/preview/src/specimens/spacer.rs` |
| `Stack` | complete | complete | `Stack.test.tsx` | `StackSpec` (`packages/contracts/components/src/stack.rs`) | `packages/render/src/stack.rs` | `packages/gpui/preview/src/specimens/stack.rs` |
| `Stepper` | complete | complete | `Stepper.test.tsx` | `StepperSpec` (`packages/contracts/components/src/stepper.rs`) | `packages/render/src/stepper.rs` | `packages/gpui/preview/src/specimens/stepper.rs` |

### Agent surfaces

| Component | React impl/export | React gallery | Focused React test | Rust declaration | Rust render | GPUI specimen |
| --- | --- | --- | --- | --- | --- | --- |
| `AgentMessage` | complete | `missing` | `missing` — smoke only | `AgentMessageSpec` (`packages/contracts/components/src/agent_message.rs`) | `packages/render/src/agent_message.rs` | `missing` |
| `AgentPlan` | `missing` | `missing` | `missing` — smoke only | `AgentPlanSpec` (`packages/contracts/components/src/agent_plan.rs`) | `packages/render/src/agent_plan.rs` | `missing` |
| `AgentPlanRecord` | `missing` | `missing` | `missing` — smoke only | `AgentPlanRecordSpec` (`packages/contracts/components/src/agent_plan_record.rs`) | `packages/render/src/agent_plan_record.rs` | `missing` |
| `AgentQuestion` | complete | complete | `missing` — smoke only | `AgentQuestionSpec` (`packages/contracts/components/src/agent_question.rs`) | `packages/render/src/agent_question.rs` | `packages/gpui/preview/src/specimens/agent_question.rs` |
| `AgentQuestionRecord` | complete | complete | `missing` — smoke only | `AgentQuestionRecordSpec` (`packages/contracts/components/src/agent_question_record.rs`) | `packages/render/src/agent_question_record.rs` | `missing` |
| `AgentSubagent` | complete | complete | `AgentSubagent.test.tsx` | `AgentSubagentSpec` (`packages/contracts/components/src/agent_subagent.rs`) | `packages/render/src/agent_subagent.rs` | `missing` |
| `AgentTranscript` | complete | complete | `AgentSubagent.test.tsx` | `AgentTranscriptSpec` (`packages/contracts/components/src/agent_transcript.rs`) | `packages/render/src/agent_transcript.rs` | `packages/gpui/preview/src/specimens/agent_transcript.rs` |
| `ChangedFiles` | complete | `missing` | `missing` — smoke only | `ChangedFilesSpec` (`packages/contracts/components/src/changed_files.rs`) | `packages/render/src/changed_files.rs` | `missing` |
| `ToolCall` | complete | `missing` | `missing` — smoke only | `ToolCallSpec` (`packages/contracts/components/src/tool_call.rs`) | `packages/render/src/tool_call.rs` | `missing` |
| `ToolCallGroup` | complete | `missing` | `missing` — smoke only | `ToolCallGroupSpec` (`packages/contracts/components/src/tool_call_group.rs`) | `packages/render/src/tool_call_group.rs` | `missing` |

### Workstation systems

| Component | React impl/export | React gallery | Focused React test | Rust declaration | Rust render | GPUI specimen |
| --- | --- | --- | --- | --- | --- | --- |
| `StatusBar` | complete | complete | `missing` — smoke only | `ShellStatusBarSpec` (`shell_status_bar.rs`, documented rename) | `packages/render/src/shell_status_bar.rs` | `packages/gpui/preview/src/specimens/status_bar.rs` |
| `StatusIndicator` | complete | complete | `missing` — smoke only | `StatusIndicatorSpec` (`packages/contracts/components/src/status_indicator.rs`) | `packages/render/src/status_indicator.rs` | `packages/gpui/preview/src/specimens/status_indicator.rs` |
| `Surface` | complete | complete | `missing` — smoke only | `SurfaceSpec` (`packages/contracts/components/src/surface.rs`) | `packages/render/src/surface.rs` | `packages/gpui/preview/src/specimens/surface.rs` |
| `Switch` | complete | complete | `interactions.test.tsx` | `SwitchSpec` (`packages/contracts/components/src/switch.rs`) | `packages/render/src/switch.rs` | `packages/gpui/preview/src/specimens/switch.rs` |
| `Text` | complete | complete | `missing` — smoke only | `TextSpec` (`packages/contracts/components/src/text.rs`) | `packages/render/src/text.rs` | `packages/gpui/preview/src/specimens/text.rs` |
| `TextLink` | complete | complete | `missing` — smoke only | `TextLinkSpec` (`packages/contracts/components/src/text_link.rs`) | `packages/render/src/text_link.rs` | `packages/gpui/preview/src/specimens/text_link.rs` |
| `Tabs` | complete | complete | `TabsRovingFocus.test.tsx` | `TabsSpec` (`packages/contracts/components/src/tabs.rs`) | `packages/render/src/tabs.rs` | `packages/gpui/preview/src/specimens/tabs.rs` |
| `Table` | complete | complete | `missing` — smoke only | `TableSpec` (`packages/contracts/components/src/table.rs`) | `packages/render/src/table.rs` | `packages/gpui/preview/src/specimens/table.rs` |
| `TimeAgo` | complete | complete | `TimeAgo.test.tsx` | `TimeAgoSpec` (`packages/contracts/components/src/time_ago.rs`) | `packages/render/src/time_ago.rs` | `packages/gpui/preview/src/specimens/time_ago_specimen.rs` |
| `TextInput` | complete | complete | `TextInput.test.tsx` | `TextInputSpec` (`packages/contracts/components/src/text_input.rs`) | `packages/render/src/text_input.rs` | `packages/gpui/preview/src/specimens/text_input.rs` |
| `TokenInput` | complete | complete | `missing` — smoke only | `TokenInputSpec` (`packages/contracts/components/src/token_input.rs`) | `packages/render/src/token_input.rs` | `packages/gpui/preview/src/specimens/token_input.rs` |
| `TimeInput` | complete | complete | `missing` — smoke only | `TimeFieldSpec` (`time_field.rs`, documented rename) | `packages/render/src/time_field.rs` | `packages/gpui/preview/src/specimens/time_field.rs` |
| `TimeZoneSelect` | complete | complete | `missing` — smoke only | `TimeZoneSelectSpec` (`packages/contracts/components/src/time_zone_select.rs`) | `packages/render/src/time_zone_select.rs` | `packages/gpui/preview/src/specimens/time_zone_select.rs` |
| `ThemeSelect` | complete | complete | `ThemeSelect.test.tsx` | `ThemeSelectSpec` (`packages/contracts/components/src/theme_select.rs`) | `packages/render/src/theme_select.rs` | `packages/gpui/preview/src/specimens/theme_select_specimen.rs` |
| `ToggleGroup` | complete | complete | `missing` — smoke only | `ToggleGroupSpec` (`packages/contracts/components/src/toggle_group.rs`) | `packages/render/src/toggle_group.rs` | `packages/gpui/preview/src/specimens/toggle_group.rs` |
| `Toolbar` | complete | complete | `missing` — smoke only | `ToolbarSpec` (`packages/contracts/components/src/toolbar.rs`) | `packages/render/src/toolbar.rs` | `packages/gpui/preview/src/specimens/toolbar.rs` |
| `Tooltip` | complete | complete | `missing` — smoke only | `TooltipSpec` (`packages/contracts/components/src/tooltip.rs`) | `packages/render/src/tooltip.rs` | `packages/gpui/preview/src/specimens/tooltip.rs` |
| `TriStateSwitch` | complete | complete | `missing` — smoke only | `TriStateSwitchSpec` (`packages/contracts/components/src/tri_state_switch.rs`) | `packages/render/src/tri_state_switch.rs` | `packages/gpui/preview/src/specimens/tri_state_switch.rs` |
| `Menubar` | complete | complete | `Menubar.test.tsx` | `MenubarSpec` (`packages/contracts/components/src/menubar.rs`) | `packages/render/src/menubar.rs` | `packages/gpui/preview/src/specimens/menubar.rs` |
| `UiPresentationProvider` | complete | complete | `missing` — smoke only | `UiPresentationProviderSpec` (`packages/contracts/components/src/ui_presentation_provider.rs`) | `missing` | `packages/gpui/preview/src/specimens/ui_presentation_provider.rs` |
| `VideoPlayer` | complete | complete | `missing` — smoke only | `VideoPlayerSpec` (`packages/contracts/components/src/video_player.rs`) | `packages/render/src/video_player.rs` | `packages/gpui/preview/src/specimens/video_player_specimen.rs` |
| `DateTimeZonePicker` | complete | complete | `missing` — smoke only | `DateTimeZonePickerSpec` (`packages/contracts/components/src/date_time_zone_picker.rs`) | `packages/render/src/date_time_zone_picker.rs` | `packages/gpui/preview/src/specimens/date_time_zone_picker.rs` |

### Composites

| Component | React impl/export | React gallery | Focused React test | Rust declaration | Rust render | GPUI specimen |
| --- | --- | --- | --- | --- | --- | --- |
| `ActionDiscoveryPanel` | complete | complete | `missing` — smoke only | `ActionDiscoveryPanelSpec` (`packages/contracts/components/src/action_discovery_panel.rs`) | `packages/render/src/action_discovery_panel.rs` | `missing` |
| `AppHeader` | complete | complete | `AppHeader.test.tsx` | `AppHeaderSpec` (`packages/contracts/components/src/app_header.rs`) | `packages/render/src/app_header.rs` | `packages/gpui/preview/src/specimens/app_header.rs` |
| `EditableList` | complete | complete | `missing` — smoke only | `EditableListSpec` (`packages/contracts/components/src/editable_list.rs`) | `packages/render/src/editable_list.rs` | `packages/gpui/preview/src/specimens/editable_list_specimen.rs` |
| `ErrorBoundary` | complete | complete | `missing` — smoke only | `ErrorBoundarySpec` (`packages/contracts/components/src/error_boundary.rs`) | `packages/render/src/error_boundary.rs` | `packages/gpui/preview/src/specimens/error_boundary_specimen.rs` |
| `BlockEditor` | complete | complete | `missing` — smoke only | `BlockEditorSpec` (`packages/contracts/components/src/block_editor.rs`) | `packages/render/src/block_editor.rs` | `packages/gpui/preview/src/specimens/block_editor_specimen.rs` |
| `CardRadioGroup` | complete | complete | `missing` — smoke only | `CardRadioGroupSpec` (`packages/contracts/components/src/card_radio_group.rs`) | `packages/render/src/card_radio_group.rs` | `packages/gpui/preview/src/specimens/card_radio_group_specimen.rs` |
| `CardToggleGroup` | complete | complete | `missing` — smoke only | `CardToggleGroupSpec` (`packages/contracts/components/src/card_toggle_group.rs`) | `packages/render/src/card_toggle_group.rs` | `packages/gpui/preview/src/specimens/card_toggle_group_specimen.rs` |
| `CommandPalette` | complete | complete | `missing` — smoke only | `CommandPaletteSpec` (`packages/contracts/components/src/command_palette.rs`) | `packages/render/src/command_palette.rs` | `packages/gpui/preview/src/specimens/command_palette.rs` |
| `ConfirmAction` | complete | complete | `missing` — smoke only | `ConfirmActionSpec` (`packages/contracts/components/src/confirm_action.rs`) | `packages/render/src/confirm_action.rs` | `packages/gpui/preview/src/specimens/confirm_action_specimen.rs` |
| `DataTable` | complete | complete | `missing` — smoke only | `DataTableSpec` (`packages/contracts/components/src/data_table.rs`) | `packages/render/src/data_table.rs` | `packages/gpui/preview/src/specimens/data_table.rs` |
| `DetailSectionGroup` | complete | complete | `missing` — smoke only | `DetailSectionGroupSpec` (`packages/contracts/components/src/detail_section_group.rs`) | `packages/render/src/detail_section_group.rs` | `packages/gpui/preview/src/specimens/detail_section_group_specimen.rs` |
| `DetailSection` | complete | complete | `missing` — smoke only | `DetailSectionSpec` (`packages/contracts/components/src/detail_section.rs`) | `packages/render/src/detail_section.rs` | `packages/gpui/preview/src/specimens/detail_section_specimen.rs` |
| `DockRegion` | complete | complete | `DockRegionDragOverGate.test.tsx`, `DockRegionTabPassThroughs.test.tsx`, `DockRegionZoneDrop.test.tsx` | `DockRegionSpec` (`packages/contracts/components/src/dock_region.rs`) | `packages/render/src/dock_region.rs` | `missing` |
| `DetailShell` | complete | complete | `missing` — smoke only | `DetailShellSpec` (`packages/contracts/components/src/detail_shell.rs`) | `packages/render/src/detail_shell.rs` | `packages/gpui/preview/src/specimens/detail_shell.rs` |
| `EmbedInput` | complete | complete | `missing` — smoke only | `EmbedInputSpec` (`packages/contracts/components/src/embed_input.rs`) | `packages/render/src/embed_input.rs` | `packages/gpui/preview/src/specimens/embed_input_specimen.rs` |
| `EmbedPreview` | complete | complete | `missing` — smoke only | `EmbedPreviewSpec` (`packages/contracts/components/src/embed_preview.rs`) | `packages/render/src/embed_preview.rs` | `packages/gpui/preview/src/specimens/embed_preview_specimen.rs` |
| `EmptyState` | complete | complete | `missing` — smoke only | `EmptyStateSpec` (`packages/contracts/components/src/empty_state.rs`) | `packages/render/src/empty_state.rs` | `missing` |
| `FilterToolbar` | complete | complete | `missing` — smoke only | `FilterToolbarSpec` (`packages/contracts/components/src/filter_toolbar.rs`) | `packages/render/src/filter_toolbar.rs` | `packages/gpui/preview/src/specimens/filter_toolbar_specimen.rs` |
| `FormDialog` | complete | complete | `DialogInitialFocus.test.tsx` | `FormDialogSpec` (`packages/contracts/components/src/form_dialog.rs`) | `packages/render/src/form_dialog.rs` | `packages/gpui/preview/src/specimens/form_dialog_specimen.rs` |
| `FormLayout` | complete | complete | `missing` — smoke only | `FormLayoutSpec` (`packages/contracts/components/src/form_layout.rs`) | `packages/render/src/form_layout.rs` | `packages/gpui/preview/src/specimens/form_layout.rs` |
| `InlineListSection` | complete | complete | `missing` — smoke only | `InlineListSectionSpec` (`packages/contracts/components/src/inline_list_section.rs`) | `packages/render/src/inline_list_section.rs` | `packages/gpui/preview/src/specimens/inline_list_section_specimen.rs` |
| `DebugDialog` | complete | complete | `missing` — smoke only | `DebugDialogSpec` (`packages/contracts/components/src/debug_dialog.rs`) | `packages/render/src/debug_dialog.rs` | `packages/gpui/preview/src/specimens/debug_dialog_specimen.rs` |
| `LicenceActivation` | complete | complete | `LicenceActivation.test.tsx` | `missing` | `missing` | `missing` |
| `LicenceSeats` | complete | complete | `LicenceSeats.test.tsx` | `missing` | `missing` | `missing` |
| `LicenceStatus` | complete | complete | `LicenceStatus.test.tsx` | `missing` | `missing` | `missing` |
| `LogList` | complete | complete | `missing` — smoke only | `LogListSpec` (`packages/contracts/components/src/log_list.rs`) | `packages/render/src/log_list.rs` | `packages/gpui/preview/src/specimens/log_list_specimen.rs` |
| `ListContainer` | complete | complete | `missing` — smoke only | `ListContainerSpec` (`packages/contracts/components/src/list_container.rs`) | `packages/render/src/list_container.rs` | `packages/gpui/preview/src/specimens/list_container_specimen.rs` |
| `MarkdownEditor` | complete | complete | `missing` — smoke only | `MarkdownEditorSpec` (`packages/contracts/components/src/markdown_editor.rs`) | `packages/render/src/markdown_editor.rs` | `packages/gpui/preview/src/specimens/markdown_editor_specimen.rs` |
| `PageLoading` | complete | complete | `missing` — smoke only | `PageLoadingSpec` (`packages/contracts/components/src/page_loading.rs`) | `packages/render/src/page_loading.rs` | `packages/gpui/preview/src/specimens/page_loading_specimen.rs` |
| `MediaPicker` | complete | complete | `missing` — smoke only | `MediaPickerSpec` (`packages/contracts/components/src/media_picker.rs`) | `packages/render/src/media_picker.rs` | `packages/gpui/preview/src/specimens/media_picker_specimen.rs` |
| `MediaBrowsePanel` | complete | complete | `missing` — smoke only | `MediaBrowsePanelSpec` (`packages/contracts/components/src/media_browse_panel.rs`) | `packages/render/src/media_browse_panel.rs` | `packages/gpui/preview/src/specimens/media_browse_panel_specimen.rs` |
| `MediaPreview` | complete | complete | `missing` — smoke only | `MediaPreviewSpec` (`packages/contracts/components/src/media_preview.rs`) | `packages/render/src/media_preview.rs` | `packages/gpui/preview/src/specimens/media_preview_specimen.rs` |
| `MediaThumbnail` | complete | complete | `missing` — smoke only | `MediaThumbnailSpec` (`packages/contracts/components/src/media_thumbnail.rs`) | `packages/render/src/media_thumbnail.rs` | `packages/gpui/preview/src/specimens/media_thumbnail_specimen.rs` |
| `PageHeader` | complete | complete | `missing` — smoke only | `PageHeaderSpec` (`packages/contracts/components/src/page_header.rs`) | `packages/render/src/page_header.rs` | `packages/gpui/preview/src/specimens/page_header_specimen.rs` |
| `PickerShell` | complete | complete | `missing` — smoke only | `PickerShellSpec` (`packages/contracts/components/src/picker_shell.rs`) | `packages/render/src/picker_shell.rs` | `packages/gpui/preview/src/specimens/picker_shell_specimen.rs` |
| `RelationPicker` | complete | complete | `missing` — smoke only | `RelationPickerSpec` (`packages/contracts/components/src/relation_picker.rs`) | `packages/render/src/relation_picker.rs` | `packages/gpui/preview/src/specimens/relation_picker_specimen.rs` |
| `SelectionSummary` | complete | complete | `missing` — smoke only | `SelectionSummarySpec` (`packages/contracts/components/src/selection_summary.rs`) | `packages/render/src/selection_summary.rs` | `packages/gpui/preview/src/specimens/selection_summary_specimen.rs` |
| `SettingsShell` | complete | complete | `SettingsShell.test.tsx` | `missing` | `missing` | `missing` |
| `SidebarNav` | complete | complete | `missing` — smoke only | `SidebarNavSpec` (`packages/contracts/components/src/sidebar_nav.rs`) | `packages/render/src/sidebar_nav.rs` | `packages/gpui/preview/src/specimens/sidebar_nav.rs` |
| `Tree` | complete | complete | `missing` — smoke only | `TreeSpec` (`packages/contracts/components/src/tree.rs`) | `packages/render/src/tree.rs` | `packages/gpui/preview/src/specimens/tree.rs` |
| `SplitView` | complete | complete | `missing` — smoke only | `SplitViewSpec` (`packages/contracts/components/src/split_view.rs`) | `packages/render/src/split_view.rs` | `packages/gpui/preview/src/specimens/split_view_specimen.rs` |
| `MetricTile` | complete | complete | `missing` — smoke only | `MetricTileSpec` (`packages/contracts/components/src/metric_tile.rs`) | `packages/render/src/metric_tile.rs` | `packages/gpui/preview/src/specimens/metric_tile_specimen.rs` |
| `StateTile` | complete | complete | `WebParityCloseout.test.tsx` | `StateTileSpec` (`packages/contracts/components/src/state_tile.rs`) | `packages/render/src/state_tile.rs` | `missing` |
| `ValidationSummary` | complete | complete | `WebParityCloseout.test.tsx` | `ValidationSummarySpec` (`packages/contracts/components/src/validation_summary.rs`) | `packages/render/src/validation_summary.rs` | `packages/gpui/preview/src/specimens/validation_summary.rs` |
| `ModelPicker` | complete | complete | `ModelPicker.test.tsx` | `ModelPickerSpec` (`packages/contracts/components/src/model_picker.rs`) | `packages/render/src/model_picker.rs` | `packages/gpui/preview/src/specimens/model_picker_specimen.rs` |
| `ModelConnectionPicker` | complete | complete | `ModelConnection.test.tsx` | `missing` | `missing` | `missing` |
| `ModelConnectionSetup` | complete | complete | `ModelConnection.test.tsx` | `missing` | `missing` | `missing` |
| `ModelConnectionCard` | complete | complete | `ModelConnection.test.tsx` | `missing` | `missing` | `missing` |
| `ModelCatalogueEditor` | complete | complete | `ModelConnection.test.tsx` | `missing` | `missing` | `missing` |
| `MessageCenter` | complete | complete | `MessageCenter.test.tsx` | `MessageCenterSpec` (`packages/contracts/components/src/message_center.rs`) | `packages/render/src/message_center.rs` | `packages/gpui/preview/src/specimens/message_center_specimen.rs` |
| `HistoryCenter` | complete | complete | `HistoryCenter.test.tsx` | `HistoryCenterSpec` (`packages/contracts/components/src/history_center.rs`) | `packages/render/src/history_center.rs` | `packages/gpui/preview/src/specimens/history_center_specimen.rs` |
| `UpdateStatus` | complete | complete | `UpdateStatus.test.tsx` | `missing` | `missing` | `missing` |
| `UpdateCenter` | complete | complete | `UpdateCenter.test.tsx` | `missing` | `missing` | `missing` |
| `ToastStack` | complete | complete | `missing` — smoke only | `ToastStackSpec` (`packages/contracts/components/src/toast_stack.rs`) | `packages/render/src/toast_stack.rs` | `packages/gpui/preview/src/specimens/toast_stack_specimen.rs` |
| `ToastHost` | complete | complete | `missing` — smoke only | `ToastHostSpec` (`packages/contracts/components/src/toast_host.rs`) | `packages/render/src/toast_host.rs` | `packages/gpui/preview/src/specimens/toast_host.rs` |

### Audio controls

| Component | React impl/export | React gallery | Focused React test | Rust declaration | Rust render | GPUI specimen |
| --- | --- | --- | --- | --- | --- | --- |
| `AudioMeter` | complete | complete | `AudioControls.test.tsx`, `MeterSurface.test.tsx` | `AudioMeterSpec` (`packages/contracts/components/src/audio_meter.rs`) | `packages/render/src/audio.rs` (batched family) | `packages/gpui/preview/src/specimens/audio_controls.rs` (batched family) |
| `MeterSurface` | complete | complete | `MeterSurface.test.tsx` | not-applicable — web-only (spec 068) | not-applicable — web-only (spec 068) | not-applicable — web-only (spec 068) |
| `AudioSwitch` | complete | complete | `AudioControls.test.tsx` | `AudioSwitchSpec` (`packages/contracts/components/src/audio_switch.rs`) | `packages/render/src/audio.rs` (batched family) | `packages/gpui/preview/src/specimens/audio_controls.rs` (batched family) |
| `DragNumberField` | complete | complete | `AudioControls.test.tsx` | `DragNumberFieldSpec` (`packages/contracts/components/src/drag_number_field.rs`) | `packages/render/src/audio.rs` (batched family) | `packages/gpui/preview/src/specimens/audio_controls.rs` (batched family) |
| `EnvelopeEditor` | complete | complete | `AudioControls.test.tsx` | `EnvelopeEditorSpec` (`packages/contracts/components/src/envelope_editor.rs`) | `packages/render/src/audio.rs` (batched family) | `packages/gpui/preview/src/specimens/audio_controls.rs` (batched family) |
| `Fader` | complete | complete | `AudioControls.test.tsx` | `FaderSpec` (`packages/contracts/components/src/fader.rs`) | `packages/render/src/audio.rs` (batched family) | `packages/gpui/preview/src/specimens/audio_controls.rs` (batched family) |
| `GainReductionMeter` | complete | complete | `AudioControls.test.tsx` | `GainReductionMeterSpec` (`packages/contracts/components/src/gain_reduction_meter.rs`) | `packages/render/src/audio.rs` (batched family) | `packages/gpui/preview/src/specimens/audio_controls.rs` (batched family) |
| `Keyboard` | complete | complete | `AudioControls.test.tsx` | `KeyboardSpec` (`packages/contracts/components/src/keyboard.rs`) | `packages/render/src/audio.rs` (batched family) | `packages/gpui/preview/src/specimens/audio_controls.rs` (batched family) |
| `Knob` | complete | complete | `AudioControls.test.tsx` | `KnobSpec` (`packages/contracts/components/src/knob.rs`) | `packages/render/src/audio.rs` (batched family) | `packages/gpui/preview/src/specimens/audio_controls.rs` (batched family) |
| `ModMatrixGrid` | complete | complete | `AudioControls.test.tsx` | `ModMatrixGridSpec` (`packages/contracts/components/src/mod_matrix_grid.rs`) | `packages/render/src/audio.rs` (batched family) | `packages/gpui/preview/src/specimens/audio_controls.rs` (batched family) |
| `ValueReadout` | complete | complete | `AudioControls.test.tsx` | `ValueReadoutSpec` (`packages/contracts/components/src/value_readout.rs`) | `packages/render/src/audio.rs` (batched family) | `packages/gpui/preview/src/specimens/audio_controls.rs` (batched family) |
| `WaveformDisplay` | complete | complete | `AudioControls.test.tsx` | `WaveformDisplaySpec` (`packages/contracts/components/src/waveform_display.rs`) | `packages/render/src/audio.rs` (batched family) | `packages/gpui/preview/src/specimens/audio_controls.rs` (batched family) |
| `XYPad` | complete | complete | `AudioControls.test.tsx` | `XYPadSpec` (`packages/contracts/components/src/xy_pad.rs`) | `packages/render/src/audio.rs` (batched family) | `packages/gpui/preview/src/specimens/audio_controls.rs` (batched family) |

## Headless Native Regression Coverage

`effigy regressions:native` (`packages/gpui/preview/tests/headless_regressions.rs`) exercises three product components and two platform infrastructure proofs. It is the certified native evidence for those components only; it does not certify any other component and no component borrows it.

| Component | Regression case |
| --- | --- |
| `Button` | `one_enter_activates_a_focused_control_exactly_once` |
| `RangeSlider` | `a_scrub_reports_change_while_dragging_and_commits_once_at_release` |
| `Popover` | `overlay_layers_survive_independent_conversions_within_one_frame`, `a_nested_popover_paints_without_nesting_deferred_draws` |

Infrastructure proofs (not component evidence): `the_driver_mounts_and_tracks_real_backend_focus`, `a_pointer_press_reaches_the_backend_listener_once`.

## Board-Level Validation (not per-component evidence)

- Svelte anatomy smoke — one named mount case per component (`smoke.test.ts`), all but `IconProvider`
- Svelte/React interactions boards, `ContractPropDrift.test.ts`, `WebParityCloseout.test.ts` (both runtimes), `test:parity`, `test/a11y` axe sweep, `test/visual` web tiers
- Drift gates: `docs:contract-drift`, `docs:spec-drift`, `docs:value-domain-drift`, `docs:callback-drift`, `drift:roles`, `drift:events`, `drift:handlers`, `docs:react-specimen-drift`, `docs:capability-drift`

Jetstream is program-deferred for every component and is not reported per row.
