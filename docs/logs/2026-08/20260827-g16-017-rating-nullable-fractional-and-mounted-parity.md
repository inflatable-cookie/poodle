# g16.017 — Rating Nullable, Fractional, And Mounted Parity

Date: 2026-08-27
Status: complete — awaiting merge
Branch: `t3code/review-rating-worker-handoff`
Card: `docs/roadmaps/g16/017-rating-nullable-fractional-and-mounted-parity.md`
Source triage: `docs/triage/20260827-222346-post-g16-016-native-lane-decision.md`

## Outcome

Shared Rust Rating used an incompatible integer-era model: concrete `f64`
values, default `step=1`, legacy `precision` / `is_readonly`, and `u32`
callbacks. The renderer always claimed radiogroup semantics and could not
express fractional input, nullable clearing, roving focus, or keyboard
behavior.

The approved pre-1.0 migration lands nullable `Option<f64>` authored/default
values, default half-step input, `Option<f64>` change payloads, shared pure
math in `poodle-headless`, whole-step RadioGroup behavior, and fractional
Slider behavior through existing node channels and host-owned rebuilds.

The generated ledger moves only Rating's GPUI mounted-behaviour cell:
`missing` → `mounted` (45 → 46 mounted, 129 → 128 missing). Known-delta totals
stay 115 present / 60 not-applicable. GPUI accessibility stays `manual`.
GPUI visual stays `missing`. Jetstream stays deferred.

## Exact API break

- `RatingSpec.value` / `default_value`: `f64` → `Option<f64>` (both default
  `None`; display resolves `value.or(default_value)`)
- default `step`: `1.0` → `0.5`
- removed: `precision`, `is_readonly`, and their builders
- change payload: `Fn(u32)` → `Fn(Option<f64>)` via `RatingHandlers`
- no aliases, shims, integer fallback callbacks, or silent conversion

## Pure-math proof

`packages/contracts/headless/src/rating.rs` mirrors `packages/core/src/rating.ts`
on step resolution, display clamp, input snap, pointer snap-up, clear-on-
reselect, fill ratio, fraction trimming, keyboard stepping, and value text.
Focused Rust vectors match the TypeScript board.

## Mounted evidence

`packages/gpui/preview/tests/headless_regressions.rs#rating_nullable_fractional_and_whole_step_through_mounted_pointer_and_keyboard`
proves, through production hit testing, focus, and key dispatch:

- default half-step pointer input produces a fractional `Option<f64>` value,
  host rebuild, fill output, and slider accessibility value/text
- fractional Arrow keys, Home, End, and clear-on-Space/Enter use the same
  pure transition path; disabled mode emits nothing
- `step=1` Rating exposes one selected radio, one roving tab stop, and
  Arrow/Home/End focus movement without selection
- whole-step Enter/Space and pointer activation report the same selected
  value, clear only when allowed, and host rebuild the control
- empty state stays `None`; arbitrary incoming fractions display without
  quantization; user input remains quantized
- separate Rating instances do not collide in native focus identity

Fixture ids are test targeting aids only. Enter clear uses `on_submit` so a
focused slider root does not clear after star scrub rebuilds mid-gesture.

## Explicit non-claims

- no Svelte/React public API or implementation change
- no new generic Node/GPUI backend vocabulary
- no broad native accessibility or visual-comparison claim
- no Select, NumberInput, EditableLabel, or sibling-family work
- no Jetstream admission, preview/QA, release, version, workflow, or
  downstream change (mechanical compile migration only)

## Validation

Ran in the worker worktree
(`/Users/tom/.t3/worktrees/poodle/t3code-d24e430e`, branch
`t3code/review-rating-worker-handoff`):

- focused `poodle-headless` Rating tests (6)
- focused `poodle-render` Rating tests (7)
- focused TypeScript core Rating math tests
- focused Svelte/React component board via `effigy test:components` (3117)
- named mounted Rating regression
- `effigy regressions:native` (92/92)
- `effigy probe:gpui-specimens`
- `effigy drift:handlers`, `effigy drift:events`
- `effigy docs:spec-drift`, `effigy docs:contract-drift`
- `effigy test:parity-evidence-ledger` and `effigy check:parity-evidence-ledger`
- `effigy ci:rust`, `effigy ci:native`, `effigy ci:web`
- `effigy docs:check`
- `effigy qa`
- `git diff --check origin/main...HEAD`

Not run / blocked:

- `effigy drift:roles` and Jetstream preview — deferred Jetstream sibling
  absent (`PAPERCUTS.md`)

`effigy doctor` baseline (generated-in-src, god-files, stale-suppressions)
unchanged. Northstar rust-quality activation is not installed in this
repository and was not absorbed.

## Remaining gaps

- native accessibility, visual comparison, and Jetstream admission unchanged
- generation returns to an orchestrator checkpoint at 46 mounted / 128 missing
