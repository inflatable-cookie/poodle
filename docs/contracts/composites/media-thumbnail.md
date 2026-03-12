# MediaThumbnail

Status: seed contract
Updated: 2026-03-11

## 1. Purpose

- Component name: `MediaThumbnail`
- Layer: `composites`
- Summary: a framed preview surface for image, audio, video, document, or embed-oriented assets
- In scope: aspect-ratio framing, preview placeholder, loading/error/empty posture, lightweight badges, optional title/meta caption
- Out of scope: real playback engines, zooming, annotation tools, file fetching, or app-specific asset actions

## 2. Accessibility

- textual identity must stay available when the visual preview cannot render
- loading, error, and empty states must remain explicit rather than collapsing into blank surfaces
- decorative overlay affordances such as play badges must not replace the accessible name
- GPUI-native accessibility mapping notes: GPUI must preserve framed preview meaning and fallback copy even where there is no HTML `figure` equivalent

## 3. Next Task

Use `MediaThumbnail` inside richer preview and asset workflows instead of rebuilding framed placeholders ad hoc.
