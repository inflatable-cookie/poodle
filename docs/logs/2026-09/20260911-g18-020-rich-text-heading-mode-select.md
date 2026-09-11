# g18.020 — RichTextEditor heading mode select

Status: ready for review
Date: 2026-09-11
Branch: `ns-276cd890-5904-4c79-a08f-253c704e4185`
Card: `docs/roadmaps/g18/020-rich-text-heading-mode-select.md`
Handoff: `docs/handoffs/20260911-121500-g18-020-rich-text-heading-mode-select.md`
Governing refs: `docs/contracts/001-working-rules.md`,
`docs/contracts/components/rich-text-editor.md`,
`packages/core/src/rich-text.ts`, `packages/core/src/select.ts`,
`packages/svelte/components/src/rich-text-engine.ts`,
`packages/react/components/src/rich-text-engine.ts`
Base: `origin/main` at `a0ef2dc7b` (g18.018 closeout; both prerequisites merged)

## Outcome

RichTextEditor's separate H1–H3 toolbar buttons are replaced by one compact
Poodle text-mode Select in both wrappers. The public command configuration
stays granular and now admits `heading-1` through `heading-6`; each consumer
chooses exactly which levels its toolbar offers, and a sparse list projects as
one selector of exactly those levels.

Heading levels are real document behavior, not toolbar labels. The shared
ProseMirror schema, structural validation, engine commands, and renderer all
grew to H4–H6 from the one `RICH_TEXT_HEADING_LEVELS` registry. H7 and
malformed levels still fail closed before any partial mount. The renderer
serializes all six levels through the same schema the editor uses.

The selector's trigger shows Normal text, the active `Heading N`, or Mixed for
a selection spanning different block modes. Choosing a level sets it exactly —
choosing the already-active level keeps it — and choosing Normal text converts
eligible blocks back to paragraphs. One choice is one document-changing
transaction, and caret, selection, focus, history, and controlled echoes all
survive. The trigger is one toolbar roving stop; while the listbox is open the
toolbar consumes its arrow keys so the listbox owns the journey.

## What changed

- `packages/core/src/rich-text.ts`: `RichTextCommand` gains `heading-4`,
  `heading-5`, `heading-6`; the headings feature, command list, automatic
  toolbar order, and shared presentation map (labels H4–H6, glyphs, group
  `headings`, no pressed-state toggle) follow. One registry now carries the
  heading level domain: `RICH_TEXT_HEADING_LEVELS`,
  `richTextHeadingCommand`, `richTextHeadingCommandLevel`,
  `isRichTextHeadingCommand`. `RichTextHeadingMode` (normal / heading / mixed),
  the selector's Normal and Mixed values and labels, `richTextHeadingModeLabel`,
  `richTextHeadingOptions`, and `projectRichTextToolbar` (all heading commands
  collapse into one `heading-select` item at the first heading position, other
  commands keep their order) are the shared projection both wrappers and both
  engines read. Exported through the core barrel and both `rich-text` package
  entries.
- `packages/svelte/components/src/rich-text-engine.ts` and the byte-identical
  React twin: admitted heading levels now come from
  `RICH_TEXT_HEADING_LEVELS`; the H7 refusal message says levels 1 to 6; the
  granular heading commands switch from `toggleHeading` to exact `setHeading`;
  `RichTextToolbarSnapshot` carries `headingMode`, recomputed from the live
  selection on every toolbar snapshot; the engine exposes `headingMode()` and
  `setHeadingMode(level | null)` (null = `setParagraph`). Both engines remain
  byte-identical.
- `packages/svelte/components/src/RichTextEditor.svelte` and
  `packages/react/components/src/RichTextEditor.tsx`: the toolbar renders
  `projectRichTextToolbar` items; the heading item is one Poodle Select with a
  custom trigger and custom option rows. The trigger keeps a fixed height and
  toolbar typography across Normal, Heading, and Mixed; menu options render the
  glyph, the plain label, and a per-level label type scale. The Select is
  disabled when the editor is disabled or read-only. Toolbar roving excludes
  Select chrome (`tabindex="-1"`) and consumes arrow keys while the heading
  listbox is open.
- `packages/core/src/select.ts`: the smallest reusable Select primitive gap.
  `selectVisibleOptions` no longer narrows by `query` for a list that is
  neither searchable nor freeform. Closing a non-searchable Select seeds
  `query` with the committed label so the trigger can display it; treating that
  label as a search term hid every other option from keyboard navigation while
  the menu still rendered them, so ArrowDown/Home/End could not move off the
  current value. Searchable and freeform lists still filter exactly as before.
  Focused core proof in `packages/core/test/select.test.ts`; the existing
  query-highlight case now states `searchable: true`, which is the mode it was
  exercising.
- `packages/core/src/styles/rich-text.css`: heading-select trigger and bounded
  option rows, with per-level option label scales (`normal`..`6`).
- Specimens: `web-editor-documents.ts` gains `RICH_TEXT_HEADING_DOCUMENT` (all
  six levels plus a paragraph) and `RICH_TEXT_SPARSE_HEADING_TOOLBAR` (Normal +
  H2 + H4 only). The paired `RichTextEditorSpecimen` gained a "Text modes"
  group (full selector and a constrained-width sparse selector); the paired
  `RichTextRendererSpecimen` gained a "Heading levels" group. The g18.013
  toolbar-presentation tests now assert one Select trigger and zero separate
  heading buttons.
- Tests: new paired component cases per wrapper and a new browser fixture +
  probe under `test/rich-text-heading-mode/`, wired as
  `test:rich-text-heading-mode` (chromium + webkit). Three new selectors in
  `tasks/effigy.tasks.toml`; not composed into `ci:web`, matching the sibling
  rich-text probes.
- `docs/contracts/components/rich-text-editor.md` already carried the g18.020
  contract language from planning; no contract edit was needed.

## Planted regression

The new cases bind the repair directly: without the core registry/projection
extension the `heading-4`..`heading-6` commands and the one-selector projection
do not exist; without the engine extension H4–H6 refuse and no `headingMode`
exists; without the wrapper composition the heading levels stay separate
buttons with no Normal/Mixed trigger. The pre-change suites asserted exactly
those old shapes (H4 refused, `[data-command="heading-1"] button`, 20 commands,
12 toggles), so the paired core, component, and browser checks fail closed on
the old behavior and pass only against the new contract.

## Explicitly not done

- No composite `text-mode` command and no public `paragraph` command: Normal
  text is selector chrome (`RICH_TEXT_HEADING_NORMAL_VALUE`), not a command.
- No arbitrary extensions, toolbar slots, H7+, engine object exposure, Markdown
  conversion, image/embed work, native parity, release, Desktop, workflow, or
  consumer change.
- g18.011 stays gated behind this repair; g18.006/g18.009/g18.012 and release
  state are untouched.

## Validation

- `effigy test:core` — 1376 pass, 0 fail (core `rich-text` 22, `select` 16).
- `effigy test:components` / `bunx vitest run` — 415 files, 4116 pass, 8
  skipped, 0 fail.
- Focused: Svelte `RichTextEditor.test.ts` 70 pass, React `RichText.test.tsx`
  62 pass; both `g18-013 rich-text toolbar presentation` tests pass.
- `effigy test:rich-text-heading-mode` (direct `bun` run) — all Chromium and
  WebKit checks passed for both Svelte and React: one selector and zero heading
  buttons, full Normal + H1–H6 menu, sparse Normal + H2/H4 menu, Normal /
  Heading / Mixed trigger with an accessible name, exact set-instead-of-toggle
  with one change and surviving focus/caret/history through the host echo,
  stable trigger height, bounded option rows with an H1 scale preview, a
  constrained pane that never forces page overflow, and the roving/listbox
  keyboard journey.
- `effigy core:build`, `effigy svelte:package`, `effigy react:package` — clean.
- `effigy check:svelte-components` — 0 errors (4 pre-existing warnings).
  `effigy check:svelte-preview` — 0 errors (6 pre-existing warnings).
- `effigy svelte:build`, `effigy react:build` — clean (pre-existing chunk-size
  warnings only).
- `effigy check:react-components` / `check:react-preview` — the same
  pre-existing backlogs recorded by g18.018 (DOM ref generics, `ControlSize`
  specimen string typing); zero RichText-related errors.
- `effigy docs:lint` — pass.
- `git diff --check` — clean.

## Continuation

One non-draft PR from the queue-owned branch, exact-head review, then merge by
the orchestrator. g18.011 remains the next gated task; no release, Desktop,
native, or retained work starts from this log.
