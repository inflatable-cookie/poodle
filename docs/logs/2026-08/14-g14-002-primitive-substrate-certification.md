# g14.002 — Primitive Substrate Certification

Date: 2026-08-14
Card: `docs/roadmaps/g14/002-primitive-substrate-certification.md`
Depends on: g14.001 / PR #10
Status: execution complete — awaiting orchestrator review

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
| Generic kernel | 2,947 | 5,051 |
| Button pilot increment | 1,575 | 1,249 |
| Button harness (inside pilot) | 1,052 | 756 |
| GPUI capture repair | — | 356 |
| Generated data (bytes) | 33,392 | 46,177 |
| Replaced hand-written | 619 | 619 |

Extraction moved the window/driver seams into generic modules
(`conformance_driver.rs`, `primitive_probes*.rs`). Button keeps the thin
adapter + fixture path. One completion path
(`conformance:complete` / `ci:conformance`) includes the primitive report.

## GPUI Capture Repair

- Preview accepts `--control-size` (canonical) with `--size` synonym.
- Compare mode is read-only; missing baseline fails with the refresh command.
- Refresh preserves `*.previous.png` and writes
  `native-visual-refresh-manifest.v1` under `test/native-visual/out/`.
- README matches the implemented behaviour.

## Legacy Capability Tooling

Disposition: **adapt**. `docs:capability-drift` remains as static debt
detection. Execution authority is `primitive-capability-report.v1`.
`timers` is retired from the primitive roster (host timing).

## Planted Failures (representative)

| Plant | Expected failure identity |
| --- | --- |
| Unknown component capability name | interface authoring / serialize |
| Drop `interaction.on_activate` on probe fixture | `activate` / render-neutral / `node-activate-channel` |
| Clear GPUI focus registry binding | `focus` / gpui / `backend-focus-registry` |
| Break web observer channel projection | matching `surface.channels` / web probe |
| Corrupt GPUI style interpretation on fixture | matching surface/layout probe field |

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
- `effigy conformance:test-primitives-gpui` (prior windowed run; evidence retained)
- `effigy conformance:primitives-report` — 17/17 owned rows passing
- `effigy conformance:cost`
- focused `test/native-visual` compare/refresh + `--control-size=sm|lg`
  - compare missing baseline fails with refresh command (no write)
  - refresh writes baseline + manifest; second refresh keeps `*.previous.png`
  - sm and lg baselines differ (control-size live end to end)
- `effigy conformance:compare` — 20 cases × 3 active runtimes passing
- `git diff --check`

Do not run Jetstream selectors.

## Unresolved / Pressure

- Spec 066 aspirational observation fields (parent, logical bounds, clip,
  layer order, typography channels) remain unimplemented; roster covers
  implemented observation shape plus node vocabulary.
- Stale GPUI baselines still need operator reclassify via explicit refresh;
  this card does not treat bulk refresh as proof of correctness.
- `ci:web` may still hit the three pre-existing
  `AppHeaderCenterHarness.svelte` Snippet identity errors from g14.001.
