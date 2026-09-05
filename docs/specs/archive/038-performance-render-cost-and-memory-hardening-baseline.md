# 038 Performance, Render-Cost, And Memory Hardening Baseline

Status: active
Updated: 2026-03-12
Depends on: `021-public-package-api-stability-and-parity-debt-baseline.md`, `../025-parity-automation-and-harness-boundary.md`, `027-docs-completeness-contract-linting-and-publish-pipeline.md`

## Purpose

Freeze the minimum performance posture Poodle should enforce during the `g03`
adoption generation, without pretending that the current repo has a full
benchmark harness or cross-runtime profiler.

This baseline exists to stop avoidable render-cost churn, make likely hotspots
explicit, and separate Svelte hardening priorities from GPUI hardening
priorities while both implementations are still maturing.

## Core Rule

Poodle should remove avoidable work first, then document the remaining expensive
areas honestly.

The current generation is not required to publish exhaustive benchmark tables.
It is required to:

- eliminate obvious repeated work in the review harness and shared surfaces
- keep expensive demo-only behavior from silently becoming production guidance
- name the highest-risk render and memory hotspots explicitly
- separate Svelte hardening work from GPUI hardening work where the runtimes
  differ materially

## Shared Expectations

These expectations now apply across the shared component suite and the preview
or docs harness:

- static demo data should be indexed once where repeated filtering or search is
  expected
- state changes should not trigger repeated full-surface recomputation when the
  changed value only affects one local slice
- route synchronization should avoid redundant history writes and redundant
  token re-reads
- token inspection should read live computed values only when theme, density,
  or control-size state actually changes
- serialization or round-trip demo checks should stay scoped to the surface
  proving that behavior rather than leaking into unrelated sections
- preview-only shells may be heavier than production usage, but they must not
  hide clearly avoidable work

## Current Svelte Hotspots

The current Svelte hardening baseline recognizes these likely hotspots:

- preview-route state changes that trigger theme attribute writes and full
  semantic token reads
- search and filter demos that repeatedly rebuild lowercase haystacks from
  static data on every keystroke
- grouped command discovery that rescans the same filtered action list multiple
  times for ranking and section assembly
- large monolithic preview modules where unrelated reactive declarations can
  still be reevaluated during interactive review
- dense workstation-shell examples that keep many panels, overlays, and derived
  layout summaries alive at once

## Current GPUI Hotspots

The current GPUI hardening baseline should assume different hotspot classes:

- retained native scene and view-tree growth from shell and dock orchestration
- repeated relayout or redraw caused by split, dock, and tab interactions
- allocation churn from transient command, picker, and notification surfaces
- text measurement and list or table updates under frequent state changes
- host integration boundaries where shell state and app-owned state may bounce
  excessively

These are priority targets for later GPUI evidence. They are not yet claimed as
automated or benchmarked in this repo.

## Current `g03.004` Seed Hardening

The current seed hardening in the Svelte preview now includes:

- static search indexing for the table, browse, picker, and command demo data
- single-pass command section assembly from ranked command results
- deduped preview token refresh keyed to theme, density, and control-size state
- synchronous token reads immediately after theme-attribute updates instead of
  unnecessary async review-harness churn
- static section and catalog lookup structures instead of repeated reactive
  rebuilding

The current seed implementation lives in:

- `packages/svelte/preview/src/App.svelte`

## Priority Model

Priority order for the remainder of `g03` is:

1. remove avoidable repeated work in the Svelte harness and shared examples
2. keep package-surface and docs evidence aligned with actual hardening status
3. collect runtime-specific hotspot evidence for GPUI without forcing GPUI
   implementation work into Svelte-only batches
4. add deeper measurement or benchmark tooling only when the current baseline
   can use it honestly

## Honesty Rule

Poodle may say:

- a likely hotspot was identified
- avoidable repeated work was removed
- a runtime-specific hardening priority was recorded

Poodle may not say:

- the suite is benchmarked comprehensively
- GPUI render cost is proven acceptable without GPUI evidence
- preview-build success proves runtime performance quality
