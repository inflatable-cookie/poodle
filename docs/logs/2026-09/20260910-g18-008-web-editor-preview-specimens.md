# g18.008 — Web editor preview specimens

Status: merged
Merge: `998b6ddc69f94e405b515f6bddd682a2e8916ea5` (PR #241) on 2026-09-10
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

## Closeout

- Merge performed by the plugin as
  `998b6ddc69f94e405b515f6bddd682a2e8916ea5` on 2026-09-10 (PR #241),
  with parents `958bf340d` (main) and `9ad76cace` (reviewed head).
- Accepted review: independent exact-head `ready_to_merge` approval of
  head `9ad76cace3a8ac9948696d8a87b5623789ae7477` by betterthanclay
  ([comment #5626460237](https://github.com/inflatable-cookie/poodle/pull/241#issuecomment-5626460237)).
  No merge blockers remained.
- Reviewed-head validation (reviewer ran at the exact head, tree left
  clean): svelte-preview 13 files / 72 tests pass (6 new specimen
  interaction tests, catalogue audit, catalogue nav); react-preview 4
  files / 14 tests pass (6 mirrored interaction tests);
  `check:svelte-preview` 0 errors; `svelte:build` and `react:build` clean;
  specimen/prop drift checks aligned (179 registered);
  full `docs:check` sequence succeeded with `gate:clean`;
  `git diff --check` clean. Native denominator unchanged: GPUI
  construction routes stay 175.
- Non-blocking reviewer notes (deferred, no acceptance impact):
  `check:react-preview` fails repo-wide on the pre-existing
  string/`ControlSize` backlog (zero errors in PR-touched files, kept off
  `ci:web` by effigy.toml); one tautological route assertion noted as a nit.
- Deferred: no release, tag, publish, Desktop, or native editor work starts
  from this task. `g18.006` still needs g18.005 completion plus explicit
  operator release authority.
