# 059 Shared Demo-App Audit And Target Freeze Baseline

Status: active
Updated: 2026-03-13
Depends on: `048-gpui-contract-audit-priority-and-side-by-side-review-baseline.md`, `049-gpui-theme-runtime-and-native-preview-app-baseline.md`, `058-cross-runtime-parity-report-delta-register-and-acceptance-harness-expansion.md`

## Purpose

Freeze the current assessment that the Svelte preview or demo surface is not
yet a clean enough target for “the same UI in Svelte and GPUI,” and define the
minimum target shape the rebuilt shared demo must satisfy before GPUI demo
parity and downstream proof continue.

## Core Rule

The shared demo target must be explicit, coherent, and separable from the
docs-shell.

Poodle may use the existing preview harness for docs, tokens, and audit
infrastructure, but the actual UI GPUI is supposed to match should not remain
buried inside one monolithic preview page with docs-only utilities and broad
section glue mixed together.

## Required Audit Artifact

The current generation must maintain a machine-readable audit artifact at:

- `packages/shared-demo-app-audit.json`

That artifact must record:

- the current source surface
- package coverage posture and priority missing exports
- the main audit findings that make the current demo a weak parity target
- the frozen target shape for the rebuilt shared demo

## Audit Rule

The audit must be honest about at least these classes of defects:

- docs-shell and target-app concerns being mixed together
- monolithic implementation shape that resists contracting and parity review
- broad but shallow component coverage
- docs-only utilities polluting target parity
- section demos that do not add up to one coherent app story
- preview glue owning too much of the visible UI

The goal is not to complain about messiness abstractly. The goal is to freeze
why the current surface is not yet good enough to serve as the main GPUI demo
target.

## Target Shape Rule

The current target freeze must define one coherent shared demo app above the
docs-shell, with explicit screen or surface families such as:

- shell or overview
- form and validation
- browse and table
- detail and related data
- picker and media
- command and workspace

This target shape may evolve in `g04.013`, but it must stop being implicit
inside the existing preview structure.

## Coverage Rule

The audit artifact must record the current package coverage posture using the
generated parity report as the authority for counts.

For the shared demo target, complete primitive coverage is required. The point
is no longer merely to know which gaps matter most. The rebuilt Svelte demo
must close those gaps so GPUI is asked to reproduce one explicit full-surface
target instead of a partially covered shell plus contract-only leftovers.

## Honesty Rule

Poodle may say:

- the current preview surface is useful as docs and audit infrastructure
- the current demo target is not yet clean enough for strong cross-runtime demo parity
- the target shape for the rebuilt demo is now explicit

Poodle may not say:

- the existing preview page is already the right GPUI parity target by default
- broad section coverage alone proves the demo target is coherent
- GPUI downstream proof should continue before the Svelte demo target is cleaned up

## Seed Evidence

- `packages/shared-demo-app-audit.json`
- `packages/svelte/preview/artifacts/parity-report.json`
- `packages/svelte/preview/src/App.svelte`
- `packages/svelte/preview/src/app.css`
- `docs/roadmaps/g04/012-shared-demo-app-audit-gap-register-and-target-shape-freeze.md`

## Next Task

Carry this audit into `g04.013` and write the explicit cross-runtime demo-app
contract, section model, and parity checklist for both Svelte and GPUI.
