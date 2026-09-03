# g16.066 — GPUI Node Tooltip Lifecycle Runtime

Status: implementation-complete — PR pending orchestrator review
Type: shared native backend repair
Opened: 2026-09-02
Implemented: 2026-09-03
Depends on: current `Node.tooltip` contract; `g16.062` only for merge ordering
Governing refs: `nucleus-gpui-parity-programme.md`,
`065-tabs-native-tooltip-parity.md`, `../../contracts/components/tooltip.md`
Log: `../../logs/2026-09/20260903-g16-066-gpui-node-tooltip-lifecycle-runtime.md`
PR: #171

## Goal

Give every non-empty `Node.tooltip` one Poodle-owned GPUI lifecycle: show after
300ms, hide on pointer leave, focus departure, Escape, disablement, removal,
teardown, or supersession. Keep the existing string field and component APIs.

## Outcome

`poodle-gpui-node-backend` no longer maps `Node.tooltip` through GPUI 0.2.2
`.tooltip()`. One window-keyed runtime owns the 300ms timer, generation
supersession, paint-authority sweep, and overlay bubble. Empty strings stay
inert. IconButton and SegmentedControl keep their tooltip text and ordinary
activation/focus behavior. No public Node or component API changed.

## Fixed Boundary

GPUI 0.2.2 `.tooltip()` hardcodes a private 500ms delay and owns a private
hover-only `ActiveTooltip`. PR #169 proved that path cannot implement Poodle's
contract. Replace that mapping inside `poodle-gpui-node-backend` with one
backend runtime. Do not add a Tabs-only overlay, another public component, or
delay/dismiss props.

The runtime owns one pending or visible tooltip per mounted window. A new
eligible target supersedes the old generation. The target's existing element
identity, current `Node.tooltip`, disabled state, paint presence, focus state,
and window key path are the authority. Stale timers and events are inert.

## Acceptance

- A non-empty tooltip remains absent at 299ms and paints at 300ms.
- Leaving before or after show hides and cancels exactly once.
- Focus departure and Escape hide even while the pointer remains over target.
- Disablement, removal, teardown, and target supersession leave no late paint
  or task residue.
- Empty/absent tooltips stay inert. Multiple windows do not share ownership.
- IconButton and SegmentedControl keep their current tooltip text and ordinary
  activation/focus behavior.
- No public Node or component API changes.

## Review Oracle

| Invariant | Counterexample | Required proof |
| --- | --- | --- |
| Delay is Poodle-owned | reuse GPUI `.tooltip()` | 300ms mounted boundary fails |
| Terminal paths converge | leave only cancels pending | visible/focused cases leak |
| Generation is exact | A timer fires after B hover | A never paints |
| Paint is authority | target removed while pending | no late tooltip |
| Window ownership is isolated | hover in two live windows | overlapping mounts; B's frame does not cancel A's pending/visible tooltip |
| Teardown is production | `reset_focus_registry` as close | `remove_window` clears pending and visible; later frames do not paint |
| Existing consumers survive | fix only Tabs | IconButton and SegmentedControl regressions stay green |

## Writable Scope

`poodle-gpui-node-backend` tooltip interaction/paint/runtime, focused backend
tests, mounted GPUI tooltip fixtures, Tooltip contract wording if needed, this
card, one log, and new papercuts. Nucleus receipt/manifest/ledger refresh is
allowed only after this runtime is committed, via the real
`effigy regressions:native` selector. No Tabs projection, public Node field,
web Tooltip behavior, visual lab, Jetstream, workflow, release, or
windowed/native-visual selector.

## Validation

Run focused node-backend and mounted tooltip tests, retained IconButton and
SegmentedControl regressions, `effigy ci:rust`, `effigy ci:native`, `effigy
docs:check`, and `git diff --check origin/main...HEAD`. No windowed selector.

## Stop Conditions

Stop for planning if the runtime requires a public Node shape, a second
renderer-neutral overlay primitive, or GPUI internals outside Poodle's backend
authority. Do not encode 500ms or hover-only behavior as an accepted delta.

## Continuation

After merge, rebase the preserved PR #169 branch and resume `g16.065`. The
Tabs card owns disabled-tab projection and the Nucleus-shaped Tabs receipt.
