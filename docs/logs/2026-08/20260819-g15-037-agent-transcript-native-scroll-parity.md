# g15.037 — AgentTranscript native scroll parity

Date: 2026-08-19
Card: `docs/roadmaps/g15/037-agent-transcript-native-scroll-parity.md`
PR: #48, absorbed review prerequisite for `g15.024`
Accepted implementation head: `99b85ff5`

## Outcome

PR #48's fake native jump counter is removed. GPUI now owns one persistent
`TrackedScrollState`, mounts shared transcript content in a real bounded
viewport, detaches when the reader scrolls away, preserves the offset across
appends, and consumes a renderer-owned jump control's intent on the GPUI
thread. Jump reaches the actual bottom and re-arms following.

The ownership seam is deliberately small. `poodle-node` carries no offsets,
GPUI types, callbacks, or scroll commands. `poodle-render` owns transcript
content and the jump-control recipe. `poodle-gpui-node-backend` owns the
`ScrollHandle`, wheel observation, pinned latch, and imperative movement.

## Change class

- **Packages changed:** `poodle-render`, `poodle-gpui-node-backend`, internal
  `poodle-gpui-preview`
- **Public-intent entry points:** additive
  `poodle_render::agent_transcript::agent_transcript_jump` and
  `poodle_gpui_node_backend::{TrackedScrollState, TrackedScrollOptions,
  tracked_vertical_scroll}`
- **Compatibility:** additive, pre-1.0; no alias, fallback, or component prop
- **Downstream re-check:** GPUI AgentTranscript hosts should retain one scroll
  state per instance, pass `jump_handler()` to the shared jump recipe, and
  mount both nodes through `tracked_vertical_scroll`

## Evidence

- owner-local pin-threshold and jump-control recipe tests
- mounted in-memory GPUI regression over initial bottom, wheel detach,
  append-while-detached, real jump activation, hidden jump control, and resumed
  following
- `effigy regressions:native`: 50 passed
- `effigy check:gpui`: passed during implementation
- full headless `effigy qa`: passed, including the mounted GPUI regression,
  all 336 shared-render tests, all 50 native regressions, and packed web
  consumer evidence

All native evidence is headless. No windowed, native-visual, conformance,
Jetstream, or release selector ran.
