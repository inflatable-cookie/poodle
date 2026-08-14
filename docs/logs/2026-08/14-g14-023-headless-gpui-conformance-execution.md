# 14 — g14.023 Headless GPUI Conformance Execution

Batch log, 2026-08-15. Card: `docs/roadmaps/g14/023-headless-gpui-conformance-execution.md`.

## What changed

The GPUI leg of the conformance kernel moved from a foreground AppKit runner
to GPUI 0.2.2's in-memory test platform (`TestAppContext`,
`VisualTestContext`, `TestWindow`). The windowed binary, the `--windowed`
foreground guard, AppKit activation/calibration/first-click-retry, and the
windowed task family are deleted. `conformance:complete` and `ci:conformance`
now execute the full active cohort headless in any local worktree.

- `packages/gpui/preview/src/conformance_driver.rs` — one generic headless
  driver: mount, full repaint (the test platform never requests frames, so
  every draw invalidates the mount view), real backend focus registry blur,
  and pointer/drag/key events through `TestWindow`'s dispatch callback. No
  component name, part list, fixture, or assertion lives here.
- `packages/gpui/preview/tests/conformance_headless.rs` — the complete board
  (39 cases across Button/RangeSlider/Tabs + 17 primitive probes, reports
  written where the comparator reads them), focused driver tests (focus
  registry, pointer activation, keyboard activation, scrub press/drag/release
  order), and planted failures for an inert listener, wrong focus target,
  missing selected state, and broken drag/keyboard event order.
- `tasks/effigy.tasks.toml` — `conformance:test-gpui`, `conformance:complete`,
  `ci:conformance`; windowed selectors and `conformance:check-gpui` removed;
  `ci:native` now executes the headless board instead of compiling a bin.
- `scripts/run-conformance-board.ts` deleted; `conformance-cost.ts` counts the
  headless driver/board instead of the bin.
- Spec 066 evidence, conformance estate, g14 index, card status, and PAPERCUTS
  windowed entries updated.

## Before / after runtime

| Board | Before (windowed) | After (headless) |
| --- | --- | --- |
| Full GPUI cohort | ~15+ min, OS focus stolen, isolated-CI-only | 0.05 s test body; any local worktree, no window, no focus |
| CI gate | `ci:conformance-windowed` on isolated macOS only | `ci:conformance` = `conformance:complete`, identical locally and in CI |

`effigy conformance:complete` (authority checks + web + full GPUI execution +
renderer-neutral Rust + compare + primitive report): green. `ci:web`,
`ci:rust`, `ci:native`, `docs:check`, `git diff --check`: green.

## Before / after source cost (GPUI runner, LOC excluding comments)

| Piece | Before | After |
| --- | --- | --- |
| Generic driver (`conformance_driver.rs`) | 505 | 214 |
| Windowed CLI (`bin/conformance.rs`) | 284 | 0 (deleted) |
| Headless board + driver/planted tests | 0 | 369 |
| Button adapter | 246 | 209 |
| RangeSlider adapter | 260 | 233 |
| Tabs adapter | 303 | 284 |
| Primitive probes (GPUI) | 150 | 121 |
| Fixture adapter (unchanged) | 325 | 325 |
| Total | 2073 | 1755 |

Full mechanism per `effigy conformance:cost`: 10,775 LOC; 90,042 bytes of
generated fixture JSON; 130 LOC replaced hand-written source. The windowed
plumbing that is gone was negative-cost complexity: no behavior was lost.

## Notes

- `.github/workflows/ci-conformance.yml` still says GPUI needs a macOS window
  server; the workflow itself is untouched (runs `effigy ci:conformance`,
  now headless) and only its comment is stale.
- The `#[gpui::test]` macro from gpui-macros 0.2.2 crashes on current rustc
  (recursion-limit failure, then SIGBUS even on a trivial test), so the board
  drives `TestAppContext::single()` from plain `#[test]` fns with the macro's
  teardown mirrored (`run_headless`).
- `Window::dispatch_event` returns a crate-private type; input goes through
  `TestWindow::simulate_input` (`VisualTestContext::simulate_event`), which is
  the same real dispatch callback the platform would use.
- gpui reuses clean views across frames, so paint-time backend observations
  (focus canvases) only ran on some frames; every draw now invalidates the
  mount view for deterministic observations.
- Keyboard activation relies on gpui's KeyUp → click synthesis for focused
  clickable elements; the driver sends key down + key up pairs through the
  dispatch tree.
