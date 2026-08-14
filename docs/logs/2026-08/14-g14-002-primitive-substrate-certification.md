# g14.002 — Primitive Substrate Certification

Date: 2026-08-14
Card: `docs/roadmaps/g14/002-primitive-substrate-certification.md`
Depends on: g14.001 / PR #10
Status: accepted and landed in PR #11 (`cc9f5613`)

## Outcome

```text
one finite primitive-capability roster
  -> web observer probes (Svelte + React)
  -> renderer-neutral poodle-node probes
  -> real GPUI backend probes
  -> primitive-capability-report.v1
  -> component completion rejects missing or unexecuted requirements
```

Jetstream remains program-deferred outside capability rows.

## Inventory Coverage

Every public `poodle-node` / layout / style vocabulary item and every
implemented `component-observation.v1` field maps to one roster row. Owned
rows (17) execute on Svelte, React, render-neutral Rust, and GPUI. Deferred
rows (10) keep channel ownership on cards 003–007:

| Deferred | Owner |
| --- | --- |
| `semantic.selected` | g14.004 |
| `semantic.expanded` | g14.005 |
| `interaction.activate-modified`, `interaction.key` | g14.004 |
| `interaction.scrub` | g14.003 |
| `interaction.drag-drop`, `interaction.context` | g14.007 |
| `overlay.intent` | g14.005 |
| `input.value`, `input.editing` | g14.006 |

Capability names are closed twice over the same generated roster: TypeScript
authoring/serialization and Rust fixture loading. Evidence files also reject
unknown capability IDs. The report unions each probe's executed observation
names and fails an owned row when any `requiredObservations` entry is absent.

## Executed Matrix

Machine gate: `test/conformance/web/out/primitive-capability-report.json`
(gitignored evidence; regenerate via `effigy conformance:primitives-report`).

| Capability | Family | Owner | Svelte | React | Rust | GPUI | Status |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `structure.identity` | structure | g14.002 | passing | passing | passing | passing | passing |
| `structure.part-resolution` | structure | g14.002 | passing | passing | passing | passing | passing |
| `layout.intent` | layout | g14.002 | passing | passing | passing | passing | passing |
| `layout.geometry` | layout | g14.002 | passing | passing | passing | passing | passing |
| `layout.position` | layout | g14.002 | passing | passing | passing | passing | passing |
| `surface.channels` | surface | g14.002 | passing | passing | passing | passing | passing |
| `surface.extended` | surface | g14.002 | passing | passing | passing | passing | passing |
| `surface.state-patches` | surface | g14.002 | passing | passing | passing | passing | passing |
| `surface.animation` | surface | g14.002 | passing | passing | passing | passing | passing |
| `content.text-icon` | content | g14.002 | passing | passing | passing | passing | passing |
| `content.typography` | content | g14.002 | passing | passing | passing | passing | passing |
| `semantic.token-roles` | semantic | g14.002 | passing | passing | passing | passing | passing |
| `toggle` | semantic | g14.002 | passing | passing | passing | passing | passing |
| `semantic.disabled` | semantic | g14.002 | passing | passing | passing | passing | passing |
| `focus` | interaction | g14.002 | passing | passing | passing | passing | passing |
| `activate` | interaction | g14.002 | passing | passing | passing | passing | passing |
| `accessibility.projection` | accessibility | g14.002 | passing | passing | passing | passing | passing |

GPUI mounted accessibility: forced-acceptance per contract 003 (outside
passing rows). No parallel GPUI accessibility tree scheduled.

## Cost (vs landed g14.001)

`effigy conformance:cost`, measured on this branch:

| Surface | g14.001 | g14.002 |
| --- | ---: | ---: |
| Generic kernel | 2,947 | 5,390 |
| Button pilot increment | 1,575 | 1,249 |
| Button harness (inside pilot) | 1,052 | 756 |
| GPUI capture repair | — | 417 |
| Generated data (bytes) | 33,392 | 46,177 |
| Replaced hand-written | 619 | 619 |

Extraction moved the window/driver seams into generic modules
(`conformance_driver.rs`, `primitive_probes*.rs`). Button keeps the thin
adapter + fixture path. One completion path
(`conformance:complete-windowed` / `ci:conformance-windowed`) includes the
primitive report.

## GPUI Capture Repair

- Preview accepts `--control-size` (canonical) with `--size` synonym and emits
  a checked `native-visual-axis-receipt.v1` for every capture.
- Compare mode is read-only; missing baseline fails with the refresh command.
- Every run keeps a timestamped evidence directory. Refresh preserves
  `*.previous.png` plus before, after, diff, receipt, and
  `native-visual-refresh-manifest.v1` paths under that directory.
- README matches the implemented behaviour.

## Legacy Capability Tooling

Disposition: **adapt**. `docs:capability-drift` remains as static debt
detection. Execution authority is `primitive-capability-report.v1`.
`timers` is retired from the primitive roster (host timing).

## Planted Failures (representative)

| Plant | Expected failure identity |
| --- | --- |
| Unknown component capability name | TypeScript authoring / serialization and Rust fixture loading |
| Drop `interaction.on_activate` on probe fixture | `activate` / render-neutral / `node-activate-channel` |
| Clear GPUI focus registry binding | `focus` / gpui / `backend-focus-registry` |
| Break web observer channel projection | matching `surface.channels` / web probe |
| Remove a GPUI style/layout emission | matching `backend.<channel>` receipt field |

Revert plants before merge review. Commands:

```sh
# unknown capability
# edit button.ts capabilities to include { name: "not-a-capability", required: true }
effigy conformance:typecheck

# inert activate channel
# build_probe_fixture(None) then cargo test primitive_probes::tests
```

## Button Regression

`effigy conformance:test-web` — Button corpus + primitive web probes pass
through the shared adapters (no copied harness path).

## Validation

- `effigy conformance:serialize` / `conformance:check`
- `effigy conformance:test-web`
- `cargo test --manifest-path packages/render/Cargo.toml primitive_probes::tests`
- `effigy conformance:test-primitives-gpui-windowed` (windowed backend receipts + focus/event path)
- `effigy conformance:primitives-report` — 17/17 owned rows passing
- `effigy conformance:cost`
- focused `test/native-visual` compare/refresh + `--control-size=sm|lg`
  - compare missing baseline fails with refresh command (no write)
  - refresh writes before/after/diff + manifest; second refresh keeps `*.previous.png`
  - non-default `lg` capture produced and passed a resolved-axis receipt
- `effigy conformance:compare` — 20 cases × 3 active runtimes passing
- `effigy ci:rust`
- `effigy ci:native`
- `effigy ci:web`
- `effigy docs:check`
- `git diff --check`

Do not run Jetstream selectors.

`ci:web` initially reproduced the known duplicate-Svelte `Snippet` identity
errors in `AppHeaderCenterHarness.svelte`. The test-only harness now crosses
that packed-install boundary explicitly; the full selector passes.

## Unresolved / Pressure

- Spec 066 aspirational observation fields (parent, logical bounds, clip,
  layer order, typography channels) remain unimplemented; roster covers
  implemented observation shape plus node vocabulary.
- Stale GPUI baselines still need operator reclassify via explicit refresh;
  this card does not treat bulk refresh as proof of correctness.
