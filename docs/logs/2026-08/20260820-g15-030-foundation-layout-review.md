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
construction/axis evidence. **Eight pages keep unchanged, one page needed bounded
Sv/Rc specimen repair, and one page records a GPUI contract/runtime blocker.**
No contract, public API, component, shared-CSS, generated catalogue, or
infrastructure file moved outside specimen presentation.

The nine human-teaching verdicts are recorded in the existing audit rows in
`docs/roadmaps/g15/specimen-catalogue-audit.md`; the screening `keep` /
"no named defect" text was replaced, not extended with a second table. Totals
were recounted once for the ResizeHandle blocker (`keep` 55, blocker 1; GPUI
B +1 / A −1).

## Review round 1 (orchestrator, PR #55)

The first pass claimed "no stop condition fired" and was wrong on two counts;
both are addressed in this revision:

1. **Web specimen ARIA range must match the specimen bounds.** The repaired
   examples start at `120px` horizontal / `80px` vertical while passing only
   `ariaValueNow`, so the default `0–100` range made the initial
   `aria-valuenow="120"` invalid. Sv/Rc now pass `48–280` / `40–120` through
   `ariaValueMin` / `ariaValueMax`, and the focused regression asserts initial
   and updated ARIA range/value alongside pane geometry.
2. **GPUI keyboard/focus/value semantics are a routed component blocker, not a
   bounded keep.** The contract requires a focusable separator with
   `aria-valuenow/min/max` and Arrow/Home/End resize steps
   (`docs/contracts/components/resize-handle.md` §5–6). The render path wires
   drag only (`packages/render/src/resize_handle.rs`); the native specimen
   therefore cannot teach the same important keyboard/value evidence as web.
   The audit row now carries disposition `contract/runtime-blocker`, the GPUI
   grade drops to B, and totals are updated. The renderer fix is **not**
   implemented here; the orchestrator routes it separately.

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

### Repaired (1, Sv/Rc only)

- **`ResizeHandle`** — the interactive horizontal and vertical split sections
  were inert on web: handles sat between panes but nothing consumed
  `onResizeMove` / `onResizeStep`. Captions promised drag gestures the page
  could not demonstrate. Sv/Rc now keep pane size in specimen state, apply
  drag/keyboard deltas through the handle callbacks, and pass the specimen's
  actual bounds through `ariaValueNow/min/max`. Disabled sections stay static.
  Focused regression tests cover web keyboard steps and ARIA range/value.

### Contract/runtime blocker (1)

- **`ResizeHandle` (GPUI)** — the render path wires drag only; it never makes
  the node focusable, installs an `on_key` handler, or projects
  `ResizeHandleSpec.aria_value_*` into the node. The GPUI specimen drag wiring
  remains directionally sound but cannot close the keyboard/value parity gap.
  Recorded as `contract/runtime-blocker` in the audit; renderer work is routed
  separately and is not hidden as specimen work.

## Changed routes for review

Changed Svelte routes: `resize-handle`
Changed React routes: `resize-handle`
Changed GPUI routes: `resize-handle`

The operator explicitly waived the remaining live Svelte and React
`resize-handle` checkpoint when directing the orchestrator to fix the final
review issues and merge. GPUI evidence stays headless per the card.

## Changed files

- `packages/svelte/preview/src/specimens/ResizeHandleSpecimen.svelte`
- `packages/react/preview/src/gallery/specimens/ResizeHandleSpecimen.tsx`
- `packages/svelte/preview/test/g15-030-foundation-layout.test.ts`
- `packages/gpui/preview/src/node_compat.rs` — preview-only `ResizeHandle::on_resize`
- `packages/gpui/preview/src/specimens/resize_handle.rs`
- `packages/gpui/preview/src/specimens/mod.rs`
- `docs/roadmaps/g15/specimen-catalogue-audit.md` — nine human verdict rows and recounted totals

## Validation

- `bunx vitest run packages/svelte/preview/test/g15-030-foundation-layout.test.ts` — 2 passed
- `effigy catalogue:check` — passed
- `effigy check:svelte` — passed
- `effigy react:build` — passed
- `effigy docs:check` — passed
- `effigy check:gpui` — passed
- `effigy regressions:native` — 50 passed
- `cargo test --manifest-path packages/gpui/preview/Cargo.toml --bin poodle-preview resize_handle` — 2 passed
- `git diff --check origin/main...HEAD` — clean

No `*-windowed`, `test:native-visual`, browser, Jetstream, or release
selector ran.
