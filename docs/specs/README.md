# Specs

Status: active
Updated: 2026-09-05

Specs define repository-wide rules that are broader than one component:
tokens, artifact generation, parity evidence, accessibility, packaging,
migration, and downstream boundaries.

## When to Use a Spec

- Use a [component contract](../contracts/components/README.md) for one
  component's inputs, behavior, accessibility, layout, and token use.
- Use a spec when a rule applies across packages or component families.
- Use [architecture](../architecture/README.md) for stable ownership and
  layering decisions.
- Use [roadmaps](../roadmaps/README.md) for delivery order and active work.

The numbered files in this directory are the complete active spec set. Earlier
specs may describe the baseline that introduced a capability; later architecture
or spec documents can supersede their implementation assumptions. Follow
explicit supersession notices and prefer current architecture for package
shape. Specs no current surface references were archived 2026-09-05 (g16.108
docs spine compaction) under [`archive/`](archive/index.md), one index line
each; archived specs are readable history, not current authority.

Current cross-cutting references:

- `001-token-source-and-artifact-contract.md` — token source and artifact
  contract (marked active 2026-09-05)
- `008-parity-evidence-documented-delta-and-downstream-extension-rules.md` —
  parity evidence, documented delta, and downstream extension rules
- `015-loading-empty-error-notification-and-remediation-rules.md` — loading,
  empty, error, notification, and remediation surface rules
- `022-packaging-versioning-and-release-channel-rules.md` — packaging,
  versioning, and release-channel rules
- `025-parity-automation-and-harness-boundary.md` — parity automation and
  harness boundary
- `026-appearance-recipes-and-downstream-override-strategy.md` — appearance
  recipe strategy (promoted into architecture 007)
- `044-deprecation-change-control-and-release-channel-operations.md` —
  deprecation, change control, and release-channel operations
- `062-headless-core-and-dual-layer-strategy.md` — headless core and dual
  layer strategy (promoted into architecture 006)
- `063-rust-authored-component-and-scene-ir.md` — retired g13 pilot record;
  its architecture 009/spec 066 successor was also rejected
- `066-executable-component-conformance.md` — rejected g14 pilot contract;
  retained as measured evidence, not current authority
- `067-model-connection-management.md` — shaping contract for reusable model
  route setup, configured connection rows, and model catalogue curation
- `068-batched-audio-meter-surface.md` — approved contract for the web
  MeterBus, AudioMeter surface mode, one-canvas MeterSurface, browser evidence,
  and the Canvas2D performance gate
- `069-dependable-drag-and-drop-substrate.md` — approved cross-runtime
  lifecycle, pointer/touch/keyboard sensors, host cross-window bridge, inbound
  files, native drag-out, migration, and certification contract
- `070-compiled-web-distribution-contract.md` — exact compiled `dist`
  inventories, Svelte client/server and React export maps, receipt schemas,
  source-free archive law, and the root-to-`./markdown` break

Specs are normative constraints, not task queues. Current execution status
belongs in the roadmap.
