# g18.008 — Web editor preview specimens

Status: ready for review
Date: 2026-09-10
Branch: `ns-d8e89ac2-2c5e-45b3-9c7a-ebfd36ff9550`
Card: `docs/roadmaps/g18/008-web-editor-preview-specimens.md`
Handoff: `docs/handoffs/20260910-g18-008-web-editor-preview-specimens.md`
Governing refs: `docs/contracts/001-working-rules.md`,
`docs/architecture/003-component-docs-ia-and-implementation-substrates.md`,
`docs/contracts/components/code-editor.md`,
`docs/contracts/components/rich-text-editor.md`,
`docs/specs/070-compiled-web-distribution-contract.md`
Base: `origin/main` at `d34bb322fcb1cb1d908c5225cb690d37257e51d5`

## Outcome

`CodeEditor`, `RichTextEditor`, and `RichTextRenderer` are findable and
reviewable in both web previews. Three web-only catalogue entries feed
matched Svelte and React specimen pages through the existing
`webOnlyComponents` supplement. Specimens mount the public `./editor` and
`./rich-text` subpath exports. No editor API, native catalogue, Desktop, or
release change.

## What changed

- Shared web-only registry: `code-editor`, `rich-text-editor`,
  `rich-text-renderer` in composition / forms-validation. Both galleries
  inherit identical slugs, labels, search, and `#components/<slug>` routes.
- Shared representative documents in
  `packages/svelte/preview/src/specimens/web-editor-documents.ts`.
- Matched Svelte and React specimens: TypeScript CodeEditor with controlled
  host value, diagnostics, read-only, and language/line-number controls;
  RichTextEditor with formatted content, a table, controlled JSON, and an
  images-off/on toggle; RichTextRenderer with the same documents and no
  editable state.
- Usage docs and regenerated `component-docs.json` artifacts. Preview Vite
  and Vitest aliases for `./editor` and `./rich-text`, matching markdown.
- Focused registry, navigation/search/direct-route, source-import, and
  interaction tests in both preview suites.

## Validation

- Svelte preview: 6 new specimen tests + catalogue audit (8) + catalogue
  nav (4) pass
- React preview: 6 new specimen tests + catalogue nav (4) pass
- `react-specimen-drift`: 179 registered, all claimed present
- `contract-prop-drift`: 138 checked, 0 findings
- `react-prop-drift`: 179 checked
- `effigy check:svelte-preview`: 0 errors
- `effigy svelte:build` and `effigy react:build`: clean
- `effigy docs:lint`: 185 contracts; GPUI headless construction routes stay
  175
- `git diff --check`: clean

`check:react-preview` remains the pre-existing string/`ControlSize`
backlog; the two new density axes are cast to `ControlDensity`.

## Remaining limits

Web-admitted only. Native CodeEditor/RichTextEditor stay future work with
no placeholder. Release and Desktop adoption need separate authority.
