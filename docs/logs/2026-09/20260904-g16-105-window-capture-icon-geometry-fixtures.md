# g16.105 — Window-Capture Icon-Geometry Fixture Kind

Status: complete — awaiting orchestrator review
Date: 2026-09-04
Card: `docs/roadmaps/g16/105-window-capture-icon-geometry-fixtures.md`
Dispatch: `docs/roadmaps/dispatch.md` revision 7
Base: `14e2fc2b4` (`origin/main`)

## Outcome

`poodle-window-capture` accepts the closed `--icon-geometry` fixture kind.
It validates one candidate pair before a window is opened, accepts only the
two directions and seven named states, advances the existing internal plan at
explicit samples, and paints a resolved geometry node through the ordinary
GPUI node backend. It adds no static SVG, public Icon surface, Button change,
runtime change, or capture-transport change.

The `poodle.icon-geometry-visual-capture.v1` receipt records pair, direction,
state, policy, sample, resolved-frame SHA-256, fixed logical viewport,
transport, foreground evidence, and the Screen Recording requirement.

## Review oracle

| Invariant | Proof |
| --- | --- |
| Registry is closed | `menu-to-x` is rejected by the parser before any capture scene is constructed. |
| Sample is exact | midpoint receipt state is realised with sample `0.5`. |
| Reverse rebases | a second target issued after sample `0.5` produces a different resolved-frame hash from midpoint. |
| Teardown is clean | teardown clears the runtime and paints the empty container scene. |
| Button path untouched | existing window-capture binary tests remain green. |

## Validation

- `cargo test --manifest-path packages/gpui/preview/Cargo.toml --bin poodle-window-capture --features window-capture`: pass, 53 tests.
- `effigy regressions:native`: pass, 203 tests.
- `effigy check:gpui`: pass.
- `effigy docs:check`: pass.
- No `*-windowed`, native-visual, or capture selector was run.

The headless test build initially could not write Clang's Metal module cache
under `~/.cache`; the same headless command passed once that local compiler
cache was authorised. This is environment friction only, not a fixture or
runtime defect.

## Closeout

Reserved for the coordinator at merge: `docs/roadmaps/g16/README.md`,
`docs/roadmaps/generation-index.md`, and `docs/roadmaps/dispatch.md`.
