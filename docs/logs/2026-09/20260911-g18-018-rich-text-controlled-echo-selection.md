# g18.018 — RichTextEditor controlled-echo selection preservation

Status: complete — merged as `08e377517af58a2033a145af2fe5c5875fb38215` (PR #248) on 2026-09-11 after exact-head independent review (PR comment `5635865293`, `ready_to_merge`) at `bcd412336a8b9035ee127127cfdf11f89df2a735` with green rust/web checks
Date: 2026-09-11
Branch: `ns-6bb46544-966a-4af0-af7c-e53846ad0407`
Card: `docs/roadmaps/g18/018-rich-text-controlled-echo-selection.md`
Handoff: `docs/handoffs/20260911-g18-018-rich-text-controlled-echo-selection.md`
Governing refs: `docs/contracts/001-working-rules.md`,
`docs/contracts/components/rich-text-editor.md`,
`packages/svelte/components/src/rich-text-engine.ts`,
`packages/react/components/src/rich-text-engine.ts`
Base: `origin/main` at `417b81472a0d6aa3da8de72c5b8d0335774379a3`
(refresh: rebased past g18.012/g18.014; the only conflict was
`tasks/effigy.tasks.toml`, resolved by keeping their language-registry and
image-policy selectors beside the new controlled-echo selectors)

## Outcome

An accepted controlled echo of a `RichTextEditor` user transaction is now a
true no-op in both web wrappers. Both public specimens echo each `onChange`
document straight back through `value`; the engine still treated every fresh
JSON object as a host replacement and routed it through TipTap `setContent`,
which replaced the document and moved the caret to its end after every typed
character.

The private engines now compare controlled values as ProseMirror document
semantics, not JavaScript object identity or JSON key order. When the incoming
host value matches the live engine document, the engine advances acceptance
bookkeeping only: it does not call `setContent`, does not remount, and leaves
caret, non-collapsed selection, focus, scroll, undo history, and IME composition
where the user left them. Typing, selection replacement, paste, and IME commit
in the middle of a multi-block document keep the exact advancing caret. A host
value that genuinely differs from the live document remains authoritative and
still replaces or rejects the local edit without a callback echo, including a
delayed or stale value.

No public prop, command, feature, schema, event payload, or engine export
changed. Both engine files remain byte-identical.

## What changed

- `rich-text-engine.ts` (Svelte and React, identical): one private canonical
  JSON serializer (`canonicalRichTextJson`, sorting object keys and preserving
  array order) replaces `JSON.stringify` for the `lastHostJson` and
  `lastAcceptedEngineJson` bookkeeping and for the controlled-value comparison.
  The "different host value" branch now first proves the incoming value differs
  from the live engine document; when it matches, it records the accepted echo
  and returns without `setContent`. The prior-value-rejection and host-revert
  branches are unchanged apart from the canonical comparison.
- `packages/svelte/components/test/RichTextEditor.test.ts` and
  `packages/react/components/test/RichText.test.tsx`: seven paired cases per
  wrapper drive the real engine through the public DOM/state surface and echo
  the emitted document back exactly as the specimens do —
  - focus and the exact advancing caret through three middle-of-block
    same-object echoes (spy proves `editor.commands.setContent` is never
    invoked);
  - a structurally identical echo with every object key order reversed;
  - a non-collapsed `" beta"` replacement followed by a paste, both keeping the
    resulting caret;
  - an IME composition commit (DOM mutation + `compositionend`) that emits one
    callback and keeps the caret after the composed character;
  - undo and redo through the real toolbar controls after the echoed typing;
  - a genuinely different host value still replacing the document (spy sees
    exactly one `setContent`, zero callbacks);
  - a delayed stale host value winning over newer local state.
  The React cases use a small stateful `EchoingEditor` host so the echo journey
  is the realistic controlled wrapper, not a manual re-render.
- `packages/svelte/preview/test/g18-008-web-editor-preview-specimens.test.ts`
  and the React counterpart: one paired specimen journey each, typed entirely
  through the DOM (composition commit) against the actual
  `RichTextEditorSpecimen` host state, asserting the caret offset, the paragraph
  text, and the host JSON readout after every character.
- `test/rich-text-controlled-echo/` (new): paired Svelte + React browser
  fixture and a headless Chromium + WebKit probe (`test:rich-text-controlled-echo`)
  with 9 checks per framework per browser — repeated real keystrokes at an
  exact mid-block caret through the immediate controlled echo, real IME
  insertion, real undo/redo through the toolbar, and a non-collapsed
  replacement. happy-dom can only approximate a contenteditable; real caret
  motion, real typing, and real history commands are what the operator-visible
  defect moved.
- `tasks/effigy.tasks.toml`: the three new browser selectors beside the
  existing CodeEditor browser probes. Not composed into `ci:web`, matching that
  precedent.
- `docs/contracts/components/rich-text-editor.md`: controlled equality is
  documented as semantic document equality (not object identity or key order),
  an accepted echo as a no-op, and delayed/stale host values as authoritative.

## Planted regression

With the two engine changes stashed at the rebased head, the five
caret/selection/IME/history journeys fail in each wrapper (the caret moves to
the document end after the first character and later characters append there),
both preview specimen journeys fail on the caret offset, and the Chromium
browser probe fails its per-character caret and IME checks. With the change
applied, all pass.

## Explicitly not done

- No public API, prop, event payload, engine export, package dependency,
  workflow, release, Desktop, native, or GPUI change.
- No toolbar or image-policy redesign, no uncontrolled mode, no selection
  callbacks, no new schema or commands.
- g18.011 remains gated behind this repair and the remaining tasks; g18.006
  stays paused and g18.009 waits on it. Merged as `08e37751` (PR #248);
  nothing was released, published, or tagged.

## Validation

- `effigy test:components` — 415 files, 4110 pass, 4 skipped.
- Focused suites at the rebased head: Svelte `RichTextEditor` 65 pass, React
  `RichText` 57 pass (7 new paired cases per wrapper); Svelte and React
  `g18-008-web-editor-preview-specimens` 7 pass each (1 new specimen journey
  per preview).
- `effigy test:rich-text-controlled-echo` — all Chromium and WebKit checks
  passed for both frameworks (36 checks).
- `effigy core:build`, `effigy svelte:package`, `effigy react:package` — clean.
- `effigy svelte:build`, `effigy react:build` — clean (pre-existing chunk
  size warnings only).
- `effigy check:svelte-components` — 0 errors (4 pre-existing warnings).
  Scoped `tsc` over the new React rich-text test and its imports — 0 errors.
  `effigy check:react-components` — 12 errors, all pre-existing in untouched
  files (`ModelConnectionPicker.tsx`, `TextInput.tsx`, `Tree.tsx`,
  `BlockEditorBlock.tsx`), the same baseline the g18.016 log records.
- `effigy docs:lint` — pass.
- `effigy docs:check` — pass.
- `git diff --check` — clean.

## Continuation

Merged. Ready g18.020 converges on the held `g18.011` web-editor acceptance
sweep, then operator acceptance and retained g18.006. No release, Desktop,
native, or retained task work starts from this log.
