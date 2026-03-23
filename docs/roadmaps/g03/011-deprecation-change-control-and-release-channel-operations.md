# g03.011 Deprecation, Change Control, And Release-Channel Operations

Status: completed
Owner: Flint Core
Updated: 2026-03-12
Depends on: g03.001, g03.003, g03.006
Primary repos: `flint`

## Goals

- [x] define how changes are proposed, promoted, deprecated, and removed
- [x] define release-channel posture for unstable versus stable surfaces

## Execution Checklist

- [x] define change proposal and promotion rules
- [x] define deprecation and removal workflow
- [x] define release-channel posture for unstable versus stable surfaces

## Acceptance Criteria

- [x] change-control posture is explicit
- [x] release-channel operations posture is explicit

## Completed Work

- added the normative operations baseline `docs/specs/044-deprecation-change-control-and-release-channel-operations.md`
- added the machine-readable operations record `packages/release-operations.json`
- updated `packages/release-manifest.json` so release classification and operations artifacts remain linked
- extended `packages/svelte/preview/scripts/lint-docs.ts` to validate release metadata, channel posture, and package inventory alignment
- refreshed the packaging and package-surface specs so their next-task language no longer points at earlier generations

## Next Task

Open `g03.012` and define ecosystem acceptance and long-tail regression
coverage now that release-channel and deprecation operations are explicit.
