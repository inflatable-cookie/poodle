# Pug GPUI Tokens

Generated Rust token surface for GPUI consumers.

## Public Surface

- crate: `pug-gpui-tokens`
- modules: `semantic`, `primitives`, `themes`, `density`, `metadata`

## Stability Notes

- this crate currently exposes generated token artifacts only
- GPUI primitives, composites, workstation shells, and parity harnesses remain
  future work and should not be inferred from the presence of token bindings
- downstream adoption should treat this crate as a token baseline, not as proof
  that wider GPUI package ergonomics are already settled

## Current Multi-App Validation Baseline

The current GPUI multi-app validation matrix lives in:

- `packages/gpui/tokens/multi-app-validation.json`

It does not claim shipped GPUI component crates. It records:

- the current GPUI target app shapes
- the shared-layer assumptions those targets should expose
- the required follow-up work
- the blockers still preventing stronger GPUI adoption claims

## Next Task

Use this crate surface while executing the later GPUI validation and adoption
tranches, without overstating token-only readiness as wider GPUI package
completeness.
