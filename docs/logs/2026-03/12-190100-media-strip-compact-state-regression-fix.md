---
title: Media strip compact-state regression fix
status: completed
owner: nucleus
updated: 2026-03-12
tags: [logs, svelte, preview, media]
---

## Summary

Fixed the remaining media-strip regression where compact asset selectors were
still rendering full thumbnail captions and empty-state copy after the toggle
adoption work.

## What changed

- replaced invalid `:global(...)` selectors in the plain preview stylesheet with
  real `.media-strip__item .media-thumbnail__...` selectors
- hid thumbnail captions and long fallback paragraphs inside the compact media
  strip
- tightened the strip thumbnail empty/error state layout so the compact selector
  stays legible while the larger media preview keeps the full fallback copy

## Validation

- `bun run docs:build`
- `git diff --check`

## Outcome

The compact media selector should now behave like a true strip of selectable
thumbnails instead of rendering the full asset preview copy inside each item.

## Next

Do one quick in-browser pass on the media strip and then return to the broader
roadmap work instead of continuing preview-only churn.
