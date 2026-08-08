---
title: g03.006 extension SDK and starter package baseline
status: completed
owner: nucleus
updated: 2026-03-12
tags: [logs, roadmap, extensions, sdk, docs]
---

## Summary

Completed `g03.006` by freezing the current extension-facing SDK posture,
starter-package guidance, and safe downstream composition rules as explicit
documentation baseline.

## What changed

- added the normative spec `docs/specs/039-extension-sdk-composition-guidance-and-starter-package-baseline.md`
- completed `docs/roadmaps/g03/006-extension-sdk-composition-guidance-and-starter-packages.md`
- rolled the generation and docs indexes forward in:
  - `docs/specs/README.md`
  - `docs/roadmaps/g03/README.md`
  - `docs/roadmaps/README.md`
  - `docs/README.md`
  - `README.md`
- froze the current SDK meaning as package APIs, contracts, bridge adapters,
  and docs rather than a new abstraction layer
- defined the allowed starter shapes for:
  - direct Svelte consumers
  - bridge-mediated consumers
  - workstation-oriented consumers
- made wrapper, adapter, branding, and host-integration ownership explicit so
  downstream repos do not infer package posture from preview-only examples

## Validation

- `bun run docs:lint`
- `bun run docs:build`
- `git diff --check`

## Outcome

`g03.006` is now explicit. The repo has a documented answer for what the
extension SDK actually is in this generation, what starter packages may
promise, and how downstream repos should compose above Pug without redefining
canonical meaning or leaking Pug where a bridge should own the boundary.

## Next

Move to `g03.007` and harden the Underlay bridge with a zero-leak proof using
the new extension-SDK and starter-package baseline as the ownership frame.
