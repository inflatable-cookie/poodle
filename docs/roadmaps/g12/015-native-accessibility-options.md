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

## Not Done

- **Breadth, not capability.** The sweep attaches accessible *names*
  everywhere. Roles, checked/expanded state and values are attached only where a
  component sets them explicitly, so a checkbox drawn out of panels still
  reports `GenericContainer` unless it says otherwise. Everything needed to fix
  that is in place; it is per-component work against each contract.
- **Not verified with a real screen reader.** The tree is proven by test up to
  the AccessKit boundary. Whether VoiceOver announces it sensibly is a different
  question and has not been checked.
- **GPUI is untouched**, per the recommendation above.
