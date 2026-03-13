# Pug GPUI Tokens

Generated Rust token surface for GPUI consumers.

## Public Surface

- crate: `pug-gpui-tokens`
- modules: `semantic`, `primitives`, `themes`, `density`, `metadata`

## Stability Notes

- this crate currently exposes generated token artifacts only
- GPUI primitives now begin in `pug-gpui-primitives`, with structural,
  action, field, text-entry, selection, feedback, date-time, overlay,
  disclosure, navigation, and menu baselines in place, and GPUI composites now
  begin in `pug-gpui-composites` with form, validation, remediation, data,
  browse, detail, picker, and media baselines, and GPUI workstation now begins
  in `pug-gpui-workstation` with shell, command, and layout baselines, and
  `g04.010` now makes the current native accessibility-proof posture explicit,
  but downstream-adoption depth still remains future work and should not be
  inferred from the presence of token bindings alone
- downstream adoption should treat this crate as a token baseline, not as proof
  that wider GPUI package ergonomics are already settled

## Current Multi-App Validation Baseline

The current GPUI multi-app validation matrix lives in:

- `packages/gpui/tokens/multi-app-validation.json`
- `packages/gpui/parity-priority-matrix.json`
- `packages/gpui/preview-app-baseline.json`
- `packages/gpui/cross-runtime-parity-report.json`
- `packages/gpui/native-accessibility-proof.json`

It does not claim shipped GPUI component crates. It records:

- the current GPUI target app shapes
- the current GPUI implementation order and parity posture by review section
- the first native preview-app and theme-runtime review baseline
- the first cross-runtime parity report and intentional delta register
- the first explicit GPUI native accessibility proof posture
- the first GPUI structural primitive crate baseline
- the shared-layer assumptions those targets should expose
- the required follow-up work
- the blockers still preventing stronger GPUI adoption claims

## Next Task

Use this crate together with `pug-gpui-primitives` and
`pug-gpui-composites` and `pug-gpui-workstation` while executing `g04.012`
and later GPUI implementation tranches, without overstating token-only
readiness, artifact-backed parity evidence, or spec-level accessibility
posture as full GPUI runtime completeness.
