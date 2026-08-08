---
title: Transient toast tone contrast hardening
status: completed
owner: nucleus
updated: 2026-03-12
tags: [logs, svelte, composites, notifications]
---

## Summary

Strengthened tone differentiation in the transient notification stack so info,
success, warning, and danger toasts no longer read as near-identical cards with
slightly different borders.

## What changed

- introduced a per-toast tone variable in `ToastStack`
- strengthened tone visibility through a tinted border, a subtle left accent
  rail, and a light tone wash in the background
- made action and dismiss controls tone-aware so repeated stacked items still
  read as belonging to distinct semantic roles

## Validation

- `bun run docs:build`
- `git diff --check`

## Outcome

Transient notifications should now separate more clearly by semantic role while
still reading as one coherent stack rather than four unrelated component
families.

## Next

Do one quick pass on the notification suite across light and dark themes, then
return to the roadmap work unless another concrete contrast issue appears.
