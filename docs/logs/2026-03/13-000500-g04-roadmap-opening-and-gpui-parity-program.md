---
title: g04 roadmap opening and gpui parity program
status: completed
owner: nucleus
updated: 2026-03-13
tags: [logs, roadmap, gpui, planning]
---

## Summary

Opened `g04` as the next explicit generation and centered it on GPUI-native
implementation parity, runtime proof, downstream GPUI adoption evidence, and
docs promotion beyond the internal preview baseline.

## What changed

- added the new generation index `docs/roadmaps/g04/README.md`
- added detailed milestone files for `g04.001` through `g04.014`
- updated `docs/roadmaps/generation-index.md` so `g04` is now the active generation
- updated `docs/roadmaps/README.md`, `docs/README.md`, and `README.md` so the
  repo starts from `g04` rather than treating `g03` as still open
- defined the planning posture explicitly: GPUI should imitate the Svelte
  surface semantically and visually where the contract says it should, but not
  by blindly cloning DOM-specific implementation behavior

## Validation

- `bun run --cwd packages/svelte/preview docs:lint`
- `git diff --check`

## Outcome

The repo now has a detailed next-generation roadmap instead of a vague
"continue hardening" posture. `g04` is the GPUI-native implementation and
runtime-proof generation, with explicit sequencing from contract audit to
primitives, composites, workstation shells, native accessibility, downstream
proof, docs promotion, and closeout.

## Next

Open `g04.001` and define the GPUI contract audit, parity priority matrix, and
implementation order before starting larger native component work.
