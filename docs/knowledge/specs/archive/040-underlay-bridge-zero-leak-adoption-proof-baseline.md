# 040 Underlay Bridge Zero-Leak Adoption Proof Baseline

Status: superseded
Updated: 2026-08-21
Superseded by: `../../architecture/001-poodle-system-shape.md#application-boundary`

> **Superseded.** `packages/bridges/underlay` has been removed from Poodle. A
> design system must not carry a package named after one of its consumers, and
> the bridge's only real caller was a single Nightfire import in Underlay.
> Underlay now consumes Poodle's published packages directly and owns any
> translation in its own source. Retained for the reasoning; not current
> guidance.
Depends on: `004-underlay-bridge-and-adapter-ownership.md`, `../008-parity-evidence-documented-delta-and-downstream-extension-rules.md`, `039-extension-sdk-composition-guidance-and-starter-package-baseline.md`

## Purpose

Freeze what counts as a zero-leak Underlay adoption proof in `g03` and make
the bridge hardening posture explicit before larger Underlay rollout work.

## Core Rule

Underlay may adopt Poodle internally only through bridge-owned aliases, wrappers,
and migration helpers.

An adoption proof is zero-leak only when Underlay app code can consume the
surface without importing Poodle directly or depending on Poodle naming.

## Zero-Leak Proof Rule

The bridge must provide a machine-readable proof artifact that states:

- the package remains bridge-owned rather than app-facing
- direct Poodle imports are not allowed in Underlay app code
- the current wrapper-backed adoption surfaces
- the bridge-local token and theme indirection posture
- the remaining adoption friction still blocking wider rollout

README prose alone is not sufficient once `g03` begins real adoption work.

## Current Proof Artifact

The current proof artifact is:

- `packages/bridges/underlay/ts/zero-leak-proof.ts`

It should remain narrow and declarative in this generation.

## Required Proof Properties

Every zero-leak proof baseline must make these claims explicit:

- app-facing imports stay Underlay-owned
- wrapper-backed surfaces may use Poodle internally
- bridge-owned token aliases remain implementation detail
- bridge-owned theme IDs and mode IDs remain mapping artifacts, not canonical renames
- remaining migration or accessibility friction is recorded rather than implied

## Bridge Hardening Rule

The bridge should be hardened by:

- tightening alias and wrapper ownership
- documenting the current proof surfaces
- validating that bridge artifacts are non-empty and internally coherent
- keeping the bridge package narrower than a second component library

The bridge should not be hardened by:

- exposing Poodle package names as recommended app imports
- widening the alias map speculatively without adoption pressure
- inventing app-facing abstractions that Underlay should own itself

## Current `g03.007` Seed Proof

The current seed proof covers:

- token alias mapping from emitted Poodle artifacts
- theme and mode mapping from emitted Poodle themes
- wrapper-preservation policies for initial Underlay-facing surfaces
- a zero-leak proof manifest with explicit remaining friction

The current seed implementation lives in:

- `packages/bridges/underlay/ts/token-map.ts`
- `packages/bridges/underlay/ts/theme-map.ts`
- `packages/bridges/underlay/ts/component-wrappers.ts`
- `packages/bridges/underlay/ts/zero-leak-proof.ts`

## Remaining Friction Rule

The zero-leak proof must still record unresolved adoption pressure when it is
known.

The current expected friction classes are:

- Underlay-owned canonical naming for theme registration
- wrapper prop translation against real existing Underlay APIs
- accessibility verification once wrappers are used in downstream repos
- pressure to widen bridge aliases beyond the current narrow baseline

## Honesty Rule

Poodle may say:

- the bridge package exposes a zero-leak proof artifact
- wrapper-backed adoption surfaces are identified
- current remaining friction is explicit

Poodle may not say:

- Underlay production rollout is complete
- zero accessibility risk remains without downstream wrapper evidence
- Underlay apps can adopt Poodle directly
