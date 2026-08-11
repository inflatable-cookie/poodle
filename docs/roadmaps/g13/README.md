# g13 — Rust-authored Cross-runtime IR

Status: active
Opened: 2026-08-11
Governing spec: `docs/specs/063-rust-authored-component-and-scene-ir.md`

## Goal

Replace four drifting component and specimen definitions with one constrained,
Rust-authored source of truth. Generate typed web inputs and evidence; consume
the same definitions directly through the shared native renderer path.

## Sequencing Rule

`001`–`008` are the runway. They establish the schema, compiler, preview shell,
and three proof components, then force an adopt/revise/reject decision.

`009`–`016` are gated on an **adopt** verdict in `008`. Do not begin family
migration early. Existing contracts and implementations remain authoritative
until their migration card closes.

## Runway

1. [001 — Authority inventory and fixture baseline](001-authority-inventory-and-fixture-baseline.md)
2. [002 — Rust IR schema and validation core](002-rust-ir-schema-and-validation-core.md)
3. [003 — Deterministic codegen and drift gate](003-deterministic-codegen-and-drift-gate.md)
4. [004 — Shared preview shell scene pilot](004-shared-preview-shell-scene-pilot.md)
5. [005 — Button component vertical slice](005-button-component-vertical-slice.md)
6. [006 — RangeSlider stateful-control proof](006-range-slider-stateful-control-proof.md)
7. [007 — TextInput environment-boundary proof](007-text-input-environment-boundary-proof.md)
8. [008 — Pilot verdict and architecture promotion](008-pilot-verdict-and-architecture-promotion.md)

## Rollout Suite

9. [009 — Shared specimen scene migration](009-shared-specimen-scene-migration.md)
10. [010 — Foundation and display component migration](010-foundation-and-display-component-migration.md)
11. [011 — Controls, forms, and audio component migration](011-controls-forms-and-audio-component-migration.md)
12. [012 — Overlay and navigation component migration](012-overlay-and-navigation-component-migration.md)
13. [013 — Data, composite, and workstation migration](013-data-composite-and-workstation-migration.md)
14. [014 — Native backend convergence](014-native-backend-convergence.md)
15. [015 — Generated contracts, registries, and parity evidence](015-generated-contracts-registries-and-parity-evidence.md)
16. [016 — Consumer proof and generation closeout](016-consumer-proof-and-generation-closeout.md)

## Non-goals

- compiling arbitrary Rust functions to TypeScript
- replacing runtime-native focus, IME, portals, measurement, or text systems
- making `poodle-node` the web authoring model
- changing component semantics during mechanical migration
- adding new frameworks before the four current targets converge

## First Task

`g13.001` freezes the authority map, measures current duplication, and captures
the fixtures the pilot must preserve.
