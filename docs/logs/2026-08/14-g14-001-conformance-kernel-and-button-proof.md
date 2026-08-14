# g14.001 — Conformance Kernel And Button Proof

Date: 2026-08-14
Card: `docs/roadmaps/g14/001-conformance-kernel-and-button-proof.md`
Spec: `docs/specs/066-executable-component-conformance.md`
Architecture: `docs/architecture/009-cross-runtime-component-conformance.md`
Review: `14-g14-001-delivery-review.md` (first pass rejected); this log
describes the replacement pass.

## Outcome

The replacement pass satisfies the revised card for the active cohort:

```text
one portable interface + one typed case corpus (19 cases)
  -> Svelte / React / GPUI execution
  -> normalized component-observation.v1
  -> three corpus-projected specimen views
  -> one failing completion gate
```

`effigy conformance:complete --component button` passes: 19 cases × 3 active
runtimes, all assertions observable and passing in every runtime, GPUI
registration verified, Jetstream reported program-deferred.

## How The Seven Blockers Were Removed

1. **Interface duplication** — `defineComponentInterface` takes a `const`
   generic; `PortablePropsOf` / `PortableEventsOf` / part ids / state names /
   event names / token roles / axes are mapped types over the interface
   value. `button.ts` declares no hand-written prop or event shape. The
   Svelte/React shells bind to the derived types: carrier prop names are
   `satisfies readonly (keyof ButtonPortableProps)[]`-checked and handlers
   are typed `PortableEventsOf<...>["press"]` — a rename fails the shells,
   the serializer gate, and (after regeneration) the Rust compile.
2. **Unbound cases** — `componentCase(buttonInterface, ...)` closes fixtures
   (prop names, enum values, nullability), regions, parts, states, events,
   token roles, and axes at authoring time; the serializer re-validates; the
   Rust codegen validates the serialized JSON against the interface JSON.
   Planted unknown props/states/axes fail authoring or generation.
3. **Button-specific observers** — the interface's part declarations carry
   resolution descriptors (web class/icon position; native self /
   root-label / first-text / icon-side / icon-named), and state declarations
   carry per-runtime observation rules. Shared runner and observer code
   contains no Button identifier, part list, icon name, class name, or
   tree branch.
4. **Vacuous native evidence** — the `vacuous` verdict is gone. Every case
   assertion must be observable by every active runtime; unobservable
   required fields fail with the runtime named and a reason. Native token
   roles now travel on a real node channel (`node.roles`, stamped by the
   renderer) and GPUI focus is real (the node-backend's focus registry, read
   through a public query). The corpus asserts only the three-way
   intersection: geometry on non-icon cases (web reads real CSS, native
   reads node style), icon presence (names recorded, not asserted — the web
   DOM does not carry them), states, token roles, and event order.
5. **GPUI activation** — the runner opens a real window, mounts the
   converted element, and drives calibrated NSEvents through the AppKit
   queue: hit testing, listener binding, click-to-focus, focus handles, and
   Enter key activation all traverse gpui itself. The runner never calls the
   node's activation callback. Planted proof: rendering the backend click
   binding inert fails the press cases.
6. **Jetstream** — removed from the required path entirely: runner, support
   module, generated copies, and spec changes deleted; the Jetstream
   specimen reverted to its pre-card state; the sibling-manifest workaround
   reverted. `conformance:compare` reports Jetstream program-deferred and
   fails on any report that claims it passes.
7. **Cost** — remeasured below (4,308 mechanism vs 622 replaced on Button;
   2,227 of the mechanism is the reusable kernel the five remaining profile
   pilots share; the per-component authority is the 843-line interface +
   corpus + projection). The stop-condition reassessment is in the batch
   notes under "Cost Reassessment".

## Real Defects The Corpus Caught

- **GPUI double activation**: the node-backend bound Enter/Space
  `on_key_down` while gpui itself synthesizes Enter/Space KeyUp → click on
  focused clickables — one Enter fired the handler twice (`press, press`).
  The redundant binding is removed; one click binding serves pointer and
  keyboard.
- **Native missing projections**: `poodle-render::button` never projected
  `aria-pressed` / `aria-expanded` / focus-visible and had no token-role
  channel. It now sets `a11y.toggled`, `a11y.expanded`, a focus style, and
  stamps `node.roles`.

## Architecture

- **Authority** — `packages/core/src/conformance/`: `define.ts` (typed
  schema, mapped projections, serialization), `button.ts` (the interface),
  `button-cases.ts` (19 cases, closed over the interface), `project.ts`
  (specimen projection), serializer script → neutral JSON fixtures
  (`packages/codegen/fixtures/conformance/`).
- **Generated Rust** — `poodle-codegen --conformance` validates the corpus
  against the interface and emits `packages/contracts/components/src/generated/button.rs`
  (struct/defaults/builders; extension module keeps token recipes) plus the
  corpus copy for the GPUI preview.
- **poodle-node** — new `roles: BTreeMap<String, String>` channel (the
  native `data-*` counterpart).
- **Observers** — web: `test/conformance/web/runner.ts` resolves parts and
  states from the interface descriptors over the real DOM; native:
  `packages/render/src/conformance.rs` resolves parts from the descriptors
  over the node tree and reads `node.roles`; both evaluate strictly.
- **GPUI runner** — `packages/gpui/preview/src/bin/conformance.rs`
  (LOCAL-ONLY): real window, app activation with polling, warmup click on
  empty chrome (macOS's first-click swallow), calibration, per-case mount
  with paint-readiness polling, real pointer/keyboard/focus dispatch,
  click retry (a real user re-clicks a swallowed click), strict evaluation,
  registry check, report emission.
- **Selectors** — `conformance:serialize[-check]`,
  `conformance:codegen[-check]`, `conformance:build`, `conformance:check`,
  `conformance:test[-web|-gpui]`, `conformance:check-gpui` (compile-only,
  in `ci:native`), `conformance:compare`, `conformance:complete`,
  `conformance:cost`. Read-only enforcement in `docs:check` and `ci:web`
  (serialize + codegen + web run); `ci:native` compiles the GPUI runner.

## Planted-Failure Proofs

All planted, verified failing with runtime/case/step/field named, reverted:

| Plant | Gate that failed |
| --- | --- |
| Rename `leadingIcon` in the interface | `conformance:serialize-check` stale; Svelte shell `satisfies` error; regenerated Rust breaks `poodle-render` compile |
| Unknown prop `varinat` in a case | TypeScript authoring error (`Object literal may only specify known properties`) |
| Unknown prop in the serialized JSON | codegen validation: `unknown prop 'plantedProp'` |
| Height +4px in `render::button` | GPUI `geometry.height` fail: expected 36, got 40 |
| Variant role hardcoded `primary` in the renderer | GPUI `tokenRole.variant` fail: expected secondary, got primary |
| Inert GPUI click binding (backend) | press cases fail with empty trace |
| Button removed from the GPUI registry | completion exits 1: `button registration missing` |
| Stale orphan in `generated/` | `conformance:codegen-check` reports the orphan |

Double generation byte-identical and check mode read-only come from the
codegen machinery (byte-exact compare, no write path in check mode).

## Cost Reassessment

From `effigy conformance:cost` (non-blank, non-comment lines; replaced
measured against `origin/main`):

| Surface | Lines |
| --- | --- |
| Authored (interface schema + button interface + corpus + projection + serializer) | 843 |
| Generated (Rust declaration + one JSON copy) | 1,257 |
| Adapters (web runner, native observer, GPUI runner, support, orchestrator) | 2,021 |
| Wiring (effigy selectors + gate lines, cost script) | 187 |
| **Mechanism total** | **4,308** |
| Replaced (ButtonSpec declaration + 3 active specimen fixtures) | 622 |

The cost trigger requires an explicit reassessment, not a deferral:

- **Reusable kernel = 2,227 lines** (observers, runners, wiring): a
  one-time investment every profile pilot and every future component
  consumes without growth. The rework deleted the Jetstream lane (≈460
  lines) and the duplicated JSON copies, and the strict/data-driven
  observers are smaller than the first pass's while doing more.
- **Per-component authority = 843 lines** (schema included in that figure;
  the Button-specific share — interface + corpus + projection — is ≈450):
  this is the cost of authoring one component's cases against a typed
  interface, and it replaces hand-written declarations and specimen
  fixtures per component.
- **Replaced on Button = 622 lines**; the corpus's horizon is the estate's
  ~86k specimen lines and ~160 components, each of which currently carries
  its own hand-written fixtures. The mechanism is data-driven: the next
  component adds only authority, no observer, runner, or gate code.
- **Stop-condition check**: mechanism grows faster than replaced *for
  Button alone* (first claim on the reusable kernel), so the amortization
  case above is the proof the card asks for. It will be re-tested against
  the RangeSlider pilot (g14.003): if the kernel needs component-specific
  additions there, the pilot stops with that evidence.

## Validation

- `effigy conformance:complete` — 19 cases × svelte/react/gpui, repeated 7×
  green, Jetstream program-deferred.
- `bunx vitest run` — all projects pass.
- `cargo test -p poodle-render` — 181 pass; `poodle-specs` 241 pass;
  `poodle-jetstream` adapter 162 pass.
- `effigy docs:check` — green including the new conformance steps.
- Web and GPUI preview builds green; both web specimen pages project the
  corpus (9 groups, identical structure).
- `git diff --check` clean.

## Known Baseline Failures (pre-existing, untouched)

- `check:svelte`: three `AppHeaderCenterHarness.svelte` Snippet identity
  errors.
- codegen `emitted_typescript_type_checks_with_no_framework_dependency`
  fails in this environment (typescript not resolvable by
  `bunx --no-install`).
- `docs:machine-shape-drift` stays red by standing decision.

## Retained / Retired From The First Pass

- Retained: corpus-driven specimen projection, generated Rust declaration
  with extension module, planted-failure legibility, the node a11y fixes.
- Replaced: the permissive schema (open string maps, `vacuous` verdicts),
  the Button-specific shared observer, the node-callback GPUI runner, the
  Jetstream lane, the doubled JSON copies.
- Retired: none kept "beside" the new authority — the hand-written
  declaration surface and the active specimen fixtures are gone from the
  active path.
