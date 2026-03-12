# MediaPreview

Status: seed contract
Updated: 2026-03-11

## 1. Purpose

- Component name: `MediaPreview`
- Layer: `composites`
- Summary: a richer asset-preview surface that combines framed media, textual identity, metadata, and optional footer actions
- In scope: title, supporting description, media frame, metadata chips, caption/body content, optional footer actions, and explicit preview state posture
- Out of scope: transport controls, editing tools, waveform/video rendering engines, or app-specific asset workflows

## 2. Accessibility

- title and supporting context must stay textual and visible outside the preview frame
- preview state messaging must remain readable even when the renderer fails or no generated preview exists
- footer actions must stay reachable independently of whether the media surface is available
- GPUI-native accessibility mapping notes: GPUI must preserve local title, metadata, and fallback structure rather than exposing the preview as an unlabeled textured surface

## 3. Next Task

Use `MediaPreview` for asset inspection, attachment, and detail-display workflows instead of mixing raw thumbnails and ad hoc metadata cards.
