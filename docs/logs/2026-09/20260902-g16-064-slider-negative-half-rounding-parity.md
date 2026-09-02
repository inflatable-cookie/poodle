# g16.064 — Slider Negative-half Rounding Parity

Status: implementation-complete — PR pending orchestrator review
Date: 2026-09-02
PR: pending — worker pushes the branch; orchestrator owns merge
Card: `docs/roadmaps/g16/064-slider-negative-half-rounding-parity.md`
Handoff: `docs/handoffs/20260902-225241-g16-064-slider-rounding.md`
Governing refs: `docs/contracts/components/slider.md`,
`docs/contracts/components/range-slider.md`,
`docs/roadmaps/g16/nucleus-gpui-parity-programme.md`
Branch: `fix/g16-064-slider-rounding`
Worktree: `/Users/tom/.paseo/worktrees/1ugbsx1t/g16-064-slider-rounding`
Planning base: `4ffa31345bc94f82c22d64d83e64b3af2613cfe3`
Promotion base (HEAD at dispatch): `a6a9d242a4473f2436e148b011c910f299ca6f36`

## Outcome

One portable step-quantization law now drives Slider and RangeSlider in both
runtimes: the raw index `(raw - min) / step` quantizes with half ties rounding
**toward positive infinity** — JavaScript `Math.round` semantics. TypeScript
kept `Math.round` and now states the law at the machine; `poodle-headless`
replaced `f64::round` (half away from zero) in `snap_to_step` with the same
`(index + 0.5).floor()` law `color.rs` already uses for JS-identical rounding.

The shared corpus gained the `slider`, `rangeSlider`, and `sliderSnap`
sections covering negative half, positive half, min-offset, non-zero min,
step larger than range, and safe-max cases. `sliderSnap` drives the exported
snap primitive directly so the tie law stays pinned even where the `min`
clamp masks drift in transition outputs.

## Falsification

The exact negative-half counterexample bit before the repair. The new shared
corpus ran on the pre-repair machines:

| Runtime | Result |
| --- | --- |
| TypeScript core conformance (206 tests incl. 11 `sliderSnap`) | all pass — `snapToStep(-1, 0, 2)` = 0 |
| Rust conformance (`slider_snap_conformance`) | FAILED — `snap_to_step(-0.5, 0, 1)` returned `-1.0`, corpus pins `0.0` |

Transition-level outputs were already equal on both runtimes: the lower clamp
pulls every below-min snap to `min`, so the drift was only observable on the
shared exported snap surface and any unclamped consumer. The corpus therefore
pins the law at the snap level as well as through INPUT/COMMIT effects.

Repair oracle proofs (corpus green on both runtimes after the fix):

| Invariant | Counterexample | Proof |
| --- | --- | --- |
| Tie law is portable | raw resolves to step index `-0.5` | `sliderSnap` 5 negative-half cases; Rust was red at `-1.0` vs pinned `0.0`, now identical |
| Offset matters | non-zero minimum with half step | `slider`, `rangeSlider`, `sliderSnap` min-offset cases use `(raw - min) / step` (min 10, step 10, raw 5 → 10; a raw/step law would yield 20) |
| Range stays paired | fix only single-thumb Slider | 8 `rangeSlider` cases drive both thumbs through the shared snap; single-thumb-only edits cannot satisfy them |
| Max remains safe | last step exceeds max | `last step exceeding max clamps to max`, `step larger than range clamps to max`, `oversized step upper thumb stays within range` — clamped values stay in `[min, safeMax]` |

## Validation

Focused (post-repair):

- `bun test packages/core/test/wave1.test.ts` + `conformance.test.ts` — 232 pass
- `cargo test -p poodle-headless --lib slider` — 10 pass (new negative-half tie test included)
- `cargo test --test conformance` — 16 pass (was 15 pass / 1 fail pre-repair)

Required boards: `effigy ci:web`, `effigy ci:rust`, `effigy docs:check`, and
`git diff --check origin/main...HEAD` — recorded in the worker report with the
exact head after the branch push.

## Limits

- No block appearance, gestures, axis, visible-text, or audio-law change.
- `audio.rs` and unrelated rounding helpers (rating, motion policy, color)
  are untouched; only `poodle-headless::slider::snap_to_step` changed law.
- No adapter, shell, GPUI, Jetstream, release, workflow, or ledger movement.
- Merge and card status remain orchestrator-owned; no windowed or release
  selector was run.
