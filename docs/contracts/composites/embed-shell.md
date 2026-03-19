# EmbedShell

Status: seed contract
Updated: 2026-03-11

## 1. Purpose

- Component name: `EmbedShell`
- Layer: `composites`
- Summary: a framed shell for embedded host-native or external content with explicit fallback posture
- In scope: local title/context, provider label, framed embed viewport, loading/error/empty states, and optional footer actions
- Out of scope: origin policy, authentication, sandboxing, permission prompts, or choosing the actual embedded runtime

## 2. Accessibility

- embeds must keep a visible title and context outside the viewport itself
- loading and failure states must preserve the framed region instead of removing structure
- fallback and recovery actions must remain explicit when embedded content is unavailable
- GPUI-native accessibility mapping notes: GPUI must recreate equivalent framed-region and fallback meaning even where the embedded destination is not web-based

## 3. Specimen Definitions

The EmbedShell contract is demonstrated through two related specimen files:
`EmbedInputSpecimen.svelte` (URL/code input) and `EmbedPreviewSpecimen.svelte`
(framed preview rendering).

All preview apps must render the following specimens identically.

### EmbedInput: URL or embed code input

A basic embed input field:

| Label | Props/Config | Expected Visual |
|-------|-------------|-----------------|
| URL or embed code input | `placeholder="Paste a YouTube URL, Vimeo link, or embed code..."`, bound value and parsed result | text input accepting URLs or embed codes; parsed result displayed as JSON when valid |

### EmbedInput: With label

An embed input with a visible label:

| Label | Props/Config | Expected Visual |
|-------|-------------|-----------------|
| With label | `label="Video embed"`, `placeholder="https://youtube.com/watch?v=..."` | labeled input field with placeholder text |

### EmbedInput: Restricted providers

An embed input limited to specific providers:

| Label | Props/Config | Expected Visual |
|-------|-------------|-----------------|
| Restricted providers | `providers=["youtube", "vimeo"]`, `placeholder="Only YouTube and Vimeo allowed..."` | input that only accepts YouTube and Vimeo URLs |

### EmbedPreview: YouTube embed

A rendered YouTube embed preview:

| Label | Props/Config | Expected Visual |
|-------|-------------|-----------------|
| YouTube embed | `parsed={ provider: "youtube", id: "dQw4w9WgXcQ", originalUrl }` | framed YouTube video embed in viewport |

### EmbedPreview: Vimeo embed

A rendered Vimeo embed preview:

| Label | Props/Config | Expected Visual |
|-------|-------------|-----------------|
| Vimeo embed | `parsed={ provider: "vimeo", id: "76979871", originalUrl }` | framed Vimeo video embed in viewport |

### EmbedPreview: Loading state

An embed preview in loading posture:

| Label | Props/Config | Expected Visual |
|-------|-------------|-----------------|
| Loading state | `loading=true` | framed region with loading indicator |

### EmbedPreview: Error state

An embed preview in error posture:

| Label | Props/Config | Expected Visual |
|-------|-------------|-----------------|
| Error state | `error="Failed to load embed. The URL may be invalid or the provider is unavailable."` | framed region with error message |

### EmbedPreview: Empty state

An embed preview with no content:

| Label | Props/Config | Expected Visual |
|-------|-------------|-----------------|
| Empty state | `emptyMessage="Paste a URL above to see a preview"` | framed region with empty-state message |

## 4. Next Task

Use `EmbedShell` anywhere framed embedded destinations need a stable fallback contract instead of letting embeds collapse to raw iframes or blank native panels.
