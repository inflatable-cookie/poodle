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

### Jetstream interaction

**No engine work is needed.** That was the initial assumption and it is wrong:

- `jetstream-ui` dispatches clicks and has tests for it.
- The preview already feeds pointer state every frame —
  `set_pointer_state(x, y, down)` followed by `game_ui.process_input(...)`.

So a handler attached today would fire. The gap is purely that no Poodle
Jetstream component ever calls `.on_click`.

What makes it more than a sweep is the signature:

```rust
pub fn on_click(mut self, handler: impl Fn(&ClickEvent) + Send + Sync + 'static) -> Self
```

`Send + Sync + 'static` means a handler cannot borrow app state the way GPUI's
`cx.listener` closures do. Poodle's Jetstream components are also plain
functions — `js_button(spec, theme)` — not builders, so there is nowhere to hang
a handler without changing every call site.

Three shapes, none obviously right, which is why this is written down rather
than guessed at:

1. **Handlers on the spec.** Cleanest at the call site, but specs derive
   `Clone`, `Debug` and `PartialEq`, and closures break all three.
2. **Sibling functions** — `js_tool_call_interactive(spec, theme, handlers)`.
   Additive and non-breaking, but doubles the surface of a 151-component set.
3. **A handler bundle parameter** on the existing functions. One consistent
   shape, but it is a breaking change to every call site and every specimen.

The decision should be made once, for the whole target, before anything is
wired — the cost of this list is 151 components either way.

## Next

1. Pick the Jetstream handler shape, deliberately.
2. Wire the agent chat set as the proof, then sweep.
3. Burn down the 34 baselined GPUI handlers.
4. Revisit the GPUI click driver if `gpui` opens `DispatchEventResult`.
