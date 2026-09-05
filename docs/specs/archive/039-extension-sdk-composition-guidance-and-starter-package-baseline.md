# 039 Extension SDK, Composition Guidance, And Starter Package Baseline

Status: active
Updated: 2026-03-12
Depends on: `../008-parity-evidence-documented-delta-and-downstream-extension-rules.md`, `021-public-package-api-stability-and-parity-debt-baseline.md`, `../026-appearance-recipes-and-downstream-override-strategy.md`, `038-performance-render-cost-and-memory-hardening-baseline.md`

## Purpose

Freeze what Poodle means by an extension-facing SDK in `g03`, how downstream apps
should compose safely from Poodle without forking canonical meaning, and what a
starter package may promise before broader adoption milestones are complete.

This baseline exists so Underlay-facing, Loophole-facing, or future app-facing
consumers do not guess their own package shape, wrapper policy, or extension
lane from incomplete examples.

## Core Rule

In `g03`, the extension SDK is primarily a documented composition contract, not
an expansive new runtime framework.

The current SDK surface is the combination of:

- canonical tokens and emitted artifacts
- contract-backed Svelte package APIs
- bridge-owned adapters where direct Poodle exposure would leak
- public docs describing safe composition, branding, and extension boundaries

Poodle should not invent a second abstraction layer above its own contracts just
to sound more “SDK-like”.

## Current Extension SDK Surface

The current extension-facing surface is:

- `@inflatable-cookie/poodle-core/tokens`
- `@inflatable-cookie/poodle-svelte`
- `@inflatable-cookie/poodle-svelte-workstation`
- consumer-owned adapter packages, which live in the consumer's repository
- contract docs under `docs/contracts/`
- normative specs under `docs/specs/`

This surface is sufficient for `g03` when combined with explicit composition
guidance. It is not yet a promise of generators, CLIs, or framework-specific
starter scaffolds.

## Composition Rule

Downstream apps should compose upward through the Poodle layers:

1. semantic tokens and emitted artifacts
2. foundation primitives
3. reusable composites
4. workstation-shell surfaces where relevant
5. app-owned wrappers, orchestration, and domain-specific workflows

They should not skip directly from tokens to app-specific end-state surfaces if
that bypasses established contracts that Poodle already owns.

## Wrapper And Adapter Rule

When a downstream app needs local naming, routing, host integration, analytics,
or compatibility preservation, the correct answer is an app-owned wrapper or a
bridge-owned adapter.

Use wrappers or adapters for:

- preserving an existing app API while adopting Poodle internals
- host-specific command registration or persistence wiring
- app-specific data loading, mutation, or synchronization policy
- branding structures or workflow shells above the canonical component layers
- Underlay aliasing that must keep Poodle out of app-facing imports

Do not use wrappers or adapters to:

- redefine canonical prop meaning
- smuggle undocumented deltas in as stable extension points
- fork token meaning under the same names

## Starter Package Rule

A Poodle starter package in `g03` is a reference composition baseline, not a
product template.

It may provide:

- package wiring and dependency posture
- token import and theme-application examples
- recommended wrapper structure for app-owned surfaces
- example docs, preview, or contract references
- bridge-entry examples where direct Poodle exposure is not desired

It must not imply:

- that app-specific workflow components belong in Poodle core
- that one starter shape is mandatory for every consumer
- that web-only starter structures prove GPUI parity
- that a starter can bypass the documented extension or bridge boundaries

## Starter Package Shapes

The current allowed starter shapes are:

### Direct Svelte Consumer

Use when the downstream app may import Poodle directly.

Expected shape:

- app imports Poodle tokens and Svelte packages directly
- app owns route structure, data loading, orchestration, and wrappers
- app branding uses recipe variables or wrapper-level styling

### Bridge-Mediated Consumer

Use when the downstream app should stay Poodle-agnostic.

Expected shape:

- bridge package imports Poodle and exposes app-local aliases
- app imports only bridge-local APIs
- bridge owns alias maps, wrapper preservation, and rollout guidance

### Workstation-Oriented Consumer

Use when an app adopts workstation shell or dock surfaces.

Expected shape:

- app composes workstation surfaces from canonical Poodle packages
- app owns command registries, layout persistence, panel policy, and workflow
  semantics
- app-specific DAW or domain widgets remain app-owned above the shared shell

## Docs Obligation Rule

Any extension-facing starter or wrapper guidance must point back to:

- the canonical contract or package it composes
- the bridge boundary if the app should stay Poodle-agnostic
- the public styling or recipe rules if it restyles the surface
- the known-delta and parity policy if it diverges intentionally

This keeps starter examples from becoming shadow contracts.

## Safe Starter Evidence

Before a starter or extension example should be treated as recommended, the repo
should be able to answer:

- which public Poodle packages it consumes
- whether the app imports Poodle directly or through a bridge
- which layer owns branding and wrapper structure
- which layer owns host integration concerns
- which parts are shared guidance versus app-specific example code

If those answers are unclear, the starter is not ready to be treated as a
recommended baseline.

## Non-Goals For `g03.006`

This milestone does not require:

- a CLI generator
- framework-specific scaffolding for every target
- automatic wrapper code generation
- an Underlay production adoption proof
- a Loophole production adoption proof

Those belong to later adoption milestones.

## Seed Baseline

The current `g03.006` seed baseline is:

- this normative spec
- `docs/specs/008-parity-evidence-documented-delta-and-downstream-extension-rules.md`
- `docs/specs/026-appearance-recipes-and-downstream-override-strategy.md`
- `docs/roadmaps/g03/006-extension-sdk-composition-guidance-and-starter-packages.md`
