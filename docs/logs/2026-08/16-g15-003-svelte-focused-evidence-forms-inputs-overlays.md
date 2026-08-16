# g15.003 — Svelte focused evidence: foundation forms, inputs & overlays

Status: complete — all three batches landed
Date: 2026-08-16
Card: `docs/roadmaps/g15/003-svelte-focused-evidence-forms-inputs-overlays.md`
Governing refs: `docs/roadmaps/g15/001-release-baseline-roster-inventory.md`,
`docs/roadmaps/g15/release-baseline-roster.md`,
`docs/roadmaps/g15/release-gap-register.md`,
`docs/contracts/001-working-rules.md`

## Batches

The card's three named batches were executed in order, each with a narrow test
round at the end. The roster and register evidence rows were updated once all
three batches were green.

- **Batch A — form primitives (10):** EditableLabel, Field, FieldSet,
  FormActions, IconButton, Meter, NumberInput, PasswordRequirements, Radio,
  RadioGroup
- **Batch B — value inputs & date/time pickers (8):** AudioPlayer,
  ColorPicker, Calendar, DatePicker, DateRangePicker, DateTimePicker,
  DateTimeRangePicker, DurationInput
- **Batch C — range, segmentation & shell chrome (8):** ResizeHandle,
  RangeSlider, SegmentedControl, ScrollShell, Separator, Slider, Pagination,
  PaginationSummary

## Evidence Landed

Every scoped component now has a named focused test case on the Svelte side
(`packages/svelte/components/test/<Name>.test.ts`) and the mirrored contract
cases on the React side (`packages/react/components/test/<Name>.test.tsx`),
asserting load-bearing observable contract behaviour: value flow and commit
semantics, validation state, focus/keyboard operation, overlay
dismissal/placement, or composed-token output. The anatomy smoke
(`smoke.test.ts`) is not reused as evidence; each file asserts behaviour
beyond mounting.

Svelte file / React file per component (also recorded in the roster):

| Component | Svelte evidence | React evidence |
| --- | --- | --- |
| EditableLabel | `EditableLabel.test.ts` | `EditableLabel.test.tsx` |
| Field | `Field.test.ts` | `Field.test.tsx` |
| FieldSet | `FieldSet.test.ts` | `FieldSet.test.tsx` |
| FormActions | `FormActions.test.ts` | `FormActions.test.tsx` |
| IconButton | `IconButton.test.ts` | `IconButton.test.tsx` |
| Meter | `Meter.test.ts` | `Meter.test.tsx` |
| NumberInput | `NumberInput.test.ts` | `NumberInput.test.tsx` |
| PasswordRequirements | `PasswordRequirements.test.ts` | `PasswordRequirements.test.tsx` |
| Radio | `Radio.test.ts` | `Radio.test.tsx` |
| RadioGroup | `RadioGroup.test.ts` | `RadioGroup.test.tsx` |
| AudioPlayer | `AudioPlayer.test.ts` | `AudioPlayer.test.tsx` |
| ColorPicker | `ColorPicker.test.ts` | `ColorPicker.test.tsx` |
| Calendar | `Calendar.test.ts` | `Calendar.test.tsx` |
| DatePicker | `DatePicker.test.ts` | `DatePicker.test.tsx` |
| DateRangePicker | `DateRangePicker.test.ts` | `DateRangePicker.test.tsx` |
| DateTimePicker | `DateTimePicker.test.ts` | `DateTimePicker.test.tsx` |
| DateTimeRangePicker | `DateTimeRangePicker.test.ts` | `DateTimeRangePicker.test.tsx` |
| DurationInput | `DurationInput.test.ts` | `DurationInput.test.tsx` |
| ResizeHandle | `ResizeHandle.test.ts` | `ResizeHandle.test.tsx` |
| RangeSlider | `RangeSlider.test.ts` | `RangeSlider.test.tsx` |
| SegmentedControl | `SegmentedControl.test.ts` | `SegmentedControl.test.tsx` |
| ScrollShell | `ScrollShell.test.ts` | `ScrollShell.test.tsx` |
| Separator | `Separator.test.ts` | `Separator.test.tsx` |
| Slider | `Slider.test.ts` | `Slider.test.tsx` |
| Pagination | `Pagination.test.ts` | `Pagination.test.tsx` |
| PaginationSummary | `PaginationSummary.test.ts` | `PaginationSummary.test.tsx` |

Supporting harnesses: `FieldHarness.svelte`, `FieldSetHarness.svelte`,
`FormActionsHarness.svelte`, and `ScrollShellHarness.svelte` (compiled
snippet content for child-bearing primitives; raw thunks cannot materialize
text under the Svelte 5 runtime).

Representative load-bearing cases per family (full list lives in the files):

- **Batch A**: EditableLabel double-click entry seeded with the value,
  Escape cancel restoring the original, Enter/blur commit with
  `{ value, previousValue }`; Field required/optional flagging and message
  projection; FieldSet column grid + space-scale gaps and numeric/full span;
  FormActions alignment flags and overflow-menu routing to the matching item
  callback; IconButton required accessible name, pressed toggle reporting
  with `aria-pressed` only when configured, loading spinner swap with
  activation gating, hover-delay tooltip and Escape dismissal; Meter
  percentage fill + native meter feed, range clamping, high/low level flags,
  ring shape via `--poodle-meter-percentage`; NumberInput parsed commit and
  null-on-empty, clamp-and-snap on blur, arrow stepping with increment/
  decrement reporting, readOnly suppression; PasswordRequirements checklist
  projection from policy config, per-item met flags, live region; Radio
  checked reporting, readOnly/disabled reversion, controlled-state
  reflection; RadioGroup controlled/uncontrolled value flow, shared
  auto-generated name, radiogroup semantics.
- **Batch B**: AudioPlayer play/pause transport with audio element sync,
  seek and volume flow, mute flag, optional speed selector applying the
  rate; ColorPicker open/close dialog semantics, swatch selection with
  normalized hex and `aria-selected`, gradient-pad arrow adjustment,
  RGB-channel and alpha edits, disabled inertness; Calendar single/range
  two-click selection with in-range flags, roving-tabindex arrow keys, nav
  buttons and PageUp/PageDown month paging landing focus on the same day,
  disabled guard; DatePicker/DateRangePicker/DateTimePicker/
  DateTimeRangePicker overlay open/close, value commit and trigger
  formatting, Escape dismissal, outside mousedown, endpoint normalization,
  partial date/time labels, disabled inertness; DurationInput segment carry
  and borrow on arrow keys, per-segment clamping, maxHours bound, invalid
  flag, disabled suppression.
- **Batch C**: ResizeHandle keyboard ±8/±9999 stepping per axis with
  off-axis keys ignored, disabled `tabindex="-1"` inertness, drag
  start/move/end lifecycle reporting the final position; RangeSlider live
  change while dragging with a single commit at release, cross-thumb clamp
  preserving lower ≤ upper, per-thumb labels/values/bounds, fill-window
  custom properties, bounds guard, disabled thumbs; SegmentedControl value
  selection and disabled gating; ScrollShell direction→overflow-axis
  mapping, focusable scroll region with `role="region"`, `onScroll`
  forwarding; Separator decorative vs semantic `role="separator"` +
  `aria-orientation`; Slider step-snap with change/commit split, out-of-range
  clamp, disabled; Pagination boundary-button disabling, page clamping,
  `aria-current` and ellipsis window, single-page auto-hide, limit change;
  PaginationSummary range math with last-page clamp, polite live region.

RangeSlider's web evidence asserts the same contract semantics as the
retained native regression (`a_scrub_reports_change_while_dragging_and_
commits_once_at_release`) without borrowing the native case: `input` reports
live change, `change` fires exactly one commit carrying the final pair.

## Bounded Fixes (contract-first)

- **Calendar PageDown/PageUp day clamping** — both calendars stepped the
  focus day via `addMonths`, which anchors to the 1st of the target month,
  so PageDown from Mar 14 landed focus on Apr 1. The contract's keyboard
  table documents "moves focus to same day next month" (`calendar.md` §6),
  so this was a code-vs-contract defect the new evidence exposed, not a
  contract ambiguity. Added `addMonthsPreservingDay` to
  `@inflatable-cookie/poodle-core` (`date.ts`; the existing `addMonths` stays
  day-1-anchored for the nav buttons, and its conformance vectors are
  untouched), re-exported it through the Svelte date module, and switched
  both calendars' PageUp/PageDown handling to it. The web evidence now
  asserts the contract case: PageDown from Mar 14 lands focus on Apr 14.
  No contract change was required.
- **React `ResizeHandle` end position** — the contract documents `onResizeEnd`
  as "final `clientX`/`clientY` position" (`resize-handle.md` §5). Svelte
  (reference) computes it from the `mouseup` event; React reported the last
  `mousemove` position instead, so a drag ending at 130 after a move to 120
  reported 120. React's `handlePointerUp` now reads the up-event position,
  matching Svelte and the contract; both tests assert 130.

## Contract Ambiguity Resolved (no fix, no contract change)

- **NumberInput stepping baseline under controlled props** — with a
  controlled `value` and no host feedback, Svelte's `$bindable` mutates the
  local value after ArrowUp (so ArrowDown steps from 6 → 5) while React keeps
  the prop (ArrowDown → 4). The contract's `bind:value` usage owns the
  feedback loop, so this is not a runtime defect; the stepping evidence uses
  the uncontrolled `defaultValue` path on both sides, where the runtimes
  behave identically (ArrowUp → 6, ArrowDown → 5).
- **RadioGroup auto-generated name format** — Svelte uses
  `poodle-radio-group-<n>` from a module counter; React uses `useId()`.
  The contract's load-bearing promise is a shared, unique name across the
  group's options when none is provided, which both satisfy; the React
  evidence asserts the shared name, not the Svelte-specific prefix.

## Observations (no change made)

- The React `DurationInput` is fully controlled (props never self-update);
  the Svelte reference uses `$bindable` segments. The React mirror drives
  the same carry/borrow cases through a small stateful host wrapper so the
  contract cases match across runtimes.
- The picker family's portalled surface (`AnchoredSurface`/`anchored`
  action) is not reachable from the render container; evidence queries
  `document` for the surface. The date-range pickers seed their calendar
  month from today, so their evidence pages back to the pinned March 2026
  via the calendar's previous-month button before clicking days.
- `ColorPicker`-style slider anatomy (`role="slider"` with
  `aria-valuetext`) is asserted via the label, not the underlying input,
  matching the contract's gradient-pad semantics.

## Validation

| Command | Result |
| --- | --- |
| Batch A narrow round (touched files) | pass |
| Batch B narrow round (touched files) | pass |
| Batch C narrow round (touched files) | pass |
| `effigy check:svelte` | pass |
| `effigy react:build` | pass |
| `effigy test:components` | pass |
| `effigy docs:check` | pass |
| `git diff --check` | pass |

No `*-windowed` selector, `test:native-visual`, `qa:jetstream`, or Jetstream
selector ran.

## Register and Roster Updates

- `release-baseline-roster.md`: the 26 components' Focused Svelte test cells
  and Focused React test cells now name the case files; summary counts moved
  to Focused Svelte 116 present / 59 missing and Focused React 113 present /
  62 missing.
- `release-gap-register.md`: the Svelte focused-evidence blocker class count
  moved 85 → 59; the "Foundation forms, inputs & overlays" family row (26
  components) is closed with evidence recorded in the roster. No status line
  was changed.
- `docs/roadmaps/g15/003-…` card, `README.md`, and `dispatch.md` were not
  modified by the worker.

## Change Footprint

`packages/svelte/components/test/` (26 new test files + 4 harnesses),
`packages/react/components/test/` (26 new test files; `RangeSlider.test.tsx`
extended), `packages/core/src/date.ts` (`addMonthsPreservingDay`),
`packages/react/components/src/ResizeHandle.tsx` (contract-true end
position), both `Calendar.svelte`/`Calendar.tsx` (PageUp/PageDown day
preservation), and the two focused-evidence docs. No contract, specimen,
package export, workflow, or downstream repository changed.
