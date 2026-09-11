# g18.013 — RichTextEditor toolbar controls

Status: ready for review
Date: 2026-09-11
Branch: `ns-e3a0e8cb-287c-447d-90c3-c226bca3d763`
Card: `docs/roadmaps/g18/013-rich-text-editor-toolbar-controls.md`
Handoff: `docs/handoffs/20260911-g18-013-rich-text-editor-toolbar-controls.md`
Governing refs: `docs/contracts/components/rich-text-editor.md`,
`docs/contracts/components/markdown-editor.md`,
`docs/contracts/components/icon.md`,
`packages/svelte/components/src/RichTextEditor.svelte`,
`packages/react/components/src/RichTextEditor.tsx`
Base: `origin/main` at `5a5dc31c6eefb442565c337d2115a3ee8882a19f`

## Outcome

The RichTextEditor toolbar is a compact, grouped Poodle control surface in both
web wrappers. Every admitted command renders as an `IconButton` with real
control chrome, an icon or a legible glyph (`H1`/`H2`/`H3`), a tooltip and
accessible name from one shared map, truthful `pressed` for toggle commands,
`disabled` for unavailable ones, and a `danger` tone for destructive table
removal. Consecutive same-group commands form intact labelled clusters that
wrap as units at constrained widths. The link editor's Apply and Remove actions
are ordinary Poodle `Button` chrome, and the keyboard flow (Enter submits,
Escape cancels, editor focus restored) is proven.

No public prop, command, feature, schema, or engine change. The `toolbar`
command-selection contract is unchanged; consumers still choose commands, not
icons or components.

## What changed

- One shared command-presentation map: `RICH_TEXT_COMMAND_PRESENTATION`,
  `RichTextCommandGroup`, and `RICH_TEXT_COMMAND_GROUP_LABELS` in
  `packages/core/src/rich-text.ts`. `RICH_TEXT_COMMAND_LABELS` and
  `RICH_TEXT_TOGGLE_COMMANDS` now derive from the map, so label, icon/glyph,
  group, toggle posture, and destructive tone have exactly one authority.
  Svelte and React maintain no independent visual vocabulary.
- Six narrowly required default Lucide icons: `strikethrough`,
  `list-ordered`, `square-code`, `table`, `between-horizontal-start`
  (add-row), `between-vertical-start` (add-column). Manifest, generated web
  modules, and render SVG assets regenerated through `effigy icons:build`;
  the core distribution inventory (`CORE_ICON_MODULES`) and the default-set
  ceiling test updated with the component-owned rationale (108 → 114).
- Svelte and React toolbars: grouped `role="group"` clusters inside the
  existing `role="toolbar"` root; `IconButton` (ghost, chrome size) per
  command; heading controls render typographic glyphs; `delete-table` renders
  `danger`; roving arrow-key focus, active/available snapshots, command
  execution, and editor focus restoration preserved.
- Toolbar CSS rebuilt on MarkdownEditor's language: compact elevated surface,
  cluster gaps, group dividers, density variants, wrapping clusters; the old
  link-like `.poodle-rich-text-editor__toolbar-button` styles are gone.
- Link editor Apply/Remove converted to `Button` (secondary; ghost danger for
  the destructive remove).
- Paired specimens expose default, active/selection, disabled, table-context,
  explicit-subset, and images-enabled postures; paired preview tests cover
  them. The images on/off specimen toggle now carries explicit identical
  chrome in both galleries (inline-flex, fit-content, bordered): the raw
  button previously inherited each gallery's page-level layout and the pair
  pixel-diffed at 1.072% on eclipse-compact-md — a pre-existing g18.008
  delta this branch repairs. Paired component tests cover control chrome,
  names, glyphs, clusters, pressed/disabled truth, exact subsets, destructive
  tone, and the link editor keyboard journey. Core tests pin the presentation
  map, its derived exports, the icon-name existence, and the destructive-tone
  reservation. Selector note: `data-command` moved from the button to a
  wrapper span (IconButton takes no data attributes); the old bespoke
  `data-pressed` attribute is replaced by IconButton's own `data-pressed`,
  and the unused `apply-link`/`remove-link` data hooks are gone (Apply and
  Remove are identified by `poodle-button` chrome inside the link editor).
- `docs/contracts/components/rich-text-editor.md`: shared-map toolbar
  semantics and cluster wrapping added to §6 and §7.
- Nucleus M1/A1 receipt `source_commit` repinned (5ce59be43 →
  10f3f335a) across the manifest and all 58 receipts, following the
  g18.004 precedent: the six new SVGs under `packages/render/assets/icons`
  are inside the ledger SOURCE_PATHS, and no observation, capture, or
  V1/Lab bundle changed. Ledger regenerates with zero delta.

## Validation

- `effigy test:components`: 3967 passed; one pre-existing parallel-load
  timeout in `g15.024 agent-transcript` caption parity (passes in isolation;
  unrelated surface).
- `effigy test:core`: 1304 pass, including the new presentation-map tests.
- `effigy audit:icons`: verified 114 default icon names from lucide-static
  1.31.0.
- `effigy check:svelte`: 0 errors (after `svelte:package`; install-smoke
  needs built dist on a cold worktree).
- `check:react` / `check:react-preview` fail repo-wide on the pre-existing
  `Tree.tsx`/`BlockEditorBlock.tsx` and string/`ControlSize` backlog; zero
  errors in PR-touched files (verified by grep at the exact head, and by
  stash-run-pop on the clean tree).
- `effigy svelte:build` and `effigy react:build`: clean.
- `effigy docs:check`: full sequence exit 0.
- `effigy test:web-pack-install`: pass. `effigy svelte:surface-audit`: 171
  components with full coverage, 0 gaps.
- Browser/visual (headless sweep-tier comparisons with `--slug`, against
  healthy same-worktree previews): `rich-text-editor` pairs match Svelte vs
  React at the exact head — 2 compared, 0 failing on both axes — after the
  specimen toggle-chrome repair; before that repair the pair failed at
  1.072% on eclipse-compact-md, reproducing a delta that already exists on
  main. `rich-text-renderer` remains as on main: a 0.047% iceberg pixel
  delta and an intermittent svelte goto timeout, both reproduced on the
  stashed clean tree and unrelated to this branch. Full-sweep parity remains
  the ci:visual gate.
- `git diff --check`: clean.

## Remaining limits

Web-admitted only; native editor parity is unchanged and out of scope. The
table-context *available* states need a live cell selection, so specimens show
them disabled until a human enters the table; no engine selection API was
added. g18.011 stays held until this repair and g18.010 close; g18.006 stays
blocked; g18.009 stays held.

## Closeout

One open non-draft PR from the queue-owned branch at the clean pushed head;
independent exact-head review before merge. Never merge from the worker; no
release, publish, Desktop, or retained-task mutations.
