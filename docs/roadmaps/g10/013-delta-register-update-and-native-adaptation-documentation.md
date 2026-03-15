# g10.013 — Delta Register Update and Native Adaptation Documentation

Status: planned
Owner: Pug Core
Updated: 2026-03-15
Depends on: g10.012
Primary repos: `pug`

## Goals

- [ ] update the intentional delta register with all Jetstream-specific
  adaptations discovered during the visual parity audit
- [ ] provide comprehensive documentation for downstream consumers

## Execution Checklist

- [ ] review all delta notes from g10.012 visual audit
- [ ] for each delta, produce a register entry with:
  - [ ] component name and slug
  - [ ] delta type: visual, behavioral, simplified, or excluded
  - [ ] description of the difference
  - [ ] Jetstream constraint reference (e.g., "no gradients", "single shadow",
    "no transforms")
  - [ ] severity: cosmetic, functional, or omission
- [ ] classify components into parity tiers:
  - [ ] **Full parity** — matches Svelte/GPUI within tolerance
  - [ ] **Partial parity** — reduced functionality documented
  - [ ] **Intentional skip** — not appropriate for game engine context
- [ ] produce summary statistics: full/partial/skip counts per tier
- [ ] update `delta-register.md` in `docs/roadmaps/g10/`
- [ ] cross-reference with g08.011 delta register and note any changes

## Acceptance Criteria

- [ ] every Jetstream adaptation is documented in the register
- [ ] each delta has a constraint reference and severity classification
- [ ] parity tier counts are accurate
- [ ] delta register is consistent with g08.011 (changes are explained)

## Next Task

Open `g10.014` and refresh the cross-runtime parity report.
