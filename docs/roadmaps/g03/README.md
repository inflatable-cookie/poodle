# g03 Hardening, Migration, And Mature Ecosystem Support

Status: planned
Updated: 2026-03-11

## Context

`g03` is the hardening generation. By this point Pug should already have a
credible token system, component catalogue, workstation-shell layer, docs
surface, and first downstream adoption tranches. The work here is to make the
system stable enough to carry real long-term consumers.

## Starting State

- `g02` has expanded the reusable component suite
- Underlay and at least one GPUI downstream adoption path are real
- packaging and versioning baselines exist
- docs and examples are already present

## Exit State

- migration and compatibility policy are explicit
- parity automation and regression evidence exist
- docs publishing and contract linting are reliable
- Underlay and Loophole-facing extension contracts are hardened
- the system can support broader ecosystem adoption without reopening its core
  shape

## Milestone Status

| ID | Milestone | Depends On | Class | Status |
|----|-----------|------------|-------|--------|
| 001 | Token evolution, migration, and compatibility policy | g02.016 | Foundation | Planned |
| 002 | Parity automation and screenshot or interaction harnesses | g02.016 | Hardening | Planned |
| 003 | Contract linting, docs completeness, and publish pipeline | g02.016 | Hardening | Planned |
| 004 | Performance, render-cost, and memory profile hardening | g02.016 | Hardening | Planned |
| 005 | Theming, branding, and downstream override strategy | 001 | Depth | Planned |
| 006 | Extension SDK, composition guidance, and starter packages | 001, 003 | Depth | Planned |
| 007 | Underlay bridge hardening and zero-leak adoption proof | 001-006 | Adoption | Planned |
| 008 | Loophole foundation adoption and DAW-extension contract proof | 001-006 | Adoption | Planned |
| 009 | Additional GPUI app adoption and multi-app validation | 001-006 | Adoption | Planned |
| 010 | Accessibility audit and assistive-technology conformance | 002-005 | Hardening | Planned |
| 011 | Deprecation, change control, and release-channel operations | 001-006 | Operations | Planned |
| 012 | Ecosystem acceptance suites and long-tail regression coverage | 002-011 | Hardening | Planned |
| 013 | Reference apps, onboarding depth, and public-facing examples | 003-012 | Adoption | Planned |
| 014 | Generation closeout and next-program cutover | 012, 013 | Closure | Planned |

## Next Task

Keep `g03` as a hardening and maturity generation; do not defer the first
version of foundational component work into it.
