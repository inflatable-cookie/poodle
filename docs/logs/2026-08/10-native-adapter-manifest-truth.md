# Native Adapter Manifest Truth

Poodle remains `strict-ready`. The release sweep confirmed that the last
critical duplicate finding is independent backend evidence, then repaired the
incorrect claims inside it and added a drift gate.

## Findings

- `packages/gpui/adapter/src/lib.rs` advertised `TimeInputSpec`, which does not
  exist, instead of its implemented `TimeFieldSpec`.
- GPUI release baselines, accessibility proof, docs lint, and the component
  contract also carried or enforced that nonexistent Rust name.
- `packages/jetstream/adapter/src/lib.rs` advertised seven retired workstation
  specs and `ReorderableListSpec` without direct `RenderComponent`
  implementations.
- Jetstream repeated six live workstation specs across two categories and
  silently deduplicated them at runtime.
- Adapter and trait comments described these legacy direct inventories as full
  runtime parity, even though shared native coverage now lives in
  `poodle-render` and its previews.
- `ci:native` did not run the GPUI adapter tests or the shared renderer tests
  its own comments said it covered.

## Changed

- Made both manifests exactly match their direct `RenderComponent`
  implementations: 100 GPUI and 107 Jetstream.
- Removed Jetstream's retired workstation list and runtime deduplication.
- Scoped `AdapterManifest` documentation to direct-adapter introspection.
- Repaired the GPUI baseline, accessibility proof, docs lint expectation, and
  TimeInput contract note to name `TimeFieldSpec` consistently.
- Added `drift:adapter-manifests`, which rejects missing, phantom, or duplicate
  manifest entries by comparing them with direct implementation declarations.
- Added GPUI adapter and shared `poodle-render` tests to `ci:native`.
- Recorded the hanging graph query in `PAPERCUTS.md`.

## Duplicate Classification

The scanner remains at 105 findings: 1 critical, 10 high, and 94 warning. The
critical and high native pairs repeat contract fixtures or backend capability
evidence intentionally. The new drift gate makes the critical pair truthful
without merging the independent runtime declarations.

## Validated

- `effigy drift:adapter-manifests`
- `effigy gpui:test`
- `effigy test:jetstream-adapter`
- `effigy check:gpui`
- `effigy ci:native`
- `effigy docs:check`
- `effigy scan duplicate-blocks`
- `git diff --check`
