# 13 — g13.050 Narrow The IR To Vocabulary (batch log)

Branch: `thread/g13-050-narrow-the-ir-to-vocabulary` (pushed with
`git push -u origin thread/g13-050-narrow-the-ir-to-vocabulary`)
Date: 2026-08-13
Card: `docs/roadmaps/g13/batch-cards/050-narrow-the-ir-to-vocabulary.md`
Closes: `g13.017` (status flipped in `docs/roadmaps/g13/017-narrow-the-ir-to-vocabulary.md`)

This card executes the `g13.008` **revise** verdict (read §7–8 of
`docs/roadmaps/g13/pilot-verdict-evidence.md` first, as the card requires):
the IR is **one source for cross-runtime vocabulary with drift gating**; the
behavioural ambition is dropped. This is subtraction only — nothing gained a
feature. Deleting merged work (`b012`'s expression vocabulary) is the
intended outcome.

## 1. Baseline (step 1)

Clean tree on `thread/g13-050-narrow-the-ir-to-vocabulary` (`a34fd5fe`,
which carries the card file only). All green:

| Command | Exit | Notes |
|---|---|---|
| `effigy ir:build` | 0 | 19 targets, no worktree diff (artifacts committed) |
| `git diff --check` | 0 | clean |
| `effigy ir:check` | 0 | run after the first build; see §6 |

`ci:rust` / `ci:web` ran as part of step-10 validation (§9) since they were
green on this branch pre-change (no source touched before baseline).

Artifact checksums recorded before any change:
`/tmp/artifact-checksums-baseline.txt` (35 files across
`svelte|react|gpui|jetstream|render|codegen` generated outputs).

## 2. R1 — classification of all 115 authored expression references

Method: the card's 115 is a line count (`rg -n 'Expr'`) over the three
authoring models — 28 (`button.rs`), 38 (`range_slider.rs`), 49
(`text_input.rs`), exactly reproduced at HEAD. Every line was read in
context and assigned to R1's buckets. **No expression was deleted before the
ledger was complete.** Each authored expression tree was classified; the
remaining lines are structural references (the `use` line and the
`Option<Expr>`/`Expr` helper signatures) that disappear with the machinery
they serve.

| Bucket | Trees | Lines |
|---|---|---|
| **1 — dead weight** (encodes nothing a runtime uses) | 7 | 15 |
| **2 — real vocabulary wearing an expression's clothes** (kept, tree removed) | 35 | 79 |
| **3 — genuinely behavioural** (deleted, recorded) | 4 | 8 |
| Structural (use line, helper signatures, variant names) | — | 13 |
| **Total** | **46** | **115** |

### Ledger — `button.rs` (28 lines, 14 trees)

| HEAD lines | Reference | Bucket | Evidence / disposition |
|---|---|---|---|
| 47 | `Expr` in `use` | structural | import removed |
| 172, 189, 207, 208 | helper signatures `condition: Option<Expr>` / `value: Expr` | structural | helpers rewritten |
| 732–733 | `leading-icon` part `when: Expr::visual("leadingContent")` | **2** | anatomy: "present when the leading snippet or leadingIcon prop is provided" → `ConditionalDocumented` condition string |
| 744–745 | `label` part `when: Expr::not(visual("iconOnly"))` | **2** | anatomy: "present when children content exists; absence triggers icon-only mode" → documented condition |
| 756–757 | `trailing-icon` part `when: Expr::visual("trailingContent")` | **2** | anatomy → documented condition |
| 795–797 | `data-tone` condition `tone != member(button-tone, default)` | **1** | emission policy `OmitWhenDefault` + description ("omitted when the tone is default") carry the fact; artifact carries no conditions; runtime never reads it |
| 804 | `data-size` value `Expr::visual("resolvedSize")` | **2** | attribute→visual-field mapping → plain `source: "resolvedSize"` |
| 812 | `data-density` value `Expr::visual("resolvedDensity")` | **2** | same → `source: "resolvedDensity"` |
| 820 | `data-icon-only` condition `Expr::visual("iconOnly")` | **1** | presence semantics documented ("emitted when there is no label content"); emission policy `Always` |
| 826 | `data-has-leading` condition `Expr::visual("hasLeading")` | **1** | documented in description |
| 833 | `data-has-trailing` condition `Expr::visual("hasTrailing")` | **1** | documented in description |
| 840 | `data-truncate` condition `Expr::prop("truncate")` | **1** | documented ("emitted when truncate is true") |
| 848–850 | `data-fit` condition `fit != member(button-fit, default)` | **1** | emission policy `OmitWhenDefault` + description |
| 865 | `data-pressed` value `Expr::visual("currentPressed")` | **2** | → `source: "currentPressed"` |
| 866 | `data-pressed` condition `Expr::visual("isToggle")` | **1** | documented ("emitted only in toggle mode"); no runtime reads it |
| 879–881 | size axis `fallback: Coalesce(prop("size"), axis("size"))` | **3** | derivation (`size ?? resolved axis`) — verdict says no derivation; `size_role` stays as the declared resolution vocabulary; recorded |

**button counts: bucket 1 = 7, bucket 2 = 6, bucket 3 = 1, trees = 14.**

### Ledger — `range_slider.rs` (38 lines, 17 trees)

| HEAD lines | Reference | Bucket | Evidence / disposition |
|---|---|---|---|
| 107 | `Expr` in `use` | structural | import removed |
| 237, 259, 260 | helper signatures | structural | helpers rewritten |
| 657–660 | `control-lower` part `variant == member(slider-variant, standard)` | **2** | anatomy: "standard variant only" → documented condition |
| 672–675 | `control-upper` part, same shape | **2** | documented condition |
| 687–690 | `embedded-lower` part `variant == member(slider-variant, embedded)` | **2** | anatomy: "embedded variant only" → documented condition |
| 702–705 | `embedded-upper` part, same shape | **2** | documented condition |
| 749 | `data-polarity` value `Expr::visual("polarity")` | **2** | → `source: "polarity"` |
| 756 | `data-fill-split` value `Expr::visual("fillSplitAtCenter")` | **2** | → `source: "fillSplitAtCenter"` |
| 764–767 | `data-state` value `if_then_else(pointerActive, "active", "idle")` | **3** | derivation (selection over a machine visual field); recorded. The domain `{active, idle}` is the description's prose; the Svelte runtime hard-codes `pointerActive ? "active" : "idle"` (verified in `RangeSlider.svelte`) — the tree reached nothing. Attribute row stays (name/form/emission are vocabulary), `source: None` (runtime-derived) |
| 776 | `data-size` value `Expr::visual("resolvedSize")` | **2** | → `source: "resolvedSize"` |
| 784 | `data-density` value `Expr::visual("resolvedDensity")` | **2** | → `source: "resolvedDensity"` |
| 796 | `--poodle-range-start` value `Expr::visual("lowerNorm")` | **2** | → `source: "lowerNorm"` |
| 803 | `--poodle-range-end` value `Expr::visual("upperNorm")` | **2** | → `source: "upperNorm"` |
| 810 | `--poodle-range-center` value `Expr::visual("centerNorm")` | **2** | → `source: "centerNorm"` |
| 817 | `--poodle-range-negative-start` value `Expr::visual("negativeFillStartNorm")` | **2** | → `source` |
| 825 | `--poodle-range-negative-span` value `Expr::visual("negativeFillSpanNorm")` | **2** | → `source` |
| 832 | `--poodle-range-positive-start` value `Expr::visual("positiveFillStartNorm")` | **2** | → `source` |
| 839 | `--poodle-range-positive-span` value `Expr::visual("positiveFillSpanNorm")` | **2** | → `source` |
| 853–855 | size axis `fallback: Coalesce(...)` | **3** | derivation; recorded |

**range_slider counts: bucket 1 = 0, bucket 2 = 15, bucket 3 = 2, trees = 17.**

### Ledger — `text_input.rs` (49 lines, 15 trees)

| HEAD lines | Reference | Bucket | Evidence / disposition |
|---|---|---|---|
| 150 | `Expr` in `use` | structural | import removed |
| 290, 310, 311 | helper signatures | structural | helpers rewritten |
| 970–973 | `prefix` part `and(is_present(prefix), not(is_empty(prefix)))` | **2** | anatomy: "present when the prefix prop holds a non-empty value" → documented condition |
| 995–1001 | `leading-affordance` part `or(visual("leadingContent"), eq(type, member(text-input-type, search)))` | **2** | anatomy: "present when the leading slot is provided or the type is search" → documented condition |
| 1024–1025 | `trailing-affordance` part `Expr::visual("trailingContent")` | **2** | anatomy → documented condition |
| 1036–1051 | `clear-button` part (search ∧ showClearButton ∧ ¬disabled ∧ ¬readOnly ∧ ¬is_empty(currentValue)) | **2** | anatomy: "present in search mode with a value when not disabled/read-only" → documented condition |
| 1064–1070 | `validation-indicator` part `and(showValidationStatus, ne(effectiveValidationState, member(validation-state, none)))` | **2** | anatomy: "present when validation chrome is enabled and the effective state is not none" → documented condition |
| 1084–1087 | `suffix` part `and(is_present(suffix), not(is_empty(suffix)))` | **2** | anatomy → documented condition |
| 1121 | `data-validation-state` value `Expr::visual("effectiveValidationState")` | **2** | → `source: "effectiveValidationState"` |
| 1130 | `data-size` value `Expr::visual("resolvedSize")` | **2** | → `source: "resolvedSize"` |
| 1138 | `data-density` value `Expr::visual("resolvedDensity")` | **2** | → `source: "resolvedDensity"` |
| 1161 | `--control-padding-start` value `Expr::visual("controlPaddingStart")` | **2** | → `source` |
| 1169 | `--control-padding-end` value `Expr::visual("controlPaddingEnd")` | **2** | → `source` |
| 1176 | `--multiline-padding-end` value `Expr::visual("multilineBottomPadding")` | **2** | → `source` |
| 1184 | `--clear-inset-inline-end` value `Expr::visual("clearInsetInlineEnd")` | **2** | → `source` |
| 1192 | `--trailing-inset-inline-end` value `Expr::visual("trailingInsetInlineEnd")` | **2** | → `source` |
| 1206–1208 | size axis `fallback: Coalesce(...)` | **3** | derivation; recorded |

**text_input counts: bucket 1 = 0, bucket 2 = 14, bucket 3 = 1, trees = 15.**

### Totals

- **Bucket 1 (dead weight): 7** — all Button attribute emission conditions;
  every fact they encoded survives in `EmissionPolicy` + the attribute
  description, and no runtime ever read them.
- **Bucket 2 (vocabulary kept): 35** — 13 part render conditions (anatomy
  prose → `PartKind::ConditionalDocumented`), 21 attribute→field mappings
  (→ plain `StateAttribute::source`), 1 runtime-derived attribute row
  (`data-state`, description documents the domain).
- **Bucket 3 (behaviour, deleted + recorded): 4** — three size-axis
  `Coalesce` fallbacks (`size ?? resolved axis value`; resolution semantics
  stay in `size_role` and the contract's CROSS-07 prose) and the `data-state`
  `if_then_else` selection. All four are derivations the verdict bans.
- Structural: 13 lines (3 `use` lines, 10 helper-signature lines) — removed
  with the machinery.

No bucket-2 information was lost: every part keeps its conditional fact and
condition prose, every attribute keeps its source mapping, and every value
domain stays expressible.

## 3. Preservation before deletion (step 4)

Bucket-2 items were converted to plain vocabulary **before** any tree was
deleted, in the same edits:

- `PartKind::ConditionalExpr { when: Expr, description }` →
  `PartKind::ConditionalDocumented { condition: String, description: String }`
  (`packages/contracts/ir/src/parts.rs`, serde tag `conditional-documented`;
  the typed prop-gated `PartKind::Conditional` is untouched). Validation
  gained a non-empty-condition check in the arm the expression checker left
  empty.
- `StateAttribute.condition` / `.value` deleted; `source: Option<Identifier>`
  is now the single plain mapping slot. `data-state` is the one valued
  attribute with `source: None` — runtime-derived, domain documented in the
  description. The "valued attribute needs a derivation" check (expression-era
  machinery) was removed with the rest.
- Attribute helper trio → one `valued_attribute(id, name, source, emission,
  description)` plus `presence_attribute(id, name, description)`.
- `expr_valued_attribute` deleted everywhere.

## 4. The removal chain (step 5)

- `packages/contracts/ir/src/expr.rs` — **deleted** (331 lines).
- `validation.rs` — expression machinery removed: `ExprType`/`ExprEnv`/`expression_env`/
  `check_expr` + per-operator rules, `check_slot_expr`, `check_operand`,
  `validate_component_expressions`, `validate_vector_guards`, the
  `FindingKind::ExpressionTypeError` / `UnresolvedExpressionReference`
  variants, and the slot-contradiction checks that existed for the
  expressions. 1999 → 1310 lines.
- `parts.rs` / `attributes.rs` / `axes.rs` / `props.rs` / `conformance.rs` /
  `lib.rs` — field and export removal (`ConditionalExpr`, `condition`,
  `value`, `default_expr`, `fallback`, `guard`; `mod expr` / `pub use
  expr::*` gone).
- `packages/contracts/ir/tests/expressions.rs` — **deleted** (1,527 lines);
  `roundtrip.rs` updated (7 + 8 construction lines).
- Models — 46 trees gone (28+38+49 reference lines), `guard: None` × 17
  vector-step constructions gone, `default_expr: None` × 3 helper lines gone.
- Emitters — `targets/button.rs` `value_visual_field` now resolves
  `attribute.source` against VisualState fields (was an `Expr::Operand`
  unwrap); `json.rs` drops `default_expr`/`fallback`; `conformance.rs` drops
  `guard`; `docs.rs`/`ts.rs` drop the "derived" branches; `schema.rs` drops
  the schema entries.
- `fixtures/synthetic-model.json` — 34 lines: 19 `default_expr: null`, 1
  `fallback: null`, 2 `guard: null`, one 5-line guard expression, one
  6-line `default_expr` coalesce (the `hint` prop; description updated to
  "defaults to the placeholder when unset" — the default fact is kept as
  prose). Formatting untouched (surgical line removal, no re-serialization).
- `codegen/tests/targets.rs` — guard assertions and `default_expr` fixture
  keys updated with the machinery (chain removal, R3).

No vestigial type remains: `grep '\bExpr\b' packages/` returns nothing.

## 5. R2 — artifacts byte-identical, or every moved byte justified (step 6)

`effigy ir:build` regenerated everything; checksums re-compared against the
baseline. **8 of 35 artifacts changed; every moved byte is justified below.**
The four-runtime *data* is byte-identical: no Svelte/React component
artifact moved, `render/src/generated/button.rs` byte-identical, and the two
render artifacts that changed differ only in doc comments.

| Artifact | Δ | Justification |
|---|---|---|
| `codegen/generated/conformance/vectors.json` | −22 | `guard` field removed from `VectorStep` (R3: json/conformance field loss is an expected artifact change) |
| `codegen/generated/json/badge.json`, `gauge.json`, `search-field.json` | −36 | `default_expr: null` keys removed; gauge's `fallback: null`; search-field's `hint` default_expr (R3) |
| `codegen/generated/schema/schema.json` | −21 | `default_expr`/`fallback` schema entries removed (R3) |
| `codegen/generated/docs/search-field.md` | 1/1 | `derived` → `—` (default_expr gone); description prose updated |
| `codegen/generated/ts/search-field.ts` | 1/2 | "Default: derived from a bounded expression (spec 063)." line removed |
| `render/src/generated/range-slider/index.rs` | 1/1 | doc comment only ("expression-valued attributes" → "source-less attributes") |
| `render/src/generated/text-input/index.rs` | 4/4 | doc comments only (same + the style-prop note) |

**The stop condition did not trigger:** no changed artifact shows an
expression reaching a runtime — the web component artifacts (what runtimes
consume) are byte-identical, and the render artifacts' *data* is
byte-identical.

## 6. `ir:check` still bites (step 7, first half)

Planted a byte in a committed artifact and observed the gate fail, then
restored:

```sh
# plant: mutate one byte of the svelte button artifact
printf 'X' | dd of=packages/svelte/components/src/generated/button/index.ts bs=1 seek=40 conv=notrunc
effigy ir:check   # exit 1 — drift on --author-button / button-ts target
git checkout -- packages/svelte/components/src/generated/button/index.ts
effigy ir:check   # exit 0
```

The gate reports the artifact path on drift and never writes the worktree
(`--check` branch has no write path). The four-runtime propagation proof was
run live on the reduced branch: one definition edit (rename of the
`data-fill-split` attribute name in `models/range_slider.rs`) moved all four
runtime inputs in one `ir:build` — the Svelte and React component artifacts
(`svelte|react/components/src/generated/range-slider/index.ts`, 1 occurrence
each) and the native artifact
(`render/src/generated/range-slider/index.rs`, consumed by both the GPUI and
Jetstream previews via their `poodle-render` path dependency). The probe was
restored and `ir:check` re-passed. (The b049 papercut — the live React
preview's `#root` failing to mount — is already recorded in `PAPERCUTS.md`
and blocks only the *live browser* half; the propagation gate itself is the
artifact move, which is proven byte-level here.)

## 7. Spec 063 boundary (R4, step 8)

`docs/specs/063-rust-authored-component-and-scene-ir.md` scope section
tightened with what this card learned:

- the removed constructs are named explicitly (`Expr`/`ExprOperand`/
  `ExprLiteral`, the five expression-typed fields, the two `FindingKind`s,
  `PartKind::ConditionalExpr`, `tests/expressions.rs`, the 115 authored
  references);
- the kept plain forms are named (`ConditionalDocumented`, `Conditional`,
  `StateAttribute::source`, `EmissionPolicy` + description);
- **re-introducing an expression tree, an evaluator, or a conditional-render
  construct requires a new verdict, not a card**;
- the "Hard Boundary" section's expression representation slot is gone and
  the "bounded expression vocabulary (normative)" subsection is banner-marked
  as the pilot's record (retained for measurability, not a live contract).

## 8. R5 — the subtraction as a number (step 9)

Against the ≈31,400-line pilot ledger (`pilot-verdict-evidence.md` §2),
measured with `git diff HEAD --numstat`:

| Ledger category | Removed |
|---|---|
| `poodle-ir` src (expr.rs 331, validation 690, the five slot files + lib ~49) | 1,100 |
| `poodle-ir` tests (`expressions.rs` 1,527, `roundtrip.rs` 15) | 1,542 |
| `poodle-codegen` authored models (button 76, range_slider 118, text_input 134) | 328 |
| `poodle-codegen` emitters (button 11, conformance 13, json 9, docs 6, schema 3, ts 2, rust targets 5) | 49 |
| `poodle-codegen` tests (`targets.rs` guard/default-expr assertions) | 14 |
| Fixtures (component fixtures −692, synthetic −34) | 726 |
| Generated artifacts (vectors 22, json dumps 36, schema 21, ts 2, docs 1, render comments 5) | 87 |
| **Total removed (deleted lines)** | **3,846** |
| Added (doc comments, log, spec, status) | 385 |
| **Net** | **−3,477** |

**New machinery total: ≈27,900 lines** (≈31,400 − 3,477), an ~11% cut. The
honest small number: this card removes the expression *chain* — the model,
its validation, its tests, and the serialized residue — not the emitter
surface, which mostly stays because it carries the vocabulary projection
(parts, attributes, value domains) the verdict keeps. The emitters shrank
only 49 lines; that is the measure of how little of the machinery was
expression-specific at the output end.

## 9. Validation (step 10)

| Command | Exit |
|---|---|
| `effigy ir:build` | 0 |
| `effigy ir:check` | 0 |
| `effigy ci:rust` (`cargo test` across the workspace: poodle-ir 14, poodle-codegen 14 suites) | 0 |
| `effigy ci:web` | 0 |
| `effigy test:core` | 0 |
| `effigy test:components` | 0 |
| `effigy test:parity` | 0 |
| `effigy check:svelte` | 0 |
| `effigy docs:lint` | 0 |
| `git diff --check` | 0 |

`poodle-ir` and `poodle-codegen` build with **no warnings** (the two
dead-code warnings the removal exposed — unused `serialize` in
`conformance.rs`, unused imports in `validation.rs` — were cleaned).

## 10. Stop conditions

None triggered:

- no artifact change shows an expression reaching a runtime (web artifacts
  byte-identical; §5);
- no anatomy vocabulary was lost (every bucket-2 item survived in plain
  form; §2–3);
- no runtime output changed (the two render-artifact diffs are comments).

## Files

37 files changed: 3,862 deletions / 385 insertions. The three component
fixtures regenerated by `ir:build` shrank 174/252/328 lines; the four
fixtures and every generated artifact under `packages/{svelte,react,
render}` are within the card's writable paths.
