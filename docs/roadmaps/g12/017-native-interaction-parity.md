# g12.017 — Native Interaction Parity

**Status: in progress.** GPUI is interactive and now gated. Jetstream is not
wired at all, and the reason is smaller than it looked.

## Problem

Poodle's contracts asserted a "shared render-only posture across all native
components" as the justification for a series of deltas: no selection, no
expansion, no toggling on either native target.

That claim was false, and it was load-bearing — it appeared in eleven contracts
and had been repeated into six new ones without anyone checking it.

Counting, rather than assuming:

| Target | Components wiring a click | Builder supports it |
|--------|---------------------------|---------------------|
| GPUI | 71 of ~97 | yes |
| Jetstream | 0 of 151 | yes — `JsEl::on_click` |

GPUI had been interactive the whole time. Jetstream was not, but not because
anything prevented it.

## What Was Actually Broken

Worse than the false claim: **35 GPUI components accepted a handler and never
read it.** `Stepper::on_change`, `Calendar::on_select`, `Select::on_change`,
`DatePicker::on_select`, `Tabs::on_change` — each a builder that type-checks,
compiles, sets a field, and does nothing. The pointing-hand cursor promised a
click; nothing happened when you made one.

An API that lies is worse than one that is missing. A missing handler is a
compile error at the call site; a dead one is a bug report six months later
from someone who assumed it worked.

## Done

- The agent chat set is interactive on GPUI: `ToolCall` output disclosure,
  `ToolCallGroup` expansion and per-call toggles, `ChangedFiles` expansion and
  file selection, `AgentQuestion` selection and dismissal, `Stepper` selection
  and re-run.
- `effigy drift:handlers` fails when a GPUI component accepts a handler it never
  reads, and runs in `ci:native`. The existing 34 are a baseline, not an
  allowlist: a new one fails immediately, and the gate also fails when a
  baselined handler starts being used, so a fix cannot leave a stale entry
  behind to rot back to dead.
- The contracts say what is true per target rather than repeating a posture.

## Not Done, And Why

### A headless click driver for GPUI

Blocked at both levels, and worth recording so nobody spends the afternoon
again:

- `Window::dispatch_event` is public but returns a **private type**, so it
  cannot be called from outside `gpui`. In-process synthesis is closed.
- `CGEvent.postToPid` posts without needing accessibility permission, and the
  window is found at its real origin — but the events never reach GPUI's run
  loop. A click on a theme swatch changed 0 of 6,459,616 pixels. Activating the
  app first changed nothing.

Driving a real GPUI click appears to need a change in `gpui` itself. Until then
"this component is interactive" is verified by the handler gate and by running
the preview, not by an automated click.

### Jetstream interaction — decided

**Decision: builder structs mirroring GPUI.** `Component::from_spec(spec, theme)`
then `.on_x(handler)`, with an `IntoJsEl` trait standing in for GPUI's
`IntoElement`. Same names, same verbs, same order as the GPUI target, so a
developer moving between the two native targets learns one vocabulary.

Nothing depends on the current shape yet, so this replaces it rather than
sitting alongside it. The three options were weighed and two rejected:

- **Handlers on the spec** — rejected. Specs derive `Clone`, `Debug` and
  `PartialEq`; closures satisfy none of them, and specs are the thing the
  contract-drift gate compares against a prop table. Putting behaviour in them
  would corrupt the one artifact that is pure data.
- **Sibling `*_interactive` functions** — rejected. Additive and non-breaking,
  but it doubles a 151-component surface and leaves two ways to render
  everything, one of which silently drops interaction.
- **Builder structs** — chosen. Handlers become additive: a component gains an
  event without any existing caller changing, which is exactly what the free
  functions could not do.

`on_click` requires `Send + Sync + 'static`, so a handler cannot borrow host
state the way GPUI's `cx.listener` closures can. Hosts capture an `Arc` — a
channel sender, a `Mutex`ed model, an atomic. That is the ordinary shape for an
immediate-mode UI where the tree is rebuilt every frame and nothing outlives it,
and it is not a workaround.

**The decisive argument is testability.** `GameUi` dispatches clicks with no
window, so a Jetstream component's interaction is provable in an ordinary unit
test — `element::click_probe::click_at` renders a tree, drives a real press and
release at a point, and lets the handlers run. GPUI cannot do this, which is
precisely how `Stepper` carried two handlers attached to nothing for weeks.

`ToolCall` is the reference implementation, with two tests: a click reaches the
handler with the right id, and a row with no output ignores clicks. Both were
checked for vacuity by removing the wiring — the positive test fails with
`on_toggle fired exactly once`.

### The migration

Free functions stay during the sweep: `IntoJsEl` wraps them, so each component
converts on its own rather than in one breaking commit. They go once the last
one is done.

Per component that is a struct with `from_spec`, an `IntoJsEl` impl delegating
to the existing render, and handler methods only where the contract has events —
mechanical, but 151 of them.

## Next

1. Convert the rest of the agent chat set to the builder shape, with a click
   test each.
2. Sweep the remaining components. Mechanical, but 151 of them: each becomes a
   struct with `from_spec`, an `IntoJsEl` impl wrapping the existing render
   function, and handler methods only where the contract has events.
3. Add a gate that fails when a Jetstream component with contract events has no
   click test — the Jetstream equivalent of `drift:handlers`, and stronger,
   because here it can assert the click actually lands.
4. Burn down the 34 baselined GPUI handlers.
5. Revisit the GPUI click driver if `gpui` opens `DispatchEventResult`.

The free functions stay for now: `IntoJsEl` wraps them, so conversion is
component-by-component rather than a single breaking commit. They go once the
sweep is done.
