# Product Guardrails

Status: active
Updated: 2026-08-09
Owner: Poodle core

- Keep Poodle focused on reusable tokens, primitives, composites, workstation
  shells, and cross-runtime contracts.
- Keep product-specific DAW, admin, and Jetstream-local behavior in its owning
  application or runtime.
- Treat contracts as behavioral authority. Preview specimens and screenshots
  are evidence, not proof by themselves.
- Change observable behavior through the shared contract before changing one
  renderer.
- Put shared web behavior and styling in `poodle-core`; put shared native
  component recipes in `poodle-render`.
- Keep Underlay-facing integration behind Underlay-owned adapters and token
  bridges.
- Keep Bits Svelte and runtime engines as implementation details rather than
  public Poodle contracts.
- Freeze a bounded owner before widening a migration or parity tranche.
- Record intentional runtime differences and validate them against the contract.
