# Handoff: Desktop adoption request — Poodle CodeEditor (g18.002)

To: Bovine Desktop owners
From: Poodle web components (g18.002 worker)
Date: 2026-09-10
Status: request — no Desktop change in this task

## What is ready for adoption

A web-admitted controlled `CodeEditor` for Svelte and React over pinned
CodeMirror 6 packages:

- `@inflatable-cookie/poodle-svelte/editor` → `CodeEditor` (+ contract types)
- `@inflatable-cookie/poodle-react/editor` → `CodeEditor`, `CodeEditorProps`

Semantics follow `docs/contracts/components/code-editor.md`: exact controlled
`value`, closed admitted language set (`plain-text`, `markdown`, `json`,
`yaml`, `toml`, `javascript`, `typescript`, `html`, `css`, `rust`, `shell`),
host-owned diagnostics, read-only/disabled, Tab focus/indent with Escape
exit, editor-local search, F8 diagnostic traversal, internal scrolling, and
the 2 MiB UTF-8 envelope. Engine types never cross the API; languages load
per-grammar chunks; root imports stay engine-free.

Desktop owns source identity, byte admission, language selection, diagnostic
meaning, drafts, revisions, saves, review/recovery policy, and any
Markdown-body/full-file projection. None of that enters the component API.

## Merge/release gate (exact)

1. Independent review accepts the worker PR from
   `ns-04d0b9b3-a9d6-46a6-82e7-c2b5bbc8bfe7` into `main` (worker never merges).
2. Chatterbox authorizes release/adoption authority on return (per the task's
   Next-task clause); release follows the certification lane, not this branch.
3. Only after (1) and (2): Desktop pins the released Poodle version and
   imports `./editor` directly. No adapter lives in Poodle.

## Evidence

- `docs/evidence/g18-002-code-editor-distribution.md` (measured bundle,
  behavior, and limit proofs)
- `docs/logs/2026-09/20260910-g18-002-codemirror-web-code-editor.md`
- Focused suites: `CodeEditor.test.*`, `TabsPinned.test.*`,
  `CodeEditorPackaging.test.ts`, core `code-editor`/`tabs-pinned` tests,
  mounted GPUI `tabs_pinned_partitions_refuse_crossing_through_mounted_gpui`.

## Staged-admission limits Desktop must respect

- Web-only admission: there is no GPUI/native CodeEditor, placeholder, or
  parity claim. Do not treat it as portable.
- Engine line-break ownership: CR/CRLF load as LF; astral/tab/trailing bytes
  are exact.
- Svelte syntax is not admitted and fails closed; request `plain-text`
  explicitly for anything outside the eleven languages.
