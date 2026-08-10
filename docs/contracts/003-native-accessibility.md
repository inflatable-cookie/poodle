# 003 - Native Accessibility

Status: active
Updated: 2026-08-10
Owner: Poodle core
Applies to: every contract with ARIA requirements, on the GPUI and Jetstream targets

## The Fact

**The two native runtimes are no longer in the same position.** Jetstream has an
accessibility API because we built one. GPUI does not, and cannot be given one
from outside without a cost that outlives the wait.

| Runtime | State | Evidence |
|---------|-------|----------|
| gpui 0.2.2 | **Nothing to call.** No `accesskit` dependency in its `Cargo.toml`, no accessibility node type, no role or label API anywhere in `src/`. 0.2.2 is the latest published version. | source search, and measured: its preview exposes **7 AX elements, 1 named** — `AXApplication`, `AXWindow`, three traffic lights and the title. That is AppKit's window chrome. None of GPUI's content is in it. |
| Jetstream | **AccessKit, live.** `jetstream-ui::accessibility` projects the retained `UiTree` into an `accesskit::TreeUpdate`; `jetstream-platform` owns an `accesskit_winit` adapter and routes action requests back through the same handlers pointer input uses. | `jetstream` commit `7e997892`, and measured: its preview exposes **471 elements of our own UI, 467 named**, read out of macOS through `AXUIElement`. |

Those two numbers on the same machine, through the same probe, are the whole
argument. This is no longer a claim about source code.

The earlier version of this document said neither runtime had an API and
recommended not scheduling the work at all. That was right about GPUI and wrong
about Jetstream, where the blocker was a decision no one had taken rather than
anything upstream. `roadmaps/g12/015-native-accessibility-options.md` costed it;
this records what shipped.

## What This Means For `aria_label`

**102 `poodle-specs` structs carry `aria_label`.** Where it goes now depends on
the target:

- **Svelte and React** consume it, held to that by the axe sweep.
- **Jetstream** consumes it. Shared render functions attach accessibility
  metadata to `poodle-node`; the Jetstream backend projects that metadata into
  AccessKit. `packages/jetstream/preview/src/bin/a11y.rs` exercises the
  rendered tree headlessly, and `effigy test:jetstream-ax` checks the mounted
  macOS accessibility tree.
- **GPUI** carries the metadata through the shared renderer into
  `poodle-node`, but its node backend cannot map it to GPUI 0.2.2 accessibility
  attributes. The backend reads the channel explicitly so the omission stays
  visible and deliberate.

So the field is no longer uniformly inert on native. Do not write "native
targets do not consume `aria_label`" — one of them does.

### The rule for Jetstream components

**A spec's `aria_label` names the component as a whole, so it belongs on that
component's root element and nowhere else.** A component that composes another
forwards the spec and lets the inner component attach it; adding it in both
places announces the name twice.

Roles and bounds are derived — `Widget` gives the role, layout gives the
`computed_rect` — so a component only states what cannot be inferred. A `Label`
and a text `Button` are announced from their own text without any explicit
labelling; an icon-only control is the case that needs it.

## Consequences For Planning

- **Do not schedule GPUI accessibility work.** There is no API to build
  against, and gpui 0.2.2 is the latest published version. Attaching an adapter
  was never the blocker: gpui implements `HasWindowHandle`, so `accesskit_macos`
  could bind its `NSView` without a fork. The blocker is that gpui is
  immediate-mode and exposes no retained tree, no roles and no laid-out bounds,
  so Poodle would build and maintain a **parallel accessibility tree across 140
  `impl IntoElement` components**, macOS-only, to be obsoleted wholesale the day
  upstream ships. Waiting costs nothing and loses nothing.
- **Do not read the GPUI accessibility artifacts as runtime proof.**
  `packages/gpui/native-accessibility-proof.json` is explicit about this in its
  own non-goals — it forbids claiming "mounted assistive-technology proof for
  sections that still only have spec-level or crate-test evidence". Its evidence
  is spec-level and crate-test level. That is the correct reading.
- **Jetstream contracts are now binding.** A Jetstream component that ignores
  its contract's ARIA section is a bug, not an accepted platform limit. What is
  still missing is breadth, not capability: accessible *names* are swept across
  every component, while roles, checked/expanded state and value are attached
  only where a component sets them explicitly. **31 components now carry the
  role their contract specifies**, with checked state on checkbox and switch and
  expanded state on collapsible and select. What is left is the remaining
  component roots, and per-element roles *inside* a component — a `menuitem`
  within a menu, a `tab` within a tab list, an `option` within a listbox — which
  are not attached at all.
- **`effigy test:jetstream-ax` is the check.** It launches the preview, reads
  its tree through `AXUIElement`, and fails on any non-structural element
  without an accessible name. It found three real defects on its first run
  (an unnamed contrast slider, an unnamed search field, and a decorative search
  icon that was being announced), which is the argument for having it.

## What Would Change The GPUI Half

gpui shipping accesskit support is the single upstream event that unblocks it —
Zed has the same need, so it is plausible rather than theoretical. Re-check on
every gpui release.

Until then the GPUI half is a **forced acceptance**, in the sense the Tree
contract already uses: not a debt anyone can pay down.

## The 48 Contracts This Governs

**48 component contracts carry ARIA requirements inside their GPUI Notes
section** — requirements that cannot be met on gpui 0.2.2. `checkbox.md` is
representative: it requires the indeterminate state to be "accessible to
assistive technology as `aria-checked="mixed"`", and requires exposing "state,
and accessible name through the native accessibility tree". There is no native
accessibility tree in GPUI to expose anything through.

Those requirements are not deleted or softened. They describe what the component
must do when the runtime can, and they are the specification a future
implementation is measured against. **This document is what makes them
non-binding on GPUI today**, per the cross-cutting rule convention: a contract
references the rule rather than restating it.

On Jetstream the same requirements *are* binding — `aria-checked="mixed"` maps
to `accesskit::Toggled::Mixed`, which `JsEl::aria_checked` sets. A reviewer
holding a native component to its contract's ARIA section should read that
section together with this one, and check which target they are looking at.

## Prior Record

This was documented before, as one row in `components/tree.md`'s Known Deltas
table — accurate, but scoped to one component when it was a property of the
whole native surface. An earlier draft of this document claimed "two other
contracts" were affected; checking rather than asserting turned up 48. That
draft also treated the two runtimes as one case, which is the error this
revision corrects.
