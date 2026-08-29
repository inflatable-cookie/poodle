# g16.029 — TimeInput Semantic Model And Native Parity

Status: complete — awaiting merge in PR #97
Date: 2026-08-29
PR: https://github.com/inflatable-cookie/poodle/pull/97
Branch: `t3code/time-input-native-parity`
Worktree: `/Users/tom/.t3/worktrees/poodle/t3code-7a753199`
Card: `docs/roadmaps/g16/029-time-input-semantic-model-and-native-parity.md`
Handoff: `docs/handoffs/20260829-000238-g16-029-time-input-native-parity.md`
Decision: `docs/triage/20260828-224148-time-input-native-editing-decision.md`

## Outcome

TimeInput now has one paired TypeScript/Rust entry model, the same native-input
commit boundary on Svelte and React, a clean pre-1.0 Rust rename, and a
segmented GPUI editor routed through that model.

Public committed values stay canonical `HH:MM` / `HH:MM:SS` or `null`. Partial
and invalid drafts stay adapter-owned. The generated ledger moves only
TimeInput's GPUI mounted-behaviour cell: 47 → 48 mounted, 127 → 126 missing.
Accessibility and GPUI visual-comparison cells do not move. Known-delta totals
stay 115 present / 60 not-applicable.

## Paired public API

`@inflatable-cookie/poodle-core` — `packages/core/src/time-input.ts`,
re-exported from the package root. Rust mirror:
`poodle_headless::time_input`.

| TypeScript | Rust |
| --- | --- |
| `TimeParts` | `TimeParts` |
| `TimeSegment` | `TimeSegment` |
| `TimeInputDraft` | `TimeInputDraft` |
| `TimeInputContext` | `TimeInputContext` |
| `TimeInputEvent` | `TimeInputEvent` |
| `TimeInputEffect` | `TimeInputEffect` |
| `TimeInputResult` | `(TimeInputContext, Vec<TimeInputEffect>)` |
| `parseTime` / `formatTime` | `parse_time` / `format_time` |
| `timeSecondsVisible` | `time_seconds_visible` |
| `timeInBounds` / `timeStepAligned` / `timeConstraintValid` | `time_in_bounds` / `time_step_aligned` / `time_constraint_valid` |
| `stepTimeSeconds` | `step_time_seconds` |
| `timeInputTransition` | `time_input_transition` |
| `timeInputInvalid` | `time_input_invalid` |

The machine emits `emitValueChange` only when `onValueChange` should fire. It
owns no focus, drawing, locale, or I/O.

## Shared vectors

`packages/contracts/headless/vectors/domain.json` `timeInput`: 73 cases covering
parse/reject, format, seconds visibility, bounds, step alignment, wrap/clamp/
overnight stepping, digit/commit/clear/replace/blur/Escape/disabled
transitions. Both domain-conformance runners execute the same file.

## Web

Svelte and React keep native `input[type=time]`. `COMMIT_TEXT` gates
`onValueChange`. Off-step and out-of-range native values stay local, set
`aria-invalid`, and revert on blur or Escape. Controlled replacement discards
the draft. Browser picker UI is unchanged.

## Clean rename

`TimeFieldSpec` / `time_field` → `TimeInputSpec` / `time_input` in specs,
renderer, GPUI/Jetstream adapters, specimens, census, and the ledger. No alias.

## Mounted GPUI

`poodle_render::time_input` is one labelled group of Hour/Minute/conditional
Second `SpinButton` segments. Arrow keys step the whole time; digits edit the
focused segment; Backspace/Delete clear a segment; Escape reverts.

`time_input_segmented_editor_commits_drafts_and_bounds` proves live commit,
invalid no-emit, Escape and leaving-control blur revert, whole-control clear,
step, linear stop, overnight wrap, conditional seconds, replacement, Tab
traversal, and disabled inertia.

## Ledger

Only TimeInput GPUI mounted behaviour: missing → mounted. Totals 48 / 126.
GPUI accessibility stays manual. GPUI visual stays missing.

## Review fixes (PR #97)

Four contract failures from the orchestrator review, still on this PR:

1. Stepping no longer clamps to a raw off-grid `min`/`max`. Last on-grid in
   range is kept; empty step-down lands on that last on-grid value. Shared
   vectors cover the linear and overnight cases.
2. Web adapters distinguish native incomplete (`value === ""` +
   `validity.badInput`) from a deliberate clear. Incomplete stays local,
   marks invalid, and reverts; clear still emits `null`. Native
   `input[type=time]` is kept.
3. `time_input_with_change` / `TimeInput::from_spec` own a persistent
   context cell. Specimens remount on context changes so a partial digit
   keeps its draft and invalid border. The mounted regression drives
   `time_input_with_persistent_context`.
4. Active spec 052 and `docs/parity/time-input.md` now use `TimeInputSpec` /
   `time-input` / `time_input.rs` only.

## Explicit non-claims

- locale / 12-hour presentation
- timezone or date ownership
- picker overlays
- raw draft callbacks
- IME
- Jetstream admission
- GPUI assistive-technology proof beyond SpinButton/group projection
- GPUI visual comparison
- NumberInput, EditableLabel, DurationInput behavior, drag-and-drop
