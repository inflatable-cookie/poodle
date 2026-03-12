# g03 Hardening, Migration, And Mature Ecosystem Support

Status: active
Updated: 2026-03-11

## Context

`g03` is the hardening and first-adoption generation. By this point Pug should
already have a credible token system, component catalogue, workstation-shell
layer, docs surface, packaging baseline, and a much cleaner review surface than
the rougher `g02` build-out phase. The work here is to start real downstream
adoption without reopening the core shape every few days.

## Starting State

- `g02` has expanded the reusable component suite
- downstream adoption is ready to begin deliberately, not by optimism
- packaging and versioning baselines exist
- docs and examples are already present

## Exit State

- migration and compatibility policy are explicit
- parity automation and regression evidence exist
- docs publishing and contract linting are reliable
- first Underlay and Loophole-facing adoption tranches are real and hardened
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

Open `g03.001` and freeze token evolution, migration, and compatibility policy
before downstream adoption begins.
