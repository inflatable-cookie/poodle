# 050 Narrow The IR To Vocabulary

Status: ready
Milestone: `g13.017` (**this card closes `g13.017`**)
Owner: Poodle core
Branch: `thread/g13-050-narrow-the-ir-to-vocabulary`
Depends on: `g13.008` (verdict **revise**, recorded 2026-08-13)
Governing refs: `docs/roadmaps/g13/017-narrow-the-ir-to-vocabulary.md`,
`docs/roadmaps/g13/pilot-verdict-evidence.md` (**read §7–8 first — this card
executes the verdict**), `docs/specs/063-rust-authored-component-and-scene-ir.md`
(the amended scope section is the authority for what stays)

## Goal

The verdict keeps the IR as **one source for cross-runtime vocabulary with
drift gating** and drops the behavioural ambition. This card removes what only
served the dropped half, and makes the boundary hard enough that it cannot
creep back.

This is **subtraction**. Nothing gains a feature.

## Current State — Measured

### Expressions are authored heavily and consumed by nothing

| Location | `Expr` references |
|---|---|
| `codegen/src/models/text_input.rs` | 49 |
| `codegen/src/models/range_slider.rs` | 38 |
| `codegen/src/models/button.rs` | 28 |
| **authoring subtotal** | **115** |
| `contracts/ir/src/expr.rs` (the definition) | 32 |
| `contracts/ir/src/validation.rs` | 30 |
| `contracts/ir/tests/expressions.rs` | 90 |
| `contracts/ir/src/{attributes,axes,props,parts,conformance}.rs` | 12 |
| `codegen/src/targets/button.rs` | **2** |

`expr.rs` is 331 lines. The expression fields are `when` (part render
conditions), `guard`, `value` and `fallback`.

**No component emitter evaluates an expression.** Measured:

- The two references in `targets/button.rs` are the `use` line and a single
  `match` that unwraps `Expr::Operand(ExprOperand::Visual(field_id))` to
  recover a plain field identifier. That is unwrapping, not evaluating.
- The only readers of `.guard` / `.fallback` are `targets/json.rs` and
  `targets/conformance.rs`, which **serialize** them into dump artifacts.
- The generated artifacts contain **zero** occurrences of a condition:
  `render/src/generated/text-input/index.rs` and the Svelte equivalent both
  return 0.

So expressions are written 115 times, validated, covered by 90 test
references, serialized into a JSON dump, and reach no runtime.

### Note for the worker

`b012` built the expression vocabulary and it was merged as a `g13.002`
deliverable. Deleting merged work is the intended outcome here, not a mistake
to hesitate over — the pilot's job was to find out, and it did.

## Fixed By Ruling (do not re-decide)

### R1 — Classify all 115 before deleting any.

The `037`/`047` pattern, which has paid off every time it has been used. For
each authored expression, decide which it is:

1. **Dead weight** — encodes nothing a runtime uses. Delete.
2. **Real vocabulary wearing an expression's clothes** — e.g. "this part
   renders only when there is a leading icon" is genuine anatomy information
   that happens to be written as an expression tree. **Keep the information,
   lose the tree**: a plain declared flag or a documented condition string.
3. **Genuinely behavioural** — describes derivation or transition. Delete, and
   record it, because the verdict says the IR does not carry behaviour.

Record all three counts. **A card that deletes all 115 without classifying has
failed even if every gate is green** — losing real anatomy vocabulary while
executing a verdict that exists to keep vocabulary would be the exact wrong
outcome.

### R2 — Artifacts are byte-identical, or every moved byte is justified.

This is the acceptance line. Run `ir:build` and diff. If a generated artifact
changes, it means an expression *was* reaching output after all — which is a
finding that changes R1's answer, not a diff to accept quietly.

Sibling target outputs (`button-*`, `range-slider-*`, `text-input-*`,
`shell-*`) are byte-compared by their tests. Those tests must pass unedited.

### R3 — Remove the authoring side too, and the validation and tests with it.

`expr.rs` cannot go while 115 references remain. The removal is the whole
chain: model fields, `expr.rs`, its share of `validation.rs`, and
`tests/expressions.rs`. Do not leave a vestigial type behind "in case".

If `json.rs` or `conformance.rs` lose fields, their artifacts change — that is
expected and is an R2 justification, not a violation.

### R4 — Write the boundary so a future card cannot read around it.

Spec 063 already carries the amended scope section. Tighten it with what this
card learns: name the removed constructs explicitly, and state that
re-introducing an expression tree, an evaluator, or a conditional-render
construct requires a new verdict, not a card.

### R5 — Report the subtraction as a number.

Removed LOC against the ≈31,400-line pilot ledger in
`pilot-verdict-evidence.md` §2. Report the new total. If the removal is smaller
than expected, say so plainly — an honest small number is the point.

## Scope

### In scope

- `packages/contracts/ir/src/expr.rs` and its references across the crate.
- `packages/contracts/ir/src/validation.rs` — expression validation only.
- `packages/contracts/ir/tests/expressions.rs`.
- `packages/codegen/src/models/*.rs` — expression authoring only.
- `packages/codegen/src/targets/*.rs` — only where an expression field is read.
- Emitter surface that served only the generative ambition, if measurement
  shows any beyond the above.
- `docs/specs/063-*.md` scope section (R4).

### Out of scope — stop conditions if reached

- Any component file, CSS, contract, or specimen. No runtime changes.
- Adding anything. This card only removes.
- `packages/contracts/headless/**` and the vectors.
- Capability or anatomy amendments — those are `g13.018`.
- Vocabulary coverage — that is `g13.019`.
- Refreshing any baseline.

## Required Tests

- `ir:build` produces byte-identical artifacts, or each change is justified
  under R2.
- `ir:check` still fails on a planted byte (plant, watch, restore).
- The four-runtime propagation proof still runs: one definition change reaches
  all four previews.
- Sibling byte-comparison tests pass unedited.
- `poodle-ir` and `poodle-codegen` still build with no dead-code warnings.

## Worker Rules

- Execute this card exactly. You have no planning or status authority.
- Do not spawn sub-agents. Read sources directly.
- **Read the verdict evidence §7–8 first.** This card executes a decision; do
  not relitigate it.
- R1 before any deletion.
- Deleting merged work is expected here (see Current State).
- Run `effigy ci:web` (includes `test:web-pack-install`) and `ci:rust`.
- Verify every governing-ref path resolves before relying on it.
- Stage only your writable paths by explicit path. Never `git add -A`.
- Commit and push with
  `git push -u origin thread/g13-050-narrow-the-ir-to-vocabulary`. Do not merge.
- `PAPERCUTS.md` is append-only and shared: do not reflow neighbours.

## Writable Paths

- `packages/contracts/ir/src/**`
- `packages/contracts/ir/tests/**`
- `packages/codegen/src/**`
- `packages/codegen/tests/**`
- `packages/codegen/fixtures/**` (only as `ir:build` rewrites them)
- `packages/{svelte,react}/components/src/generated/**` (only as `ir:build`
  rewrites them)
- `packages/render/src/generated/**` (only as `ir:build` rewrites them)
- `docs/specs/063-rust-authored-component-and-scene-ir.md`
- `docs/roadmaps/g13/017-narrow-the-ir-to-vocabulary.md` (status only)
- `docs/logs/2026-08/<DD>-g13-050-narrow-the-ir-to-vocabulary.md`
- `PAPERCUTS.md`

## Steps

1. Baseline: `effigy ir:build`, `ir:check`, `ci:rust`, `ci:web`,
   `git diff --check`. All green. Record artifact checksums.
2. Read the verdict evidence §7–8.
3. Classify all 115 authored expressions into R1's three buckets. Record the
   evidence and the counts.
4. Preserve every bucket-2 item as plain vocabulary before deleting its tree.
5. Remove the chain: model fields, `expr.rs`, its validation, its tests.
6. `ir:build`; diff artifacts against the recorded checksums; justify every
   moved byte or stop.
7. Prove `ir:check` still bites, and that propagation still reaches all four.
8. Tighten the spec 063 boundary (R4).
9. Report removed LOC and the new machinery total (R5).
10. Validate:
    ```sh
    effigy ir:build
    effigy ir:check
    effigy ci:rust
    effigy ci:web
    effigy test:core
    effigy test:components
    effigy test:parity
    effigy check:svelte
    effigy docs:lint
    git diff --check
    ```

## Acceptance Criteria

- [ ] All 115 authored expressions classified into R1's three buckets, with
  counts.
- [ ] Every bucket-2 item survives as plain vocabulary.
- [ ] `expr.rs`, its validation and its tests are gone; no vestigial type.
- [ ] Artifacts byte-identical, or every moved byte justified.
- [ ] `ir:check` still fails on a planted byte; propagation still reaches four.
- [ ] Spec 063 names the removed constructs and the new-verdict bar (R4).
- [ ] Removed LOC reported against the ≈31,400 ledger.
- [ ] `g13.017` marked complete; all step-10 commands exit 0.

## Stop Conditions

- A generated artifact changes in a way that shows an expression *was* reaching
  a runtime.
- Removing an expression would lose anatomy vocabulary that cannot be expressed
  without a tree.
- Any runtime's output changes.

Stop with exact paths, commands, and the smallest unresolved question.
