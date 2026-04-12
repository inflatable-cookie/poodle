# g10.007 GPUI Long-Tail Component Parity Sweep And Closeout Checkpoint

Status: complete
Owner: Poodle core
Depends on: g10.006
Updated: 2026-04-12

## Context

The current GPUI parity lane already closed a large amount of shell, registry,
token, and specimen drift. After `g10.005` and `g10.006`, the remaining work
should be treated as a deliberate long-tail sweep instead of more ad-hoc review
threads.

This milestone turns the remaining component-level gaps into one bounded sweep,
refreshes explicit parity evidence, and leaves a clean checkpoint for deciding
whether GPUI parity should continue immediately or yield to the Jetstream lane.

## Governing Refs

- `docs/specs/002-component-contract-template-and-parity-rules.md`
- `docs/specs/008-parity-evidence-documented-delta-and-downstream-extension-rules.md`
- `docs/specs/019-advanced-catalog-accessibility-focus-keyboard-and-state-rules.md`
- `docs/specs/020-docs-site-example-and-component-discoverability-rules.md`
- `docs/specs/058-cross-runtime-parity-report-delta-register-and-acceptance-harness-expansion.md`
- `docs/contracts/components/`

## Goals

- run a findings-first sweep across the full unified component registry
- upgrade thin or misleading GPUI specimens in meaningful clusters
- close the highest-signal remaining behavior, wording, and page-framing gaps
- leave an explicit closeout checkpoint and residual-gap list instead of
  another fuzzy continuation

## Non-Goals

- reopening completed shell work without a concrete finding
- Jetstream implementation depth
- large contract redesigns unrelated to parity defects

## Execution Plan

### Batch 7.1 - Registry-Wide Findings Sweep

- [x] audit the full GPUI registry against the current Svelte preview surface
- [x] classify remaining gaps by type:
      specimen depth, page shape, docs behavior, shell behavior, or accepted
      runtime delta
- [x] freeze one explicit fix list before coding starts

### Batch 7.2 - Clustered Fix Batches

- [x] execute the remaining parity fixes in grouped component clusters rather
      than one-off micro-patches
- [x] keep any honest GPUI-native delta explicit when exact visual parity is
      not the right target
- [x] update any stale registry or preview copy uncovered during the sweep

### Batch 7.3 - Evidence Refresh And Checkpoint

- [x] refresh parity evidence surfaces affected by the sweep
- [x] leave a residual-gap register for anything intentionally deferred
- [x] decide whether the next active lane is more GPUI parity or the Jetstream
      implementation seam
- [x] validate with `cargo check --manifest-path packages/gpui/preview/Cargo.toml`
      and `git diff --check`

## Outcome

`g10.007` closed the long-tail GPUI parity sweep. Shell drift, catalogue drift,
token-page drift, component-page drift, and large specimen-cluster drift were
reduced in grouped batches instead of one-off churn. The GPUI preview now reads
like the current Svelte preview in the areas that matter most for review.

## Accepted Native Deltas

- GPUI still renders its own docs UI layer even though the docs data source is
  now shared with Svelte.
- `HoverCard` still uses a GPUI-native delayed preview-state bridge instead of
  fully component-owned hover state.
- `Popover`, `Dialog`, and `Drawer` still receive explicit preview-owned `open`
  state, even though their callback shape and specimen sync are now much closer
  to Svelte.

These are now treated as conscious native deltas, not hidden parity misses.

## Exit Criteria

- the remaining GPUI parity queue is explicit and materially smaller
- thin or misleading specimens are reduced through grouped fixes
- residual gaps are documented as conscious debt or accepted delta
- the post-parity planning checkpoint is explicit

## Next Task

GPUI preview parity is at closeout checkpoint. Next active lane: return to
Jetstream implementation work in `g10`, unless a new parity defect appears with
fresh evidence.
