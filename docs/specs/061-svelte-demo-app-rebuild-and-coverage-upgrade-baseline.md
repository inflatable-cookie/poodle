# 061 Svelte Demo-App Rebuild And Coverage Upgrade Baseline

Status: active
Updated: 2026-03-13
Depends on: `059-shared-demo-app-audit-and-target-freeze-baseline.md`, `060-shared-demo-app-contract-section-model-and-parity-checklist.md`

## Purpose

Freeze the rebuilt Svelte demo app as the primary shared-runtime UI target that
GPUI should now imitate in `g04.015`.

This milestone exists so the repo stops aiming GPUI at a dense preview page and
instead aims it at one coherent Svelte demo shell with explicit screens,
docs-only boundaries, and materially stronger direct component adoption.

## Core Rule

The rebuilt Svelte demo is the contract-owned target app, while the docs shell
remains a host and inspection surface around it.

That means:

- docs-only catalog and token tools stay outside the target app
- the Svelte demo must implement the six-screen model frozen in the shared
  demo contract
- the GPUI demo must now treat the rebuilt Svelte demo as the strongest current
  implementation reference

## Implementation Shape

The current Svelte preview implementation should reflect three explicit layers:

- a docs-shell host for navigation, catalog framing, and token inspection
- a shared demo shell that implements the six contract-owned screens
- preview-local component extraction so the entrypoint no longer owns the full
  render tree inline

The repo does not need a separate package for the Svelte demo target yet, but
the render tree must no longer behave like one monolithic section page.

## Coverage Rule

The rebuilt Svelte demo must provide complete direct primitive coverage instead
of merely improving the old preview counts.

As of this baseline, the parity artifact should reflect:

- `@pug/svelte-primitives`: `63/63` directly previewed exports
- `@pug/svelte-composites`: `20/20` directly previewed exports
- `@pug/svelte-workstation`: `14/14` directly previewed exports

These counts are evidence that the rebuilt demo is now a full public-surface
comparison target rather than a mostly coherent workflow shell with remaining
contract-only primitive gaps.

## Boundary Rule

The rebuilt Svelte demo may live inside the docs host, but it must preserve the
docs-shell boundary frozen earlier:

- `catalog-hub`
- `token-summary-section`
- `token-inspector`

Those remain docs-owned tools. They are not part of the shared demo target GPUI
is supposed to match.

## Honesty Rule

Pug may say:

- the Svelte demo target is rebuilt enough to be the primary GPUI reference
- the demo now uses materially more of the public package surface directly
- side-by-side GPUI review should now happen against this shell and screen model

Pug may not say:

- the Svelte demo is finished forever
- every public export is now meaningfully demonstrated
- GPUI parity is proven before `g04.015` implements the matching demo app

## Seed Evidence

- `packages/shared-demo-app-contract.json`
- `packages/shared-demo-app-audit.json`
- `packages/svelte/preview/artifacts/parity-report.json`
- `docs/roadmaps/g04/014-svelte-demo-app-rebuild-component-adoption-and-coverage-upgrade.md`

## Next Task

Carry this rebuilt Svelte target into `g04.015` and implement the same demo
app in GPUI with side-by-side review against the Svelte shell, screens, and
interaction posture.
