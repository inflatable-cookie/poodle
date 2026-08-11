---
title: g13 batch 012 — IR bounded expression vocabulary
status: complete
milestone: g13.002
owner: Poodle core
updated: 2026-08-11
tags: [log, g13, IR, poodle-ir, expressions, validation, spec-063]
---

## What this batch did

Executed batch card
`docs/roadmaps/g13/batch-cards/012-ir-expression-vocabulary.md` on branch
`thread/g13-012-ir-expression-vocabulary`: added the bounded expression
vocabulary from spec 063's normative section ("The bounded expression
vocabulary") to `poodle-ir`, exactly as ruled — the 14-operator closed set,
the five operand kinds, the four sanctioned slots, no evaluator, no codegen,
no macros, no new dependencies (`poodle-tokens` and serde only).

The card exists because `g13-b011` found no first-class way to express
derived state (`CROSS-20` `isUnavailable = disabled || loading` had to be
approximated). This card adds `Expr` and type-checks it inside the existing
`validate(&IrModel) -> Vec<Finding>` entry point, so expression errors arrive
with all other findings.

## Deliverables (only the scoped writes)

- `packages/contracts/ir/src/expr.rs` — new module: `Expr` (operators exactly
  the ruled set, serialized under spec names), `ExprOperand` (prop, state,
  VisualState field, slot presence, resolved axis; plus the literal),
  `ExprLiteral` (bool, integer, string, shared-type member), builder
  constructors.
- `packages/contracts/ir/src/validation.rs` — two new `FindingKind` variants
  (`ExpressionTypeError`, `UnresolvedExpressionReference`); the expression
  type checker (`ExprType`, `ExprEnv`, `check_expr` + per-operator rules);
  slot validation in `validate_component_expressions` and
  `validate_vector_guards`; slot contradiction rules (source + value,
  presence-only + value, default + default_expr, valued with neither).
- Expression slots wired into the four sanctioned places, each an additive
  `#[serde(default)]` field (b011-schema documents still deserialize,
  IR-09):
  - `StateAttribute.condition` / `StateAttribute.value` (emission conditions
    and values)
  - `PartKind::ConditionalExpr` (part render conditions)
  - `Prop.default_expr` + `SizeAxis.fallback` (prop default and axis fallback
    resolution)
  - `VectorStep.guard` (guard conditions on transitions/effect-intents)
- `packages/contracts/ir/tests/expressions.rs` — new integration suite: one
  test per conformance-set row, exclusion tests, round-trip/ordering tests,
  negative type-check and slot-contradiction tests (30 tests).
- `packages/contracts/ir/tests/roundtrip.rs` — new fields added to existing
  fixture construction sites (no behavior change).
- `docs/logs/2026-08/12-g13-012-ir-expression-vocabulary.md` — this log.

Nothing outside `packages/contracts/ir/` changed except this log.

## The model, per spec 063

- **Operands** are exactly the spec list: a declared prop (`CROSS-02`), a
  declared state field (`CROSS-04` controlled state; the type is inferred
  from the controlled prop), a VisualState projection field (`CROSS-14`),
  a part's presence (`CROSS-12` slots — the expression form of
  `Boolean(slot)`), a resolved axis value (`CROSS-07/08/11`), or a literal
  boolean / integer / string / shared-type member.
- **Shared-type member literals carry their shared type explicitly**
  (`ExprLiteral::Member { shared_type, member }`), because member names
  collide across shared types (`default` on tone and density, `none` on
  validation state); the pair is type-checked against the other side of the
  comparison.
- **Resolved axis operands** (`Expr::axis`) resolve their type through a
  documented domain mapping (`size` → `control-size`, `density` →
  `control-density`, `orientation` → `orientation` shared types) and require
  the axis to be declared on the component and the shared type to exist in
  the model.
- **Operator typing**: `and`/`or`/`not` boolean; `eq`/`ne` compare a state
  reference or expression against a literal or shared-type member (both
  sides same type); `is_null`/`is_present` apply to references only (a
  computed expression is never null); `coalesce` requires same-typed
  operands; `gt/gte/lt/lte` integers only; `is_empty` strings and
  collections; `if` boolean condition with same-typed arms. Slots carry
  expected types (conditions/guards boolean; prop defaults the prop's type;
  the size fallback a `control-size` member).
- **Total and pure by construction**: no null literal (nullability is
  `is_null`/`is_present`), no arithmetic/calls/iteration/recursion/binding/
  indexing variants — there is no error case in the result type and no
  side-effect surface. The crate models and type-checks only; no evaluator.

## Conformance set — all twelve rows expressed and tested

| Row | Derivation | Home in the model | Exercises |
|---|---|---|---|
| 1 | `disabled \|\| loading` (`CROSS-20`) | `data-unavailable` emission condition | `or` |
| 2 | `pressed !== null \|\| defaultPressed !== null` (`BTN-14`) | `data-pressed` emission condition | `is_present`, `or` |
| 3 | `!children` (`BTN-09`) | `data-icon-only` emission condition | slot presence, `not` |
| 4 | `Boolean(leading) \|\| Boolean(leadingIcon) \|\| loading` (`BTN-17`) | `data-has-leading` emission condition | slot presence, `or` |
| 5 | `pressedControlled ? pressed === true : uncontrolledPressed` (`CROSS-04`) | `data-current-pressed` valued attribute expression | `if`/`then`/`else`, `eq` |
| 6 | `size ?? resolveSemanticControlSize(...)` (`CROSS-07`) | `SizeAxis.fallback` | `coalesce`, axis-resolution operand |
| 7 | `type === "search"` (`TXT-08`) | search-affordance render condition | `eq` vs shared-type member |
| 8 | `type === "multiline" \|\| (type === "text" && rows !== null && rows > 1)` (`TXT-06`) | input-control render condition | `or`, `and`, `is_present`, `gt` |
| 9 | `isSearch && showClearButton && !disabled && !readOnly && currentValue.length > 0` (`TXT-08`) | clear-button render condition | `and`, `not`, `is_empty` |
| 10 | `maxLength !== null && charCount > maxLength` (`TXT-14`) | `data-char-over` emission condition | `is_present`, `and`, `gt` |
| 11 | `showValidationStatus && effectiveValidationState !== "none"` (`TXT-12`) | validation-indicator render condition | `and`, `ne` |
| 12 | `data-tone` omitted when default (`BTN-18`) | `data-tone` emission condition `tone != default` | attribute emission condition |

**Transcriptions (row interpretations), per the card's log requirement:**

- `pressedControlled` → `is_present(pressed)` and `uncontrolledPressed` →
  the `defaultPressed` seed prop (row 5). A bound controlled prop is
  present, and the seed prop carries the uncontrolled value — both stay
  inside the declared operand vocabulary. Recorded rather than inventing a
  derived-state operand kind (the spec's operand list has no "derived
  state" slot; derived values live in VisualState projection fields).
- `currentValue.length > 0` → `not(is_empty(state "value"))` (row 9); the
  length check is the emptiness group, and `currentValue` is the declared
  `value` controlled state (`TXT-02`).
- `Boolean(slot)` calls (`BTN-17`) → slot-presence operands (row 4); the
  spec's operand list calls this out directly, and it is the sanctioned
  replacement for the excluded `Boolean(...)` call.
- Row 1 (`isUnavailable`) lives in an attribute emission condition rather
  than a VisualState field derivation, because the four sanctioned slots are
  fixed and do not include projection-field derivation; the projection still
  declares `enabled`/`loading` fields as before.
- Row 12 is expressed as an explicit emission condition
  (`ne(tone, member default)`) alongside the existing declarative
  `OmitWhenDefault` policy; the policy and the expression gate are both
  valid and may coexist.

**Exclusions — proven, not widened:**

- `Expr` has no arithmetic/call/iteration/recursion/binding/indexing
  variants: `serde_json` rejects any smuggled `{"add":…}`, `{"call":…}`,
  `{"index":…}` document (test `excluded_arithmetic_and_call_operators_
  cannot_be_constructed`), and arithmetic-shaped usage fails type-check
  (`is_empty` on a number, ordering on a string).
- `visualState.lowerNorm * 100` (`RNG-17`), the `TXT-16` adornment padding,
  `${charCount}/${maxLength}` (`TXT-14`), `safeSliderMax`/`normalizeRangeValue`
  (`CROSS-19`, `RNG-02`), and `slugify` (`TXT-09`) are recorded as
  projection fields / conformance vectors / machines: the fixture sources
  the padding custom property from a declared `controlPaddingStart` Number
  projection field (no expression), and the degenerate-range guard stays a
  machine step with `guard: None` (test `excluded_derivations_route_to_
  projection_fields_and_machines`).
- **No conformance row needed an excluded operator.** No stop condition was
  reached. `cargo fmt --check` at baseline reported nothing outside the
  crate, so no fmt stop occurred (the historical papercut did not recur in
  this worktree).

## Slot wiring summary

The four sanctioned slots, and no others, carry `Expr`:

1. State-derived attribute emission conditions (`StateAttribute.condition`)
   and values (`StateAttribute.value`).
2. Part render conditions (`PartKind::ConditionalExpr`).
3. Prop default (`Prop.default_expr`) and size axis fallback resolution
   (`SizeAxis.fallback`).
4. Guard conditions on transitions/effect-intents (`VectorStep.guard`).

Vector guards type-check as boolean expressions in a model-only scope:
vectors are shared machine semantics and name no component state, so a
reference operand in a vector guard is an `UnresolvedExpressionReference`
finding — machine guards are vector machines (`CROSS-19`, `RNG-02`,
`TXT-09`), never expressions. The fixture carries a literal guard
(`not(false)`) on the commit-on-release effect-intent to prove the slot.

## Validation rules added

- New `FindingKind::ExpressionTypeError` — operator/operand type mismatch or
  a slot expected-type mismatch (e.g. `and` on a number, `gt` on a string,
  a non-boolean emission condition, a prop default of the wrong type).
- New `FindingKind::UnresolvedExpressionReference` — a reference outside the
  slot's declared scope, or a member literal naming a member the shared type
  does not define / a shared type that does not exist.
- Slot contradictions (`ImpossibleBinding`): valued attribute with neither
  source nor value expression; attribute with both source and value
  expression; presence-only attribute with a value expression; prop with
  both `default` and `default_expr`.
- Existing source-resolution rule extended: valued attributes may now derive
  from a value expression instead of a `source` reference.

## Tests (44 total, all passing)

- `tests/expressions.rs` — 30 tests: 12 conformance rows (each asserts the
  fixture's slot expression equals the derivation and validation is clean);
  prop-default and vector-guard slot wiring; 3 exclusion tests; 9 negative
  type-check/reference/contradiction tests; expression findings reported
  alongside other findings; JSON round-trip + deterministic ordering +
  spec-name operator shapes (`{"or":[…]`, `{"if":{"condition":…}}`,
  member literal carrying its shared type).
- `tests/roundtrip.rs` — existing 14 tests unchanged in behavior, fixture
  construction sites extended with the new `None` fields.

## Validation

| Command | Exit state |
|---------|-----------|
| `cargo build --manifest-path packages/contracts/ir/Cargo.toml` | 0 |
| `cargo test --manifest-path packages/contracts/ir/Cargo.toml` | 0 — 44 passed (30 new + 14 existing) |
| `cargo clippy --manifest-path packages/contracts/ir/Cargo.toml -- -D warnings` | 0 |
| `cargo fmt --manifest-path packages/contracts/ir/Cargo.toml -- --check` | 0 — baseline check clean, no files outside the crate; one `#[allow(clippy::should_implement_trait)]` on the `not` builder (spec-named helper, not an `ops::Not` impl) |
| `effigy docs:lint` | 0 — the `poodle-ir` release-manifest entry added with the g13-011 merge satisfies the reverse check |
| `git diff --check` | 0 |
| `git status --porcelain` | only `packages/contracts/ir/` modified/untracked (plus this log) |

## Not done

Per batch card and worker rules: no evaluator, no codegen/emission, no
macros, no new dependencies, no real Button/RangeSlider/TextInput
definitions, no roadmap/status/dispatch edits, no merge, no `git add -A`.
`IR_SCHEMA_VERSION` stays 1: the new expression fields are additive
`#[serde(default)]` options, so documents authored against the b011 schema
still deserialize (IR-09 stable migration).
