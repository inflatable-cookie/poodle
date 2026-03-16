# g10.013 — Delta Register Update and Native Adaptation Documentation

Status: complete
Owner: Pug Core
Updated: 2026-03-16
Depends on: g10.012
Primary repos: `pug`

## Goals

- [x] update the intentional delta register with all Jetstream-specific
  adaptations discovered during the visual parity audit
- [x] provide comprehensive documentation for downstream consumers

## Execution Checklist

- [x] review all delta notes from g10.012 visual audit
- [x] for each delta, produce a register entry with:
  - [x] component name and slug
  - [x] delta type: visual, behavioral, simplified, or excluded
  - [x] description of the difference
  - [x] Jetstream constraint reference (e.g., "no gradients", "single shadow",
    "no transforms")
  - [x] severity: cosmetic, functional, or omission
- [x] classify components into parity tiers:
  - [x] **Full parity** — 109 components (93.2%)
  - [x] **Partial parity** — 8 components (6.8%)
  - [x] **Intentional skip** — 0 components
- [x] produce summary statistics: full/partial/skip counts per tier
- [x] update `delta-register.md` in `docs/roadmaps/g10/`
- [x] cross-reference with g08.011 delta register and note any changes

## Acceptance Criteria

- [x] every Jetstream adaptation is documented in the register
- [x] each delta has a constraint reference and severity classification
- [x] parity tier counts are accurate
- [x] delta register is consistent with g08.011 (changes are explained)

## Next Task

Open `g10.014` and refresh the cross-runtime parity report.
