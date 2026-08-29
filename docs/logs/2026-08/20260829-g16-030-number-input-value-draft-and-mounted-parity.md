# g16.030 — NumberInput Value, Draft, And Mounted Parity

Status: complete — awaiting review/merge in PR #98
Date: 2026-08-29
PR: https://github.com/inflatable-cookie/poodle/pull/98
Branch: `t3code/number-input-parity`
Worktree: `/Users/tom/.t3/worktrees/poodle/t3code-1787d137`
Card: `docs/roadmaps/g16/030-number-input-value-draft-and-mounted-parity.md`
Handoff: `docs/handoffs/20260829-103447-g16-030-number-input-parity.md`
Decision: `docs/triage/20260826-213343-number-input-native-value-model.md`

## Outcome

NumberInput now has one paired TypeScript/Rust numeric editing model, a clean
pre-1.0 web/native public migration, and a mounted GPUI SpinButton editor.

Committed values are `number | null` / `Option<f64>`. Raw drafts stay on an
explicit optional channel. Invalid drafts emit no committed value. Blur and
Escape revert unresolved drafts. Enter and successful steps fire `onCommit`.

The ledger moves only NumberInput's GPUI mounted-behaviour cell: 48 → 49
mounted, 126 → 125 missing. Accessibility and visual-comparison cells do not
move. Known-delta totals stay 116 present / 59 not-applicable.

## Paired public API

`@inflatable-cookie/poodle-core` — `packages/core/src/number-input.ts`.
Rust mirror: `poodle_headless::number_input`.

Shared vectors: `packages/contracts/headless/vectors/domain.json` `numberInput`
(58 cases). Both domain-conformance runners execute the same file.

## Clean removals

- string unions on `value` / `defaultValue` / `min` / `max` / `step`
- value-mode inference and string coercion helpers in Svelte/React
- `onSubmit`, `onIncrement`, `onDecrement`
- concrete-`f64` / infinity-sentinel `NumberInputSpec`
- static native value label plus increment/decrement-only handlers

## Migration table (inspected, not edited)

| Consumer | Pattern | Required follow-up |
| --- | --- | --- |
| Poodle ColorPicker / FilterBuilder | already numeric; tightened to `number \| null` | none in this card |
| Acowtancy | many string-bound numeric form fields | rebind committed `number \| null` and optional `draftValue` |
| Jetstream inspector | engine text bindings | mechanical compile maintenance only until admission |
| Underlay action dialog | schema-driven numeric fields | bind committed number + draft when empty/partial text is needed |

## Mounted GPUI

Ten named headless regressions cover direct edit, invalid no-emit, clear,
blur/Escape, Enter commit, fractional step/precision/bounds/Home/End,
controlled replacement, two-instance identity, disabled/read-only inertia, and
SpinButton accessibility projection.

## Validation

Focused core/domain, Svelte/React NumberInput, poodle-specs/render,
`number_input_mounted*`, and `effigy probe:gpui-specimens` passed during
implementation. Final board: `effigy qa`, contract/callback/value-domain/
capability drift, `effigy check:parity-evidence-ledger`,
`git diff --check origin/main...HEAD`, and removal searches.

## Next

Do not start EditableLabel, continuous audio, or drag cards from this PR.
After operator-authorized merge, the orchestrator chooses from `g16.022` and
the component-continuation runway.
