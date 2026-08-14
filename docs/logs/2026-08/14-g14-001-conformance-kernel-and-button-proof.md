# g14.001 — Conformance Kernel And Button Proof

Date: 2026-08-14
Card: `docs/roadmaps/g14/001-conformance-kernel-and-button-proof.md`
Spec: `docs/specs/066-executable-component-conformance.md`
Architecture: `docs/architecture/009-cross-runtime-component-conformance.md`

## Outcome

The smallest complete conformance loop now exists and passes for Button:

```text
one portable interface + one typed case corpus (19 cases)
  -> Svelte / React / GPUI / Jetstream execution
  -> normalized component-observation.v1
  -> four corpus-projected specimen views
  -> one failing completion gate (conformance:complete)
```

`effigy conformance:complete --component button` is green: 19 cases × 4
runtimes, all assertions pass, no vacuous-only assertions, both native
registrations verified.

## Before / After Cost

Mechanism LOC (non-blank, non-comment), from `effigy conformance:cost`:

| Surface | Lines |
| --- | --- |
| Authored (interface schema, button interface, corpus, projection, serializer) | 584 |
| Generated (Rust declaration + JSON copies) | 2,380 |
| Adapters (web runner, native observer, 2 bins, 2 support modules, orchestrator) | 1,989 |
| Wiring (effigy selectors + gate lines, cost script) | 192 |
| **Mechanism total** | **5,145** |
| Replaced (hand-written ButtonSpec declaration + 4 specimen fixtures, via git) | 844 |

The Button proof's own ongoing authoring cost is the 584-line authority
(interface + corpus + schema). The mechanism is the investment; profile
pilots 2–6 (RangeSlider, Tabs, Popover, TextInput, HistoryCenter) reuse the
schema, observers, runners, and gates without new mechanism. The 844 replaced
lines are the first claim on that investment; the estate's ~86k specimen LOC
is the horizon, with the standing rule that the mechanism must not grow
faster than the duplication it removes (spec 066 stop condition, checked at
g14.010).

## What Was Built

- `packages/core/src/conformance/` — TS authority: `define.ts` (constrained
  interface + case builders), `button.ts` (portable interface), `button-cases.ts`
  (19 cases: variants, tones, states, icons, press pointer/keyboard, toggle,
  focus-visible, size/density axes), `project.ts` (specimen projection).
- `packages/core/scripts/conformance-serialize.ts` — deterministic neutral
  JSON fixtures into `packages/codegen/fixtures/conformance/` (+ `--check`).
- `poodle-codegen --conformance` + two targets: `conformance-rust` emits
  `packages/contracts/components/src/generated/button.rs` (the ButtonSpec
  declaration: struct, defaults, builders — replacing the hand-written
  surface); `conformance-cases` copies the JSON into both native previews'
  `src/generated/conformance/`.
- `packages/contracts/components/src/button.rs` — shrunk to the hand-written
  extension (token recipes, derived queries) beside the generated surface.
- Web harness: `test/conformance/web/` vitest project (real CSS, real DOM,
  real events; Svelte host component for controlled pressed state).
- Native harness: `packages/render/src/conformance.rs` (generic node
  observer + part convention + assertion evaluator), Jetstream bin (real
  `GameUi` pointer + keyboard dispatch, backend a11y projection), GPUI bin
  (real `to_gpui` conversion + node-level dispatch).
- Four specimen pages are now corpus projections; all hand-written Button
  specimen fixtures deleted.
- Effigy selectors: `conformance:serialize[-check]`, `conformance:codegen[-check]`,
  `conformance:build`, `conformance:check`, `conformance:test[-web|-gpui|-jetstream]`,
  `conformance:compare`, `conformance:complete`, `conformance:cost`. Read-only
  enforcement wired into `docs:check`, `ci:web` (authority drift + web run) and
  `ci:native` (GPUI run).

## Contradictions Found And Resolved

1. **Event order on toggle press**: Svelte/React emit `pressedChange` before
   `press`; the corpus now pins that order, and the native hosts mirror it.
2. **Native dropped the a11y projections**: `poodle-render::button` never
   projected `aria-pressed`/`aria-expanded`; now sets `a11y.toggled` /
   `a11y.expanded`. The toggle case asserts the pressed state natively.
3. **Native ignored `fit`/`truncate`/`max_width`** — recorded as estate debt
   (web-only behaviour today), not covered by required cases.
4. **Loading implies disabled**: `isUnavailable = disabled || loading` —
   corpus asserts disabled=true on the loading case.
5. **Icon-side padding inset**: loading/icon cases shrink the leading
   padding by `icon_side_inset`; geometry assertions on those cases were
   dropped (spinner-dependent), not silently loosened.
6. **Keyboard confirm on Jetstream** emits `UiEvent::Activated` without
   re-firing the click handler — the host reacts to the event stream (the
   real host contract); the corpus caught this during development.

## Planted-Failure Proofs

All planted, verified failing with runtime/case/step/field named, reverted:

| Plant | Gate that failed |
| --- | --- |
| Rename `leadingIcon` in the interface | `conformance:serialize-check` stale; regenerated Rust breaks `poodle-render` compile |
| Height +4px in `render::button` | GPUI (and Jetstream) `geometry.height` fail, expected 36 got 40 |
| Svelte hardcodes `data-variant="secondary"` | Svelte `tokenRole.variant` fail, expected primary got secondary |
| Stale orphan in `generated/` | `conformance:codegen-check` reports the orphan |
| Inert handler | press cases fail on empty trace (proven live: the keyboard-CONFIRM gap above was caught exactly this way) |

Byte-identical double generation and read-only check mode come from the
existing codegen machinery (byte-exact compare, no write path in check mode)
and pass on every `conformance:codegen-check` run.

## Known Baseline Failures (pre-existing, untouched)

- `check:svelte`: three `AppHeaderCenterHarness.svelte` Snippet identity
  errors (recorded in the estate).
- codegen `emitted_typescript_type_checks_with_no_framework_dependency`
  fails in this environment (typescript not resolvable by `bunx --no-install`).
- `docs:machine-shape-drift` stays red by standing decision.

## Environment Note

The Jetstream runner needs the sibling jetstream repo. Its `jetstream-poodle`
crate hardcodes the poodle checkout at `../poodle` (an independent clone, on
the g14-005 thread branch). To run the jetstream lane from this worktree the
sibling manifest was temporarily repointed at
`../../../t3code-36a5816f/packages/...` and the stale clone moved aside
(`poodle-clone-g14-005`). This is a local-only, out-of-repo change; restore
the sibling manifest (`git -C ../jetstream checkout crates/jetstream-poodle/Cargo.toml`)
and the clone location when the lane is validated on a checkout whose crates
align with the sibling path.
