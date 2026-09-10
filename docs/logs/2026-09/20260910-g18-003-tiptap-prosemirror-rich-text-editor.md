# g18.003 — TipTap/ProseMirror rich-text editor

Status: ready for review
Date: 2026-09-10
Branch: `ns-edb2d303-38c9-44a8-aa2c-a6ff5821643e`
Card: `docs/roadmaps/g18/003-tiptap-prosemirror-rich-text-editor.md`
Handoff: `docs/handoffs/20260910-g18-003-tiptap-prosemirror-rich-text-editor.md`
Governing refs: `docs/contracts/components/rich-text-editor.md`,
`docs/contracts/001-working-rules.md`,
`docs/architecture/001-poodle-system-shape.md`,
`docs/specs/070-compiled-web-distribution-contract.md`

## Outcome

Paired Svelte and React `RichTextEditor` and `RichTextRenderer` over pinned
TipTap 3 / ProseMirror packages, behind dedicated `./rich-text` entries.
ProseMirror document JSON and schema semantics stay authoritative; no Poodle
block schema exists. Engines stay private; root, `./markdown`, and `./editor`
consumers load no TipTap or ProseMirror. No native work, no Desktop changes,
no release.

## What changed

- Core (`packages/core/src/rich-text.ts`, `index.ts`,
  `styles/rich-text.css`): engine-free structural ProseMirror JSON carrier
  types, the closed feature and command registries, the standard feature set
  (tables in, images opt-in), feature/command validation, toolbar resolution
  (auto derivation + explicit-list refusal), command labels, and the
  2 MiB / 10,000-node envelope.
- Engines (`rich-text-engine.ts`, mirrored in both shells): curated schema
  assembly from features only (always `doc`/`paragraph`/`text`/`hardBreak` +
  UndoRedo; each feature module adds exactly its nodes/marks), exact
  pre-mount validation (unknown node/mark/attribute, URL scheme, heading
  level, content-fit `check()`), controlled no-echo sync with host-revert
  rejection, schema-valid live reconfiguration with closed-refusal,
  toolbar command state, editor-owned link affordance (no browser prompt),
  retained-selection image insertion with position mapping and
  resolve/cancel/reject/disable/unmount insert-once semantics, table
  commands, Escape-then-Tab focus escape, internal placeholder.
- Shells: `RichTextEditor.svelte` / `RichTextEditor.tsx` and
  `RichTextRenderer.svelte` / `RichTextRenderer.tsx` (SSR-safe mount/update/
  destroy; renderer has no contenteditable state), `src/rich-text.ts`
  entries, `./rich-text` manifests.
- Distribution: `shell-contract.ts` rich-text entry + 19 TipTap externals;
  spec 070 dependency ownership adds the rich-text graph; core styles count
  168 → 169; React export count 180 → 181.
- Contracts/specs/notices: RichTextEditor §9 controlled-sync and
  focus-escape notes, spec 070 dependency ownership, `THIRD_PARTY_NOTICES.md`
  TipTap 3 / ProseMirror / linkifyjs pins.

## Validation

Core 1296 pass (12 new rich-text tests). Component boards 2906 pass across
both shells (a11y + parity sweeps cover both rich-text components); 41
focused Svelte cases and 32 focused React cases; SSR 2 pass; packaging
proofs pass; distribution suites 63 pass; shell-build suites pass;
`svelte:package`, `react:package`, both preview builds clean;
`web-preview.ts` certification passes with all 23 falsification oracles;
docs drift selectors (contract, capability, react-prop, value-domain,
snippet, callback, spec, focus-ring) and drift selectors pass;
`git diff --check` clean.

## Remaining staged-admission limits

`web-admitted` only; no native placeholder or parity claim. Preview catalogue
admission is reserved generation-index closeout. Release and Desktop adoption
need separate authority (see the adoption-request handoff).

## Pre-existing red, unchanged (verified byte-identical versus dispatch head)

`audit:licenses` (`deny.toml` still claims bzip2, which no lockfile
resolves), `drift:roles` (invokes a script path that does not exist on
disk), `check:react-components` and `check:react-preview` (265 + specimen
pre-existing TS errors). None of these touch rich-text surfaces; recorded in
`PAPERCUTS.md` for the ordinary-JS repair lane.
