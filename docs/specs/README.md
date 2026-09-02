# Specs

Status: active
Updated: 2026-09-02

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

The numbered files in this directory are the complete spec set. Earlier specs
may describe the baseline that introduced a capability; later architecture or
spec documents can supersede their implementation assumptions. Follow explicit
supersession notices and prefer current architecture for package shape.

Important current cross-cutting references include:

- `001-token-source-and-artifact-contract.md`
- `002-component-contract-template-and-parity-rules.md`
- `003-accessibility-and-assistive-technology-baseline.md`
- `008-parity-evidence-documented-delta-and-downstream-extension-rules.md`
- `021-public-package-api-stability-and-parity-debt-baseline.md`
- `022-packaging-versioning-and-release-channel-rules.md`
- `024-token-evolution-migration-and-compatibility-policy.md`
- `044-deprecation-change-control-and-release-channel-operations.md`
- `062-headless-core-and-dual-layer-strategy.md`
- `063-rust-authored-component-and-scene-ir.md` — retired g13 pilot record;
  its architecture 009/spec 066 successor was also rejected
- `064-cross-runtime-machine-pinning.md` — retired g14 false-start record;
  its `066` successor was also rejected
- `065-scene-authoring-and-specimen-fixtures.md` — retired scene-fixture
  contract; human-centred specimen work continues under the g15
  specimen-catalogue lane (carried forward from roadmap `g14.026`)
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
