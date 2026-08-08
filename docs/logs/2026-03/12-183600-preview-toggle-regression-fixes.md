---
title: Preview toggle regression fixes
status: completed
owner: nucleus
updated: 2026-03-12
tags: [logs, svelte, preview, primitives]
---

## Summary

Fixed regressions introduced by using primitive toggles as stacked card and media-strip surfaces in the preview shell.

## What changed

- Added a `layout="stack"` mode to `Toggle` so card-like and thumbnail-like selection surfaces can render as full-width block controls instead of inheriting inline button sizing.
- Applied that stacked toggle layout to the catalog section cards and the media strip in the preview shell.
- Hardened `ToggleGroup` selected-state styling to use an explicit `selected` class and string `data-selected` attribute, avoiding ambiguous selected rendering in the rail control groups.
- Moved the media-strip active styling onto the primitive’s existing `data-pressed` state.

## Validation

- `bun run docs:build`
- `git diff --check`

## Outcome

The media suite selection strip should now lay out as real card-like thumbnails again, and the rail toggle groups should reflect the actual selected value instead of sticking on a stale-looking visual state.

## Next

Do one live pass across the rail controls and media suite to catch any remaining primitive-adoption regressions, then return to the broader hardening track instead of widening preview churn.
