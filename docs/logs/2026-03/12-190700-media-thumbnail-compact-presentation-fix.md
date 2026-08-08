---
title: Media thumbnail compact presentation fix
status: completed
owner: nucleus
updated: 2026-03-12
tags: [logs, svelte, preview, media, composites]
---

## Summary

Moved the compact media-strip behavior into `MediaThumbnail` so strip padding and content density no longer depend on parent CSS competing with scoped thumbnail styles.

## What changed

- added `presentation="compact"` to `MediaThumbnail`
- compact presentation now suppresses the caption, removes long fallback copy, tightens the state layout, and slightly repositions the badge inside the component itself
- updated the preview media strip to opt into that compact presentation instead of relying on preview-only descendant overrides
- removed the redundant strip-specific descendant overrides from the preview stylesheet

## Validation

- `bun run docs:build`
- `git diff --check`

## Outcome

The media strip should now render with stable internal padding because the thumbnail component owns compact-mode layout directly rather than relying on override order.

## Next

Do one quick browser pass on the media strip and then return to roadmap work instead of adding more preview-only exceptions.
