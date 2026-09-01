# g16.035 — MarkdownEditor Bounded Preview Scroll

Status: ready
Opened: 2026-09-01
Depends on: current MarkdownEditor contract and active-cohort implementations;
independent of `g16.034`
Governing refs: `../../contracts/001-working-rules.md`,
`../../contracts/components/markdown-editor.md`,
`../../parity/markdown-editor.md`
Reported failure: a long rendered preview contributes its full intrinsic height,
so the MarkdownEditor expands its surrounding editor layout instead of keeping
the preview inside a scroll viewport

## Goal

Make MarkdownEditor respect a constrained host height. Short content remains
content-sized when the host does not constrain it; long preview content shrinks
inside the available editor body and scrolls vertically without moving adjacent
layout.

This is a bounded issue-fix lane. Diagnosis belongs to the worker. No new
public sizing prop, markdown parser, editor auto-size system, or application
layout policy is implied.

## Fixed Behavior Envelope

- In preview and split modes, rendered preview height cannot force the editor
  past a definite height supplied by its containing layout.
- The preview is the vertical scroll owner when its content exceeds its visible
  pane. Its scroll offset can change while the editor root and adjacent content
  remain stationary.
- In split mode, the textarea and preview share one bounded body height. Long
  preview markup does not make the preview taller than the editing pane or host
  constraint.
- Without a definite host height, short content keeps the existing natural
  layout and `minHeight` remains the editing-pane minimum. The repair must not
  make every editor viewport-height or introduce a fixed default maximum.
- Svelte and React remain structurally and visually aligned through the shared
  stylesheet. Shared Rust/GPUI must express the same shrink/scroll ownership
  where the existing node vocabulary supports it; any genuine runtime limit is
  recorded rather than hidden.
- Editing, mode switching, formatting actions, controlled/uncontrolled value
  behavior, accessibility labels, tokens, density, and size behavior stay
  unchanged.

## Ordered Work

1. Reproduce the failure with a long markdown document inside a definitely
   height-constrained host in both mounted web shells. Record root, body,
   preview, and adjacent-layout geometry plus scroll ownership.
2. Diagnose the intrinsic-size/constraint chain. Implement the smallest shared
   contract-valid repair; do not preselect a new public sizing API.
3. Align renderer-neutral Rust/GPUI shrink and overflow declarations when the
   existing layout vocabulary can express the same result. Stop if parity would
   require a new public prop or backend scrolling architecture.
4. Add biting focused evidence for constrained preview-only and split modes,
   plus a short unconstrained counterexample that stays naturally sized.
5. Reconcile the component contract, parity note, this card, and one September
   execution log. Run the bounded headless validation and open one PR.

## Acceptance

- A long preview in a constrained host has `scrollHeight > clientHeight`, its
  vertical scroll offset changes, and the MarkdownEditor root stays within the
  host before and after scrolling in Svelte and React.
- Content following the constrained editor keeps the same position when the
  preview mounts and scrolls.
- Split mode keeps the preview and textarea within one body height; long preview
  content does not determine the component's outer height.
- An unconstrained short document remains naturally sized. No viewport-height,
  fixed maximum, or new public height prop enters the diff.
- Shared CSS owns the paired web layout rule; the two shells do not fork
  component-specific sizing logic.
- Rust/GPUI either carries the equivalent existing overflow/shrink declaration
  with focused render/mounted proof or the worker stops on a real capability
  boundary. Source-text-versus-rendered-HTML parity remains outside this fix.
- Existing MarkdownEditor interaction and package checks stay green.

## Review Oracle

| Invariant | Smallest adversarial counterexample | Required proof |
| --- | --- | --- |
| Host constraint beats preview intrinsic height | a 16rem host contains a preview with dozens of headings and paragraphs | mounted Svelte and React geometry show editor height at or below the host and preview overflow with a mutable `scrollTop` |
| Scroll is internal | place a labelled sibling immediately after the editor, then scroll the preview | sibling and editor root rectangles are unchanged while preview `scrollTop` increases |
| Split panes share one body | short textarea value renders a much longer HTML preview | textarea and preview client heights stay bounded by the same body; deleting the shrink rule makes the preview or root grow |
| Natural sizing is preserved | render one short paragraph without a definite ancestor height | editor does not become viewport-height or acquire a fixed maximum; existing `minHeight` semantics remain |
| Paired and native posture stays honest | fix only one web shell, or claim GPUI scroll without an overflow declaration | shared stylesheet evidence covers both shells; Rust render proof names the exact declaration or records the capability stop |

Commit the real proof before planting the pre-fix behavior. Confirm each planted
counterexample fails for the intended geometry or scroll reason, restore, and
rerun green.

## Writable Scope

- `packages/core/src/styles/markdown-editor.css`;
- Svelte and React MarkdownEditor focused tests and the smallest mounted browser
  fixture/probe needed for real layout evidence;
- `packages/contracts/components/src/markdown_editor.rs`,
  `packages/render/src/markdown_editor.rs`, and a focused headless GPUI
  regression only if the existing node layout vocabulary can express the same
  bounded scroll ownership;
- `docs/contracts/components/markdown-editor.md`,
  `docs/parity/markdown-editor.md`, this card, one September execution log, and
  `PAPERCUTS.md` only for new execution friction.

The orchestrator owns `docs/roadmaps/g16/README.md` and
`docs/roadmaps/generation-index.md` integration after merge. Do not edit motion
policy/provider code, its eight pilot components, drag-and-drop surfaces,
versions, releases, workflows, downstream repositories, or Jetstream admission.

## Validation

Use Effigy selector discovery in the worker worktree. At minimum:

- focused Svelte and React MarkdownEditor tests;
- one bounded headless browser geometry/scroll probe in Chromium and WebKit;
- focused `poodle-specs` / `poodle-render` and mounted GPUI checks if Rust
  declarations change;
- relevant contract/style/prop drift checks;
- `effigy ci:web` and, when Rust changes, `effigy ci:rust` plus
  `effigy ci:native`;
- `effigy docs:check`;
- `git diff --check origin/main...HEAD` and exact diff-scope checks.

Do not run `*-windowed`, native visual, release, tag, publication, workflow
mutation, or sibling-repository commands.

## Stop Conditions

- Correct behavior requires choosing a new public `height`/`maxHeight` prop,
  changing `minHeight` meaning, or defining automatic textarea growth.
- The surrounding host supplies no definite constraint and an application-level
  default maximum would be required.
- GPUI parity requires a new scroll-state/backend architecture rather than the
  existing node overflow and layout vocabulary.
- The fix changes markdown rendering, editing semantics, mode ownership,
  accessibility, tokens, or a g16.034-owned surface.
- A mounted browser proof cannot distinguish internal scrolling from document
  growth.

## Continuation

On accepted merge, close this card and let the orchestrator reconcile the g16
front doors against whichever of `g16.034` and `g16.035` merges first. The
post-motion block-slider lane remains serial on `g16.034`, not on this fix.
