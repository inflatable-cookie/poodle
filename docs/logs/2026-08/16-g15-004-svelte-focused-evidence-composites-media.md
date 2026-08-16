# g15.004 — Svelte focused evidence: composites & media

Status: complete — all four batches landed (PR pending)
Date: 2026-08-16
Card: `docs/roadmaps/g15/004-svelte-focused-evidence-composites-media.md`
Governing refs: `docs/roadmaps/g15/release-baseline-roster.md`,
`docs/roadmaps/g15/release-gap-register.md`,
`docs/contracts/001-working-rules.md`

## Batches

The card's four named batches were executed in order, each with a narrow test
round at the end. The roster and register evidence rows were updated once all
four batches were green.

- **Batch A — data & list composites (9):** DataTable, DetailSection,
  DetailSectionGroup, DetailShell, ListContainer, LogList, MetricTile,
  SelectionSummary, SidebarNav
- **Batch B — media & content editors (8):** BlockEditor, EmbedInput,
  EmbedPreview, MarkdownEditor, MediaBrowsePanel, MediaPicker, MediaPreview,
  MediaThumbnail
- **Batch C — selection & feedback composites (8):** CardRadioGroup,
  CardToggleGroup, CommandPalette, ConfirmAction, EmptyState, ErrorBoundary,
  ToastStack, ToastHost
- **Batch D — workflow & shell composites (10):** ActionDiscoveryPanel,
  DebugDialog, EditableList, FilterToolbar, FormLayout, InlineListSection,
  PageHeader, PageLoading, PickerShell, RelationPicker

## Evidence Landed

Every scoped component now has a named focused test case on the Svelte side
(`packages/svelte/components/test/<Name>.test.ts`) and the mirrored contract
cases on the React side (`packages/react/components/test/<Name>.test.tsx`),
asserting load-bearing observable contract behaviour: selection and value
flows, workflow states, keyboard/accessibility projection, or composition
surface semantics. The anatomy smoke (`smoke.test.ts`) is not reused as
evidence; each new file asserts behaviour beyond mounting.

Svelte file / React file per component (also recorded in the roster):

| Component | Svelte evidence | React evidence |
| --- | --- | --- |
| DataTable | `DataTable.test.ts` | `DataTable.test.tsx` |
| DetailSection | `DetailSection.test.ts` | `DetailSection.test.tsx` |
| DetailSectionGroup | `DetailSectionGroup.test.ts` | `DetailSectionGroup.test.tsx` |
| DetailShell | `DetailShell.test.ts` | `DetailShell.test.tsx` |
| ListContainer | `ListContainer.test.ts` | `ListContainer.test.tsx` |
| LogList | `LogList.test.ts` | `LogList.test.tsx` |
| MetricTile | `MetricTile.test.ts` | `MetricTile.test.tsx` |
| SelectionSummary | `SelectionSummary.test.ts` | `SelectionSummary.test.tsx` |
| SidebarNav | `SidebarNav.test.ts` | `SidebarNav.test.tsx` |
| BlockEditor | `BlockEditor.test.ts` | `BlockEditor.test.tsx` |
| EmbedInput | `EmbedInput.test.ts` | `EmbedInput.test.tsx` |
| EmbedPreview | `EmbedPreview.test.ts` | `EmbedPreview.test.tsx` |
| MarkdownEditor | `MarkdownEditor.test.ts` | `MarkdownEditor.test.tsx` |
| MediaBrowsePanel | `MediaBrowsePanel.test.ts` | `MediaBrowsePanel.test.tsx` |
| MediaPicker | `MediaPicker.test.ts` | `MediaPicker.test.tsx` |
| MediaPreview | `MediaPreview.test.ts` | `MediaPreview.test.tsx` |
| MediaThumbnail | `MediaThumbnail.test.ts` | `MediaThumbnail.test.tsx` |
| CardRadioGroup | `CardRadioGroup.test.ts` | `CardRadioGroup.test.tsx` |
| CardToggleGroup | `CardToggleGroup.test.ts` | `CardToggleGroup.test.tsx` |
| CommandPalette | `CommandPalette.test.ts` | `CommandPalette.test.tsx` |
| ConfirmAction | `ConfirmAction.test.ts` | `ConfirmAction.test.tsx` |
| EmptyState | `EmptyState.test.ts` | `EmptyState.test.tsx` |
| ErrorBoundary | `ErrorBoundary.test.ts` | `ErrorBoundary.test.tsx` |
| ToastStack | `ToastStack.test.ts` | `ToastStack.test.tsx` |
| ToastHost | `ToastHost.test.ts` | `ToastHost.test.tsx` |
| ActionDiscoveryPanel | `ActionDiscoveryPanel.test.ts` | `ActionDiscoveryPanel.test.tsx` |
| DebugDialog | `DebugDialog.test.ts` | `DebugDialog.test.tsx` |
| EditableList | `EditableList.test.ts` | `EditableList.test.tsx` |
| FilterToolbar | `FilterToolbar.test.ts` | `FilterToolbar.test.tsx` |
| FormLayout | `FormLayout.test.ts` | `FormLayout.test.tsx` |
| InlineListSection | `InlineListSection.test.ts` | `InlineListSection.test.tsx` |
| PageHeader | `PageHeader.test.ts` | `PageHeader.test.tsx` |
| PageLoading | `PageLoading.test.ts` | `PageLoading.test.tsx` |
| PickerShell | `PickerShell.test.ts` | `PickerShell.test.tsx` |
| RelationPicker | `RelationPicker.test.ts` | `RelationPicker.test.tsx` |

Supporting harnesses: `ErrorBoundaryHarness.svelte` + `ErrorBoundaryBomb.svelte`
(one-shot throwing child for the Svelte `svelte:boundary` catch-and-retry
path), and `ConfirmActionTriggerHarness.svelte` (compiled trigger snippet; raw
thunks cannot materialize interactive content under the Svelte 5 runtime).

Representative load-bearing cases per family (full list lives in the files):

- **Batch A**: DataTable sort direction from controlled state, row/select-all
  toggles, aria-sort projection, pagination footer clamping, CSV export with
  hidden-column exclusion (URL.createObjectURL stubbed); DetailSection header
  conditional rendering, h3/description, data-columns/separated attributes;
  DetailSectionGroup layout attributes and min-column custom properties;
  DetailShell body/state region switching with loading spinner; ListContainer
  state switching, pagination summary, page-change forwarding; LogList
  stream level/text filtering, maxEntries capping, audit actor/resource
  linking and clear-filters gating; MetricTile accessible-name computation,
  trend data + hidden arrow, sparkline path from 2+ points; SelectionSummary
  truncation overflow, remove/clear payloads, split activate/remove mode;
  SidebarNav anchor/button roots, aria-current, disabled inertness, empty-group
  filtering.
- **Batch B**: EmbedInput debounced provider detection, provider restriction,
  custom resolver routing; EmbedPreview render-priority states, sandboxed
  privacy-enhanced YouTube/Vimeo iframes, raw/trusted HTML containers, fallback
  identity, fixed-aspect recording; MediaPicker listbox semantics, select +
  auto-close, search filtering, upload tab; MediaBrowsePanel loading/error/
  empty/ready states and load-more disabling; MediaThumbnail kind/state/aspect
  data attributes, play indicator, default state titles, badge/caption,
  compact suppression; MediaPreview metadata list with thumbnailMeta prepend,
  error posture passthrough, badge overlay; BlockEditor labelled groups,
  textarea fallback, move/remove flows with boundary guards, single-mode
  suppression; MarkdownEditor edit/preview/split panes, tool disabling in
  preview, custom renderer, Bold wrap insertion.
- **Batch C**: CardRadioGroup radiogroup semantics, checked dot, roving
  tabindex, arrow-key navigation over enabled items; CardToggleGroup
  aria-pressed, deactivation clearing to null, reselect re-emission;
  CommandPalette modal dialog, query reporting, option selection, no-results
  status copy, Escape/close/Enter flows, invocation hint; ConfirmAction
  trigger-to-confirm/cancel, custom trigger, danger tone derivation;
  EmptyState labelled section, variant/size attributes, decorative visual,
  actions region gating; ErrorBoundary catch-and-retry on both runtimes
  (React uses a class boundary; Svelte uses `svelte:boundary`); ToastStack
  tone escalation to assertive for danger, dismiss/action payloads; ToastHost
  store-driven placement, variant-to-tone normalisation, auto-dismiss timers
  preserving sticky toasts, onDismiss/onAction forwarding.
- **Batch D**: ActionDiscoveryPanel listbox grouping, active aria-selected,
  shortcut/badge chips, five-row loading skeletons, contextual empty states;
  DebugDialog null-value hiding, pretty-printed JSON; EditableList
  listbox labels, add/remove payloads, keyboard grab-move-drop reorder,
  maxItems counter, dirty-gated workflow chrome, alert/status surfaces;
  FilterToolbar toolbar semantics, collapse toggle, sticky flag; FormLayout
  field-error alert list, callout messaging, column custom property;
  InlineListSection titled section, count badge, framed/bare postures, empty
  copy; PageHeader heading level, eyebrow/section/subtitle, count Pill,
  back-link label resolution + contextual dot, banner Callout,
  entity-detail posture swap; PageLoading status region, overlay/inline
  backdrop, determinate progress, cancel; PickerShell variant/state
  attributes, body/state switching, spinner fallback, sr-only live region;
  RelationPicker candidate listbox, single/multiple toggle semantics,
  disabled candidate inertness, confirm/cancel payloads, query reporting,
  selection-summary gating, drill-down advance with breadcrumbs.

## Contract Ambiguity Resolved (no fix, no contract change)

- **PickerShell state-title fallback** — the contract (§3/§4) requires
  per-state fallback copy (`"Loading results"`, `"Something went wrong"`,
  `"Nothing here yet"`, `"No results"` with matching messages), but the Svelte
  build falls back to a flat `"Picker state"` literal. The contract documents
  this as a known Svelte gap in §9 and keeps the contract authoritative, so the
  evidence asserts only explicit `stateTitle`/`stateMessage` projection and the
  fallback spinner; it does not assert the missing per-state copy. The gap
  stays recorded for a separate fix, not this card.

## Bounded Fixes (contract-first)

None. All scoped evidence passed against the current implementations without a
fix; the only implementation-adjacent work was test-side URL stubbing
(`URL.createObjectURL` for the DataTable CSV export case) and test harnesses.

## Observations (no change made)

- Svelte 5 raw snippet thunks render as comment nodes under happy-dom, so
  content-bearing snippet assertions in FilterToolbar/FormLayout/PageHeader/
  PickerShell use element-presence or structural assertions (matching the
  `PAPERCUTS.md` note); the ConfirmAction custom-trigger case needed a compiled
  harness.
- React 19 error boundaries re-render the failed subtree once on catch, so a
  one-shot throwing child never shows the fallback; the React ErrorBoundary
  suite drives the child from an external flag cleared before retry. React
  also logs `console.error` on boundary catches, which the global setup
  fails on — the suite silences it locally.
- React fake timers conflict with React's own scheduler, so the ToastHost
  timer suites use small real delays on the React side and `vi.useFakeTimers`
  on the Svelte side.
- `PickerShell`'s flat `"Picker state"` fallback is the known Svelte gap
  recorded in the contract (§9); see "Contract Ambiguity Resolved".

## Validation

| Command | Result |
| --- | --- |
| Batch A narrow round (`vitest run` svelte-components + react-components, touched files) | pass (122 tests) |
| Batch B narrow round (touched files) | pass (110 tests) |
| Batch C narrow round (touched files) | pass (90 tests) |
| Batch D narrow round (touched files) | pass (122 tests) |
| `effigy check:svelte` | pass (0 errors) |
| `effigy react:build` | pass |
| `effigy test:components` | pass (233 files, 2031 tests) |
| `effigy docs:check` | pass |
| `git diff --check` | pass |

No `*-windowed` selector, `test:native-visual`, `qa:jetstream`, or Jetstream
selector ran.

## Register and Roster Updates

- `release-baseline-roster.md`: the 35 components' Focused Svelte test cells
  and Focused React test cells now name the case files; summary counts moved
  to Focused Svelte 125 present / 50 missing and Focused React 122 present /
  53 missing.
- `release-gap-register.md`: the Svelte focused-evidence blocker class count
  moved 85 → 50; the "Composites & media" family row (35 components) is closed
  with evidence recorded in the roster. No status line was changed.
- `docs/roadmaps/g15/004-…` card, `README.md`, and `dispatch.md` were not
  modified by the worker.

## Change Footprint

`packages/svelte/components/test/` (35 new test files + 3 harnesses),
`packages/react/components/test/` (35 new test files), and the two
focused-evidence docs. No Svelte or React implementation, contract, specimen,
package export, workflow, or downstream repository changed.
