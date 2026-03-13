# g04.015 GPUI Demo-App Parity Implementation, And Side-By-Side Review

Status: planned
Owner: Pug Core
Updated: 2026-03-13
Depends on: g04.013, g04.014
Primary repos: `pug`

## Goals

- [ ] implement the same demo app in GPUI against the explicit demo contract
- [ ] make side-by-side Svelte versus GPUI review concrete instead of mostly
  artifact-backed

## Execution Checklist

- [ ] implement the contracted demo screens and shell structure in GPUI
- [ ] match the rebuilt Svelte demo closely enough that direct comparison
  exposes real hierarchy, state, and interaction differences
- [ ] use the intentional delta register where runtime-specific differences are
  still valid
- [ ] record remaining GPUI gaps honestly instead of calling the demo parity
  complete by implication

## Acceptance Criteria

- [ ] GPUI demo-app parity posture is explicit
- [ ] side-by-side Svelte and GPUI review posture is explicit

## Next Task

Open `g04.016` and move from demo parity into downstream or reference-app GPUI
implementation proof.
