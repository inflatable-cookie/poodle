# 002 — Web focus rings follow input modality

Status: complete
Owner: Poodle core
Created: 2026-09-07
Depends on: none
Governing refs: `../../contracts/001-working-rules.md`, architecture 006,
affected component contracts

## Outcome

Web composite focus treatments now appear for keyboard interaction and remain
off for pointer focus, without a public component API change.

## Shipped result

- One idempotent, SSR-safe per-document input-modality tracker.
- Seven composite `:focus-within` treatments gated by keyboard modality.
- Bare `:focus` rules replaced by `:focus-visible` where required.
- Svelte/React mounting and component contracts aligned.

## Evidence

PR #228 merged at `365feb7dcebcc2e2db33e97d0bb6b3581231fab7`
on 2026-09-07. Chromium/WebKit probes covered pointer and keyboard focus;
component, docs, and web validation passed.

## Limits and continuation

No GPUI behavior changed. GPUI keyboard-origin focus remains a separate item in
the g17 runway and requires its own ready task.
