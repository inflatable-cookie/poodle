# 041 Loophole Foundation Adoption And DAW Extension Boundary

Status: active
Updated: 2026-03-12
Depends on: `../008-parity-evidence-documented-delta-and-downstream-extension-rules.md`, `017-app-shell-and-workspace-shell-depth-rules.md`, `039-extension-sdk-composition-guidance-and-starter-package-baseline.md`, `040-underlay-bridge-zero-leak-adoption-proof-baseline.md`

## Purpose

Freeze what counts as a Loophole-facing foundation adoption proof in `g03` and
make the DAW-extension boundary explicit before later app-adoption work widens.

## Core Rule

Loophole may consume Poodle directly as a downstream foundation consumer, but it
must do so by building upward from canonical tokens, primitives, composites,
and workstation shells.

Loophole-specific DAW widgets remain Loophole-owned even when they sit on top
of Poodle foundations.

## Adoption Rule

The current allowed Loophole adoption layers are:

- `@inflatable-cookie/poodle-core/tokens`
- `@inflatable-cookie/poodle-svelte`
- `@inflatable-cookie/poodle-svelte-workstation`

These packages are sufficient for a first foundation adoption proof when the
downstream app still owns:

- workflow semantics
- persistence backends
- command registries
- DAW interaction policy
- domain-specific shell structures above the generic workstation layer

## DAW Boundary Rule

Poodle may own generic workstation shell and panel-system surfaces.

Poodle may not own Loophole-specific DAW surfaces such as:

- transport bars
- timelines
- mixer strips
- automation lanes
- plugin racks
- clip editors
- session rulers
- meter-bridge or console-specific widgets

Those remain downstream-owned even when they compose Poodle buttons, tabs, panel
surfaces, split views, and command discovery.

## Proof Artifact Rule

The Loophole foundation adoption proof must exist as a machine-readable
artifact that records:

- the allowed Poodle package surface
- the currently approved workstation exports
- the DAW surfaces that remain downstream-owned
- the current boundary rules
- the remaining adoption friction

## Current Proof Artifact

The current proof artifact is:

- `packages/svelte/workstation/loophole-foundation-proof.json`

## Workstation Rule

The workstation package may serve Loophole as shared shell foundation for:

- app and project headers
- workspace shells
- panel surfaces and panel headers
- dock regions and split views
- surface and panel tabs
- shell status bars
- command palette and action discovery posture
- layout snapshot serialization helpers

This does not turn the workstation package into a DAW kit.

## Remaining Friction Rule

The current adoption proof must still record:

- GPUI parity as an open blocker for broader cross-runtime Loophole rollout
- the need for downstream wrapper evidence around generic versus DAW-owned surfaces
- the need to prove Loophole-specific branding without redefining canonical token meaning

## Honesty Rule

Poodle may say:

- Loophole can adopt Poodle foundations directly
- the DAW-extension boundary is explicit
- the shared workstation layer is suitable as generic shell foundation

Poodle may not say:

- Loophole production adoption is complete
- DAW widgets now belong in Poodle core
- workstation-shell parity alone proves all Loophole runtime concerns are solved

## Seed Baseline

The current `g03.008` seed baseline is:

- this normative spec
- `packages/svelte/workstation/loophole-foundation-proof.json`
- `packages/svelte/workstation/README.md`
- `docs/contracts/workstation/README.md`
- `docs/roadmaps/g03/008-loophole-foundation-adoption-and-daw-extension-contract-proof.md`
