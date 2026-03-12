# g02.006 Media Preview, Embed, And Asset-Surface Suite

Status: completed
Date: 2026-03-11
Owner: Pug Core

## Summary

- completed `g02.006`
- added reusable media composites at `packages/svelte/composites/src/MediaThumbnail.svelte`, `packages/svelte/composites/src/MediaPreview.svelte`, and `packages/svelte/composites/src/EmbedShell.svelte`
- extended the preview with a media and asset suite that exercises framed preview state, asset switching, document/image/audio/video-style surfaces, and embed fallback posture
- added media contracts at `docs/contracts/composites/media-thumbnail.md`, `docs/contracts/composites/media-preview.md`, and `docs/contracts/composites/embed-shell.md`
- added the normative media baseline at `docs/specs/014-media-preview-embed-and-asset-surface-rules.md`

## Validation

- `bun run preview:build`
- `bun run tokens:build`
- `git diff --check`

## Notes

- this tranche deliberately freezes framing, identity, and fallback posture without pretending Pug owns playback engines or embed-runtime selection
- accessibility focus remains on textual identity, explicit state, and recovery affordances surviving renderer failure

## Next Task

Open `docs/roadmaps/g02/007-loading-empty-error-notification-and-remediation-depth.md` and build the next hardening batch above the now-complete browse, picker, and media fallback baseline.
