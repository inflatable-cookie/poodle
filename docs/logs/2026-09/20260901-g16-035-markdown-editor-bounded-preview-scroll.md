# g16.035 — MarkdownEditor Bounded Preview Scroll

Status: complete; merged
Date: 2026-09-01
PR: https://github.com/inflatable-cookie/poodle/pull/123
Merge: `8a15fcf5d131bca6b5fcd88e79e25e16527ae005`
Card: `docs/roadmaps/g16/035-markdown-editor-bounded-preview-scroll.md`
Handoff: `docs/handoffs/20260901-142220-g16-035-markdown-editor-preview-scroll.md`
Governing refs: `docs/contracts/components/markdown-editor.md`,
`docs/parity/markdown-editor.md`, `docs/contracts/001-working-rules.md`
Branch: `fix/g16-035-markdown-editor-preview-scroll`
Base: `main` at `701ab1c11046d760509d0aaed74ace03a9e50923` (handoff planning
base `4b87baaba` is an ancestor; current pushed main advanced before
implementation)

## Outcome

A long MarkdownEditor preview no longer dictates outer layout height. Under a
definite host height the shared stylesheet keeps the root at or below the host
(`max-height: 100%`, column flex, `min-height: 0` on body/preview) and the
preview pane owns vertical scroll. Short unconstrained content stays naturally
sized — no `height: 100%` default and no new public height prop.

Native render mirrors the same ownership with existing node vocabulary:
preview `LayoutDirection::Column` + `LayoutOverflow::Scroll`, body/preview
`min_height: 0` / `fill_height`, root `min_height: 0` (editing `minHeight` on
the textarea pane). Source-text versus rendered-HTML remains Tier-3 and
untouched.

## Diagnosis

`overflow-y: auto` alone was not enough. Root was not a shrinkable column flex,
body/preview defaulted to content min-size, so intrinsic preview height
expanded the editor. The host must supply a definite height; the component
supplies the shrink/scroll chain.

## Evidence

- Focused Svelte/React stylesheet proofs (injected shared CSS under vitest).
- `effigy test:markdown-editor-preview-scroll` — Chromium + WebKit, both shells:
  constrained preview/split stay in a 16rem host, preview `scrollTop` moves,
  siblings stay put, short unconstrained preview stays natural.
- `poodle-render` unit tests for preview Scroll + body shrink.
- Mounted GPUI regression
  `markdown_editor_bounded_preview_scrolls_under_host_height`: unmodified
  production renderer inside a fixed 16rem host; fixture stamps runtime ids +
  synthetic overflow only (no sizing/overflow mutations, no declaration
  assert); real wheel input; clipped fixture tail activates only after scroll.
- Oracle falsification: planted pre-fix (removed CSS shrink chain; cleared
  native Scroll/body shrink). Probe failed on internal overflow / stuck
  `scrollTop`; vitest and render tests failed for the intended reasons.
  Second mounted falsification planted pre-fix native shrink/overflow
  (root `minHeight`, no body/preview shrink, preview Row + Visible); mounted
  GPUI regression failed on geometry —
  `preview sits under the toolbar inside the host: preview=2765 host=256` —
  before wheel/hit-test. Restored and reran green.

## Validation run

- `cargo test --manifest-path packages/render/Cargo.toml markdown_editor` — pass
- `cargo test --manifest-path packages/gpui/preview/Cargo.toml --test headless_regressions markdown_editor_bounded_preview_scrolls_under_host_height` — pass
- `effigy docs:check` — pass
- `effigy ci:rust` — pass
- `effigy ci:native` — pass
- `git diff --check` — clean
- Prior web suite (`effigy test:markdown-editor-preview-scroll`, `effigy ci:web`) unchanged and previously green

## Review follow-up

Northstar `oracle-gap` on PR #123 (twice): declaration-only native tests were
not enough; the first mounted draft still force-sized the editor in the fixture
and stopped falsification on a `Visible` vs `Scroll` assert. Second follow-up
keeps the production editor unmodified under the host, moves preview to Column
so vertical overflow is real, and falsifies on mounted geometry as above.

## Scope kept honest

Did not edit g16 README, generation-index, motion/g16.034 surfaces, or
drag-and-drop. No public API change. Web implementation left as accepted.
