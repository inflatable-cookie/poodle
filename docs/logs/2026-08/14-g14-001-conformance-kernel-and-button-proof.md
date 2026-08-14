# g14.001 — Conformance Kernel And Button Proof

Date: 2026-08-14
Card: `docs/roadmaps/g14/001-conformance-kernel-and-button-proof.md`
Spec: `docs/specs/066-executable-component-conformance.md`
Architecture: `docs/architecture/009-cross-runtime-component-conformance.md`
Review: `14-g14-001-delivery-review.md` (first pass rejected; replacement
re-reviewed; this log describes the second replacement, which corrects the
re-review's seven findings).

## Outcome

```text
one portable interface + one typed case corpus (20 cases)
  -> Svelte / React / GPUI execution
  -> normalized component-observation.v1, compared field-for-field
  -> three corpus-projected specimen views
  -> one failing completion gate
```

`effigy conformance:complete --component button` passes: 20 cases × 3 active
runtimes, every assertion observable and passing in every runtime, the
normalized observations agreeing across runtimes (shape and value), GPUI
registration verified, Jetstream reported program-deferred.

## How The Re-review Findings Were Corrected

1. **Nullable/absent shape equivalence.** The generated Rust now emits
   `Option` for every nullable prop (`size`, `density`, `default_pressed`
   joined `leading_icon` etc. that were already `Option`). Absence is
   `None`/`null` on both surfaces; the renderer resolves absent size/density
   to the md/default defaults exactly like the web pair. `defaultPressed`
   is `Option<bool>` — `is_toggle_mode` is
   `pressed.is_some() || default_pressed.is_some()`, so explicit
   `defaultPressed=false` enters toggle mode identically everywhere. The
   corpus pins it with `button/default-pressed-toggle`. A planted collapse
   (`default_pressed.unwrap_or(false)`) fails that case on GPUI.
2. **Normalized observations compared, not just asserted fields.**
   `conformance:compare` now compares every observation frame across the
   cohort: identity fields (present/role/name/text/icon/focusable/focused/
   focusVisible, states, token roles) must agree in shape — which runtimes
   observe a value — and in value; geometry compares with a bounded
   tolerance where the reference observes it; the trace must match exactly.
   The `label` part normalizes to text-carrier semantics on all three
   runtimes (role/name/interactivity belong to root). A planted label leak
   (role/name on the label part) fails the shape comparison; a planted
   size-ladder divergence fails the value comparison.
3. **No Button knowledge in shared observers.** The web runner's part
   selectors and icon-identity attributes come from the interface
   descriptors; the native observer resolves parts by descriptor and reads
   roles from `a11y.role` (the renderer declares `NodeRole::Button`) and
   labels through `Node::intrinsic_text()` (the vocabulary's own accessor).
   No class selector, icon name, part list, or `NodeKind` branch remains in
   shared runner/observer code.
4. **Icon identity observed and asserted.** The web Button projects
   `data-icon` channels (leading/trailing icon spans, spinner, chevron);
   the descriptors name the attribute; the corpus asserts the names on all
   three runtimes (`plus`/`check`/`spinner`/`chevron-down`). A planted
   wrong identity fails the web pair; the native pair reads the Icon node
   names. Channels stay recorded, not compared — web cannot resolve the
   color-mix recipes headlessly, and the comparison says so rather than
   claiming a channel check it does not make.
5. **GPUI execution on a standing surface.** `conformance:test-gpui` runs
   inside `ci:native` (the standing local native board, macOS, where the
   runner's real window opens). Compile-only cannot catch an inert
   listener, so the executed run is the enforcement; the scheduled GitHub
   workflow keeps its check-only mirror (no windowed CI runs by standing
   decision). Smallest viable options for remote enforcement, if wanted:
   (a) add the windowed run to the manual `ci-native.yml` macOS job
   (GitHub macOS runners have a window server; the repo's history avoids
   windowed CI for flake/allowance reasons — the driver is now
   deterministic across repeated runs, which was the original blocker);
   (b) keep executed enforcement local and accept check-only CI. The
   orchestrator owns that call.
6. **Exhaustive cost inventory.** `conformance:cost` now inventories every
   mechanism file and all four committed JSON artifacts (interface
   fixture, case fixture, gpui interface copy, gpui case copy) and
   measures concrete deletions against `origin/main`. Numbers below.
7. **Branch hygiene.** Rebased onto current main; both committed conflict
   markers resolved; `git diff --check` clean; the stale claims removed
   from this log.

## Real Drift The Normalized Comparison Caught

- **The native size ladder disagreed with the web CSS**: heights rendered
  28/30/42/44 for xs/sm/lg/xl against the CSS's 24/28/44/52, and a
  per-size padding offset the CSS does not have. `render::button` now
  follows the CSS ladder (`control_height_rem`) and density-only padding
  offsets; the render test pins the CSS values.
- **Per-size icon insets** were flat; button.css varies them per size
  (xs −3px, sm −4px, md −2px, lg 0, xl +1px). Now mirrored.
- **Click-to-focus asymmetry**: gpui focuses on pointer press, the headless
  web DOM does not — the web harness now performs the browser default, and
  both sides observe the same focused state.
- **Focus state leaking across cases** on GPUI: the driver now blurs
  through the window root's real focus handle (tracked on the root
  element) and verifies the app is active and the window key before
  driving — `Window::focus` silently no-ops on a non-key window, which the
  driver now treats as a precondition, not an assumption.
- **Trace payload shapes** differed (web objects vs native strings); the
  native host now records the same `{event, payload}` objects.
- The keyboard-activation double-fire and the missing a11y/token-role
  projections from the first rework remain fixed.

## Architecture

- **Authority** — `packages/core/src/conformance/`: `define.ts` (const-
  generic typed schema, mapped projections, closed authoring, loose
  serialized forms), `button.ts` (the interface — descriptors for parts,
  per-runtime state observation rules, token roles, axes), `button-cases.ts`
  (20 cases), `project.ts`, serializer → neutral JSON fixtures.
- **Generated Rust** — `poodle-codegen --conformance` validates the corpus
  against the interface and emits `generated/button.rs` (nullable props as
  `Option`) plus the gpui preview corpus copy.
- **poodle-node** — `roles` token-role channel, `intrinsic_text()`,
  `token_role()` builder.
- **Observers** — web (`test/conformance/web/runner.ts`): descriptor-driven
  part resolution, icon identity channels, root-only identity fields;
  native (`packages/render/src/conformance.rs`): descriptor-driven part
  resolution, `a11y`-only roles, intrinsic-text labels, strict verdicts.
- **GPUI runner** — real window, activation + key-ness verification,
  calibrated NSEvents, real focus handles with blur and readiness polls,
  click retry, strict evaluation, registry check.
- **Comparison** — `test/conformance/compare.ts`: shape + value agreement
  across the cohort, bounded geometry tolerance, exact traces, Jetstream
  reported program-deferred.

## Planted-Failure Proofs

| Plant | Gate that failed |
| --- | --- |
| Rename `leadingIcon` in the interface | serializer drift; Svelte shell `satisfies` error; regenerated Rust breaks compile |
| Unknown prop/state/axis in a case | authoring-time TS error; codegen validation |
| Collapse `default_pressed` to `unwrap_or(false)` | GPUI `default-pressed-toggle` events + pressed state fail |
| Label part leaks root role/name | normalized shape comparison: `label.role` observed by gpui only |
| Wrong `data-icon` on the leading span | web pair icon assertion fails, expected plus got check |
| Height +4px / variant role hardcoded | geometry / tokenRole fail with runtime/case/step/field named |
| Inert GPUI click binding | press cases fail with empty trace |
| Button removed from the GPUI registry | completion exits 1 |
| Stale orphan in `generated/` | `conformance:codegen-check` reports it |

## Cost (exhaustive inventory, `effigy conformance:cost`)

| Surface | Lines |
| --- | --- |
| Authored (schema + interface + corpus + projection + serializer) | 876 |
| Codegen (parsing, validation, targets) | 302 |
| Generated artifacts (declaration + four committed JSON artifacts) | 3,387 |
| Observers and runners | 1,498 |
| Supporting deltas (vocabulary, renderer, backends, shells) | 45 |
| Wiring (selectors + cost script) | 155 |
| **Mechanism total** | **7,451** |
| Replaced (declaration + three active specimen fixtures) | 617 |

Stop-condition statement: the reusable kernel (codegen + observers +
runners + supporting + wiring = ~2,000 lines) exceeds the 617 replaced
lines on Button alone; the mechanism is a one-time investment the five
remaining profile pilots consume without growth, and the per-component
authority (876 lines, schema included) replaces per-component hand-written
declarations and specimens. That amortization is a claim, not evidence —
it is re-tested at the RangeSlider pilot (g14.003), and the pilot stops
with this report if the kernel needs component-specific growth there.

## Validation

- `effigy conformance:complete` — 20 cases × svelte/react/gpui, repeated
  green, normalized observations compared, Jetstream program-deferred.
- `bunx vitest run` — 1,229 tests across 90 files.
- `cargo test` — render 181, poodle-specs 241, node-backend 11, codegen 13.
- `effigy docs:check` green; svelte + react preview builds green.
- `check:svelte` — 3 pre-existing `AppHeaderCenterHarness` Snippet errors
  (baseline, untouched).
- `git diff --check` clean; branch rebased onto main.

## Retained / Retired

- Retained from earlier passes: corpus-driven specimen projection,
  generated Rust declaration with the extension module, real-window GPUI
  execution, planted-failure legibility.
- Replaced in this pass: the permissive observers (Button knowledge),
  vacuous verdicts, node-callback activation, collapsed nullable shapes,
  the pairwise failure-only comparison.
- Retired: nothing sits beside the new authority; the four JSON artifacts
  are gated byte-exact by `conformance:check`.
