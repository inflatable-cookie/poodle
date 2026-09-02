# g15 — Release Baseline Roster (frozen v0.2.0 denominator)

Status: complete — measured by `g15.001`
Date: 2026-08-16
Updated: 2026-09-02 — `g16.056` froze 176 as the compiled-distribution denominator and named the root markdown break
Card: `docs/roadmaps/g15/001-release-baseline-roster-inventory.md`
Governing refs: `docs/roadmaps/g15/001-release-baseline-roster-inventory.md`, `docs/roadmaps/g14/022-generation-closeout.md`, `docs/contracts/001-working-rules.md`, `docs/roadmaps/g14/conformance-estate.md`, `docs/specs/070-compiled-web-distribution-contract.md`

## Denominator

The live public Svelte denominator is **176 component exports**, enumerated mechanically from `export { default as <Name> } from "./<Name>.svelte"` in `packages/svelte/components/src/index.ts` (176 matches) and verified one-to-one against those 176 component files. Two additional `.svelte` files (`DragDropProvider`, `MenuSurface`) exist as internals and are not denominator members. Packed reachability today still ships `src` through `.`, `./*.svelte`, and `./types`. The compiled `0.3.0` contract in spec 070 keeps the same 176 names: `AgentMessage` and `MarkdownEditor` stay in the denominator, leave shell root barrels for `./markdown`, and remain reachable as Svelte `./<Name>.svelte` and React `./<Name>`. Root-barrel membership becomes 174 components plus helpers. A 175/176 split between this roster, spec 070, and `test/package-install/roster.ts` is a blocking defect. `MotionPolicyProvider` joined the roster in g16.034.

Public types and helpers are recorded separately and are **not** part of the denominator: the `types` block and the `file-upload`, `theme-controller`, `date`, `presentation`, `anchored`, `portal`, `embed-input`, `media-workflow`, `persistence`, and `icon-registry` exports. The canonical preview catalogue (175 portable slugs plus the web-only `meter-surface`) maps one-to-one onto this roster.

| Surface | present | missing | not-applicable |
| --- | ---: | ---: | ---: |
| Implementation file present (`src/<Name>.svelte`) | 176 | 0 | 0 |
| Export from index + packed `exports` map | 176 | 0 | 0 |
| Contract (`docs/contracts/components/<name>.md`) | 176 | 0 | 0 |
| Svelte preview specimen (dedicated file or scene-shared) | 176 | 0 | 0 |
| Focused Svelte test (named file/case, beyond anatomy smoke) | 176 | 0 | 0 |
| React implementation + export | 176 | 0 | 0 |
| React gallery specimen | 176 | 0 | 0 |
| Focused React test | 176 | 0 | 0 |
| Rust declaration (`<Name>Spec`, including documented aliases) | 175 | 0 | 1 |
| Rust render module (`poodle-render`) | 175 | 0 | 1 |
| GPUI specimen | 175 | 0 | 1 |
| `test:web-pack-install` Svelte packed root-import proof | 176 | 0 | 0 |
| `test:web-pack-install` React packed root-import proof | 176 | 0 | 0 |
| Downstream consumer use (16 canonical consumers scanned) | 110 | 66 (no use found) | 0 |
| Jetstream | 0 (program-deferred) | — | — |

`not-applicable` is exactly one component on exactly one axis each: `MeterSurface` is web-only by fixed decision (spec 068) and has no Rust declaration, Rust render, or GPUI counterpart. It still counts as a member of the denominator (exported, contracted, implemented, specified in Svelte) and its `not-applicable` rows are recorded as such, not as missing or present.

## Count Method (reproducible)

- **Implementation / Export**: 175 `export { default as <Name> } from "./<Name>.svelte"` lines in `packages/svelte/components/src/index.ts`, each matched to a file of the same name; package `exports` map and `files` array checked once for packed reachability.
- **Contract**: one `docs/contracts/components/<kebab>.md` per component (kebab-case from the export name); 175 of 175 present, verified by direct file check.
- **Specimen**: keys of `specimenMap` in `packages/svelte/preview/src/specimens/registry.ts` against the canonical slugs (174 portable + web-only `meter-surface`); 175 entries. 168 map to a dedicated `*Specimen.svelte`; 7 map to a shared specimen (5 `SceneSpecimen`, 1 `ListCardSpecimen` for `ListCardCounter`, 1 `MetaBarSpecimen` for `MetaItem`).
- **Focused Svelte test**: component imports resolved across all files in `packages/svelte/components/test/` (`.test.ts` and harness `.svelte` files); a component counts when at least one named test file mounts and asserts it beyond the anatomy smoke. 175 count; 0 record `missing` — `g15.005` closed the final 24 (workstation systems and agent surfaces).
- **React implementation/export**: named component exports in `packages/react/components/src/index.ts` (175); React gallery: `specimen-map.ts` keys against canonical slugs (175); focused React test: same import-resolution method over `packages/react/components/test/` (175; 0 missing — `g15.005` closed the final 23; `AgentSubagent` already had React evidence).
- **Rust declaration**: `pub struct <Name>Spec` searched recursively in `packages/contracts/components/src` (174 after `g15.009` closed UpdateStatus, UpdateCenter, SettingsShell, and Radio). Three documented naming discrepancies count as present: `CallOutSpec` (`Callout`), `ShellStatusBarSpec` (`StatusBar`), `TimeFieldSpec` (`TimeInput`). `MeterSurface` has no declaration and records not-applicable per spec 068.
- **Rust render**: module names in `packages/render/src/lib.rs` (174 after `g15.043`). Documented naming discrepancies count as present: `bx.rs` (`Box`), `shell_status_bar.rs` (`StatusBar`), `time_field.rs` (`TimeInput`), `context.rs` (`UiPresentationProvider`, whose renderer is the crate-root `ui_presentation_provider` cascade boundary), and the batched `audio.rs` covering the 12 audio widgets (13 audio components minus `MeterSurface`). `MeterSurface` records not-applicable.
- **GPUI specimen**: file presence in `packages/gpui/preview/src/specimens/` per component (174 after `g15.010`). The batched `audio_controls.rs` covers 12 audio widgets; the 12 audio widgets are those covered — `audio_controls.rs` has no `meter_surface` function and `MeterSurface` records not-applicable. Counts do not include the `mod.rs` dispatch fallback (`missing_specimen`).
- **Pack-install**: `test:web-pack-install` derives the frozen component names
  from this roster and both package-root indexes, then compares the exact
  runtime export sets from clean installed tarballs. The proof is 175/175 for
  Svelte and React. The retained representative mount set is separate: 9
  Svelte components and 13 React components exercise runtime machinery.
- **Downstream use**: import statements of `@inflatable-cookie/poodle-svelte` / `-react` resolved (single- and multi-line) across source files of the 16 canonical consumers under `~/Dev/projects`: acowtancy, bovine-accelerator-desktop, compli-me, composer, contact-patch, figmatic, finch, longhorn, loophole, loophole-legacy, nucleus, songsprout, soundcheck, soundcheck-library, underlay, underlay-reference. Excluded: `poodle` itself (source), `jetstream` (program-deferred), worktree/absorbed duplicates (e.g. `soundcheck-wt`, `acowtancy/dairy-card011-worktree`), vendored/build/generated/fixture/example/archive paths, and test directories. No canonical consumer imports `poodle-react`; all component imports resolve to `poodle-svelte`.

## Posture Legend

Per-surface posture is `complete` / `partial` / `missing` / `not-applicable`, always with exact evidence. `missing` is recorded from direct inspection of the tree; no posture is inferred from another runtime's pass. The Svelte anatomy smoke (`packages/svelte/components/test/smoke.test.ts`) generates one named mount case per component through a module glob (all but `IconProvider`, excluded with a recorded reason); it is board-level health and is not counted as focused evidence.

Rust declarations use the documented naming discrepancies where they exist: `CallOutSpec` for `Callout`, `ShellStatusBarSpec` for `StatusBar`, `TimeFieldSpec` for `TimeInput` (each self-documented in the declaration file header).

## Svelte Denominator Surfaces (per component)

### Foundations (primitives)

| Component | Contract | Specimen | Focused Svelte test | Pack-install | Downstream use |
| --- | --- | --- | --- | --- | --- |
| `Accordion` | `docs/contracts/components/accordion.md` | `AccordionSpecimen.svelte` | `Accordion.test.ts` | `test:web-pack-install` | `finch` |
| `AgentChatInput` | `docs/contracts/components/agent-chat-input.md` | `AgentChatInputSpecimen.svelte` | `AgentChatInput.test.ts` | `test:web-pack-install` | no consumer use found (absence is not a release failure) |
| `AudioPlayer` | `docs/contracts/components/audio-player.md` | `AudioPlayerSpecimen.svelte` | `AudioPlayer.test.ts` | `test:web-pack-install` | `acowtancy` |
| `AlertDialog` | `docs/contracts/components/alert-dialog.md` | `AlertDialogSpecimen.svelte` | `AlertDialog.test.ts` | `test:web-pack-install` | `acowtancy`, `compli-me`, `contact-patch`, `underlay`, `underlay-reference` |
| `Avatar` | `docs/contracts/components/avatar.md` | shared `SceneSpecimen.svelte` (generated scene) | `Avatar.test.ts` | `test:web-pack-install` | `acowtancy` |
| `Box` | `docs/contracts/components/box.md` | `BoxSpecimen.svelte` | `Box.test.ts` | `test:web-pack-install` | `acowtancy` |
| `Breadcrumbs` | `docs/contracts/components/breadcrumbs.md` | `BreadcrumbsSpecimen.svelte` | `Breadcrumbs.test.ts` | `test:web-pack-install` | `acowtancy`, `underlay` |
| `BulkActionBar` | `docs/contracts/components/bulk-action-bar.md` | `BulkActionBarSpecimen.svelte` | `BulkActionBar.test.ts` | `test:web-pack-install` | `acowtancy`, `underlay` |
| `Button` | `docs/contracts/components/button.md` | `ButtonSpecimen.svelte` | `Button.test.ts` | `test:web-pack-install` | `acowtancy`, `bovine-accelerator-desktop`, `compli-me`, `composer`, `contact-patch`, `figmatic`, `finch`, `longhorn`, `loophole`, `loophole-legacy`, `nucleus`, `songsprout`, `soundcheck`, `soundcheck-library`, `underlay`, `underlay-reference` |
| `Callout` | `docs/contracts/components/callout.md` | shared `SceneSpecimen.svelte` (generated scene) | `Callout.test.ts` | `test:web-pack-install` | `acowtancy`, `compli-me`, `composer`, `contact-patch`, `longhorn`, `loophole-legacy`, `nucleus`, `songsprout`, `soundcheck`, `soundcheck-library`, `underlay`, `underlay-reference` |
| `RemediationBanner` | `docs/contracts/components/remediation-banner.md` | `RemediationBannerSpecimen.svelte` | `WebParityCloseout.test.ts` | `test:web-pack-install` | no consumer use found (absence is not a release failure) |
| `Card` | `docs/contracts/components/card.md` | `CardSpecimen.svelte` | `Card.test.ts` | `test:web-pack-install` | `acowtancy`, `compli-me`, `composer`, `contact-patch`, `songsprout`, `underlay`, `underlay-reference` |
| `Code` | `docs/contracts/components/code.md` | `CodeSpecimen.svelte` | `Code.test.ts` | `test:web-pack-install` | `acowtancy`, `compli-me`, `composer`, `contact-patch`, `underlay`, `underlay-reference` |
| `ColorPicker` | `docs/contracts/components/color-picker.md` | `ColorPickerSpecimen.svelte` | `ColorPicker.test.ts` | `test:web-pack-install` | `underlay-reference` |
| `Checkbox` | `docs/contracts/components/checkbox.md` | `CheckboxSpecimen.svelte` | `interactions.test.ts` | `test:web-pack-install` | `finch`, `soundcheck`, `soundcheck-library`, `underlay` |
| `Calendar` | `docs/contracts/components/calendar.md` | `CalendarSpecimen.svelte` | `Calendar.test.ts` | `test:web-pack-install` | no consumer use found (absence is not a release failure) |
| `ContextMenu` | `docs/contracts/components/context-menu.md` | `ContextMenuSpecimen.svelte` | `ContextMenu.test.ts` | `test:web-pack-install` | `figmatic`, `loophole-legacy`, `soundcheck-library` |
| `CollapseToggle` | `docs/contracts/components/collapse-toggle.md` | `CollapseToggleSpecimen.svelte` | `CollapseToggle.test.ts` | `test:web-pack-install` | no consumer use found (absence is not a release failure) |
| `Collapsible` | `docs/contracts/components/collapsible.md` | `CollapsibleSpecimen.svelte` | `Collapsible.test.ts` | `test:web-pack-install` | `acowtancy` |
| `DetailItem` | `docs/contracts/components/detail-item.md` | `DetailItemSpecimen.svelte` | `DetailItem.test.ts` | `test:web-pack-install` | `acowtancy`, `composer`, `contact-patch`, `longhorn`, `nucleus`, `underlay`, `underlay-reference` |
| `DatePicker` | `docs/contracts/components/date-picker.md` | `DatePickerSpecimen.svelte` | `DatePicker.test.ts` | `test:web-pack-install` | no consumer use found (absence is not a release failure) |
| `DateRangePicker` | `docs/contracts/components/date-range-picker.md` | `DateRangePickerSpecimen.svelte` | `DateRangePicker.test.ts` | `test:web-pack-install` | no consumer use found (absence is not a release failure) |
| `DateTimePicker` | `docs/contracts/components/date-time-picker.md` | `DateTimePickerSpecimen.svelte` | `DateTimePicker.test.ts` | `test:web-pack-install` | no consumer use found (absence is not a release failure) |
| `DateTimeRangePicker` | `docs/contracts/components/date-time-range-picker.md` | `DateTimeRangePickerSpecimen.svelte` | `DateTimeRangePicker.test.ts` | `test:web-pack-install` | no consumer use found (absence is not a release failure) |
| `Dialog` | `docs/contracts/components/dialog.md` | `DialogSpecimen.svelte` | `DialogControlled.svelte.test.ts`, `DialogDismissOutside.svelte.test.ts`, `DialogInitialFocus.svelte.test.ts`, `PopoverInDialog.svelte.test.ts` | `test:web-pack-install` | `acowtancy`, `finch`, `longhorn`, `loophole-legacy`, `nucleus`, `soundcheck-library`, `underlay`, `underlay-reference` |
| `Drawer` | `docs/contracts/components/drawer.md` | `DrawerSpecimen.svelte` | `DrawerDismissOutside.svelte.test.ts` | `test:web-pack-install` | `acowtancy`, `underlay`, `underlay-reference` |
| `DurationInput` | `docs/contracts/components/duration-input.md` | `DurationInputSpecimen.svelte` | `DurationInput.test.ts` | `test:web-pack-install` | `acowtancy` |
| `EditableLabel` | `docs/contracts/components/editable-label.md` | `EditableLabelSpecimen.svelte` | `EditableLabel.test.ts` | `test:web-pack-install` | `loophole`, `loophole-legacy`, `nucleus`, `soundcheck-library` |
| `Eyebrow` | `docs/contracts/components/eyebrow.md` | `EyebrowSpecimen.svelte` | `Eyebrow.test.ts` | `test:web-pack-install` | `acowtancy` |
| `Field` | `docs/contracts/components/field.md` | `FieldSpecimen.svelte` | `Field.test.ts` | `test:web-pack-install` | `acowtancy`, `compli-me`, `composer`, `contact-patch`, `longhorn`, `loophole-legacy`, `songsprout`, `underlay`, `underlay-reference` |
| `FieldSet` | `docs/contracts/components/field-set.md` | `FieldSetSpecimen.svelte` | `FieldSet.test.ts` | `test:web-pack-install` | `acowtancy`, `composer`, `contact-patch`, `underlay-reference` |
| `FileUpload` | `docs/contracts/components/file-upload.md` | `FileUploadSpecimen.svelte` | `FileUpload.test.ts` | `test:web-pack-install` | `underlay` |
| `FilterBuilder` | `docs/contracts/components/filter-builder.md` | `FilterBuilderSpecimen.svelte` | `FilterBuilder.test.ts` | `test:web-pack-install` | no consumer use found (absence is not a release failure) |
| `FormActions` | `docs/contracts/components/form-actions.md` | `FormActionsSpecimen.svelte` | `FormActions.test.ts` | `test:web-pack-install` | `acowtancy`, `compli-me`, `composer`, `contact-patch`, `longhorn`, `songsprout`, `underlay`, `underlay-reference` |
| `Grid` | `docs/contracts/components/grid.md` | `GridSpecimen.svelte` | `Grid.test.ts` | `test:web-pack-install` | `acowtancy`, `compli-me`, `loophole-legacy`, `songsprout`, `underlay`, `underlay-reference` |
| `HoverCard` | `docs/contracts/components/hover-card.md` | `HoverCardSpecimen.svelte` | `HoverCard.test.ts` | `test:web-pack-install` | no consumer use found (absence is not a release failure) |
| `Icon` | `docs/contracts/components/icon.md` | `IconSpecimen.svelte` | `IconProviderHarness.svelte` | `test:web-pack-install` | `acowtancy`, `loophole-legacy`, `nucleus`, `soundcheck`, `soundcheck-library`, `underlay` |
| `IconButton` | `docs/contracts/components/icon-button.md` | `IconButtonSpecimen.svelte` | `IconButton.test.ts` | `test:web-pack-install` | `acowtancy`, `compli-me`, `composer`, `figmatic`, `loophole-legacy`, `nucleus`, `soundcheck`, `soundcheck-library`, `underlay`, `underlay-reference` |
| `IconProvider` | `docs/contracts/components/icon-provider.md` | `IconProviderSpecimen.svelte` | `IconProviderHarness.svelte` | `test:web-pack-install` | `acowtancy`, `compli-me`, `composer`, `figmatic`, `loophole-legacy`, `nucleus`, `underlay-reference` |
| `Meter` | `docs/contracts/components/meter.md` | `MeterSpecimen.svelte` | `Meter.test.ts` | `test:web-pack-install` | `soundcheck-library` |
| `ListCard` | `docs/contracts/components/list-card.md` | `ListCardSpecimen.svelte` | `ListCard.test.ts` | `test:web-pack-install` | `figmatic`, `underlay`, `underlay-reference` |
| `ListCardCounter` | `docs/contracts/components/list-card-counter.md` | shared specimen (composed inside hosting specimen) | `ListCardCounter.test.ts` | `test:web-pack-install` | `underlay` |
| `ListGrid` | `docs/contracts/components/list-grid.md` | `ListGridSpecimen.svelte` | `ListGrid.test.ts` | `test:web-pack-install` | `acowtancy`, `underlay` |
| `Menu` | `docs/contracts/components/menu.md` | `MenuSpecimen.svelte` | `Menu.test.ts`, `OverlayGeometry.svelte.test.ts` | `test:web-pack-install` | `figmatic`, `loophole-legacy`, `nucleus`, `soundcheck-library`, `underlay` |
| `MetaBar` | `docs/contracts/components/meta-bar.md` | `MetaBarSpecimen.svelte` | `MetaBar.test.ts` | `test:web-pack-install` | `acowtancy`, `compli-me`, `composer`, `contact-patch`, `underlay`, `underlay-reference` |
| `MetaItem` | `docs/contracts/components/meta-item.md` | shared specimen (composed inside hosting specimen) | `MetaBar.test.ts` | `test:web-pack-install` | `acowtancy`, `compli-me`, `composer`, `contact-patch`, `underlay`, `underlay-reference` |
| `NumberInput` | `docs/contracts/components/number-input.md` | `NumberInputSpecimen.svelte` | `NumberInput.test.ts` | `test:web-pack-install` | `acowtancy`, `underlay` |
| `OrderBy` | `docs/contracts/components/order-by.md` | `OrderBySpecimen.svelte` | `OrderBy.test.ts` | `test:web-pack-install` | `acowtancy` |
| `NavCard` | `docs/contracts/components/nav-card.md` | `NavCardSpecimen.svelte` | `NavCard.test.ts` | `test:web-pack-install` | `acowtancy`, `compli-me`, `composer`, `songsprout`, `underlay` |
| `NavigationMenu` | `docs/contracts/components/navigation-menu.md` | `NavigationMenuSpecimen.svelte` | `NavigationMenu.test.ts` | `test:web-pack-install` | no consumer use found (absence is not a release failure) |
| `Pill` | `docs/contracts/components/pill.md` | shared `SceneSpecimen.svelte` (generated scene) | `Pill.test.ts` | `test:web-pack-install` | `acowtancy`, `compli-me`, `composer`, `contact-patch`, `loophole-legacy`, `songsprout`, `soundcheck`, `soundcheck-library`, `underlay`, `underlay-reference` |
| `CodeInput` | `docs/contracts/components/code-input.md` | `CodeInputSpecimen.svelte` | `CodeInput.test.ts` | `test:web-pack-install` | `acowtancy`, `compli-me`, `contact-patch`, `underlay`, `underlay-reference` |
| `Popover` | `docs/contracts/components/popover.md` | `PopoverSpecimen.svelte` | `OverlayGeometry.svelte.test.ts`, `PopoverInDialog.svelte.test.ts`, `PopoverRetained.svelte.test.ts` | `test:web-pack-install` | `acowtancy`, `loophole-legacy`, `nucleus`, `underlay` |
| `Pagination` | `docs/contracts/components/pagination.md` | `PaginationSpecimen.svelte` | `Pagination.test.ts` | `test:web-pack-install` | `underlay` |
| `PaginationSummary` | `docs/contracts/components/pagination-summary.md` | `PaginationSummarySpecimen.svelte` | `PaginationSummary.test.ts` | `test:web-pack-install` | no consumer use found (absence is not a release failure) |
| `PasswordRequirements` | `docs/contracts/components/password-requirements.md` | `PasswordRequirementsSpecimen.svelte` | `PasswordRequirements.test.ts` | `test:web-pack-install` | `underlay` |
| `Progress` | `docs/contracts/components/progress.md` | `ProgressSpecimen.svelte` | `Progress.test.ts` | `test:web-pack-install` | `acowtancy`, `soundcheck`, `underlay`, `underlay-reference` |
| `Radio` | `docs/contracts/components/radio.md` | `RadioSpecimen.svelte` | `Radio.test.ts` | `test:web-pack-install` | `loophole-legacy` |
| `RefSelect` | `docs/contracts/components/ref-select.md` | `RefSelectSpecimen.svelte` | `RefSelect.test.ts` | `test:web-pack-install` | no consumer use found (absence is not a release failure) |
| `RadioGroup` | `docs/contracts/components/radio-group.md` | `RadioGroupSpecimen.svelte` | `RadioGroup.test.ts` | `test:web-pack-install` | `nucleus` |
| `Rating` | `docs/contracts/components/rating.md` | `RatingSpecimen.svelte` | `Rating.test.ts` | `test:web-pack-install` | no consumer use found (absence is not a release failure) |
| `Region` | `docs/contracts/components/region.md` | `RegionSpecimen.svelte` | `Region.test.ts` | `test:web-pack-install` | no consumer use found (absence is not a release failure) |
| `ResizeHandle` | `docs/contracts/components/resize-handle.md` | `ResizeHandleSpecimen.svelte` | `ResizeHandle.test.ts` | `test:web-pack-install` | no consumer use found (absence is not a release failure) |
| `RangeSlider` | `docs/contracts/components/range-slider.md` | `RangeSliderSpecimen.svelte` | `RangeSlider.test.ts` | `test:web-pack-install` | no consumer use found (absence is not a release failure) |
| `SegmentedControl` | `docs/contracts/components/segmented-control.md` | `SegmentedControlSpecimen.svelte` | `SegmentedControl.test.ts` | `test:web-pack-install` | `acowtancy`, `loophole-legacy`, `nucleus` |
| `Select` | `docs/contracts/components/select.md` | `SelectSpecimen.svelte` | `Select.test.ts` | `test:web-pack-install` | `acowtancy`, `bovine-accelerator-desktop`, `composer`, `longhorn`, `loophole`, `loophole-legacy`, `nucleus`, `soundcheck`, `soundcheck-library`, `underlay`, `underlay-reference` |
| `ScrollShell` | `docs/contracts/components/scroll-shell.md` | `ScrollShellSpecimen.svelte` | `ScrollShell.test.ts` | `test:web-pack-install` | no consumer use found (absence is not a release failure) |
| `Separator` | `docs/contracts/components/separator.md` | `SeparatorSpecimen.svelte` | `Separator.test.ts` | `test:web-pack-install` | `acowtancy`, `finch`, `loophole-legacy` |
| `SplitButton` | `docs/contracts/components/split-button.md` | `SplitButtonSpecimen.svelte` | `SplitButton.test.ts` | `test:web-pack-install` | `acowtancy`, `contact-patch`, `underlay`, `underlay-reference` |
| `Skeleton` | `docs/contracts/components/skeleton.md` | `SkeletonSpecimen.svelte` | `Skeleton.test.ts` | `test:web-pack-install` | no consumer use found (absence is not a release failure) |
| `Slider` | `docs/contracts/components/slider.md` | `SliderSpecimen.svelte` | `Slider.test.ts` | `test:web-pack-install` | `acowtancy`, `loophole-legacy` |
| `Spinner` | `docs/contracts/components/spinner.md` | shared `SceneSpecimen.svelte` (generated scene) | `Spinner.test.ts` | `test:web-pack-install` | `acowtancy`, `figmatic`, `soundcheck-library` |
| `Spacer` | `docs/contracts/components/spacer.md` | `SpacerSpecimen.svelte` | `Spacer.test.ts` | `test:web-pack-install` | no consumer use found (absence is not a release failure) |
| `Stack` | `docs/contracts/components/stack.md` | `StackSpecimen.svelte` | `Stack.test.ts` | `test:web-pack-install` | `acowtancy`, `figmatic`, `longhorn`, `underlay` |
| `Stepper` | `docs/contracts/components/stepper.md` | `StepperSpecimen.svelte` | `Stepper.test.ts` | `test:web-pack-install` | no consumer use found (absence is not a release failure) |

### Agent surfaces

| Component | Contract | Specimen | Focused Svelte test | Pack-install | Downstream use |
| --- | --- | --- | --- | --- | --- |
| `AgentMessage` | `docs/contracts/components/agent-message.md` | `AgentMessageSpecimen.svelte` | `AgentMessage.test.ts` | `test:web-pack-install` | no consumer use found (absence is not a release failure) |
| `AgentPlan` | `docs/contracts/components/agent-plan.md` | `AgentPlanSpecimen.svelte` | `AgentChatInputPlan.svelte.test.ts` | `test:web-pack-install` | no consumer use found (absence is not a release failure) |
| `AgentPlanRecord` | `docs/contracts/components/agent-plan-record.md` | `AgentPlanRecordSpecimen.svelte` | `AgentPlanRecord.test.ts` | `test:web-pack-install` | no consumer use found (absence is not a release failure) |
| `AgentQuestion` | `docs/contracts/components/agent-question.md` | `AgentQuestionSpecimen.svelte` | `AgentQuestion.test.ts` | `test:web-pack-install` | no consumer use found (absence is not a release failure) |
| `AgentQuestionRecord` | `docs/contracts/components/agent-question-record.md` | `AgentQuestionRecordSpecimen.svelte` | `AgentQuestionRecord.test.ts` | `test:web-pack-install` | no consumer use found (absence is not a release failure) |
| `AgentSubagent` | `docs/contracts/components/agent-subagent.md` | `AgentSubagentSpecimen.svelte` | `AgentSubagent.test.ts` | `test:web-pack-install` | no consumer use found (absence is not a release failure) |
| `AgentTranscript` | `docs/contracts/components/agent-transcript.md` | `AgentTranscriptSpecimen.svelte` | `AgentTranscriptSubagent.svelte.test.ts` | `test:web-pack-install` | no consumer use found (absence is not a release failure) |
| `ChangedFiles` | `docs/contracts/components/changed-files.md` | `ChangedFilesSpecimen.svelte` | `ChangedFiles.test.ts` | `test:web-pack-install` | no consumer use found (absence is not a release failure) |
| `ToolCall` | `docs/contracts/components/tool-call.md` | `ToolCallSpecimen.svelte` | `ToolCall.test.ts` | `test:web-pack-install` | no consumer use found (absence is not a release failure) |
| `ToolCallGroup` | `docs/contracts/components/tool-call-group.md` | `ToolCallGroupSpecimen.svelte` | `ToolCallGroup.test.ts` | `test:web-pack-install` | no consumer use found (absence is not a release failure) |

### Workstation systems

| Component | Contract | Specimen | Focused Svelte test | Pack-install | Downstream use |
| --- | --- | --- | --- | --- | --- |
| `StatusBar` | `docs/contracts/components/status-bar.md` | `StatusBarSpecimen.svelte` | `StatusBar.test.ts` | `test:web-pack-install` | no consumer use found (absence is not a release failure) |
| `StatusIndicator` | `docs/contracts/components/status-indicator.md` | `StatusIndicatorSpecimen.svelte` | `StatusIndicator.test.ts` | `test:web-pack-install` | `finch`, `longhorn`, `underlay` |
| `Surface` | `docs/contracts/components/surface.md` | `SurfaceSpecimen.svelte` | `Surface.test.ts` | `test:web-pack-install` | `acowtancy`, `longhorn`, `loophole-legacy`, `nucleus` |
| `Switch` | `docs/contracts/components/switch.md` | `SwitchSpecimen.svelte` | `interactions.test.ts` | `test:web-pack-install` | `acowtancy`, `bovine-accelerator-desktop`, `loophole`, `loophole-legacy`, `nucleus` |
| `Text` | `docs/contracts/components/text.md` | `TextSpecimen.svelte` | `Text.test.ts` | `test:web-pack-install` | `acowtancy`, `figmatic`, `nucleus` |
| `TextLink` | `docs/contracts/components/text-link.md` | `TextLinkSpecimen.svelte` | `TextLink.test.ts` | `test:web-pack-install` | `acowtancy`, `underlay` |
| `Tabs` | `docs/contracts/components/tabs.md` | `TabsSpecimen.svelte` | `interactions.test.ts` | `test:web-pack-install` | `acowtancy`, `compli-me`, `contact-patch`, `figmatic`, `longhorn`, `loophole-legacy`, `nucleus`, `songsprout`, `soundcheck-library`, `underlay`, `underlay-reference` |
| `Table` | `docs/contracts/components/table.md` | `TableSpecimen.svelte` | `Table.test.ts` | `test:web-pack-install` | `longhorn` |
| `TimeAgo` | `docs/contracts/components/time-ago.md` | `TimeAgoSpecimen.svelte` | `TimeAgo.test.ts` | `test:web-pack-install` | `acowtancy`, `compli-me`, `composer`, `contact-patch`, `underlay`, `underlay-reference` |
| `TextInput` | `docs/contracts/components/text-input.md` | `TextInputSpecimen.svelte` | `TextInput.test.ts` | `test:web-pack-install` | `acowtancy`, `compli-me`, `composer`, `contact-patch`, `longhorn`, `loophole-legacy`, `nucleus`, `songsprout`, `soundcheck`, `soundcheck-library`, `underlay`, `underlay-reference` |
| `TokenInput` | `docs/contracts/components/token-input.md` | `TokenInputSpecimen.svelte` | `TokenInput.test.ts` | `test:web-pack-install` | `acowtancy` |
| `TimeInput` | `docs/contracts/components/time-input.md` | `TimeInputSpecimen.svelte` | `TimeInput.test.ts` | `test:web-pack-install` | no consumer use found (absence is not a release failure) |
| `TimeZoneSelect` | `docs/contracts/components/time-zone-select.md` | `TimeZoneSelectSpecimen.svelte` | `TimeZoneSelect.test.ts` | `test:web-pack-install` | no consumer use found (absence is not a release failure) |
| `ThemeSelect` | `docs/contracts/components/theme-select.md` | `ThemeSelectSpecimen.svelte` | `ThemeSelect.test.ts` | `test:web-pack-install` | no consumer use found (absence is not a release failure) |
| `ToggleGroup` | `docs/contracts/components/toggle-group.md` | `ToggleGroupSpecimen.svelte` | `ToggleGroup.test.ts` | `test:web-pack-install` | no consumer use found (absence is not a release failure) |
| `Toolbar` | `docs/contracts/components/toolbar.md` | `ToolbarSpecimen.svelte` | `Toolbar.test.ts` | `test:web-pack-install` | `loophole-legacy`, `soundcheck-library` |
| `Tooltip` | `docs/contracts/components/tooltip.md` | `TooltipSpecimen.svelte` | `Tooltip.test.ts` | `test:web-pack-install` | `acowtancy` |
| `TriStateSwitch` | `docs/contracts/components/tri-state-switch.md` | `TriStateSwitchSpecimen.svelte` | `TriStateSwitch.test.ts` | `test:web-pack-install` | no consumer use found (absence is not a release failure) |
| `Menubar` | `docs/contracts/components/menubar.md` | `MenubarSpecimen.svelte` | `Menubar.test.ts` | `test:web-pack-install` | no consumer use found (absence is not a release failure) |
| `UiPresentationProvider` | `docs/contracts/components/ui-presentation-provider.md` | `UiPresentationProviderSpecimen.svelte` | `UiPresentationProvider.test.ts` | `test:web-pack-install` | `loophole`, `loophole-legacy` |
| `MotionPolicyProvider` | `docs/contracts/components/motion-policy-provider.md` | `MotionPolicyProviderSpecimen.svelte` | `MotionPolicyProvider.test.ts` | `test:web-pack-install` | no consumer use found (absence is not a release failure) |
| `VideoPlayer` | `docs/contracts/components/video-player.md` | `VideoPlayerSpecimen.svelte` | `VideoPlayer.test.ts` | `test:web-pack-install` | no consumer use found (absence is not a release failure) |
| `DateTimeZonePicker` | `docs/contracts/components/date-time-zone-picker.md` | `DateTimeZonePickerSpecimen.svelte` | `DateTimeZonePicker.test.ts` | `test:web-pack-install` | no consumer use found (absence is not a release failure) |

### Composites

| Component | Contract | Specimen | Focused Svelte test | Pack-install | Downstream use |
| --- | --- | --- | --- | --- | --- |
| `ActionDiscoveryPanel` | `docs/contracts/components/action-discovery-panel.md` | `ActionDiscoveryPanelSpecimen.svelte` | `ActionDiscoveryPanel.test.ts` | `test:web-pack-install` | no consumer use found (absence is not a release failure) |
| `AppHeader` | `docs/contracts/components/app-header.md` | `AppHeaderSpecimen.svelte` | `AppHeader.svelte.test.ts` | `test:web-pack-install` | `figmatic`, `nucleus` |
| `EditableList` | `docs/contracts/components/editable-list.md` | `EditableListSpecimen.svelte` | `EditableList.test.ts` | `test:web-pack-install` | `composer` |
| `ErrorBoundary` | `docs/contracts/components/error-boundary.md` | `ErrorBoundarySpecimen.svelte` | `ErrorBoundary.test.ts` | `test:web-pack-install` | `acowtancy`, `contact-patch`, `songsprout`, `underlay-reference` |
| `BlockEditor` | `docs/contracts/components/block-editor.md` | `BlockEditorSpecimen.svelte` | `BlockEditor.test.ts` | `test:web-pack-install` | `underlay` |
| `CardRadioGroup` | `docs/contracts/components/card-radio-group.md` | `CardRadioGroupSpecimen.svelte` | `CardRadioGroup.test.ts` | `test:web-pack-install` | `acowtancy`, `soundcheck-library` |
| `CardToggleGroup` | `docs/contracts/components/card-toggle-group.md` | `CardToggleGroupSpecimen.svelte` | `CardToggleGroup.test.ts` | `test:web-pack-install` | no consumer use found (absence is not a release failure) |
| `CommandPalette` | `docs/contracts/components/command-palette.md` | `CommandPaletteSpecimen.svelte` | `CommandPalette.test.ts` | `test:web-pack-install` | `longhorn`, `nucleus` |
| `ConfirmAction` | `docs/contracts/components/confirm-action.md` | `ConfirmActionSpecimen.svelte` | `ConfirmAction.test.ts` | `test:web-pack-install` | `longhorn`, `nucleus` |
| `DataTable` | `docs/contracts/components/data-table.md` | `DataTableSpecimen.svelte` | `DataTable.test.ts` | `test:web-pack-install` | `acowtancy`, `composer`, `underlay` |
| `DetailSectionGroup` | `docs/contracts/components/detail-section-group.md` | `DetailSectionGroupSpecimen.svelte` | `DetailSectionGroup.test.ts` | `test:web-pack-install` | `acowtancy`, `underlay` |
| `DetailSection` | `docs/contracts/components/detail-section.md` | `DetailSectionSpecimen.svelte` | `DetailSection.test.ts` | `test:web-pack-install` | `acowtancy`, `composer`, `finch`, `underlay` |
| `DockRegion` | `docs/contracts/components/dock-region.md` | `DockRegionSpecimen.svelte` | `DockRegionDragOverGate.svelte.test.ts`, `DockRegionExternalDrag.svelte.test.ts`, `DockRegionTabPassThroughs.svelte.test.ts`, `DockRegionZoneDrop.svelte.test.ts` | `test:web-pack-install` | `longhorn`, `loophole-legacy` |
| `DetailShell` | `docs/contracts/components/detail-shell.md` | `DetailShellSpecimen.svelte` | `DetailShell.test.ts` | `test:web-pack-install` | `finch` |
| `EmbedInput` | `docs/contracts/components/embed-input.md` | `EmbedInputSpecimen.svelte` | `EmbedInput.test.ts` | `test:web-pack-install` | no consumer use found (absence is not a release failure) |
| `EmbedPreview` | `docs/contracts/components/embed-preview.md` | `EmbedPreviewSpecimen.svelte` | `EmbedPreview.test.ts` | `test:web-pack-install` | `acowtancy` |
| `EmptyState` | `docs/contracts/components/empty-state.md` | shared `SceneSpecimen.svelte` (generated scene) | `EmptyState.test.ts` | `test:web-pack-install` | `acowtancy`, `compli-me`, `composer`, `contact-patch`, `figmatic`, `songsprout`, `soundcheck`, `soundcheck-library`, `underlay`, `underlay-reference` |
| `FilterToolbar` | `docs/contracts/components/filter-toolbar.md` | `FilterToolbarSpecimen.svelte` | `FilterToolbar.test.ts` | `test:web-pack-install` | `acowtancy` |
| `FormDialog` | `docs/contracts/components/form-dialog.md` | `FormDialogSpecimen.svelte` | `FormDialogInitialFocusHarness.svelte` | `test:web-pack-install` | `acowtancy`, `contact-patch`, `underlay`, `underlay-reference` |
| `FormLayout` | `docs/contracts/components/form-layout.md` | `FormLayoutSpecimen.svelte` | `FormLayout.test.ts` | `test:web-pack-install` | `acowtancy`, `composer` |
| `InlineListSection` | `docs/contracts/components/inline-list-section.md` | `InlineListSectionSpecimen.svelte` | `InlineListSection.test.ts` | `test:web-pack-install` | `acowtancy`, `longhorn`, `underlay` |
| `DebugDialog` | `docs/contracts/components/debug-dialog.md` | `DebugDialogSpecimen.svelte` | `DebugDialog.test.ts` | `test:web-pack-install` | no consumer use found (absence is not a release failure) |
| `LicenceActivation` | `docs/contracts/components/licence-activation.md` | `LicenceActivationSpecimen.svelte` | `LicenceActivation.test.ts` | `test:web-pack-install` | `longhorn` |
| `LicenceSeats` | `docs/contracts/components/licence-seats.md` | `LicenceSeatsSpecimen.svelte` | `LicenceSeats.test.ts` | `test:web-pack-install` | `longhorn` |
| `LicenceStatus` | `docs/contracts/components/licence-status.md` | `LicenceStatusSpecimen.svelte` | `LicenceStatus.test.ts` | `test:web-pack-install` | `longhorn` |
| `LogList` | `docs/contracts/components/log-list.md` | `LogListSpecimen.svelte` | `LogList.test.ts` | `test:web-pack-install` | `contact-patch`, `underlay-reference` |
| `ListContainer` | `docs/contracts/components/list-container.md` | `ListContainerSpecimen.svelte` | `ListContainer.test.ts` | `test:web-pack-install` | `underlay-reference` |
| `MarkdownEditor` | `docs/contracts/components/markdown-editor.md` | `MarkdownEditorSpecimen.svelte` | `MarkdownEditor.test.ts` | `test:web-pack-install` | `acowtancy`, `underlay`, `underlay-reference` |
| `PageLoading` | `docs/contracts/components/page-loading.md` | `PageLoadingSpecimen.svelte` | `PageLoading.test.ts` | `test:web-pack-install` | `acowtancy`, `compli-me`, `composer`, `contact-patch`, `underlay`, `underlay-reference` |
| `MediaPicker` | `docs/contracts/components/media-picker.md` | `MediaPickerSpecimen.svelte` | `MediaPicker.test.ts` | `test:web-pack-install` | no consumer use found (absence is not a release failure) |
| `MediaBrowsePanel` | `docs/contracts/components/media-browse-panel.md` | `MediaBrowsePanelSpecimen.svelte` | `MediaBrowsePanel.test.ts` | `test:web-pack-install` | no consumer use found (absence is not a release failure) |
| `MediaPreview` | `docs/contracts/components/media-preview.md` | `MediaPreviewSpecimen.svelte` | `MediaPreview.test.ts` | `test:web-pack-install` | no consumer use found (absence is not a release failure) |
| `MediaThumbnail` | `docs/contracts/components/media-thumbnail.md` | `MediaThumbnailSpecimen.svelte` | `MediaThumbnail.test.ts` | `test:web-pack-install` | `acowtancy`, `underlay` |
| `PageHeader` | `docs/contracts/components/page-header.md` | `PageHeaderSpecimen.svelte` | `PageHeader.test.ts` | `test:web-pack-install` | `acowtancy`, `compli-me`, `composer`, `contact-patch`, `figmatic`, `songsprout`, `soundcheck-library`, `underlay`, `underlay-reference` |
| `PickerShell` | `docs/contracts/components/picker-shell.md` | `PickerShellSpecimen.svelte` | `PickerShell.test.ts` | `test:web-pack-install` | no consumer use found (absence is not a release failure) |
| `RelationPicker` | `docs/contracts/components/relation-picker.md` | `RelationPickerSpecimen.svelte` | `RelationPicker.test.ts` | `test:web-pack-install` | `underlay` |
| `SelectionSummary` | `docs/contracts/components/selection-summary.md` | `SelectionSummarySpecimen.svelte` | `SelectionSummary.test.ts` | `test:web-pack-install` | no consumer use found (absence is not a release failure) |
| `SettingsShell` | `docs/contracts/components/settings-shell.md` | `SettingsShellSpecimen.svelte` | `SettingsShell.test.ts` | `test:web-pack-install` | `longhorn` |
| `SidebarNav` | `docs/contracts/components/sidebar-nav.md` | `SidebarNavSpecimen.svelte` | `SidebarNav.test.ts` | `test:web-pack-install` | no consumer use found (absence is not a release failure) |
| `Tree` | `docs/contracts/components/tree.md` | `TreeSpecimen.svelte` | `Tree.test.ts` | `test:web-pack-install` | `figmatic` |
| `SplitView` | `docs/contracts/components/split-view.md` | `SplitViewSpecimen.svelte` | `SplitView.svelte.test.ts` | `test:web-pack-install` | `longhorn`, `loophole-legacy`, `nucleus`, `soundcheck-library` |
| `MetricTile` | `docs/contracts/components/metric-tile.md` | `MetricTileSpecimen.svelte` | `MetricTile.test.ts` | `test:web-pack-install` | `acowtancy`, `compli-me`, `composer`, `contact-patch`, `underlay-reference` |
| `StateTile` | `docs/contracts/components/state-tile.md` | `StateTileSpecimen.svelte` | `WebParityCloseout.test.ts` | `test:web-pack-install` | no consumer use found (absence is not a release failure) |
| `ValidationSummary` | `docs/contracts/components/validation-summary.md` | `ValidationSummarySpecimen.svelte` | `WebParityCloseout.test.ts` | `test:web-pack-install` | no consumer use found (absence is not a release failure) |
| `ModelPicker` | `docs/contracts/components/model-picker.md` | `ModelPickerSpecimen.svelte` | `ModelPicker.test.ts` | `test:web-pack-install` | no consumer use found (absence is not a release failure) |
| `ModelConnectionPicker` | `docs/contracts/components/model-connection-picker.md` | `ModelConnectionPickerSpecimen.svelte` | `ModelConnectionPicker.test.ts` | `test:web-pack-install` | no consumer use found (absence is not a release failure) |
| `ModelConnectionSetup` | `docs/contracts/components/model-connection-setup.md` | `ModelConnectionSetupSpecimen.svelte` | `ModelConnectionSetup.test.ts` | `test:web-pack-install` | no consumer use found (absence is not a release failure) |
| `ModelConnectionCard` | `docs/contracts/components/model-connection-card.md` | `ModelConnectionCardSpecimen.svelte` | `ModelConnectionCard.test.ts` | `test:web-pack-install` | no consumer use found (absence is not a release failure) |
| `ModelCatalogueEditor` | `docs/contracts/components/model-catalogue-editor.md` | `ModelCatalogueEditorSpecimen.svelte` | `ModelCatalogueEditor.test.ts` | `test:web-pack-install` | no consumer use found (absence is not a release failure) |
| `MessageCenter` | `docs/contracts/components/message-center.md` | `MessageCenterSpecimen.svelte` | `MessageCenter.test.ts` | `test:web-pack-install` | `bovine-accelerator-desktop`, `figmatic`, `nucleus`, `soundcheck` |
| `HistoryCenter` | `docs/contracts/components/history-center.md` | `HistoryCenterSpecimen.svelte` | `HistoryCenter.test.ts` | `test:web-pack-install` | `soundcheck` |
| `UpdateStatus` | `docs/contracts/components/update-status.md` | `UpdateStatusSpecimen.svelte` | `UpdateStatus.test.ts` | `test:web-pack-install` | `longhorn` |
| `UpdateCenter` | `docs/contracts/components/update-center.md` | `UpdateCenterSpecimen.svelte` | `UpdateCenter.test.ts` | `test:web-pack-install` | `longhorn` |
| `ToastStack` | `docs/contracts/components/toast-stack.md` | `ToastStackSpecimen.svelte` | `ToastStack.test.ts` | `test:web-pack-install` | `longhorn` |
| `ToastHost` | `docs/contracts/components/toast-host.md` | `ToastHostSpecimen.svelte` | `ToastHost.test.ts` | `test:web-pack-install` | `acowtancy`, `compli-me`, `contact-patch`, `longhorn`, `nucleus`, `songsprout`, `underlay`, `underlay-reference` |

### Audio controls

| Component | Contract | Specimen | Focused Svelte test | Pack-install | Downstream use |
| --- | --- | --- | --- | --- | --- |
| `AudioMeter` | `docs/contracts/components/audio-meter.md` | `AudioMeterSpecimen.svelte` | `AudioControls.svelte.test.ts`, `MeterSurface.svelte.test.ts` | `test:web-pack-install` | `loophole` |
| `MeterSurface` | `docs/contracts/components/meter-surface.md` | `MeterSurfaceSpecimen.svelte` | `MeterSurface.svelte.test.ts` | `test:web-pack-install` | no consumer use found (absence is not a release failure) |
| `AudioSwitch` | `docs/contracts/components/audio-switch.md` | `AudioSwitchSpecimen.svelte` | `AudioControls.svelte.test.ts` | `test:web-pack-install` | no consumer use found (absence is not a release failure) |
| `DragNumberField` | `docs/contracts/components/drag-number-field.md` | `DragNumberFieldSpecimen.svelte` | `AudioControls.svelte.test.ts` | `test:web-pack-install` | `loophole` |
| `EnvelopeEditor` | `docs/contracts/components/envelope-editor.md` | `EnvelopeEditorSpecimen.svelte` | `AudioControls.svelte.test.ts` | `test:web-pack-install` | no consumer use found (absence is not a release failure) |
| `Fader` | `docs/contracts/components/fader.md` | `FaderSpecimen.svelte` | `AudioControls.svelte.test.ts` | `test:web-pack-install` | `loophole` |
| `GainReductionMeter` | `docs/contracts/components/gain-reduction-meter.md` | `GainReductionMeterSpecimen.svelte` | `AudioControls.svelte.test.ts` | `test:web-pack-install` | no consumer use found (absence is not a release failure) |
| `Keyboard` | `docs/contracts/components/keyboard.md` | `KeyboardSpecimen.svelte` | `AudioControls.svelte.test.ts` | `test:web-pack-install` | no consumer use found (absence is not a release failure) |
| `Knob` | `docs/contracts/components/knob.md` | `KnobSpecimen.svelte` | `AudioControls.svelte.test.ts` | `test:web-pack-install` | `loophole` |
| `ModMatrixGrid` | `docs/contracts/components/mod-matrix-grid.md` | `ModMatrixGridSpecimen.svelte` | `AudioControls.svelte.test.ts` | `test:web-pack-install` | no consumer use found (absence is not a release failure) |
| `ValueReadout` | `docs/contracts/components/value-readout.md` | `ValueReadoutSpecimen.svelte` | `AudioControls.svelte.test.ts` | `test:web-pack-install` | `loophole` |
| `WaveformDisplay` | `docs/contracts/components/waveform-display.md` | `WaveformDisplaySpecimen.svelte` | `AudioControls.svelte.test.ts` | `test:web-pack-install` | no consumer use found (absence is not a release failure) |
| `XYPad` | `docs/contracts/components/xy-pad.md` | `XYPadSpecimen.svelte` | `AudioControls.svelte.test.ts` | `test:web-pack-install` | no consumer use found (absence is not a release failure) |

## Cross-Runtime Surfaces (per component)

One runtime never borrows another runtime's pass. React mirror posture names implementation+export, gallery specimen, focused test, and the exact packed root-import proof. Rust declaration and render are recorded independently. GPUI posture names the specimen file; headless regression coverage is listed separately below the table. Pack-install cells prove symbol reachability from installed tarballs; they do not claim that every component was mounted.

### Foundations (primitives)

| Component | React impl/export | React gallery | Focused React test | Pack-install | Rust declaration | Rust render | GPUI specimen |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `Accordion` | complete | complete | `Accordion.test.tsx` | `test:web-pack-install` | `AccordionSpec` (`packages/contracts/components/src/accordion.rs`) | `packages/render/src/accordion.rs` | `packages/gpui/preview/src/specimens/accordion.rs` |
| `AgentChatInput` | complete | complete | `AgentChatInput.test.tsx` | `test:web-pack-install` | `AgentChatInputSpec` (`packages/contracts/components/src/agent_chat_input.rs`) | `packages/render/src/agent_chat_input.rs` | `packages/gpui/preview/src/specimens/agent_chat_input_specimen.rs` |
| `AudioPlayer` | complete | complete | `AudioPlayer.test.tsx` | `test:web-pack-install` | `AudioPlayerSpec` (`packages/contracts/components/src/audio_player.rs`) | `packages/render/src/audio_player.rs` | `packages/gpui/preview/src/specimens/audio_player_specimen.rs` |
| `AlertDialog` | complete | complete | `AlertDialog.test.tsx` | `test:web-pack-install` | `AlertDialogSpec` (`packages/contracts/components/src/alert_dialog.rs`) | `packages/render/src/alert_dialog.rs` | `packages/gpui/preview/src/specimens/alert_dialog.rs` |
| `Avatar` | complete | complete | `Avatar.test.tsx` | `test:web-pack-install` | `AvatarSpec` (`packages/contracts/components/src/avatar.rs`) | `packages/render/src/avatar.rs` | `packages/gpui/preview/src/specimens/avatar.rs` |
| `Box` | complete | complete | `Box.test.tsx` | `test:web-pack-install` | `BoxSpec` (`packages/contracts/components/src/box.rs`) | `packages/render/src/bx.rs` | `packages/gpui/preview/src/specimens/bx.rs` |
| `Breadcrumbs` | complete | complete | `Breadcrumbs.test.tsx` | `test:web-pack-install` | `BreadcrumbsSpec` (`packages/contracts/components/src/breadcrumbs.rs`) | `packages/render/src/breadcrumbs.rs` | `packages/gpui/preview/src/specimens/breadcrumbs_specimen.rs` |
| `BulkActionBar` | complete | complete | `BulkActionBar.test.tsx` | `test:web-pack-install` | `BulkActionBarSpec` (`packages/contracts/components/src/bulk_action_bar.rs`) | `packages/render/src/bulk_action_bar.rs` | `packages/gpui/preview/src/specimens/bulk_action_bar_specimen.rs` |
| `Button` | complete | complete | `Button.test.tsx` | `test:web-pack-install` | `ButtonSpec` (`packages/contracts/components/src/button.rs`) | `packages/render/src/button.rs` | `packages/gpui/preview/src/specimens/button.rs` |
| `Callout` | complete | complete | `Callout.test.tsx` | `test:web-pack-install` | `CallOutSpec` (`call_out.rs`, documented rename) | `packages/render/src/callout.rs` | `packages/gpui/preview/src/specimens/callout.rs` |
| `RemediationBanner` | complete | complete | `WebParityCloseout.test.tsx` | `test:web-pack-install` | `RemediationBannerSpec` (`packages/contracts/components/src/remediation_banner.rs`) | `packages/render/src/remediation_banner.rs` | `packages/gpui/preview/src/specimens/remediation_banner.rs` |
| `Card` | complete | complete | `Card.test.tsx` | `test:web-pack-install` | `CardSpec` (`packages/contracts/components/src/card.rs`) | `packages/render/src/card.rs` | `packages/gpui/preview/src/specimens/card_specimen.rs` |
| `Code` | complete | complete | `Code.test.tsx` | `test:web-pack-install` | `CodeSpec` (`packages/contracts/components/src/code.rs`) | `packages/render/src/code.rs` | `packages/gpui/preview/src/specimens/code.rs` |
| `ColorPicker` | complete | complete | `ColorPicker.test.tsx` | `test:web-pack-install` | `ColorPickerSpec` (`packages/contracts/components/src/color_picker.rs`) | `packages/render/src/color_picker.rs` | `packages/gpui/preview/src/specimens/color_picker.rs` |
| `Checkbox` | complete | complete | `interactions.test.tsx` | `test:web-pack-install` | `CheckboxSpec` (`packages/contracts/components/src/checkbox.rs`) | `packages/render/src/checkbox.rs` | `packages/gpui/preview/src/specimens/checkbox.rs` |
| `Calendar` | complete | complete | `Calendar.test.tsx` | `test:web-pack-install` | `CalendarSpec` (`packages/contracts/components/src/calendar.rs`) | `packages/render/src/calendar.rs` | `packages/gpui/preview/src/specimens/calendar.rs` |
| `ContextMenu` | complete | complete | `ContextMenu.test.tsx` | `test:web-pack-install` | `ContextMenuSpec` (`packages/contracts/components/src/context_menu.rs`) | `packages/render/src/context_menu.rs` | `packages/gpui/preview/src/specimens/context_menu.rs` |
| `CollapseToggle` | complete | complete | `CollapseToggle.test.tsx` | `test:web-pack-install` | `CollapseToggleSpec` (`packages/contracts/components/src/collapse_toggle.rs`) | `packages/render/src/collapse_toggle.rs` | `packages/gpui/preview/src/specimens/collapse_toggle.rs` |
| `Collapsible` | complete | complete | `Collapsible.test.tsx` | `test:web-pack-install` | `CollapsibleSpec` (`packages/contracts/components/src/collapsible.rs`) | `packages/render/src/collapsible.rs` | `packages/gpui/preview/src/specimens/collapsible.rs` |
| `DetailItem` | complete | complete | `DetailItem.test.tsx` | `test:web-pack-install` | `DetailItemSpec` (`packages/contracts/components/src/detail_item.rs`) | `packages/render/src/detail_item.rs` | `packages/gpui/preview/src/specimens/detail_item_specimen.rs` |
| `DatePicker` | complete | complete | `DatePicker.test.tsx` | `test:web-pack-install` | `DatePickerSpec` (`packages/contracts/components/src/date_picker.rs`) | `packages/render/src/date_picker.rs` | `packages/gpui/preview/src/specimens/date_picker.rs` |
| `DateRangePicker` | complete | complete | `DateRangePicker.test.tsx` | `test:web-pack-install` | `DateRangePickerSpec` (`packages/contracts/components/src/date_range_picker.rs`) | `packages/render/src/date_range_picker.rs` | `packages/gpui/preview/src/specimens/date_range_picker.rs` |
| `DateTimePicker` | complete | complete | `DateTimePicker.test.tsx` | `test:web-pack-install` | `DateTimePickerSpec` (`packages/contracts/components/src/date_time_picker.rs`) | `packages/render/src/date_time_picker.rs` | `packages/gpui/preview/src/specimens/date_time_picker.rs` |
| `DateTimeRangePicker` | complete | complete | `DateTimeRangePicker.test.tsx` | `test:web-pack-install` | `DateTimeRangePickerSpec` (`packages/contracts/components/src/date_time_range_picker.rs`) | `packages/render/src/date_time_range_picker.rs` | `packages/gpui/preview/src/specimens/date_time_range_picker.rs` |
| `Dialog` | complete | complete | `DialogDismissOutside.test.tsx`, `DialogInitialFocus.test.tsx` | `test:web-pack-install` | `DialogSpec` (`packages/contracts/components/src/dialog.rs`) | `packages/render/src/dialog.rs` | `packages/gpui/preview/src/specimens/dialog.rs` |
| `Drawer` | complete | complete | `DrawerDismissOutside.test.tsx` | `test:web-pack-install` | `DrawerSpec` (`packages/contracts/components/src/drawer.rs`) | `packages/render/src/drawer.rs` | `packages/gpui/preview/src/specimens/drawer.rs` |
| `DurationInput` | complete | complete | `DurationInput.test.tsx` | `test:web-pack-install` | `DurationInputSpec` (`packages/contracts/components/src/duration_input.rs`) | `packages/render/src/duration_input.rs` | `packages/gpui/preview/src/specimens/duration_input_specimen.rs` |
| `EditableLabel` | complete | complete | `EditableLabel.test.tsx` | `test:web-pack-install` | `EditableLabelSpec` (`packages/contracts/components/src/editable_label.rs`) | `packages/render/src/editable_label.rs` | `packages/gpui/preview/src/specimens/editable_label.rs` |
| `Eyebrow` | complete | complete | `Eyebrow.test.tsx` | `test:web-pack-install` | `EyebrowSpec` (`packages/contracts/components/src/eyebrow.rs`) | `packages/render/src/eyebrow.rs` | `packages/gpui/preview/src/specimens/eyebrow.rs` |
| `Field` | complete | complete | `Field.test.tsx` | `test:web-pack-install` | `FieldSpec` (`packages/contracts/components/src/field.rs`) | `packages/render/src/field.rs` | `packages/gpui/preview/src/specimens/field.rs` |
| `FieldSet` | complete | complete | `FieldSet.test.tsx` | `test:web-pack-install` | `FieldSetSpec` (`packages/contracts/components/src/field_set.rs`) | `packages/render/src/field_set.rs` | `packages/gpui/preview/src/specimens/field_set_specimen.rs` |
| `FileUpload` | complete | complete | `FileUpload.test.tsx` | `test:web-pack-install` | `FileUploadSpec` (`packages/contracts/components/src/file_upload.rs`) | `packages/render/src/file_upload.rs` | `packages/gpui/preview/src/specimens/file_upload.rs` |
| `FilterBuilder` | complete | complete | `FilterBuilder.test.tsx` | `test:web-pack-install` | `FilterBuilderSpec` (`packages/contracts/components/src/filter_builder.rs`) | `packages/render/src/filter_builder.rs` | `packages/gpui/preview/src/specimens/filter_builder_specimen.rs` |
| `FormActions` | complete | complete | `FormActions.test.tsx` | `test:web-pack-install` | `FormActionsSpec` (`packages/contracts/components/src/form_actions.rs`) | `packages/render/src/form_actions.rs` | `packages/gpui/preview/src/specimens/form_actions.rs` |
| `Grid` | complete | complete | `Grid.test.tsx` | `test:web-pack-install` | `GridSpec` (`packages/contracts/components/src/grid.rs`) | `packages/render/src/grid.rs` | `packages/gpui/preview/src/specimens/grid.rs` |
| `HoverCard` | complete | complete | `HoverCard.test.tsx` | `test:web-pack-install` | `HoverCardSpec` (`packages/contracts/components/src/hover_card.rs`) | `packages/render/src/hover_card.rs` | `packages/gpui/preview/src/specimens/hover_card.rs` |
| `Icon` | complete | complete | `IconProvider.test.tsx` | `test:web-pack-install` | `IconSpec` (`packages/contracts/components/src/icon.rs`) | `packages/render/src/icon.rs` | `packages/gpui/preview/src/specimens/icon.rs` |
| `IconButton` | complete | complete | `IconButton.test.tsx` | `test:web-pack-install` | `IconButtonSpec` (`packages/contracts/components/src/icon_button.rs`) | `packages/render/src/icon_button.rs` | `packages/gpui/preview/src/specimens/icon_button.rs` |
| `IconProvider` | complete | complete | `IconProvider.test.tsx` | `test:web-pack-install` | `IconProviderSpec` (`packages/contracts/components/src/icon_provider.rs`) | `packages/render/src/icon_provider.rs` | `packages/gpui/preview/src/specimens/icon_provider.rs` |
| `Meter` | complete | complete | `Meter.test.tsx` | `test:web-pack-install` | `MeterSpec` (`packages/contracts/components/src/meter.rs`) | `packages/render/src/meter.rs` | `packages/gpui/preview/src/specimens/meter.rs` |
| `ListCard` | complete | complete | `ListCard.test.tsx` | `test:web-pack-install` | `ListCardSpec` (`packages/contracts/components/src/list_card.rs`) | `packages/render/src/list_card.rs` | `packages/gpui/preview/src/specimens/list_card.rs` |
| `ListCardCounter` | complete | complete | `ListCardCounter.test.tsx` | `test:web-pack-install` | `ListCardCounterSpec` (`packages/contracts/components/src/list_card_counter.rs`) | `packages/render/src/list_card_counter.rs` | `packages/gpui/preview/src/specimens/list_card_counter.rs` |
| `ListGrid` | complete | complete | `ListGrid.test.tsx` | `test:web-pack-install` | `ListGridSpec` (`packages/contracts/components/src/list_grid.rs`) | `packages/render/src/list_grid.rs` | `packages/gpui/preview/src/specimens/list_grid.rs` |
| `Menu` | complete | complete | `Menu.test.tsx`, `OverlayGeometry.test.tsx` | `test:web-pack-install` | `MenuSpec` (`packages/contracts/components/src/menu.rs`) | `packages/render/src/menu.rs` | `packages/gpui/preview/src/specimens/menu.rs` |
| `MetaBar` | complete | complete | `MetaBar.test.tsx` | `test:web-pack-install` | `MetaBarSpec` (`packages/contracts/components/src/meta_bar.rs`) | `packages/render/src/meta_bar.rs` | `packages/gpui/preview/src/specimens/meta_bar.rs` |
| `MetaItem` | complete | complete | `MetaBar.test.tsx` | `test:web-pack-install` | `MetaItemSpec` (`packages/contracts/components/src/meta_item.rs`) | `packages/render/src/meta_item.rs` | `packages/gpui/preview/src/specimens/meta_item.rs` |
| `NumberInput` | complete | complete | `NumberInput.test.tsx` | `test:web-pack-install` | `NumberInputSpec` (`packages/contracts/components/src/number_input.rs`) | `packages/render/src/number_input.rs` | `packages/gpui/preview/src/specimens/number_input.rs` |
| `OrderBy` | complete | complete | `OrderBy.test.tsx` | `test:web-pack-install` | `OrderBySpec` (`packages/contracts/components/src/order_by.rs`) | `packages/render/src/order_by.rs` | `packages/gpui/preview/src/specimens/order_by_specimen.rs` |
| `NavCard` | complete | complete | `NavCard.test.tsx` | `test:web-pack-install` | `NavCardSpec` (`packages/contracts/components/src/nav_card.rs`) | `packages/render/src/nav_card.rs` | `packages/gpui/preview/src/specimens/nav_card.rs` |
| `NavigationMenu` | complete | complete | `NavigationMenu.test.tsx` | `test:web-pack-install` | `NavigationMenuSpec` (`packages/contracts/components/src/navigation_menu.rs`) | `packages/render/src/navigation_menu.rs` | `packages/gpui/preview/src/specimens/navigation_menu.rs` |
| `Pill` | complete | complete | `Pill.test.tsx` | `test:web-pack-install` | `PillSpec` (`packages/contracts/components/src/pill.rs`) | `packages/render/src/pill.rs` | `packages/gpui/preview/src/specimens/pill.rs` |
| `CodeInput` | complete | complete | `CodeInput.test.tsx` | `test:web-pack-install` | `CodeInputSpec` (`packages/contracts/components/src/code_input.rs`) | `packages/render/src/code_input.rs` | `packages/gpui/preview/src/specimens/code_input.rs` |
| `Popover` | complete | complete | `OverlayGeometry.test.tsx`, `PopoverRetained.test.tsx` | `test:web-pack-install` | `PopoverSpec` (`packages/contracts/components/src/popover.rs`) | `packages/render/src/popover.rs` | `packages/gpui/preview/src/specimens/popover.rs` |
| `Pagination` | complete | complete | `Pagination.test.tsx` | `test:web-pack-install` | `PaginationSpec` (`packages/contracts/components/src/pagination.rs`) | `packages/render/src/pagination.rs` | `packages/gpui/preview/src/specimens/pagination.rs` |
| `PaginationSummary` | complete | complete | `PaginationSummary.test.tsx` | `test:web-pack-install` | `PaginationSummarySpec` (`packages/contracts/components/src/pagination_summary.rs`) | `packages/render/src/pagination_summary.rs` | `packages/gpui/preview/src/specimens/pagination_summary_specimen.rs` |
| `PasswordRequirements` | complete | complete | `PasswordRequirements.test.tsx` | `test:web-pack-install` | `PasswordRequirementsSpec` (`packages/contracts/components/src/password_requirements.rs`) | `packages/render/src/password_requirements.rs` | `packages/gpui/preview/src/specimens/password_requirements.rs` |
| `Progress` | complete | complete | `Progress.test.tsx` | `test:web-pack-install` | `ProgressSpec` (`packages/contracts/components/src/progress.rs`) | `packages/render/src/progress.rs` | `packages/gpui/preview/src/specimens/progress.rs` |
| `Radio` | complete | complete | `Radio.test.tsx` | `test:web-pack-install` | `RadioSpec` (`packages/contracts/components/src/radio.rs`) | `packages/render/src/radio.rs` | `packages/gpui/preview/src/specimens/radio.rs` |
| `RefSelect` | complete | complete | `RefSelect.test.tsx` | `test:web-pack-install` | `RefSelectSpec` (`packages/contracts/components/src/ref_select.rs`) | `packages/render/src/ref_select.rs` | `packages/gpui/preview/src/specimens/ref_select_specimen.rs` |
| `RadioGroup` | complete | complete | `RadioGroup.test.tsx` | `test:web-pack-install` | `RadioGroupSpec` (`packages/contracts/components/src/radio_group.rs`) | `packages/render/src/radio_group.rs` | `packages/gpui/preview/src/specimens/radio_group.rs` |
| `Rating` | complete | complete | `Rating.test.tsx` | `test:web-pack-install` | `RatingSpec` (`packages/contracts/components/src/rating.rs`) | `packages/render/src/rating.rs` | `packages/gpui/preview/src/specimens/rating.rs` |
| `Region` | complete | complete | `Region.test.tsx` | `test:web-pack-install` | `RegionSpec` (`packages/contracts/components/src/region.rs`) | `packages/render/src/region.rs` | `packages/gpui/preview/src/specimens/region.rs` |
| `ResizeHandle` | complete | complete | `ResizeHandle.test.tsx` | `test:web-pack-install` | `ResizeHandleSpec` (`packages/contracts/components/src/resize_handle.rs`) | `packages/render/src/resize_handle.rs` | `packages/gpui/preview/src/specimens/resize_handle.rs` |
| `RangeSlider` | complete | complete | `RangeSlider.test.tsx` | `test:web-pack-install` | `RangeSliderSpec` (`packages/contracts/components/src/range_slider.rs`) | `packages/render/src/range_slider.rs` | `packages/gpui/preview/src/specimens/range_slider.rs` |
| `SegmentedControl` | complete | complete | `SegmentedControl.test.tsx` | `test:web-pack-install` | `SegmentedControlSpec` (`packages/contracts/components/src/segmented_control.rs`) | `packages/render/src/segmented_control.rs` | `packages/gpui/preview/src/specimens/segmented_control.rs` |
| `Select` | complete | complete | `Select.test.tsx` | `test:web-pack-install` | `SelectSpec` (`packages/contracts/components/src/select.rs`) | `packages/render/src/select.rs` | `packages/gpui/preview/src/specimens/select.rs` |
| `ScrollShell` | complete | complete | `ScrollShell.test.tsx` | `test:web-pack-install` | `ScrollShellSpec` (`packages/contracts/components/src/scroll_shell.rs`) | `packages/render/src/scroll_shell.rs` | `packages/gpui/preview/src/specimens/scroll_shell.rs` |
| `Separator` | complete | complete | `Separator.test.tsx` | `test:web-pack-install` | `SeparatorSpec` (`packages/contracts/components/src/separator.rs`) | `packages/render/src/separator.rs` | `packages/gpui/preview/src/specimens/separator.rs` |
| `SplitButton` | complete | complete | `SplitButton.test.tsx` | `test:web-pack-install` | `SplitButtonSpec` (`packages/contracts/components/src/split_button.rs`) | `packages/render/src/split_button.rs` | `packages/gpui/preview/src/specimens/split_button.rs` |
| `Skeleton` | complete | complete | `Skeleton.test.tsx` | `test:web-pack-install` | `SkeletonSpec` (`packages/contracts/components/src/skeleton.rs`) | `packages/render/src/skeleton.rs` | `packages/gpui/preview/src/specimens/skeleton.rs` |
| `Slider` | complete | complete | `Slider.test.tsx` | `test:web-pack-install` | `SliderSpec` (`packages/contracts/components/src/slider.rs`) | `packages/render/src/slider.rs` | `packages/gpui/preview/src/specimens/slider.rs` |
| `Spinner` | complete | complete | `Spinner.test.tsx` | `test:web-pack-install` | `SpinnerSpec` (`packages/contracts/components/src/spinner.rs`) | `packages/render/src/spinner.rs` | `packages/gpui/preview/src/specimens/spinner.rs` |
| `Spacer` | complete | complete | `Spacer.test.tsx` | `test:web-pack-install` | `SpacerSpec` (`packages/contracts/components/src/spacer.rs`) | `packages/render/src/spacer.rs` | `packages/gpui/preview/src/specimens/spacer.rs` |
| `Stack` | complete | complete | `Stack.test.tsx` | `test:web-pack-install` | `StackSpec` (`packages/contracts/components/src/stack.rs`) | `packages/render/src/stack.rs` | `packages/gpui/preview/src/specimens/stack.rs` |
| `Stepper` | complete | complete | `Stepper.test.tsx` | `test:web-pack-install` | `StepperSpec` (`packages/contracts/components/src/stepper.rs`) | `packages/render/src/stepper.rs` | `packages/gpui/preview/src/specimens/stepper.rs` |

### Agent surfaces

| Component | React impl/export | React gallery | Focused React test | Pack-install | Rust declaration | Rust render | GPUI specimen |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `AgentMessage` | complete | complete | `AgentMessage.test.tsx` | `test:web-pack-install` | `AgentMessageSpec` (`packages/contracts/components/src/agent_message.rs`) | `packages/render/src/agent_message.rs` | `packages/gpui/preview/src/specimens/agent_message.rs` |
| `AgentPlan` | complete | complete | `AgentPlan.test.tsx` | `test:web-pack-install` | `AgentPlanSpec` (`packages/contracts/components/src/agent_plan.rs`) | `packages/render/src/agent_plan.rs` | `packages/gpui/preview/src/specimens/agent_plan.rs` |
| `AgentPlanRecord` | complete | complete | `AgentPlanRecord.test.tsx` | `test:web-pack-install` | `AgentPlanRecordSpec` (`packages/contracts/components/src/agent_plan_record.rs`) | `packages/render/src/agent_plan_record.rs` | `packages/gpui/preview/src/specimens/agent_plan_record.rs` |
| `AgentQuestion` | complete | complete | `AgentQuestion.test.tsx` | `test:web-pack-install` | `AgentQuestionSpec` (`packages/contracts/components/src/agent_question.rs`) | `packages/render/src/agent_question.rs` | `packages/gpui/preview/src/specimens/agent_question.rs` |
| `AgentQuestionRecord` | complete | complete | `AgentQuestionRecord.test.tsx` | `test:web-pack-install` | `AgentQuestionRecordSpec` (`packages/contracts/components/src/agent_question_record.rs`) | `packages/render/src/agent_question_record.rs` | `packages/gpui/preview/src/specimens/agent_question_record.rs` |
| `AgentSubagent` | complete | complete | `AgentSubagent.test.tsx` | `test:web-pack-install` | `AgentSubagentSpec` (`packages/contracts/components/src/agent_subagent.rs`) | `packages/render/src/agent_subagent.rs` | `packages/gpui/preview/src/specimens/agent_subagent.rs` |
| `AgentTranscript` | complete | complete | `AgentSubagent.test.tsx` | `test:web-pack-install` | `AgentTranscriptSpec` (`packages/contracts/components/src/agent_transcript.rs`) | `packages/render/src/agent_transcript.rs` | `packages/gpui/preview/src/specimens/agent_transcript.rs` |
| `ChangedFiles` | complete | complete | `ChangedFiles.test.tsx` | `test:web-pack-install` | `ChangedFilesSpec` (`packages/contracts/components/src/changed_files.rs`) | `packages/render/src/changed_files.rs` | `packages/gpui/preview/src/specimens/changed_files.rs` |
| `ToolCall` | complete | complete | `ToolCall.test.tsx` | `test:web-pack-install` | `ToolCallSpec` (`packages/contracts/components/src/tool_call.rs`) | `packages/render/src/tool_call.rs` | `packages/gpui/preview/src/specimens/tool_call.rs` |
| `ToolCallGroup` | complete | complete | `ToolCallGroup.test.tsx` | `test:web-pack-install` | `ToolCallGroupSpec` (`packages/contracts/components/src/tool_call_group.rs`) | `packages/render/src/tool_call_group.rs` | `packages/gpui/preview/src/specimens/tool_call_group.rs` |

### Workstation systems

| Component | React impl/export | React gallery | Focused React test | Pack-install | Rust declaration | Rust render | GPUI specimen |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `StatusBar` | complete | complete | `StatusBar.test.tsx` | `test:web-pack-install` | `ShellStatusBarSpec` (`shell_status_bar.rs`, documented rename) | `packages/render/src/shell_status_bar.rs` | `packages/gpui/preview/src/specimens/status_bar.rs` |
| `StatusIndicator` | complete | complete | `StatusIndicator.test.tsx` | `test:web-pack-install` | `StatusIndicatorSpec` (`packages/contracts/components/src/status_indicator.rs`) | `packages/render/src/status_indicator.rs` | `packages/gpui/preview/src/specimens/status_indicator.rs` |
| `Surface` | complete | complete | `Surface.test.tsx` | `test:web-pack-install` | `SurfaceSpec` (`packages/contracts/components/src/surface.rs`) | `packages/render/src/surface.rs` | `packages/gpui/preview/src/specimens/surface.rs` |
| `Switch` | complete | complete | `interactions.test.tsx` | `test:web-pack-install` | `SwitchSpec` (`packages/contracts/components/src/switch.rs`) | `packages/render/src/switch.rs` | `packages/gpui/preview/src/specimens/switch.rs` |
| `Text` | complete | complete | `Text.test.tsx` | `test:web-pack-install` | `TextSpec` (`packages/contracts/components/src/text.rs`) | `packages/render/src/text.rs` | `packages/gpui/preview/src/specimens/text.rs` |
| `TextLink` | complete | complete | `TextLink.test.tsx` | `test:web-pack-install` | `TextLinkSpec` (`packages/contracts/components/src/text_link.rs`) | `packages/render/src/text_link.rs` | `packages/gpui/preview/src/specimens/text_link.rs` |
| `Tabs` | complete | complete | `TabsRovingFocus.test.tsx` | `test:web-pack-install` | `TabsSpec` (`packages/contracts/components/src/tabs.rs`) | `packages/render/src/tabs.rs` | `packages/gpui/preview/src/specimens/tabs.rs` |
| `Table` | complete | complete | `Table.test.tsx` | `test:web-pack-install` | `TableSpec` (`packages/contracts/components/src/table.rs`) | `packages/render/src/table.rs` | `packages/gpui/preview/src/specimens/table.rs` |
| `TimeAgo` | complete | complete | `TimeAgo.test.tsx` | `test:web-pack-install` | `TimeAgoSpec` (`packages/contracts/components/src/time_ago.rs`) | `packages/render/src/time_ago.rs` | `packages/gpui/preview/src/specimens/time_ago_specimen.rs` |
| `TextInput` | complete | complete | `TextInput.test.tsx` | `test:web-pack-install` | `TextInputSpec` (`packages/contracts/components/src/text_input.rs`) | `packages/render/src/text_input.rs` | `packages/gpui/preview/src/specimens/text_input.rs` |
| `TokenInput` | complete | complete | `TokenInput.test.tsx` | `test:web-pack-install` | `TokenInputSpec` (`packages/contracts/components/src/token_input.rs`) | `packages/render/src/token_input.rs` | `packages/gpui/preview/src/specimens/token_input.rs` |
| `TimeInput` | complete | complete | `TimeInput.test.tsx` | `test:web-pack-install` | `TimeFieldSpec` (`time_field.rs`, documented rename) | `packages/render/src/time_field.rs` | `packages/gpui/preview/src/specimens/time_field.rs` |
| `TimeZoneSelect` | complete | complete | `TimeZoneSelect.test.tsx` | `test:web-pack-install` | `TimeZoneSelectSpec` (`packages/contracts/components/src/time_zone_select.rs`) | `packages/render/src/time_zone_select.rs` | `packages/gpui/preview/src/specimens/time_zone_select.rs` |
| `ThemeSelect` | complete | complete | `ThemeSelect.test.tsx` | `test:web-pack-install` | `ThemeSelectSpec` (`packages/contracts/components/src/theme_select.rs`) | `packages/render/src/theme_select.rs` | `packages/gpui/preview/src/specimens/theme_select_specimen.rs` |
| `ToggleGroup` | complete | complete | `ToggleGroup.test.tsx` | `test:web-pack-install` | `ToggleGroupSpec` (`packages/contracts/components/src/toggle_group.rs`) | `packages/render/src/toggle_group.rs` | `packages/gpui/preview/src/specimens/toggle_group.rs` |
| `Toolbar` | complete | complete | `Toolbar.test.tsx` | `test:web-pack-install` | `ToolbarSpec` (`packages/contracts/components/src/toolbar.rs`) | `packages/render/src/toolbar.rs` | `packages/gpui/preview/src/specimens/toolbar.rs` |
| `Tooltip` | complete | complete | `Tooltip.test.tsx` | `test:web-pack-install` | `TooltipSpec` (`packages/contracts/components/src/tooltip.rs`) | `packages/render/src/tooltip.rs` | `packages/gpui/preview/src/specimens/tooltip.rs` |
| `TriStateSwitch` | complete | complete | `TriStateSwitch.test.tsx` | `test:web-pack-install` | `TriStateSwitchSpec` (`packages/contracts/components/src/tri_state_switch.rs`) | `packages/render/src/tri_state_switch.rs` | `packages/gpui/preview/src/specimens/tri_state_switch.rs` |
| `Menubar` | complete | complete | `Menubar.test.tsx` | `test:web-pack-install` | `MenubarSpec` (`packages/contracts/components/src/menubar.rs`) | `packages/render/src/menubar.rs` | `packages/gpui/preview/src/specimens/menubar.rs` |
| `UiPresentationProvider` | complete | complete | `UiPresentationProvider.test.tsx` | `test:web-pack-install` | `UiPresentationProviderSpec` (`packages/contracts/components/src/ui_presentation_provider.rs`) | `packages/render/src/context.rs` (`RenderContext`, crate-root `ui_presentation_provider`, scoped `SlotBuilder` host slots) | `packages/gpui/preview/src/specimens/ui_presentation_provider.rs` (real cascade: root, inherited scopes, nested override, explicit reset, with mounted headless geometry evidence) |
| `MotionPolicyProvider` | complete | complete | `MotionPolicyProvider.test.tsx` | `test:web-pack-install` | `MotionPolicyProviderSpec` (`packages/contracts/components/src/motion_policy_provider.rs`) | `packages/render/src/context.rs` (`RenderContext`, crate-root `motion_policy_provider`) | `packages/gpui/preview/src/specimens/motion_policy_provider.rs` |
| `VideoPlayer` | complete | complete | `VideoPlayer.test.tsx` | `test:web-pack-install` | `VideoPlayerSpec` (`packages/contracts/components/src/video_player.rs`) | `packages/render/src/video_player.rs` | `packages/gpui/preview/src/specimens/video_player_specimen.rs` |
| `DateTimeZonePicker` | complete | complete | `DateTimeZonePicker.test.tsx` | `test:web-pack-install` | `DateTimeZonePickerSpec` (`packages/contracts/components/src/date_time_zone_picker.rs`) | `packages/render/src/date_time_zone_picker.rs` | `packages/gpui/preview/src/specimens/date_time_zone_picker.rs` |

### Composites

| Component | React impl/export | React gallery | Focused React test | Pack-install | Rust declaration | Rust render | GPUI specimen |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `ActionDiscoveryPanel` | complete | complete | `ActionDiscoveryPanel.test.tsx` | `test:web-pack-install` | `ActionDiscoveryPanelSpec` (`packages/contracts/components/src/action_discovery_panel.rs`) | `packages/render/src/action_discovery_panel.rs` | `packages/gpui/preview/src/specimens/action_discovery_panel.rs` |
| `AppHeader` | complete | complete | `AppHeader.test.tsx` | `test:web-pack-install` | `AppHeaderSpec` (`packages/contracts/components/src/app_header.rs`) | `packages/render/src/app_header.rs` | `packages/gpui/preview/src/specimens/app_header.rs` |
| `EditableList` | complete | complete | `EditableList.test.tsx` | `test:web-pack-install` | `EditableListSpec` (`packages/contracts/components/src/editable_list.rs`) | `packages/render/src/editable_list.rs` | `packages/gpui/preview/src/specimens/editable_list_specimen.rs` |
| `ErrorBoundary` | complete | complete | `ErrorBoundary.test.tsx` | `test:web-pack-install` | `ErrorBoundarySpec` (`packages/contracts/components/src/error_boundary.rs`) | `packages/render/src/error_boundary.rs` | `packages/gpui/preview/src/specimens/error_boundary_specimen.rs` |
| `BlockEditor` | complete | complete | `BlockEditor.test.tsx` | `test:web-pack-install` | `BlockEditorSpec` (`packages/contracts/components/src/block_editor.rs`) | `packages/render/src/block_editor.rs` | `packages/gpui/preview/src/specimens/block_editor_specimen.rs` |
| `CardRadioGroup` | complete | complete | `CardRadioGroup.test.tsx` | `test:web-pack-install` | `CardRadioGroupSpec` (`packages/contracts/components/src/card_radio_group.rs`) | `packages/render/src/card_radio_group.rs` | `packages/gpui/preview/src/specimens/card_radio_group_specimen.rs` |
| `CardToggleGroup` | complete | complete | `CardToggleGroup.test.tsx` | `test:web-pack-install` | `CardToggleGroupSpec` (`packages/contracts/components/src/card_toggle_group.rs`) | `packages/render/src/card_toggle_group.rs` | `packages/gpui/preview/src/specimens/card_toggle_group_specimen.rs` |
| `CommandPalette` | complete | complete | `CommandPalette.test.tsx` | `test:web-pack-install` | `CommandPaletteSpec` (`packages/contracts/components/src/command_palette.rs`) | `packages/render/src/command_palette.rs` | `packages/gpui/preview/src/specimens/command_palette.rs` |
| `ConfirmAction` | complete | complete | `ConfirmAction.test.tsx` | `test:web-pack-install` | `ConfirmActionSpec` (`packages/contracts/components/src/confirm_action.rs`) | `packages/render/src/confirm_action.rs` | `packages/gpui/preview/src/specimens/confirm_action_specimen.rs` |
| `DataTable` | complete | complete | `DataTable.test.tsx` | `test:web-pack-install` | `DataTableSpec` (`packages/contracts/components/src/data_table.rs`) | `packages/render/src/data_table.rs` | `packages/gpui/preview/src/specimens/data_table.rs` |
| `DetailSectionGroup` | complete | complete | `DetailSectionGroup.test.tsx` | `test:web-pack-install` | `DetailSectionGroupSpec` (`packages/contracts/components/src/detail_section_group.rs`) | `packages/render/src/detail_section_group.rs` | `packages/gpui/preview/src/specimens/detail_section_group_specimen.rs` |
| `DetailSection` | complete | complete | `DetailSection.test.tsx` | `test:web-pack-install` | `DetailSectionSpec` (`packages/contracts/components/src/detail_section.rs`) | `packages/render/src/detail_section.rs` | `packages/gpui/preview/src/specimens/detail_section_specimen.rs` |
| `DockRegion` | complete | complete | `DockRegionDragOverGate.test.tsx`, `DockRegionTabPassThroughs.test.tsx`, `DockRegionZoneDrop.test.tsx` | `test:web-pack-install` | `DockRegionSpec` (`packages/contracts/components/src/dock_region.rs`) | `packages/render/src/dock_region.rs` | `packages/gpui/preview/src/specimens/dock_region.rs` |
| `DetailShell` | complete | complete | `DetailShell.test.tsx` | `test:web-pack-install` | `DetailShellSpec` (`packages/contracts/components/src/detail_shell.rs`) | `packages/render/src/detail_shell.rs` | `packages/gpui/preview/src/specimens/detail_shell.rs` |
| `EmbedInput` | complete | complete | `EmbedInput.test.tsx` | `test:web-pack-install` | `EmbedInputSpec` (`packages/contracts/components/src/embed_input.rs`) | `packages/render/src/embed_input.rs` | `packages/gpui/preview/src/specimens/embed_input_specimen.rs` |
| `EmbedPreview` | complete | complete | `EmbedPreview.test.tsx` | `test:web-pack-install` | `EmbedPreviewSpec` (`packages/contracts/components/src/embed_preview.rs`) | `packages/render/src/embed_preview.rs` | `packages/gpui/preview/src/specimens/embed_preview_specimen.rs` |
| `EmptyState` | complete | complete | `EmptyState.test.tsx` | `test:web-pack-install` | `EmptyStateSpec` (`packages/contracts/components/src/empty_state.rs`) | `packages/render/src/empty_state.rs` | `packages/gpui/preview/src/specimens/empty_state.rs` |
| `FilterToolbar` | complete | complete | `FilterToolbar.test.tsx` | `test:web-pack-install` | `FilterToolbarSpec` (`packages/contracts/components/src/filter_toolbar.rs`) | `packages/render/src/filter_toolbar.rs` | `packages/gpui/preview/src/specimens/filter_toolbar_specimen.rs` |
| `FormDialog` | complete | complete | `DialogInitialFocus.test.tsx` | `test:web-pack-install` | `FormDialogSpec` (`packages/contracts/components/src/form_dialog.rs`) | `packages/render/src/form_dialog.rs` | `packages/gpui/preview/src/specimens/form_dialog_specimen.rs` |
| `FormLayout` | complete | complete | `FormLayout.test.tsx` | `test:web-pack-install` | `FormLayoutSpec` (`packages/contracts/components/src/form_layout.rs`) | `packages/render/src/form_layout.rs` | `packages/gpui/preview/src/specimens/form_layout.rs` |
| `InlineListSection` | complete | complete | `InlineListSection.test.tsx` | `test:web-pack-install` | `InlineListSectionSpec` (`packages/contracts/components/src/inline_list_section.rs`) | `packages/render/src/inline_list_section.rs` | `packages/gpui/preview/src/specimens/inline_list_section_specimen.rs` |
| `DebugDialog` | complete | complete | `DebugDialog.test.tsx` | `test:web-pack-install` | `DebugDialogSpec` (`packages/contracts/components/src/debug_dialog.rs`) | `packages/render/src/debug_dialog.rs` | `packages/gpui/preview/src/specimens/debug_dialog_specimen.rs` |
| `LicenceActivation` | complete | complete | `LicenceActivation.test.tsx` | `test:web-pack-install` | `LicenceActivationSpec` (`packages/contracts/components/src/licence_activation.rs`) | `packages/render/src/licence_activation.rs` | `packages/gpui/preview/src/specimens/licence_activation.rs` |
| `LicenceSeats` | complete | complete | `LicenceSeats.test.tsx` | `test:web-pack-install` | `LicenceSeatsSpec` (`packages/contracts/components/src/licence_seats.rs`) | `packages/render/src/licence_seats.rs` | `packages/gpui/preview/src/specimens/licence_seats.rs` |
| `LicenceStatus` | complete | complete | `LicenceStatus.test.tsx` | `test:web-pack-install` | `LicenceStatusSpec` (`packages/contracts/components/src/licence_status.rs`) | `packages/render/src/licence_status.rs` | `packages/gpui/preview/src/specimens/licence_status.rs` |
| `LogList` | complete | complete | `LogList.test.tsx` | `test:web-pack-install` | `LogListSpec` (`packages/contracts/components/src/log_list.rs`) | `packages/render/src/log_list.rs` | `packages/gpui/preview/src/specimens/log_list_specimen.rs` |
| `ListContainer` | complete | complete | `ListContainer.test.tsx` | `test:web-pack-install` | `ListContainerSpec` (`packages/contracts/components/src/list_container.rs`) | `packages/render/src/list_container.rs` | `packages/gpui/preview/src/specimens/list_container_specimen.rs` |
| `MarkdownEditor` | complete | complete | `MarkdownEditor.test.tsx` | `test:web-pack-install` | `MarkdownEditorSpec` (`packages/contracts/components/src/markdown_editor.rs`) | `packages/render/src/markdown_editor.rs` | `packages/gpui/preview/src/specimens/markdown_editor_specimen.rs` |
| `PageLoading` | complete | complete | `PageLoading.test.tsx` | `test:web-pack-install` | `PageLoadingSpec` (`packages/contracts/components/src/page_loading.rs`) | `packages/render/src/page_loading.rs` | `packages/gpui/preview/src/specimens/page_loading_specimen.rs` |
| `MediaPicker` | complete | complete | `MediaPicker.test.tsx` | `test:web-pack-install` | `MediaPickerSpec` (`packages/contracts/components/src/media_picker.rs`) | `packages/render/src/media_picker.rs` | `packages/gpui/preview/src/specimens/media_picker_specimen.rs` |
| `MediaBrowsePanel` | complete | complete | `MediaBrowsePanel.test.tsx` | `test:web-pack-install` | `MediaBrowsePanelSpec` (`packages/contracts/components/src/media_browse_panel.rs`) | `packages/render/src/media_browse_panel.rs` | `packages/gpui/preview/src/specimens/media_browse_panel_specimen.rs` |
| `MediaPreview` | complete | complete | `MediaPreview.test.tsx` | `test:web-pack-install` | `MediaPreviewSpec` (`packages/contracts/components/src/media_preview.rs`) | `packages/render/src/media_preview.rs` | `packages/gpui/preview/src/specimens/media_preview_specimen.rs` |
| `MediaThumbnail` | complete | complete | `MediaThumbnail.test.tsx` | `test:web-pack-install` | `MediaThumbnailSpec` (`packages/contracts/components/src/media_thumbnail.rs`) | `packages/render/src/media_thumbnail.rs` | `packages/gpui/preview/src/specimens/media_thumbnail_specimen.rs` |
| `PageHeader` | complete | complete | `PageHeader.test.tsx` | `test:web-pack-install` | `PageHeaderSpec` (`packages/contracts/components/src/page_header.rs`) | `packages/render/src/page_header.rs` | `packages/gpui/preview/src/specimens/page_header_specimen.rs` |
| `PickerShell` | complete | complete | `PickerShell.test.tsx` | `test:web-pack-install` | `PickerShellSpec` (`packages/contracts/components/src/picker_shell.rs`) | `packages/render/src/picker_shell.rs` | `packages/gpui/preview/src/specimens/picker_shell_specimen.rs` |
| `RelationPicker` | complete | complete | `RelationPicker.test.tsx` | `test:web-pack-install` | `RelationPickerSpec` (`packages/contracts/components/src/relation_picker.rs`) | `packages/render/src/relation_picker.rs` | `packages/gpui/preview/src/specimens/relation_picker_specimen.rs` |
| `SelectionSummary` | complete | complete | `SelectionSummary.test.tsx` | `test:web-pack-install` | `SelectionSummarySpec` (`packages/contracts/components/src/selection_summary.rs`) | `packages/render/src/selection_summary.rs` | `packages/gpui/preview/src/specimens/selection_summary_specimen.rs` |
| `SettingsShell` | complete | complete | `SettingsShell.test.tsx` | `test:web-pack-install` | `SettingsShellSpec` (`packages/contracts/components/src/settings_shell.rs`) | `packages/render/src/settings_shell.rs` | `packages/gpui/preview/src/specimens/settings_shell.rs` |
| `SidebarNav` | complete | complete | `SidebarNav.test.tsx` | `test:web-pack-install` | `SidebarNavSpec` (`packages/contracts/components/src/sidebar_nav.rs`) | `packages/render/src/sidebar_nav.rs` | `packages/gpui/preview/src/specimens/sidebar_nav.rs` |
| `Tree` | complete | complete | `Tree.test.tsx` | `test:web-pack-install` | `TreeSpec` (`packages/contracts/components/src/tree.rs`) | `packages/render/src/tree.rs` | `packages/gpui/preview/src/specimens/tree.rs` |
| `SplitView` | complete | complete | `SplitView.test.tsx` | `test:web-pack-install` | `SplitViewSpec` (`packages/contracts/components/src/split_view.rs`) | `packages/render/src/split_view.rs` | `packages/gpui/preview/src/specimens/split_view_specimen.rs` |
| `MetricTile` | complete | complete | `MetricTile.test.tsx` | `test:web-pack-install` | `MetricTileSpec` (`packages/contracts/components/src/metric_tile.rs`) | `packages/render/src/metric_tile.rs` | `packages/gpui/preview/src/specimens/metric_tile_specimen.rs` |
| `StateTile` | complete | complete | `WebParityCloseout.test.tsx` | `test:web-pack-install` | `StateTileSpec` (`packages/contracts/components/src/state_tile.rs`) | `packages/render/src/state_tile.rs` | `packages/gpui/preview/src/specimens/state_tile.rs` |
| `ValidationSummary` | complete | complete | `WebParityCloseout.test.tsx` | `test:web-pack-install` | `ValidationSummarySpec` (`packages/contracts/components/src/validation_summary.rs`) | `packages/render/src/validation_summary.rs` | `packages/gpui/preview/src/specimens/validation_summary.rs` |
| `ModelPicker` | complete | complete | `ModelPicker.test.tsx` | `test:web-pack-install` | `ModelPickerSpec` (`packages/contracts/components/src/model_picker.rs`) | `packages/render/src/model_picker.rs` | `packages/gpui/preview/src/specimens/model_picker_specimen.rs` |
| `ModelConnectionPicker` | complete | complete | `ModelConnection.test.tsx` | `test:web-pack-install` | `ModelConnectionPickerSpec` (`packages/contracts/components/src/model_connection_picker.rs`) | `packages/render/src/model_connection_picker.rs` | `packages/gpui/preview/src/specimens/model_connection_picker_specimen.rs` |
| `ModelConnectionSetup` | complete | complete | `ModelConnection.test.tsx` | `test:web-pack-install` | `ModelConnectionSetupSpec` (`packages/contracts/components/src/model_connection_setup.rs`) | `packages/render/src/model_connection_setup.rs` | `packages/gpui/preview/src/specimens/model_connection_setup_specimen.rs` |
| `ModelConnectionCard` | complete | complete | `ModelConnection.test.tsx` | `test:web-pack-install` | `ModelConnectionCardSpec` (`packages/contracts/components/src/model_connection_card.rs`) | `packages/render/src/model_connection_card.rs` | `packages/gpui/preview/src/specimens/model_connection_card_specimen.rs` |
| `ModelCatalogueEditor` | complete | complete | `ModelConnection.test.tsx` | `test:web-pack-install` | `ModelCatalogueEditorSpec` (`packages/contracts/components/src/model_catalogue_editor.rs`) | `packages/render/src/model_catalogue_editor.rs` | `packages/gpui/preview/src/specimens/model_catalogue_editor_specimen.rs` |
| `MessageCenter` | complete | complete | `MessageCenter.test.tsx` | `test:web-pack-install` | `MessageCenterSpec` (`packages/contracts/components/src/message_center.rs`) | `packages/render/src/message_center.rs` | `packages/gpui/preview/src/specimens/message_center_specimen.rs` |
| `HistoryCenter` | complete | complete | `HistoryCenter.test.tsx` | `test:web-pack-install` | `HistoryCenterSpec` (`packages/contracts/components/src/history_center.rs`) | `packages/render/src/history_center.rs` | `packages/gpui/preview/src/specimens/history_center_specimen.rs` |
| `UpdateStatus` | complete | complete | `UpdateStatus.test.tsx` | `test:web-pack-install` | `UpdateStatusSpec` (`packages/contracts/components/src/update_status.rs`) | `packages/render/src/update_status.rs` | `packages/gpui/preview/src/specimens/update_status.rs` |
| `UpdateCenter` | complete | complete | `UpdateCenter.test.tsx` | `test:web-pack-install` | `UpdateCenterSpec` (`packages/contracts/components/src/update_center.rs`) | `packages/render/src/update_center.rs` | `packages/gpui/preview/src/specimens/update_center.rs` |
| `ToastStack` | complete | complete | `ToastStack.test.tsx` | `test:web-pack-install` | `ToastStackSpec` (`packages/contracts/components/src/toast_stack.rs`) | `packages/render/src/toast_stack.rs` | `packages/gpui/preview/src/specimens/toast_stack_specimen.rs` |
| `ToastHost` | complete | complete | `ToastHost.test.tsx` | `test:web-pack-install` | `ToastHostSpec` (`packages/contracts/components/src/toast_host.rs`) | `packages/render/src/toast_host.rs` | `packages/gpui/preview/src/specimens/toast_host.rs` |

### Audio controls

| Component | React impl/export | React gallery | Focused React test | Pack-install | Rust declaration | Rust render | GPUI specimen |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `AudioMeter` | complete | complete | `AudioControls.test.tsx`, `MeterSurface.test.tsx` | `test:web-pack-install` | `AudioMeterSpec` (`packages/contracts/components/src/audio_meter.rs`) | `packages/render/src/audio.rs` (batched family) | `packages/gpui/preview/src/specimens/audio_controls.rs` (batched family) |
| `MeterSurface` | complete | complete | `MeterSurface.test.tsx` | `test:web-pack-install` | not-applicable — web-only (spec 068) | not-applicable — web-only (spec 068) | not-applicable — web-only (spec 068) |
| `AudioSwitch` | complete | complete | `AudioControls.test.tsx` | `test:web-pack-install` | `AudioSwitchSpec` (`packages/contracts/components/src/audio_switch.rs`) | `packages/render/src/audio.rs` (batched family) | `packages/gpui/preview/src/specimens/audio_controls.rs` (batched family) |
| `DragNumberField` | complete | complete | `AudioControls.test.tsx` | `test:web-pack-install` | `DragNumberFieldSpec` (`packages/contracts/components/src/drag_number_field.rs`) | `packages/render/src/audio.rs` (batched family) | `packages/gpui/preview/src/specimens/audio_controls.rs` (batched family) |
| `EnvelopeEditor` | complete | complete | `AudioControls.test.tsx` | `test:web-pack-install` | `EnvelopeEditorSpec` (`packages/contracts/components/src/envelope_editor.rs`) | `packages/render/src/audio.rs` (batched family) | `packages/gpui/preview/src/specimens/audio_controls.rs` (batched family) |
| `Fader` | complete | complete | `AudioControls.test.tsx` | `test:web-pack-install` | `FaderSpec` (`packages/contracts/components/src/fader.rs`) | `packages/render/src/audio.rs` (batched family) | `packages/gpui/preview/src/specimens/audio_controls.rs` (batched family) |
| `GainReductionMeter` | complete | complete | `AudioControls.test.tsx` | `test:web-pack-install` | `GainReductionMeterSpec` (`packages/contracts/components/src/gain_reduction_meter.rs`) | `packages/render/src/audio.rs` (batched family) | `packages/gpui/preview/src/specimens/audio_controls.rs` (batched family) |
| `Keyboard` | complete | complete | `AudioControls.test.tsx` | `test:web-pack-install` | `KeyboardSpec` (`packages/contracts/components/src/keyboard.rs`) | `packages/render/src/audio.rs` (batched family) | `packages/gpui/preview/src/specimens/audio_controls.rs` (batched family) |
| `Knob` | complete | complete | `AudioControls.test.tsx` | `test:web-pack-install` | `KnobSpec` (`packages/contracts/components/src/knob.rs`) | `packages/render/src/audio.rs` (batched family) | `packages/gpui/preview/src/specimens/audio_controls.rs` (batched family) |
| `ModMatrixGrid` | complete | complete | `AudioControls.test.tsx` | `test:web-pack-install` | `ModMatrixGridSpec` (`packages/contracts/components/src/mod_matrix_grid.rs`) | `packages/render/src/audio.rs` (batched family) | `packages/gpui/preview/src/specimens/audio_controls.rs` (batched family) |
| `ValueReadout` | complete | complete | `AudioControls.test.tsx` | `test:web-pack-install` | `ValueReadoutSpec` (`packages/contracts/components/src/value_readout.rs`) | `packages/render/src/audio.rs` (batched family) | `packages/gpui/preview/src/specimens/audio_controls.rs` (batched family) |
| `WaveformDisplay` | complete | complete | `AudioControls.test.tsx` | `test:web-pack-install` | `WaveformDisplaySpec` (`packages/contracts/components/src/waveform_display.rs`) | `packages/render/src/audio.rs` (batched family) | `packages/gpui/preview/src/specimens/audio_controls.rs` (batched family) |
| `XYPad` | complete | complete | `AudioControls.test.tsx` | `test:web-pack-install` | `XYPadSpec` (`packages/contracts/components/src/xy_pad.rs`) | `packages/render/src/audio.rs` (batched family) | `packages/gpui/preview/src/specimens/audio_controls.rs` (batched family) |

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
