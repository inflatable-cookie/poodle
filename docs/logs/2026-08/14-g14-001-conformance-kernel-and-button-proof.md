# g14.001 — Conformance Kernel And Button Proof

Date: 2026-08-14
Card: `docs/roadmaps/g14/001-conformance-kernel-and-button-proof.md`
Spec: `docs/specs/066-executable-component-conformance.md`
Architecture: `docs/architecture/009-cross-runtime-component-conformance.md`
Review: `14-g14-001-delivery-review.md` (three passes; this log describes the
third correction batch, which addresses the second re-review's six findings).

## Outcome

```text
one portable interface + one typed case corpus (20 cases)
  -> Svelte / React / GPUI execution
  -> normalized component-observation.v1, compared field-for-field
  -> three corpus-projected specimen views
  -> one failing completion gate
```

`effigy ci:conformance` (the executed standing board) and
`effigy conformance:complete --component button` pass: 20 cases × 3 active
runtimes, every assertion observable and passing in every runtime, the
normalized observations agreeing across runtimes (shape and value, with
assertion-local geometry bounds), GPUI registration verified, Jetstream
reported program-deferred.

## The Six Findings, Corrected

1. **Standing enforcement.** `conformance:test-gpui` is removed from
   `ci:jetstream` (it made Jetstream setup a prerequisite) and from any
   claim of being in `ci:native`. The executed proof lives on a new named
   board `ci:conformance` = check + typecheck + web execution + windowed
   GPUI execution + comparison. `docs:check` and `ci:web` carry the
   authority drift checks, the narrow type gate, and the web execution;
   `ci:native` carries the compile-only leg (explicitly not execution
   evidence). The headless boards (`qa`, `ci:native`) keep their contract;
   the windowed run sits beside them the same way `test:native-visual`
   does. Board options are below — this is a handback, not a resolution
   of the windowed-CI question.
2. **Native semantic invention removed.** `role_of` reads `a11y.role`
   alone — no `NodeKind::Button` fallback — and `label_text` uses the
   vocabulary's `Node::intrinsic_text()` accessor plus the first Text
   child. No Button branch, identifier, class selector, icon name, or
   part list remains in shared observer code. Planted: removing the
   renderer's `a11y.role` declaration fails the role assertion with
   `gpui button/default step 0 role: expected button, not observed by
   gpui`.
3. **The typed corpus type-checks.** `PartExpectation` gained the missing
   `icon` field; a new `conformance:typecheck` selector runs a narrow
   `tsc` over the four conformance source files (strict, no baseline
   coupling) and is wired into `conformance:check`, `docs:check`, and
   `ci:web`. Zero errors.
4. **Button's public web callback contract restored.** The Svelte and
   React `onClick` props keep the contract's `MouseEvent` shape; the
   binding to the semantic `press` name is mechanical
   (`WebHandler<"press">` reads the interface's `webCarrier` marker), so
   renaming the event fails the shells — the internal casts are gone.
   `PayloadArgs` now produces one object argument for multi-field
   payloads (not a union of field values), with type-level tests in
   `button.ts` pinning `press → ()` and `pressedChange → (boolean)`.
   Planted: renaming `press` fails the corpus authoring and both shells.
5. **Assertion-local geometry bounds.** The blanket
   `GEOMETRY_TOLERANCE` is gone. The comparison reads the corpus's
   authored geometry assertions and compares exactly those fields with
   exactly those named tolerances; unasserted geometry is recorded, not
   compared. Planted: +0.5px on native height passes (within the
   authored bound of 1), +2px fails with `geometry.height: expected 36,
   got 38 (bound 1)`.
6. **Cost: copies removed, numbers grounded, stop condition honored.**
   The `conformance-cases` target and both GPUI copies are deleted; the
   GPUI code reads the canonical fixtures directly. The report counts the
   two remaining JSON fixtures in bytes and inventories every mechanism
   line. The condition is still triggered — see below.

## Cost (exhaustive, post-copy-removal, `effigy conformance:cost`)

| Surface | Units (LOC; JSON in KB) |
| --- | --- |
| Authored (schema + interface + corpus + projection + serializer) | 886 |
| Codegen (parsing, validation, Rust target) | 291 |
| Generated (Rust declaration + two canonical JSON fixtures) | 1,760 |
| Observers and runners | 1,498 |
| Supporting deltas (vocabulary, renderer, backends, shells) | 26 |
| Wiring (selectors + cost script) | 164 |
| **Mechanism total** | **5,909** |
| Replaced (declaration + three active specimen fixtures, vs main) | 619 |

**The spec 066 stop condition is triggered on Button alone: mechanism
5,909 vs replaced 619.** This card must not start a second component to
manufacture amortization evidence, so the decision is returned to the
orchestrator with three measured options:

- **Accept the kernel cost.** The reusable kernel (codegen + observers +
  runners + supporting + wiring ≈ 2,180 units) is a one-time investment;
  the per-component authority is ~886 units and replaces per-component
  hand-written declarations and specimen fixtures (the estate holds ~86k
  specimen lines across ~160 components). Acceptance = an explicit
  orchestrator ruling that g14.002 may proceed under this cost model.
- **Simplify the mechanism before acceptance.** Candidates, measured:
  fold the serializer into the codegen pass (removes ~60 units and one
  artifact round-trip); drop the standalone specimen projection in favour
  of case-driven rendering (removes ~66 units of projection code but
  keeps the same specimen output); shrink the GPUI driver's activation
  and calibration choreography (~120 units) once the windowed board is
  stable. Each is a real cut, none changes the proof's claims.
- **Reject at Button.** Close g14.001 without acceptance and re-plan the
  conformance lane; the evidence stays on this branch.

The branch does not claim completion beyond `ci:conformance` green; the
cost ruling is the orchestrator's.

## Standing-Enforcement Options (windowed GPUI)

The executed GPUI proof requires a macOS window server. The repo's
headless boards cannot host it by their contract, so the branch ships
`ci:conformance` as the named executed board (local, macOS — the same
class as `test:native-visual`). Three concrete options for the standing
question:

- **A: `ci:conformance` as shipped** — a named local board, deterministic
  across repeated runs (10+ consecutive green), ~15s after the first
  build. Trade-off: not composed into `qa`, so it runs when invoked, not
  on every `effigy qa`.
- **B: Add the executed run to the manual `ci-native.yml` GitHub
  workflow** — macos-latest runners have a window server; the run is
  deterministic now. Trade-off: macOS Actions minutes (~10× billing per
  the workflow's own note) and the repo's standing decision against
  windowed CI; the workflow is manual-dispatch only.
- **C: Compose `ci:conformance` into `qa`** — changes `qa`'s documented
  headless/CPU-only contract. Trade-off: every `effigy qa` gains a
  window + ~15s; contract change needs an orchestrator ruling.

## Planted-Failure Evidence (this batch)

| Plant | Gate that failed |
| --- | --- |
| Rename `press` in the interface | corpus authoring TS errors; Svelte + React shells fail (`WebHandler<"press">`) |
| Rename `leadingIcon` | serializer drift; shell `satisfies` error; regenerated Rust breaks compile |
| Unknown prop/state/axis in a case | authoring TS error; codegen validation |
| Collapse `default_pressed` to `unwrap_or(false)` | GPUI `default-pressed-toggle` fails |
| Label part leaks root role/name | shape comparison: `label.role` observed by gpui only |
| Wrong `data-icon` on the leading span | web icon assertion fails, expected plus got check |
| Renderer `a11y.role` declaration removed | GPUI role assertion fails: `not observed by gpui` |
| Height +0.5px / +2px | within the authored bound passes; outside fails with the bound named |
| Inert GPUI click binding | press cases fail with empty trace |
| Button removed from the GPUI registry | completion exits 1 |
| Stale orphan in `generated/` | `conformance:codegen-check` reports it |

## Validation

- `effigy test --plan` (vitest suite selected) before the batch.
- `effigy ci:conformance` green; `effigy conformance:complete` green ×2.
- `bunx vitest run` — 1,229 tests across 90 files.
- `cargo test` — render 181, poodle-specs 241, node-backend 11, codegen 13.
- `effigy test:web-pack-install` — 5/5.
- `effigy docs:check` green; svelte + react preview builds green.
- `check:svelte` — 3 pre-existing `AppHeaderCenterHarness` Snippet errors
  (baseline, untouched).
- `git diff --check` clean; branch rebased onto main (this batch's base).

## Retained / Retired

- Retained: corpus-driven specimen projection, generated Rust declaration
  with the extension module, real-window GPUI execution, the normalized
  comparison, the strict verdicts.
- Retired this batch: the conformance-cases target and both GPUI JSON
  copies, the observer's Button branches, the blanket geometry tolerance,
  the web callback casts, the `ci:jetstream` placement, the stale
  four-artifact cost numbers.
- Unresolved, returned to the orchestrator: the cost stop condition and
  the windowed-CI standing question — both with measured options above.
