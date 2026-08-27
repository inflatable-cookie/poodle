# g16.009 — DurationInput Single Source And Mounted Behaviour

Date: 2026-08-27
Status: complete — PR #83, pending operator merge authority
Branch: `t3code/duration-input-single-source`
Card: `docs/roadmaps/g16/009-duration-input-single-source-and-mounted-behaviour.md`
Source triage: `docs/triage/20260827-094214-post-g16-008-native-lane-decision.md`

## Outcome

DurationInput's public Rust value is the three segment fields. The formatted
`value` string and caller-supplied `validation_state` are gone. Display text,
totals, and the invalid border all derive from hours/minutes/seconds.
`show_seconds` defaults to `true`. Shared totals and the native change callback
use `u64`. One named mounted GPUI regression drives real focus and key dispatch
through a host that rebuilds from those fields only.

The generated ledger moves only DurationInput's GPUI mounted-behaviour cell
from `missing` to `mounted`. Summary: 37 → 38 mounted; 137 → 136 missing. GPUI
accessibility stays `manual`. GPUI visual stays `missing`. Jetstream stays
deferred. No other component's cell moves.

## Removed fields

- `DurationInputSpec.value` and `with_value`
- `DurationInputSpec.validation_state` and `with_validation_state`

No aliases, parsing fallbacks, or dual-source synchronization remain.

## Derived totals and bounds

- `poodle_headless::duration::duration_total_seconds` and
  `DurationInputSpec::total_seconds` are `u64`. A hours value whose product
  overflows `u32` still reports the true total.
- Invalid presentation is `total < min_total_seconds` or, when a maximum
  exists, `total > max_total_seconds`. Inclusive endpoints are valid. Zero is
  valid at the default minimum.
- Bounds do not clamp edits. Segment transitions keep the existing carry,
  borrow, digit-shift, and `max_hours` swallow rules; the host receives the
  actual edited total.

## Callers

Migrated in this branch:

- GPUI preview specimen and `DurationInput` node_compat callback
- GPUI specimen host state (`SpecimenState.durations` / `SetDuration`)
- Jetstream preview specimen (compile against the shared spec only)
- focused renderer tests and the `g16.008` routing fixture

Svelte and React DurationInput tests were not changed. They already agreed on
segments, `showSeconds=true`, carry/borrow, bounds, and disabled behaviour.

## Mounted evidence

`packages/gpui/preview/tests/headless_regressions.rs#duration_input_segments_edit_and_rebuild_the_host_spec`
proves, through production focus/key dispatch and host rebuilds:

- Hours is the entry stop; Tab leaves after the visible segments
- ArrowUp carry, ArrowDown borrow, digit-shift entry
- `max_hours` swallowing and clamping, with no callback on a swallowed hours
  step
- exact `{hours, minutes, seconds, total}` callback values
- `show_seconds=false` keeps Seconds in stored state and totals while exposing
  only Hours and Minutes stops
- disabled DurationInput exposes no segment stops and emits no change

The `g16.008` routing test
`code_and_duration_inputs_traverse_on_tab_without_mutating` stays green and
still proves `H → M → S → out` without claiming full DurationInput parity.

## Explicit non-claims

- no native IME, free-form parsing, selection ranges, or new editor
- no Svelte/React public behaviour change
- no NumberInput, TimeInput, EditableLabel, IconButton, or date pickers
- no GPUI accessibility or visual promotion
- no Jetstream admission; in-repo fixtures compile against the shared spec
- no other ledger row

## Validation

Focused `poodle-specs` DurationInput tests (5), `poodle-headless` duration
tests (9) plus `duration_conformance`, `poodle-render` duration_input (9),
Svelte and React DurationInput tests (14), named mounted DurationInput plus
retained routing. `effigy regressions:native` (84),
`effigy probe:gpui-specimens` (8), `effigy test:parity-evidence-ledger` (5),
`effigy check:parity-evidence-ledger` (175 rows), `effigy docs:check`,
`effigy ci:native`, `effigy ci:web`, `effigy qa`, and
`git diff --check origin/main...HEAD`.

`effigy doctor` is already red on the planning base (generated-in-src,
god-files, stale-suppressions). That baseline is unchanged and was not
absorbed.

Jetstream preview cannot compile in this worktree: the sibling
`jetstream-input` path is not present. Adapter tests that construct
`DurationInputSpec` pass. No Jetstream behaviour is claimed.

## Remaining gaps

- NumberInput's value model stays open in
  `docs/triage/20260826-213343-number-input-native-value-model.md`
- multiline, slug, validation timing, IME, native accessibility, visual
  comparison, and Jetstream admission are unchanged and unclaimed
- the next evidence lane is an orchestrator checkpoint against 38 mounted /
  136 missing
