# g15.037 — AgentTranscript native scroll and jump parity

Status: **complete in PR #48** — the review prerequisite was resolved inside
the paused branch after explicit operator direction to fix and merge
Discovered by: PR #48 review of `g15.024`
Blocks: `g15.024`, `g15.026`, `g15.013`
Governing refs: `../../contracts/001-working-rules.md`,
`../../contracts/components/agent-transcript.md`,
`../../architecture/001-poodle-system-shape.md`

## Why

`g15.024` tried to teach AgentTranscript's contracted detached-scroll state in
the GPUI specimen. The current native component cannot do it:

- Svelte and React own a bounded viewport, detect whether the reader is pinned,
  stop following when the reader scrolls up, and show a jump-to-latest button.
- `AgentTranscriptSpec` already carries `is_virtualized`, `is_auto_scroll`,
  `pin_threshold`, and `jump_label`.
- `poodle_render::agent_transcript` renders an unbounded column and ignores the
  scroll/follow fields. It emits no viewport, pinned state, or jump control.
- the GPUI node backend can paint overflow scrolling, but `poodle-node` carries
  no tracked-scroll handle, offset observation, or scroll-to-bottom intent.
- GPUI 0.2.2 provides `ScrollHandle`, including offset/max-offset reads and
  `scroll_to_bottom()`. That is runtime capability, not yet a Poodle adapter
  seam.

PR #48's attempted workaround — a separate button that increments a specimen
counter — does not move the viewport or re-arm following. It is fabricated
evidence and must not land.

The component contract already classifies following and the jump affordance as
strict parity. Working rule 001 says an active-runtime absence is a gap to port,
not an accepted delta.

## Ownership Decision

Use a reusable GPUI node-backend viewport helper without adding a scroll
channel to `poodle-node`:

- `TrackedScrollState` lives in `poodle-gpui-node-backend`; the GPUI host
  retains one value per transcript instance. Its clones share the same
  non-Send `ScrollHandle` and pinned latch.
- `tracked_vertical_scroll` wraps renderer-owned content in the real viewport,
  observes the handle after GPUI wheel dispatch, and rebuilds only when the
  pinned posture changes.
- `poodle-render::agent_transcript_jump` owns the control's content, semantics,
  and token recipe. `TrackedScrollState::jump_handler()` is a send-safe atomic
  intent bridge; the next runtime-thread build consumes it and calls the real
  `scroll_to_bottom()`.
- While pinned, builds request bottom anchoring after layout. While detached,
  builds do not touch the offset, so appends stay put. Jump sets the latch and
  re-arms following.
- The in-memory GPUI regression reads the real handle through the state and
  drives wheel and pointer input through GPUI's dispatch tree.

This is smaller than the proposed renderer-neutral node channel: no GPUI type,
offset, callback, or transient state enters the node vocabulary or component
spec. It is more reusable than a preview-only AgentTranscript wrapper and keeps
runtime mechanics at the runtime boundary.

## Required Behaviour

- AgentTranscript owns or is given one bounded vertical viewport in GPUI.
- Initial and explicitly re-armed following lands at the latest block.
- Reader scroll away from the bottom enters detached state and preserves the
  offset while new blocks append.
- Detached state shows one real jump control using `jump_label`.
- Activating the jump control moves the real viewport to the bottom, hides the
  control, and re-arms following.
- `End` reaches the same final state when the runtime can route it.
- empty transcripts never show the jump control.
- grouping, disclosure, activity, size, density, and accessibility semantics
  remain unchanged.

## Evidence

- owner-local pure tests for the pinned predicate and any new renderer state;
- mounted in-memory GPUI regression that:
  1. renders enough mixed-height blocks to overflow;
  2. proves the initial bottom position;
  3. scrolls away and observes the jump control;
  4. appends while detached without losing the offset;
  5. activates the real jump control and observes bottom position plus hidden
     control;
  6. appends again and observes following;
- a focused GPUI AgentTranscript specimen assertion over mounted behavior, not
  source-token regexes or click counters;
- `g15.024` then rebases, removes its simulated host button, and teaches the
  real capability in `Streaming and detached scroll`.

All GPUI evidence must use the in-memory test platform. No OS window may open or
take focus.

## Writable Scope

Final scope follows the approved seam. Expected surfaces are bounded to:

- `packages/contracts/node` only if the reusable node channel is chosen;
- `packages/contracts/components/src/agent_transcript.rs` where native state or
  handlers need renderer-neutral declaration;
- `packages/render/src/agent_transcript.rs` and focused tests;
- `packages/gpui/node-backend` or `packages/gpui/adapter`, not both without a
  written reason;
- `packages/gpui/preview/src/node_compat.rs` and AgentTranscript specimen/test
  surfaces;
- AgentTranscript contract/runtime notes if the ownership explanation changes;
- one batch log and the release gap register.

No Svelte or React component implementation change is expected. Jetstream
remains program-deferred and is not a validation dependency.

## Acceptance

- [x] The approved ownership seam is recorded before implementation begins.
- [x] GPUI demonstrates the full detached-scroll state machine on a real
      viewport.
- [x] Jump activation changes actual scroll state; a counter, label swap, or
      pre-rendered posture is not evidence.
- [x] Shared Rust composition remains the source of component structure and
      recipes; GPUI owns only runtime scroll mechanics.
- [x] Headless mounted evidence covers detach, append-while-detached, jump, and
      resumed following.
- [x] `g15.024` removes its workaround and completes the exact teaching
      outline without a runtime-specific lie.

## Outcome

The fake ScrollShell/counter row is gone. The native specimen uses the real
tracked viewport. Owner-local pin-predicate evidence and the mounted 50-test
native board cover initial following, detach, offset preservation across an
append, real jump activation, hidden control after jump, and resumed following.
No OS window, node-vocabulary change, public component prop, or Jetstream work
was introduced.

## Stop Conditions

- The repair becomes a second cross-runtime observation or conformance plane.
- The node vocabulary starts carrying GPUI types or scroll physics.
- A public API change is proposed before its cross-runtime meaning is settled.
- The mounted test cannot observe real scroll position on GPUI's in-memory
  platform.
- The implementation needs unrelated transcript windowing, markdown, or
  Jetstream work.

## Validation

- focused AgentTranscript Rust and GPUI tests
- `effigy check:gpui`
- `effigy regressions:native`
- `effigy docs:check`
- `git diff --check origin/main...HEAD`

Headless only. Do not run windowed, native-visual, conformance, Jetstream, or
release selectors.
