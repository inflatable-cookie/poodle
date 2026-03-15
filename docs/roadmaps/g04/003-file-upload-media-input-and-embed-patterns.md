# g04.003 File Upload, Media Input, And Embed Patterns

Status: planned
Owner: Pug Core
Updated: 2026-03-14
Depends on: g04.001
Primary repos: `pug`

## Goals

- [ ] implement FileUpload as a primitive with drag-and-drop, file type
  filtering, and upload progress
- [ ] implement MediaPicker as a composite for browsing and selecting media
  assets
- [ ] implement EmbedInput as a composite for URL-to-embed conversion
- [ ] implement EmbedPreview as a composite for rendering rich embed cards

## Execution Checklist

- [ ] write contract for FileUpload: drop zone, file type accept list, size
  limits, multiple files, progress callback, preview thumbnails
- [ ] implement FileUpload primitive in `@pug/svelte-primitives`
- [ ] write contract for MediaPicker: gallery view, search, filtering, selection
  mode, upload integration
- [ ] implement MediaPicker composite in `@pug/svelte-composites`
- [ ] write contract for EmbedInput: URL input, paste detection, provider
  resolution, preview generation
- [ ] implement EmbedInput composite in `@pug/svelte-composites`
- [ ] write contract for EmbedPreview: provider icon, title, description,
  thumbnail, link
- [ ] implement EmbedPreview composite in `@pug/svelte-composites`
- [ ] create specimens for all four components
- [ ] register in component-registry.ts and specimen registry

## Acceptance Criteria

- [ ] FileUpload supports drag-and-drop with visual feedback, click-to-browse,
  and progress indication
- [ ] FileUpload validates file type and size before accepting
- [ ] MediaPicker renders a browsable asset gallery with search and selection
- [ ] EmbedInput resolves pasted URLs into embed metadata
- [ ] EmbedPreview renders a rich card with provider attribution
- [ ] all four components pass build and render in the preview catalogue

## Next Task

Open `g04.004` and implement button and card pattern extensions.
