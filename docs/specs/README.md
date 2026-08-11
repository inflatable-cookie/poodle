# Specs

Status: active
Updated: 2026-08-11

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
- `063-rust-authored-component-and-scene-ir.md` — provisional g13 authority
  for the Rust-authored cross-runtime IR pilot

Specs are normative constraints, not task queues. Current execution status
belongs in the roadmap.
