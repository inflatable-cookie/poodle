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

## Next Task

Use this crate surface while executing `g02.016`, confirming how token-level
Rust artifacts should be represented in the first downstream-adoption
generation without overstating wider GPUI readiness.
