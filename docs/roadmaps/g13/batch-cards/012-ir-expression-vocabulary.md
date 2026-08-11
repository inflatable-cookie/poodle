# 012 IR Bounded Expression Vocabulary

Status: merged (`911fdfd8`)
Milestone: `g13.002`
Owner: Poodle core
Branch: `thread/g13-012-ir-expression-vocabulary`
Depends on: `g13-b011` merged (`4a22c8d8`)
Governing refs: `docs/specs/063-rust-authored-component-and-scene-ir.md`
§"The bounded expression vocabulary (normative)",
`docs/roadmaps/g13/pilot-expressiveness-corpus.md`

## Goal

Add the bounded expression vocabulary to `poodle-ir`. `g13-b011` modelled all
129 corpus requirements but found no first-class way to express derived state —
`CROSS-20`'s `isUnavailable = disabled || loading` had to be approximated.

The vocabulary is now bounded normatively in spec 063. Implement exactly it.

## Fixed By Ruling (do not re-decide, do not extend)

Spec 063 fixes the operand set, the operator set, the exclusions, and the slots
where expressions may appear. Read that section before writing anything.

Operators, complete: `and`, `or`, `not`, `eq`, `ne`, `is_null`, `is_present`,
`coalesce`, `gt`, `gte`, `lt`, `lte`, `is_empty`, `if/then/else`.

**Excluded deliberately:** arithmetic, string manipulation, interpolation,
formatting, function calls, iteration, recursion, variable binding, indexing.

Adding any operator outside that list is a stop condition, not initiative.
If a pilot derivation appears to need one, it is a projection field, a
conformance vector, an adapter capability, or a runtime extension — that is the
spec's own escape list, and the answer is to record it, not to widen the
language.

## Required Properties

- **Total.** Evaluation cannot fail, panic, or diverge. There is no error case
  in the evaluation result type.
- **Pure.** No side effects, no environment access, no I/O.
- **Typed.** Validation type-checks every expression against the declared prop,
  state, and projection types. A type error is a validation `Finding` at the
  authored source, with the offending identifier — not a codegen-time surprise.
- **Serializable and deterministic.** Round-trips through JSON preserving
  meaning and ordering, like the rest of the crate.

## Scope

### In scope

- An `Expr` model in `packages/contracts/ir/src/expr.rs` covering the operand
  and operator sets exactly.
- Type checking, integrated into the existing `validate(&IrModel) -> Vec<Finding>`
  so expression errors arrive with all other findings, not separately.
- New `FindingKind` variants for expression type errors and unresolved
  expression references.
- Wiring `Expr` into the four sanctioned slots: state-derived attribute
  emission conditions and values, part render conditions, prop default and axis
  fallback resolution, and guard conditions on transitions/effect-intents.
- A **conformance set**: every derivation below, expressed as a test.

### Out of scope — stop conditions if reached

- Any evaluator that executes expressions against runtime values. This card
  models and type-checks; it does not interpret.
- Any code generation or emission.
- Macros.
- Changing the modules `g13-b011` delivered, beyond adding the expression slots
  and finding variants.
- Authoring real component definitions. Fixtures stay minimal and synthetic.

## Conformance Set

These are the real pilot derivations the vocabulary must express. Each becomes
a test. They come from `$derived` in the three pilot components.

**Must express:**

| Derivation | Source | Exercises |
|---|---|---|
| `disabled \|\| loading` | Button `isUnavailable`, `CROSS-20` | `or` |
| `pressed !== null \|\| defaultPressed !== null` | Button `isToggle`, `BTN-14` | `is_present`, `or` |
| `!children` | Button `iconOnly`, `BTN-09` | slot presence, `not` |
| `Boolean(leading) \|\| Boolean(leadingIcon) \|\| loading` | Button `hasLeading`, `BTN-17` | slot presence, `or` |
| `pressedControlled ? pressed === true : uncontrolledPressed` | Button `currentPressed`, `CROSS-04` | `if/then/else`, `eq` |
| `size ?? resolveSemanticControlSize(...)` | all three, `CROSS-07` | `coalesce`, axis-resolution operand |
| `type === "search"` | TextInput `isSearch`, `TXT-08` | `eq` against shared-type member |
| `type === "multiline" \|\| (type === "text" && rows !== null && rows > 1)` | TextInput `isMultiline`, `TXT-06` | `or`, `and`, `is_present`, `gt` |
| `isSearch && showClearButton && !disabled && !readOnly && currentValue.length > 0` | TextInput `canClear`, `TXT-08` | `and`, `not`, `is_empty` |
| `maxLength !== null && charCount > maxLength` | TextInput char-over, `TXT-14` | `is_present`, `and`, `gt` |
| `showValidationStatus && effectiveValidationState !== "none"` | TextInput indicator, `TXT-12` | `and`, `ne` |
| `data-tone` omitted when default | Button `BTN-18` | attribute emission condition |

**Must NOT express — assert these are rejected or routed elsewhere:**

| Case | Correct home |
|---|---|
| `visualState.lowerNorm * 100` | VisualState projection field (`RNG-17`) |
| `Number(canClear) + Number(showValidationIndicator)` | projection field (`TXT-16` adornment padding) |
| `` `${charCount}/${maxLength}` `` | formatting concern, not IR (`TXT-14`) |
| `safeSliderMax(min, max)`, `normalizeRangeValue(...)` | conformance vector / machine (`CROSS-19`, `RNG-02`) |
| `slugify(source)` | machine (`TXT-09`) |

Add at least one test proving an arithmetic or call-shaped expression cannot be
constructed or does not type-check.

## Worker Rules

- Execute this card exactly. You have no planning or status authority.
- Do not spawn sub-agents. Read sources directly.
- Read spec 063's normative expression section and
  `packages/contracts/ir/src/validation.rs` before designing anything. Match the
  existing crate's module style, doc-comment convention (cite requirement IDs),
  and finding shape.
- Do not extend the operator set. Do not add an evaluator.
- Do not touch anything outside `packages/contracts/ir/`.
- Do not add `poodle-ir` to any other crate's manifest; dependencies stay
  `poodle-tokens` and serde.
- `cargo fmt --check` first; if it reports files outside the crate, stop and
  report rather than formatting.
- Stage only your writable paths by explicit path. Never `git add -A`.
- Commit and push with
  `git push -u origin thread/g13-012-ir-expression-vocabulary`. Do not merge.

## Writable Paths

- `packages/contracts/ir/**`
- `docs/logs/2026-08/<DD>-g13-012-ir-expression-vocabulary.md`
- `PAPERCUTS.md` (new, non-duplicate friction only)

## Steps

1. Baseline: `cargo test` and `cargo clippy -- -D warnings` on the crate,
   `effigy docs:lint`, `git diff --check`. Record exit states.
2. Read spec 063's expression section, then `validation.rs`, `props.rs`,
   `state.rs`, `attributes.rs`, `parts.rs`, `visual.rs`.
3. Model `Expr` in `src/expr.rs`. Operands and operators exactly as ruled.
4. Type-check expressions inside the existing `validate` entry point. New
   `FindingKind` variants; all findings still returned together.
5. Wire `Expr` into the four sanctioned slots.
6. Tests: one per conformance-set row, plus the exclusion test, plus JSON
   round-trip and ordering for expressions.
7. Validate:
   ```sh
   cargo build --manifest-path packages/contracts/ir/Cargo.toml
   cargo test --manifest-path packages/contracts/ir/Cargo.toml
   cargo clippy --manifest-path packages/contracts/ir/Cargo.toml -- -D warnings
   cargo fmt --manifest-path packages/contracts/ir/Cargo.toml -- --check
   effigy docs:lint
   git diff --check
   git status --porcelain
   ```

## Acceptance Criteria

- [x] `Expr` implements exactly the ruled operand and operator sets — no more.
- [x] Expressions are total (no error case in evaluation typing), pure, and
  serializable with deterministic ordering.
- [x] Type errors and unresolved references surface as validation `Finding`s
  alongside all others, with identifier and actionable message.
- [x] `Expr` is wired into all four sanctioned slots and no others.
- [x] Every "must express" conformance row has a passing test.
- [x] At least one test proves an excluded shape cannot be expressed.
- [x] No evaluator, no codegen, no macro, no new dependency.
- [x] Nothing outside `packages/contracts/ir/` changed except log and papercuts.
- [x] All step-7 commands exit 0.
- [x] Batch log records commands, exit states, and any conformance row that
  needed interpretation.

## Stop Conditions

- A conformance-set row cannot be expressed without an excluded operator.
- Type checking a slot requires type information the schema does not carry.
- Wiring a slot forces a breaking change to a `g13-b011` type.
- `cargo fmt --check` reports files outside the crate.

Stop with the row, the operator you would have needed, exact paths, and the
smallest unresolved question. A row that genuinely needs an escape (projection,
vector, capability, extension) is a **finding to record**, not a reason to widen
the language.
