# g12.015 — Native Accessibility: Options Assessment

**Status: assessed, not built.** `003-native-accessibility.md` recorded the
fact — neither native runtime exposes an accessibility API. This card answers
the next question: *what would it take, per engine?*

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

## Recommendation

- **Jetstream: propose it to the engine.** Roles, bounds and the tree are
  already there; the adapter is compatible now. The Poodle-side half is adding
  an accessible-name field to `JsEl` and populating it — worth doing *when* the
  engine side is agreed, and not before, since a field with no sink is the
  inert-`aria_label` situation moved one layer down.
- **GPUI: hold, and watch the dependency.** Re-check on every gpui release.
  This is the one upstream event that changes the answer.
- **`003-native-accessibility.md` stands**, with its planning advice narrowed:
  the blanket "do not schedule native accessibility work" applies to GPUI. For
  Jetstream the blocker is a decision no one has taken yet.

## Not Done

Nothing was built. No `JsEl` field was added, no adapter wired, no engine
proposal filed. This card is the assessment that was asked for, and the point of
writing it down is that the next person does not have to re-derive which of the
two engines is actually blocked.
