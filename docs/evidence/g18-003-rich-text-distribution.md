# g18.003 rich-text distribution evidence

Status: measured on the worker branch before PR
Date: 2026-09-10
Task: `docs/roadmaps/g18/003-tiptap-prosemirror-rich-text-editor.md`
Base: `main` at `9237380fdc1621d3733ad0c5ac2889d8c1022d60`

All facts below were read off the built `dist/` trees
(`core:build`, `svelte:package`, `react:package`) and the test runners on this
branch. Nothing here is hand-claimed.

## Package isolation

- Root bundles carry zero engine references (`grep -c tiptap`):
  `svelte/dist/index.client.js` 0, `svelte/dist/index.server.js` 0,
  `react/dist/index.js` 0. No `rich-text-engine` string in any root bundle.
- Both `./rich-text` entries reach the engine only through external imports
  (19 exact specifiers in each lane, client and server):
  `@tiptap/core`, `@tiptap/pm`, `@tiptap/extensions`,
  `@tiptap/extension-{blockquote,bold,code,code-block,document,hard-break,heading,horizontal-rule,image,italic,link,list,paragraph,strike,table,text}`.
  No bundled engine copy inside `dist/`.
- `./editor` and `./markdown` entries carry zero TipTap references
  (`RichTextPackaging.test.ts`).
- Rich-text declarations carry zero engine types (`grep -c tiptap`):
  `svelte/dist/rich-text.d.ts` 0, `react/dist/rich-text.d.ts` 0.
- `RichTextEditor`/`RichTextRenderer` are absent from both root barrels, the
  176-name roster, and every successor denominator
  (`RichTextPackaging.test.ts`, roster projection).
- The 176/171 roster counts are unchanged; `CORE_STYLE_FILES` grows
  168 → 169 for `rich-text.css` only (`core-build.test.ts`).

## Pinned engine versions (both shell manifests, exact, MIT)

`@tiptap/core`, `@tiptap/pm` (ProseMirror bundle), `@tiptap/extensions`, and
the fifteen admitted feature modules (`document`, `paragraph`, `text`,
`hard-break`, `bold`, `italic`, `strike`, `code`, `heading`, `link`, `list`,
`blockquote`, `code-block`, `horizontal-rule`, `table`, `image`) — all pinned
`3.31.3`. Transitive `linkifyjs 4.3.3` (MIT). Recorded in
`THIRD_PARTY_NOTICES.md`.

## Behavior proof (all green on this branch)

- Core: 1296 pass (`bun test` in `packages/core`), including the closed
  feature/command registries, feature-to-command ownership, toolbar
  derivation (images require a present `requestImage`), and the
  2 MiB / 10,000-node envelope.
- Svelte board: 50 focused rich-text cases (engine validation, no-echo and
  host-revert control, toolbar derivation and arrow roving, table insertion,
  host-echo no-op, and Escape-Tab focus escape, optional images with resolve/
  cancel/reject/unmount/disable/insert-once/URL-admission races, feature
  reconfiguration with refusal and a fresh validator, image onChange
  round-trip, heading gating, real paste sanitization, renderer equivalence,
  large-document refusal, SSR cleanup), plus the full
  component board 2906 pass across both shells (a11y and parity sweeps cover
  both rich-text components automatically).
- React board: 42 focused rich-text cases mirroring the Svelte families.
- SSR: `RichTextSsr.test.ts` proves server HTML carries roots/viewport/
  content region with no `ProseMirror`, `contenteditable`, or document
  markup; the engine imports clean in the node-like lane.
- Distribution: core-build/pack-archive/scope/gate suites 63 pass,
  shell-build suites pass (React export count 180 → 181 for `./rich-text`),
  both packages build clean with receipts, `web-preview.ts` certification
  passes with all 23 falsification oracles intact.

## Known staged-admission limits (unchanged by this task)

- `RichTextEditor`/`RichTextRenderer` are `web-admitted`, not
  parity-complete: no Rust declaration, no `poodle-render` composition, no
  GPUI implementation, no placeholder or receipt anywhere in native targets.
- Preview catalogue admission (specimen route, generation index) is reserved
  closeout work; focused tests are the specimen evidence for now.
- Embeds, mentions, collaboration, comments, tracked changes, uploads,
  Markdown/HTML conversion are outside v1 by contract.
- Paste makes no lossless-import claim: clipboard content parses only
  through the active schema; unsafe HTML never survives.
- Pre-existing red on the dispatch head, unchanged by this task:
  `audit:licenses` (`deny.toml` still claims bzip2), `drift:roles` (script
  path missing), `check:react-components` and `check:react-preview`
  (pre-existing TS errors, byte-identical diffs versus `main`). Recorded in
  `PAPERCUTS.md`.
