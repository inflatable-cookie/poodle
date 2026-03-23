# 060 Shared Demo-App Contract, Section Model, And Parity Checklist

Status: active
Updated: 2026-03-13
Depends on: `048-gpui-contract-audit-priority-and-side-by-side-review-baseline.md`, `058-cross-runtime-parity-report-delta-register-and-acceptance-harness-expansion.md`, `059-shared-demo-app-audit-and-target-freeze-baseline.md`

## Purpose

Freeze one explicit cross-runtime demo-app contract that both the rebuilt
Svelte demo and later GPUI demo must implement.

This milestone exists so the repo stops aiming GPUI at a pile of preview
sections and instead aims both runtimes at one coherent app model with explicit
screens, shell regions, state posture, and parity checkpoints.

## Core Rule

The shared demo app is a contract-owned target, not a loose implementation
detail.

That means the screen list, shell regions, state matrix, interaction
checkpoints, and ownership boundary between the demo app and the docs shell
must be explicit and machine-readable.

## Required Contract Artifact

The current generation must maintain a machine-readable contract artifact at:

- `packages/shared-demo-app-contract.json`

That artifact must record:

- the demo app id and dependency on the audit artifact
- the docs-shell boundary
- the shell region model
- the screen contracts and their source section mapping
- the parity checklist for cross-runtime review
- the current runtime bindings for Svelte and GPUI

## Docs-Shell Boundary Rule

The demo app contract must keep docs-only surfaces outside the target app:

- `catalog-hub`
- `token-summary-section`
- `token-inspector`

These may remain useful preview or inspection utilities, but they must not be
required pieces of the shared demo target GPUI is supposed to match.

## Screen Model Rule

The current cross-runtime demo target must be organized into coherent screens
rather than independent section demos.

At minimum, the contract must define:

- overview shell
- form and validation
- browse and table
- detail and related data
- picker and media
- command and workspace

Each screen must map back to the current source sections so the rebuild path is
traceable from the old preview surface to the new target app.

## Region Rule

The demo contract must define explicit shell regions such as:

- app header
- screen tabs
- context toolbar
- primary content
- companion panel
- status bar
- modal layer

These are contract-owned regions. Runtime-specific layout mechanics may differ,
but the conceptual regions and their jobs should remain stable.

## State And Interaction Rule

Every screen contract must include:

- a state matrix
- interaction checkpoints
- a comparison mode

The state matrix exists so rebuilt Svelte and GPUI demos expose the same review
jobs. The interaction checkpoints exist so parity review is not reduced to
visual screenshots.

`comparisonMode` may be:

- `direct-parity`
- `native-adaptation`

This keeps the demo contract aligned with the existing GPUI parity language
without pretending every screen is identical in runtime mechanics.

## Component Expectation Rule

Each screen contract must record the main public component families expected to
be visible in that screen.

The workflow screens do not need one detached specimen per export, but the
shared demo shell as a whole must provide direct review coverage for every
public primitive export. That coverage may be grouped on a dedicated primitive
coverage deck so long as it remains part of the same contract-owned demo app.

## Runtime Binding Rule

The contract must attach to both runtimes without making implementation
mechanics canonical.

The current contract should therefore record:

- the Svelte rebuild milestone and implementation root
- the GPUI demo-implementation milestone and implementation root

It may not encode Bits-specific or GPUI-specific mechanics as part of the
contract itself.

## Honesty Rule

Poodle may say:

- the shared demo target is now explicit
- Svelte and GPUI both have one contract-owned demo app to implement
- screen-level parity review can now be organized around one stateful target

Poodle may not say:

- the contract is satisfied before the Svelte rebuild exists
- the docs shell is the same thing as the demo app
- every component export must become a standalone detached screen regardless of
  screen coherence

## Seed Evidence

- `packages/shared-demo-app-contract.json`
- `packages/shared-demo-app-audit.json`
- `docs/roadmaps/g04/013-cross-runtime-demo-app-contract-section-model-and-parity-checklist.md`

## Next Task

Carry this contract into `g04.014` and rebuild the Svelte demo app against the
explicit screen model, shell regions, state matrix, and parity checklist.
