# g12.017 — Native Interaction Parity

**Status: in progress.** GPUI is interactive and now gated. Jetstream's shape is
decided and the agent chat set is converted; the remaining ~144 components are
a mechanical sweep.

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
- The agent chat set is interactive on Jetstream, with a click test per handler
  and `effigy drift:clicks` holding the rest of the sweep to the same bar.
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

### The agent chat set — converted

All seven are builders, and every event a contract names is wired and tested:

| Component | Handlers |
|-----------|----------|
| `ToolCall` | `on_toggle` |
| `ToolCallGroup` | `on_toggle`, `on_call_toggle` |
| `ChangedFiles` | `on_toggle`, `on_file_select` |
| `AgentQuestion` | `on_select`, `on_dismiss` |
| `AgentTranscript` | `on_tool_run_toggle`, `on_tool_call_toggle`, `on_changed_files_toggle`, `on_file_select` |
| `AgentMessage` | none — inline nodes flatten to text, so there is no link to click |
| `AgentQuestionRecord` | none — an answer the agent already has cannot be changed |

The last two are the point of the shape as much as the first five. A builder
with no handlers says the component has no events; a builder with a handler
that nothing reads is the GPUI defect this whole roadmap item started from.

Ten click tests, each driving a real press and release. All ten were checked for
vacuity in two passes — first stripping the leaf wiring, then stripping only the
forwarding — because the two failure modes are distinct: a leaf that ignores
clicks, and a parent that never passes the handler down. Six tests failed on the
first pass and the remaining three on the second, which is what tells you the
forwarding tests were measuring forwarding rather than riding on the leaf.

Tests click **by text** (`click_probe::click_text`) rather than by coordinate.
The first version of the group test hard-coded `y=52`, missed the toggle, and
would have silently started hitting the row above the first time a padding token
moved.

`AgentTranscript` forwards handlers into whichever block raises the event. It is
the only level that sees every block and the host holds all the expansion state,
so that is where a host attaches — and it is one level further than the GPUI
transcript goes today, which forwards nothing.

### The gate

`effigy drift:clicks` — the Jetstream sibling of `drift:handlers`, and stronger
for the same reason the shape was chosen: GPUI can only check that a handler
field is *read* somewhere, while here a test can drive the click and assert what
happened.

Three rules:

1. Every `pub fn on_x` needs a test in its own file that passes a handler to it
   **and** drives a click through `click_probe`.
2. A component whose contract names events but which takes no handler is
   unconverted, and must sit in the baseline — 87 of them, the sweep's worklist.
3. A converted component still listed in the baseline fails, so a conversion
   cannot leave the gate believing there is work outstanding.

It found a real gap on its first run: `AgentTranscript::on_changed_files_toggle`
was wired and forwarded correctly, and nothing clicked it. Three of the four
transcript events had tests and the fourth did not, which is exactly the kind of
gap that survives review — the file plainly had click tests in it.

All three rules were checked by breaking them: a converted component added to
the baseline reports stale, a baselined one removed reports unconverted, and the
untested-handler rule had already proved itself by finding the transcript gap.

`ci:native` also never ran the Jetstream component tests — 819 of them, on a
task list whose comment still claimed the native crates had none. `test:jetstream`
runs them now.

### The sweep

Batches land with the gate holding each one to a click test.

| Batch | Components | Baseline |
|-------|------------|----------|
| Agent chat | 7 | 94 → 87 |
| Core controls | 8 | 87 → 79 |
| Overlays | 5 converted, 3 exempt | 79 → 71 |

**Payload shapes.** The rule that fell out of the controls batch: report what
leaves the host with nothing to re-derive. Buttons take no payload — a press is
the whole event. Two-state controls report the state being moved *to*, matching
GPUI's `Fn(&bool)`, because a stateless host would otherwise recompute it.
`TriStateSwitch` reports the segment chosen, since three states have no "next".
`ToggleGroup` reports the option activated rather than the resulting set — in
multi-select the host owns the set, and returning one would make the component
decide whether a click adds or removes.

**Inert controls stay inert, and it is tested**: disabled everywhere, loading on
the buttons, read-only on `Checkbox` and `Switch`. Read-only is the interesting
one — not disabled, still focusable and full strength, but it cannot change, so
it must not report a change.

**Overlays needed a rule the web gets for free.** Clicks bubble to the nearest
clickable ancestor, so a backdrop handler fires for clicks *inside* the dialog
too — pressing "Save" would dismiss the dialog it was saving. The panel takes an
inert handler of its own, which makes it the nearest clickable and ends the
click there. Removing it fails `a_click_inside_the_panel_does_not_dismiss`, so
the arrangement is pinned.

`Popover`, `Tooltip` and `HoverCard` are exempt rather than baselined: each
renders a panel whose trigger and open state belong to the consumer, so there is
nothing in them to click. The gate distinguishes the two — "not done yet" goes
in the baseline, "nothing to draw" needs a stated reason.

### A parity defect the click tests found

`DialogSpec::effective_dismiss_on_backdrop` read
`dismiss_on_backdrop && !is_alert_dialog()`, so **every native alert dialog was
undismissable by backdrop**. `AlertDialog.svelte` passes
`dismissOnBackdrop={!working}` — the role has nothing to do with it, and only
the in-flight confirm suppresses dismissal. A unit test asserting the backdrop
reports cancel is what surfaced it; the spec helper had a test of its own that
asserted the wrong behaviour and passed.

Svelte is the parity authority, so the carve-out is gone and its test now
asserts the opposite.

### The migration

Free functions stay during the sweep: `IntoJsEl` wraps them, so each component
converts on its own rather than in one breaking commit. They go once the last
one is done.

Per component that is a struct with `from_spec`, an `IntoJsEl` impl delegating
to the existing render, and handler methods only where the contract has events —
mechanical, but 151 of them.

## Next

1. Sweep the 71 components left in the `drift:clicks` baseline. Mechanical: each
   becomes a struct with `from_spec`, an `IntoJsEl` impl wrapping the existing
   render function, and handler methods only where the contract has events. The
   gate holds each one to a click test as it lands, and the baseline count is
   the progress bar.
2. Forward the transcript's events on GPUI too. The blocks are interactive
   there, but `AgentTranscript` passes nothing down, so a GPUI host has to
   attach to the block components itself.
3. Burn down the 34 baselined GPUI handlers.
4. Revisit the GPUI click driver if `gpui` opens `DispatchEventResult`.

Two contract gaps found while converting, both recorded as deltas rather than
quietly implemented: neither native draws the "Open diff" action, and neither
has `onLinkClick` because inline nodes flatten to text.

The free functions stay for now: `IntoJsEl` wraps them, so conversion is
component-by-component rather than a single breaking commit. They go once the
sweep is done.
