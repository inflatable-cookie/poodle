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

## 3. Specimen Definitions

All preview apps must render the following specimens identically.

### Image preview

A media preview configured for an image asset:

| Label | Props/Config | Expected Visual |
|-------|-------------|-----------------|
| Image preview | `title="Hero banner"`, `description="Main landing page banner image for the product launch."`, `eyebrow="Image"`, `meta=["1920 x 1080", "245 KB", "PNG"]`, `kind="image"`, `aspectRatio="landscape"`, media slot with placeholder | framed landscape media area with title, description, eyebrow label, and metadata chips below |

### Video preview

A media preview configured for a video asset:

| Label | Props/Config | Expected Visual |
|-------|-------------|-----------------|
| Video preview | `title="Onboarding walkthrough"`, `eyebrow="Video"`, `meta=["3:42", "48 MB"]`, `kind="video"`, `aspectRatio="video"`, media slot with placeholder | framed video-ratio media area with title, eyebrow, and duration/size metadata |

### Error state

A media preview in error posture:

| Label | Props/Config | Expected Visual |
|-------|-------------|-----------------|
| Error state | `title="Corrupted file"`, `kind="document"`, `state="error"`, `stateTitle="Preview unavailable"`, `stateMessage="This file cannot be previewed."`, `aspectRatio="landscape"` | framed area with error messaging replacing media content; title and error details visible |

## 4. Next Task

Use `MediaPreview` for asset inspection, attachment, and detail-display workflows instead of mixing raw thumbnails and ad hoc metadata cards.
