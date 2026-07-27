# g12.015 — Native Accessibility: Options, Then The Jetstream Build

**Status: assessed, and the Jetstream half built.** `003-native-accessibility.md`
recorded the fact — neither native runtime exposed an accessibility API. This
card answered the next question, *what would it take, per engine?*, and then
acted on the answer: Jetstream now has one.

The previous document's advice was "do not schedule native accessibility work."
That was right for GPUI and **too broad for Jetstream**. The difference is now
measured rather than assumed.

## What Was Checked

Versions and APIs, on this machine, on 2026-07-27:

| Fact | Evidence |
|------|----------|
| `gpui = "0.2.2"` is the latest published version | `cargo search gpui` returns 0.2.2 |
| gpui 0.2.2 has no accesskit | no `accesskit` in its `Cargo.toml`; no role/label API in `src/` |
| gpui exposes its native window handle | `impl HasWindowHandle for Window` — `window.rs:4845`, public |
| Jetstream is on `winit = "0.30"` | `jetstream/Cargo.toml:61` |
| `accesskit_winit 0.33.2` requires `winit ^0.30.5` | crates.io dependency listing |
| Jetstream has no accesskit anywhere | no match in any workspace `Cargo.toml` |

The last two are the finding. **The winit adapter is version-compatible with
Jetstream today** — not after an upgrade, not in principle.

## Jetstream — Tractable, And Most Of The Parts Exist

AccessKit needs a tree of nodes carrying role, name, state and bounds, plus a
platform adapter. Jetstream's retained UI tree already holds nearly all of it:

| AccessKit needs | Jetstream has |
|-----------------|---------------|
| a node tree with parent/child links | `UiTree` — `UiNode { parent: Option<usize>, children: Vec<usize> }` (`jetstream-ui/src/tree.rs:9`) |
| bounds per node | `UiNode::computed_rect` — already populated by Taffy |
| a role per node | `WidgetKind` — `Button`, `Label`, `Slider`, `ProgressBar`, `TextInput`… maps almost one-to-one onto accesskit `Role` |
| focus | `UiTree` already tracks `hovered` and pressed node state |
| a platform adapter | `accesskit_winit`, against the `ApplicationHandler` in `jetstream-platform/src/lib.rs` |

What is missing is small and specific:

1. **`JsEl` carries no accessible name.** Its fields are kind, layout, style,
   children, id and handlers — there is no `aria_label`, so Poodle's 33
   `aria_label` reads in `packages/jetstream/components/src/` have nowhere to
   put the value. This is the one gap that is Poodle-adjacent.
2. **No `WidgetKind` → `Role` mapping** and no state projection (checked,
   expanded, disabled, value ranges).
3. **No adapter wiring** in the event loop, and no action handler routing
   AccessKit actions (click, focus, set value) back into the UI's input path.

Points 2 and 3 are engine work in the sibling repo. Point 1 is a field on a
struct.

**This is a decision, not a wait** — the phrasing `003` already used, now with a
cost attached. The engine is first-party, the adapter is compatible, and the
tree it needs is the tree that already exists.

## GPUI — Possible Without A Fork, But The Tree Is The Problem

The obvious blocker is not the real one. gpui implements `HasWindowHandle`, so
`accesskit_macos` — which attaches to an `NSView` — could be wired up **from
outside gpui, without forking it**. Attaching the adapter is not what makes this
hard.

What makes it hard is that gpui is immediate-mode and hands consumers no
retained element tree, no roles and no laid-out bounds. Poodle would have to
build and maintain a **parallel accessibility tree of its own**: every one of
the 140 `impl IntoElement` components reporting its own role, name, state and
rect, rebuilt each frame, with focus and hit-testing routed separately from
gpui's.

Bounds capture is at least proven rather than theoretical — `range_slider.rs`
already collects a child's layout rect through `on_children_prepainted` for
pointer maths. The pattern works. Doing it for 140 components, in every state,
and keeping it correct as they change, is the cost.

Options, ranked:

1. **Wait for upstream.** gpui gaining accesskit is one dependency bump for us
   and a real prospect — Zed has the same requirement and is the primary
   consumer. Cost: zero. Risk: no timeline, no public commitment.
2. **External accesskit_macos adapter + a Poodle-maintained a11y tree.** No
   fork, macOS only, and a permanent obligation across 140 components that
   duplicates what a future upstream implementation would replace wholesale.
3. **Fork gpui.** 78,734 lines, moving fast. Rejected — the merge cost outlives
   any benefit.

**Recommendation: (1).** Not from pessimism — option 2 is genuinely available —
but because the work it requires is precisely the work upstream would obsolete,
and it would be macOS-only in the meantime.

## The Decision

Put to the user with the costs above; the answer was to build it. So the
recommendation that stood at assessment time — *propose it, do the Poodle half
only once the engine half is agreed* — was overtaken in the same sitting, and
both halves landed together. That ordering was the point of the caveat rather
than a change of mind: a `JsEl` field with no sink would have been the inert
`aria_label` moved one layer down.

**GPUI: hold, and watch the dependency.** Re-check on every gpui release. That
is the one upstream event that changes the answer.

## What Shipped For Jetstream

The assessment said the parts were already there. They were, and the estimate
held: the engine work is one module per crate.

**`jetstream-ui/src/accessibility.rs`** (sibling repo, commit `7e997892`) —
`Accessibility` on `NodeStyle` (label, role override, description, toggled,
expanded, selected, level, required, numeric value, hidden) plus `tree_update`,
which walks the live `UiTree` into an `accesskit::TreeUpdate`. Pure: no window,
no adapter, so it is tested headlessly.

Three things there were easy to get quietly wrong, and each has a test:

- **Recycled slots.** `UiNodeId` is `{index, generation}` and both go into the
  AccessKit id. Index alone would alias a reused slot onto the node that used to
  live there, and AccessKit caches by id.
- **Hidden nodes.** A node dropped from the tree must also leave its parent's
  child list, or AccessKit rejects the update for referencing a node that is not
  in it.
- **Dangling focus.** The focused node must be reported with *every* update and
  must exist; one that has since been hidden falls back to the window.

A synthetic `WINDOW_ID` node parents the UI root, because AccessKit wants the
root to be the window and making the UI's own root `Role::Window` would let a
role override break the tree.

**`jetstream-platform/src/accessibility.rs`** — owns the `accesskit_winit`
adapter. Two constraints shaped it. The adapter must be attached *before* the
window is first shown, so the window is now created hidden and shown once the
adapter exists. And AccessKit calls back on threads and at times of its own
choosing, including asking for a full tree before the first frame — so the most
recent update is retained and handed straight back, rather than made to wait for
a frame boundary.

Actions go the other way and are **queued, not handled inline**: a request
arriving on an arbitrary thread must not reach into the tree while the frame
callback owns it. They drain into the normal event stream as
`PlatformEvent::AccessibilityAction`, and `GameUi::handle_accessibility_action`
routes them through the same paths as pointer input — so a screen reader
activating a button runs that button's `on_click`. That is a test, not a claim.

**Poodle side** — `packages/jetstream/components/src/aria.rs` and a sweep of
**108 component files**, each attaching its spec's `aria_label` to its root. The
rule is one place per component: a component that composes another forwards the
spec instead, or the name is announced twice.

The end-to-end test is the one that matters, because every link in the chain
existed before except the last two: `ButtonSpec::with_aria_label` → `js_button`
→ materialized `UiTree` → the AccessKit tree, asserting the label is in the
nodes a screen reader would read.

The preview publishes its tree every frame rather than only when the shell is
dirty — focus and hover move the tree without a rebuild, and a screen reader
attaching mid-session needs the current screen.

## Verified Through macOS, Not Just Through Our Own Tests

The Rust tests prove `tree_update` builds the right AccessKit nodes — which is
our code agreeing with our code. `test/native-visual/ax-probe.swift` goes out
through `AXUIElement`, the same API a screen reader uses, so the answer comes
from the operating system.

| | GPUI preview | Jetstream preview |
|-|--------------|-------------------|
| AX elements | **7** | **571** |
| of our own UI | 0 | **471** |
| named | 1 | **467** |

GPUI's seven are `AXApplication`, `AXWindow`, the three traffic lights and the
title — AppKit's window chrome, present whether or not an app implements
anything. That is what "no accessibility API" looks like from outside, and it is
also how the audit knows which elements are not ours.

**Two things had to be found rather than assumed**, and both looked exactly like
a broken adapter at first:

- **The app must be activated.** An unactivated process exposes *nothing*, not
  even `AXApplication`. The first probe returned one empty element and read as
  "the adapter never attached". It had attached; AccessKit builds its tree
  lazily, and the app becoming frontmost is what asks for it. The audit retries
  rather than sleeping on a guess — the mistake `014` made five times.
- **The system menu bar dominates the count.** ~86 `AXMenuItem`s belong to
  macOS, and its separators are legitimately unnamed. Auditing raw totals would
  have reported a permanent, meaningless failure.

**The audit found three real defects on its first green run**, all in the
preview shell: an unnamed contrast slider, an unnamed search field, and a
decorative search icon that was being announced alongside the field it
duplicates. All three are fixed — the icon with `aria_hidden`, which is the
first real use of that escape hatch. A gate that finds nothing on its first run
has usually not been pointed at anything.

A third thing was found the hard way, after the numbers above were taken:
**a locked screen is indistinguishable from a broken adapter.** No window is
composited, so macOS builds no content tree and the probe returns the system
menu bar and nothing else. That is the same machine state that defeats
`screencapture` in `014`, defeating a different API, and it was mistaken for a
regression for a while. The audit now checks `CGSSessionScreenIsLocked` first
and says which it is.

`effigy test:jetstream-ax` runs it; `jetstream-ax:dump` prints the tree.
Local-only: it needs a window server, an unlocked display, and Accessibility
permission, and it names whichever is missing rather than reporting an empty
tree.

**The cycle guard took two attempts.** Guarding by depth alone hangs: the
pre-activation tree really is cyclic (the application element is its own child),
and a depth cap still branches at every level. The obvious fix — a global
visited-set — is also wrong, because sibling elements are not always distinct
under `CFEqual`, so it prunes real subtrees and reports an almost-empty UI. What
works is tracking the current *path*: only an element that is its own ancestor
is a cycle. Re-measured after that fix: **570 / 470 / 467**, and 466-467 named
across three runs. The one-element drift is an `AXGroup` appearing or not, so a
single-element difference is noise and anything larger is a change.

## Not Done

- **Roles: a first pass, not a sweep.** 31 components now carry the role their
  contract specifies, taken from the contracts rather than guessed — the ARIA
  roles were extracted from every `docs/contracts/components/*.md` and mapped
  onto `accesskit::Role`. Checkbox and switch also carry checked state, and
  collapsible and select carry expanded state. The tri-state mapping is named
  once in `aria::toggled`, because the contracts say `aria-checked="mixed"`,
  the specs say `checked: None`, and AccessKit says `Toggled::Mixed` — three
  spellings of one idea, and rendering `None` as unchecked would be a specific
  and untrue claim about the control.

  The remaining components still report `GenericContainer`, and per-element
  roles inside a component (a `menuitem` within a menu, a `tab` within a tab
  list, an `option` within a listbox) are not attached at all — only the
  component root. That is the bulk of what is left.
- **The audit sees one screen**, and this is now the binding limitation. It
  reads whatever the preview happens to be showing — the shell — so of the 31
  roles just attached, only `AXTabGroup` appears in it. The checkbox role is
  proven by a Rust test against the projected tree, not by macOS. Extending the
  audit per-slug is the same shape as the visual gate's sweep and is what would
  make the rest of this verifiable end to end.

  It still earned its keep on the shell: attaching `TabList` immediately made
  the audit fail on an unnamed tab group, which is announced as "tab group" and
  nothing else. Named, and green again at 468.
- **Nobody has listened to it.** The tree is correct and named; whether
  VoiceOver's *announcements* are sensible in order and phrasing is a judgement
  a machine cannot make.
- **GPUI is untouched**, per the recommendation above.
