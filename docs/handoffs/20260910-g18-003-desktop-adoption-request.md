# Handoff: Desktop adoption request — Poodle RichTextEditor / RichTextRenderer (g18.003)

To: Bovine Desktop owners
From: Poodle web components (g18.003 worker)
Date: 2026-09-10
Status: request — no Desktop change in this task

## What is ready for adoption

A web-admitted controlled rich-text pair for Svelte and React over pinned
TipTap 3 and ProseMirror packages:

- `@inflatable-cookie/poodle-svelte/rich-text` → `RichTextEditor`,
  `RichTextRenderer` (+ public ProseMirror JSON carrier types, feature and
  command identifiers, `RICH_TEXT_STANDARD_FEATURES`)
- `@inflatable-cookie/poodle-react/rich-text` → the same surface

Semantics follow `docs/contracts/components/rich-text-editor.md`: one
ProseMirror document authority (host-controlled JSON, no Poodle schema),
curated composable feature modules (formatting, headings, links, lists,
blockquote, code block, horizontal rule, tables in the standard profile;
images opt-in per project), curated toolbar commands only, controlled
no-echo updates with fail-closed invalid-document refusal, an editor-owned
async `requestImage` asset seam (no uploads, no storage), a matching
read-only renderer, table keyboard navigation with an Escape-then-Tab focus
escape, and the 2 MiB / 10,000-node document envelope. Engine types never
cross the API; root, `./markdown`, and `./editor` imports stay engine-free.

Desktop owns document envelopes, feature selection, image sources and CSP,
persistence, drafts, revisions, saves, review/recovery policy, and any
Markdown or HTML conversion. None of that enters the component API.

## Merge/release gate (exact)

1. Independent review accepts the worker PR from
   `ns-edb2d303-38c9-44a8-aa2c-a6ff5821643e` into `main` (worker never
   merges).
2. Chatterbox authorizes release/adoption authority on return (per the
   task's Next-task clause); release follows the certification lane, not
   this branch.
3. Only after (1) and (2): Desktop pins the released Poodle version and
   imports `./rich-text` directly. No adapter lives in Poodle.

## Evidence

- `docs/evidence/g18-003-rich-text-distribution.md` (measured bundle,
  behavior, and limit proofs)
- `docs/logs/2026-09/20260910-g18-003-tiptap-prosemirror-rich-text-editor.md`
- Focused suites: `RichTextEditor.test.ts` (svelte, 41),
  `RichText.test.tsx` (react, 32), `RichTextPackaging.test.ts`,
  `RichTextSsr.test.ts`, core `rich-text.test.ts`, distribution build
  suites, `web-preview.ts` certification.

## Staged-admission limits Desktop must respect

- Web-only admission: there is no GPUI/native rich-text surface, no
  placeholder, and no parity claim. Do not treat it as portable.
- Documents are portable only between consumers that enable a compatible
  feature set; Poodle never strips unsupported content to manufacture
  compatibility.
- Embeds, mentions, custom nodes, collaboration, comments, tracked changes,
  uploads, and Markdown/HTML conversion are outside v1 and require later
  contract work.
- Paste parses only through the active schema; no lossless-import claim.
- Image insertion delegates one asset choice per request through
  `requestImage`; Poodle refuses executable URL schemes and requires string
  `alt` (empty means decorative).
