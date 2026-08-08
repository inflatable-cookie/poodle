---
title: Preview primitive adoption freeze point
status: completed
owner: nucleus
updated: 2026-03-12
tags: [logs, svelte, preview, primitives]
---

## Summary

Closed the remaining preview-control adoption gap so the docs shell is no longer styling ad hoc HTML buttons for section navigation chips.

## What changed

- Added external `className` support to `Button` so preview surfaces can style primitive instances directly instead of targeting descendant `button` elements.
- Moved catalog-hub docs-link chip styling onto the primitive-owned `.docs-link-chip` class in the preview shell.
- Confirmed the section cards and media strip are already backed by `Toggle`, then updated parity coverage so `Toggle` is counted as directly previewed.

## Validation

- `bun run docs:lint`
- `bun run parity:report`
- `bun run docs:build`
- `git diff --check`

## Outcome

The preview shell is now at a cleaner freeze point for control adoption: it uses shipped primitives for its interactive shell controls, while any remaining shell-specific styling is attached to primitive instances rather than raw HTML element selectors. Parity coverage now records `@pug/svelte-primitives` at `14/63` directly previewed exports.

## Next

Freeze preview adoption here and move into `g03.004` performance, render-cost, and memory-profile hardening unless a specific preview surface still needs corrective cleanup.
