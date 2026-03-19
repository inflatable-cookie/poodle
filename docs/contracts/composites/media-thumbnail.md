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

## 3. Specimen Definitions

All preview apps must render the following specimens identically.

### Image thumbnails

A row of image and video thumbnails in a grid:

| Label | Props/Config | Expected Visual |
|-------|-------------|-----------------|
| Photo 1 | `kind="image"`, `title="Photo 1"`, `badge="New"`, `aspectRatio="square"` | square thumbnail frame with image placeholder and New badge |
| Photo 2 | `kind="image"`, `title="Photo 2"`, `meta="2.4 MB"`, `aspectRatio="square"` | square thumbnail frame with image placeholder and size metadata |
| Clip | `kind="video"`, `title="Clip"`, `badge="HD"`, `meta="1:24"`, `aspectRatio="square"` | square thumbnail frame with video placeholder, HD badge, and duration metadata |

### Compact presentation

A row of thumbnails in compact presentation mode:

| Label | Props/Config | Expected Visual |
|-------|-------------|-----------------|
| Report.pdf | `kind="document"`, `title="Report.pdf"`, `presentation="compact"`, `aspectRatio="landscape"` | compact landscape thumbnail with document placeholder |
| Interview.mp3 | `kind="audio"`, `title="Interview.mp3"`, `presentation="compact"`, `aspectRatio="landscape"` | compact landscape thumbnail with audio placeholder |

### Loading state

A single thumbnail in loading posture:

| Label | Props/Config | Expected Visual |
|-------|-------------|-----------------|
| Loading | `kind="image"`, `state="loading"`, `aspectRatio="square"` | square thumbnail frame showing loading indicator |

## 4. Next Task

Use `MediaThumbnail` inside richer preview and asset workflows instead of rebuilding framed placeholders ad hoc.
