# Web Parity Gap Closeout

Date: 2026-08-10

## Scope

Reconcile component contracts that still claimed a missing web implementation
after the native renderer consolidation. Separate intentional composition from
real public-surface gaps.

## Findings

- `FormShell` remains a caller-composed Rust form spec, not a standalone Svelte
  component.
- `TabStrip` remains embedded in the public `Tabs` surface.
- Web inline remediation intentionally composes `Callout`; its contract header
  incorrectly described that choice as pending work.
- `ValidationSummary`, `StateTile`, and `RemediationBanner` were real Svelte and
  React gaps.
- `docs/parity/` described the deleted GPUI and Jetstream component-package
  tiers. Its hand-maintained matrix was neither complete nor used by current
  parity gates.

## Changes

- Added contract-aligned Svelte and React implementations for the three real
  gaps, with shared token CSS and matching public types.
- Added focused behavior tests, smoke/a11y fixtures, catalogue entries, live
  specimens, and generated API documentation.
- Added `dismiss_label` to `RemediationBannerSpec` so the accessible command
  label remains part of the shared semantic contract.
- Corrected the four affected contracts and recorded the host-owned StateTile
  sparkline composition point.
- Replaced the stale parity index with a historical notice and links to current
  contracts, generated reports, tests, and previews.
- Removed current-source comments that treated historical parity files as live
  authority.

## Validation

- `effigy check:svelte`
- `effigy test:components` — 721 tests passed
- `effigy svelte:surface-audit` — 149 public components, 149 fully covered
- `effigy svelte:build`
- `effigy react:build`
- `effigy docs:check`
- `effigy parity:check`
- `effigy ci:web` — 347 core tests and 721 component/a11y tests passed
- `effigy test:contracts` — all Rust contract suites passed
- `git diff --check`
