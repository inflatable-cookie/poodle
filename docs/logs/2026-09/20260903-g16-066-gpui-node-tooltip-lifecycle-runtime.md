# g16.066 — GPUI Node Tooltip Lifecycle Runtime

Status: implemented — awaiting orchestrator review
Date: 2026-09-03
Card: `docs/roadmaps/g16/066-gpui-node-tooltip-lifecycle-runtime.md`
Handoff: `docs/handoffs/20260902-234015-g16-066-gpui-tooltip-runtime.md`
Governing refs: `docs/contracts/components/tooltip.md`,
`docs/roadmaps/g16/nucleus-gpui-parity-programme.md`
Branch: `fix/g16-066-gpui-tooltip-runtime`
Worktree: `/Users/tom/.paseo/worktrees/1ugbsx1t/g16-066-gpui-tooltip-runtime`
Planning base: `200c15780` (`origin/main` after g16.062 / PR #170)

## Outcome

Every non-empty `Node.tooltip` now uses one Poodle-owned GPUI lifecycle
instead of GPUI 0.2.2 `.tooltip()`. Open delay is 300ms on hover or focus.
Dismiss is immediate on pointer leave, focus departure, Escape, pointer
press, disablement, removal, or target supersession. State is keyed by
`AnyWindowHandle`. Empty strings stay inert. No public Node or component
API changed.

## What landed

- Backend runtime: `packages/gpui/node-backend/src/tooltip.rs`.
- Interaction: hover/focus listeners and paint presence replace `.tooltip()`.
- Hosts: preview root, headless driver, and inset-evidence capture keep the
  existing `overlay_frame_begin` / `overlay_frame_end` pair; tooltip prepare
  and sweep ride those hooks.
- Contract: GPUI notes record that `Node.tooltip` is backend-owned.

## Falsification

| Invariant | Counterexample | Proof |
| --- | --- | --- |
| Delay is Poodle-owned | GPUI `.tooltip()` 500ms | mounted 299ms absent / 300ms painted |
| Terminal paths converge | leave only cancels pending | visible hide on leave, blur, Escape, press |
| Generation is exact | A timer fires after B hover | A never paints; B shows after its own 300ms |
| Paint is authority | target removed while pending | no late tooltip after 500ms |
| Window ownership is isolated | hover in two windows | sequential windows, distinct handles, no inherited paint |
| Existing consumers survive | fix only Tabs | IconButton and SegmentedControl regressions green |

## Validation

Focused:

- `cargo test --manifest-path packages/gpui/node-backend/Cargo.toml --lib` — 51 pass
- `cargo test --manifest-path packages/gpui/preview/Cargo.toml --test headless_regressions gpui_node_tooltip` — 4 pass
- `icon_button_activation_toggle_and_tooltip_through_mounted_pointer_and_keyboard` — pass
- `segmented_control_exclusive_focus_identity_and_disabled_paths` — pass
- `overlay_layers_survive_independent_conversions_within_one_frame` — pass

Required boards: `effigy ci:rust`, `effigy ci:native`, `effigy docs:check`,
and `git diff --check origin/main...HEAD`. No windowed or release selector.

## Limits

- Two-window isolation is proven with sequential mounts; `HeadlessDriver`
  holds `&mut TestAppContext`, so concurrent dual-borrow is not possible.
  Overlay frame begin/end stays process-global like the layer registry; a
  second window's paint can still reset another window's `painted_this_frame`.
- Tooltip bubble chrome is the previous hardcoded native styling, not the
  web Tooltip overlay recipe. Tabs projection stays `g16.065`.
- Window teardown relies on dropping the spawned timer with the window and
  `reset_focus_registry` in tests; there is no extra close-event hook.
- `effigy docs:check` fails the Nucleus receipt source pin: mounted tooltip
  fixtures live in `packages/gpui/preview`, which g16.062 pins to
  `d42b59950`. Refreshing those receipts is orchestrator-owned. No windowed
  or release selector was run. Merge and card completion remain
  orchestrator-owned.
