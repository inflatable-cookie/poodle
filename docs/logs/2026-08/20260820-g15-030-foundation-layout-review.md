# g15.030 — Screen-clear review: foundation layout

Date: 2026-08-20
Card: `docs/roadmaps/g15/030-review-foundation-layout.md`
Handoff: `docs/handoffs/20260820-215106-g15-030-review-foundation-layout.md`
Parent: `docs/roadmaps/g15/027-screen-clear-human-review.md`
PR: #55

## Outcome

Third serial screen-clear review child. All nine owned foundation-layout pages
received the human teaching review against the carried rubric — live Svelte and
React source and routes, GPUI specimen source, and the `g15.026` headless
construction/axis evidence. **Eight pages keep unchanged, and one page needed
bounded specimen repairs across Svelte, React, and GPUI.** No contract, public
API, component, shared-CSS, generated catalogue, or infrastructure file moved
outside specimen presentation.

The nine human-teaching verdicts are recorded in the existing audit rows in
`docs/roadmaps/g15/specimen-catalogue-audit.md`; the screening `keep` /
"no named defect" text was replaced, not extended with a second table. Grades
and dispositions are unchanged, so totals were not recounted.

## Verdict inventory

### Unchanged (8)

| Page | Verdict |
| --- | --- |
| `Box` | keep — default teaches containment and padding; fixed dimensions and overflow clip are distinct; Sv/Rc paired verbatim; Gp mirrors all four sections |
| `Grid` | keep — three column layouts teach arrangement, not a prop matrix; Sv/Rc paired; Gp mirrors all three sections |
| `Region` | keep — default placeholder, labeled stack, and nav/toolbar/content composition teach boundary role; Sv/Rc paired; Gp mirrors all three sections |
| `ScrollShell` | keep — vertical and horizontal scroll with realistic overflow content; Sv/Rc paired; Gp mirrors both sections |
| `Separator` | keep — horizontal, vertical, and decorative separators with surrounding context; Sv/Rc paired; Gp mirrors all three sections |
| `Spacer` | keep — toolbar-style push-apart compositions teach flex spacing; Sv/Rc paired; Gp mirrors both sections |
| `Stack` | keep — column, row, alignment, and wrap teach arrangement within the section budget; Sv/Rc paired; Gp mirrors all five sections |
| `Surface` | keep — panel/canvas/elevated/no-border teach tone and container role; Sv/Rc paired; Gp adds renderer-owned border/padding/role sections while preserving the tone evidence |

### Repaired (1)

- **`ResizeHandle`** — the interactive horizontal and vertical split sections
  were inert on all runtimes: handles sat between panes but nothing consumed
  `onResizeMove` / `onResizeStep` (web) or `on_resize` (Gp). Captions promised
  drag gestures the page could not demonstrate. Sv/Rc now keep pane size in
  specimen state and apply drag/keyboard deltas through the handle callbacks.
  Gp adds `ResizeHandle::on_resize` in the preview compat wrapper and routes
  deltas through specimen state the same way `SplitView` does. Disabled
  sections stay static. Focused regression tests cover the web keyboard step
  and the native drag handler wiring.

No stop condition fired. No contract, public API, or component-semantic change
was required.

## Changed routes for review

Changed Svelte routes: `resize-handle`
Changed React routes: `resize-handle`
Changed GPUI routes: `resize-handle`

Operator live review of the Svelte and React `resize-handle` pages is required
before this child completes. GPUI evidence stays headless per the card.

## Changed files

- `packages/svelte/preview/src/specimens/ResizeHandleSpecimen.svelte`
- `packages/react/preview/src/gallery/specimens/ResizeHandleSpecimen.tsx`
- `packages/svelte/preview/test/g15-030-foundation-layout.test.ts`
- `packages/gpui/preview/src/node_compat.rs` — preview-only `ResizeHandle::on_resize`
- `packages/gpui/preview/src/specimens/resize_handle.rs`
- `packages/gpui/preview/src/specimens/mod.rs`
- `docs/roadmaps/g15/specimen-catalogue-audit.md` — nine human verdict rows

## Validation

- `bunx vitest run packages/svelte/preview/test/g15-030-foundation-layout.test.ts` — 1 passed
- `effigy catalogue:check` — passed
- `effigy check:svelte` — passed
- `effigy react:build` — passed
- `effigy docs:check` — passed
- `effigy check:gpui` — passed
- `effigy regressions:native` — 50 passed
- `cargo test --manifest-path packages/gpui/preview/Cargo.toml specimens::resize_handle::drag_tests` — 2 passed
- `git diff --check origin/main...HEAD` — clean

No `*-windowed`, `test:native-visual`, browser, Jetstream, or release
selector ran.
