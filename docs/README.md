# Poodle Documentation

Use this page to find the shortest path from your question to the authoritative
answer. You do not need to read the roadmap or project logs to use Poodle.

## Adopt Poodle

Choose the runtime used by your application:

- [Svelte developer guide](guides/svelte-developer-guide.md) — package setup,
  themes, icons, components, and application integration
- [React package guide](../packages/react/components/README.md) — current React
  surface and experimental-package constraints
- [GPUI developer guide](guides/gpui-developer-guide.md) — Rust contracts,
  themes, node rendering, and the GPUI backend
- [Jetstream developer guide](guides/jetstream-developer-guide.md) — deferred
  paired integration for Rust contracts, node rendering, and conversion
- [Application pattern recipes](guides/README.md) — forms, lists, dialogs,
  media workflows, and admin shells

Poodle is pre-1.0. `@inflatable-cookie/poodle-core` and
`@inflatable-cookie/poodle-svelte` publish to npm on the preview channel;
`@inflatable-cookie/poodle-react` is packed and certified but stays
source-only; the Rust crates are source/tag distribution. Preview means
pre-release: breaking changes may ship in `0.x` minor releases and no `stable`
channel exists yet. See the [release notes](release-notes/README.md).

## Understand the System

- [System shape](architecture/001-poodle-system-shape.md) explains contracts,
  renderer boundaries, parity, and application ownership.
- [Tokens and package layout](architecture/002-token-system-and-package-layout.md)
  explains the token build, published surfaces, themes, and runtime packages.
- [Component documentation structure](architecture/003-component-docs-ia-and-implementation-substrates.md)
  explains how contracts and implementation evidence relate.

## Look Up Component Behavior

[Component contracts](contracts/components/README.md) are the source of truth
for public inputs, states, events, accessibility, layout, and token usage. Use
them when integrating a component, comparing runtimes, or proposing an API
change.

Contracts describe observable behavior. Framework and engine implementation
details may differ when the contract permits it.

## Contribute

- [Working rules](contracts/001-working-rules.md) define cross-runtime and
  contract-first expectations.
- [Architecture index](architecture/README.md) identifies structural authority.
- [Specs index](specs/README.md) collects repository-wide normative rules and
  generated baselines.
- [Roadmaps](roadmaps/README.md) contain the active generation and Northstar tasks.

Run documentation locally with:

```sh
bun install
effigy docs:dev
```

`docs:dev` is `svelte:preview`: it rebuilds package distributions before Vite.
Use `svelte:run` only when you already trust the ignored dist trees.

Validate documentation changes with:

```sh
effigy docs:check
```

## Project Record

The remaining sections preserve decision and delivery context:

- `vision/` — long-range intent and scope
- `roadmaps/` — generation runway, Northstar tasks, and compact roll-ups
- `evidence/` — current generated or validated evidence that spans generations
- `logs/` — completed work and validation evidence
- `research/` — external comparisons and early investigation
- `handoffs/` — point-in-time continuation briefs. Retention rule: a handoff
  is archived when its lane closes. Closed-lane briefs move to
  `handoffs/archive/YYYY-MM/` (by the month in the filename) and stay there;
  briefs of open lanes remain at the top level until their lane closes.
- `triage/` — open observations and operator decisions awaiting promotion;
  promoted, superseded, or executed notes are removed
- `archive/` — retired directories kept for provenance; current parity is
  generated from contracts, runtime reports, tests, and previews

These records are useful to contributors, but they are not part of the operator
learning path and may describe superseded implementation states.
<!-- northstar:lifecycle:begin schema=northstar.lifecycle.projection.v2 digest=sha256:42a285f9c13209058b319db2eaaca8e7c64bab8071482df9c4ab16cd814fcb8e -->
| Generation | Disposition | Runway state |
| --- | --- | --- |
| g18 | open | planning_required |
| Task | Status | Stage | Revision | Record digest |
| --- | --- | --- | --- | --- |
| g18.034 | complete | none | 8 | sha256:bd6863d116045dec331e616c30985260c441d1ea02735bcfb4bdbdb275f1c106 |
| g18.035 | complete | none | 8 | sha256:daefcba434d2d1941bb670de98b3faf47d988cb82732c6f2f6a848e4f38614e3 |
| g18.036 | complete | none | 8 | sha256:b4f79112ff283c2cf7b3c32214cdeeaab547f0766609dde3d2a9a4f67a5649d3 |
| g18.037 | complete | none | 8 | sha256:f85ff1b17dcdc8ddb826ba4dea3118c3617f8e14d694a18665602730471de89a |
| g18.038 | complete | none | 8 | sha256:9979bf9354226ec68d1f962e9681be65a3c7bab0d1252135a827dd40ab31d08a |
| g18.039 | complete | none | 8 | sha256:551d8a4f7d02019f4dad4934f4d30ef302264c8a9130041f35ac45c204827d06 |
| g18.040 | complete | none | 8 | sha256:14ad8a28202d68ff74e8f5391a2d30c5e0fdf1108ca7f0115e4c8c61930bc085 |
<!-- northstar:lifecycle:end -->
