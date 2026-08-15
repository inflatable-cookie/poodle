# 14 — g14.005 Popover Overlay And Focus Proof

Batch log, 2026-08-15. Card:
`docs/roadmaps/g14/005-popover-overlay-focus-proof.md`.

## What changed

The first complete overlay profile ran through the landed conformance kernel:
one portable Popover interface + 22 typed cases executed headless in Svelte,
React, and GPUI, through real layer registration, dismissal routes,
placement, focus entry, and focus restoration.

- **Interface authority** — `packages/core/src/conformance/popover.ts`
  (`defineComponentInterface`, profile `overlay`): controlled/uncontrolled
  open, all 12 placements, numeric offset, the outside guard, the three
  focus strategies, surface width strategy and bounds, and the trigger +
  children regions. Svelte and React shells bind to the inferred portable
  prop/event types; the two web-only extensions stay beside the adapters.
- **Corpus** — `popover-cases.ts` (23 cases) covering closed default,
  uncontrolled pointer/keyboard open, controlled ownership, disabled
  inertness (including the programmatic open-direction), all three focus
  strategies, every close path with trigger focus restoration, the outside
  guard, the nested dismiss-stack contract (two cases), the placement
  families and end rule, offset, trigger-width, the width-bound override
  (the rem portable unit, proven mounted on both runtimes), and
  semantics/tokens.
- **Generated declaration** — `generated/popover/mod.rs` replaces the
  hand-written `PopoverSpec`; the extension keeps the token recipes and
  derived queries. The Rust target gained `number` prop support and the
  `Eq`-conditional derive.
- **Generic vocabulary** — `define.ts` gained the `dismiss` and
  inside/outside `pointer` actions, `part-present` state rule,
  `relativeTo` part anchors, the relative logical-bounds geometry fields,
  `parent`/`overlay`/`expanded`/`focusedText`/`layerCount` observations,
  and opaque `host` fixture data (nested-layer proof, focusable content
  lists). The web runner, native observer, and comparator implement them
  with no component identifier.
- **Web execution** — real DOM pointer/keyboard/focus/Escape/outside routes
  through the shared dismissable-layer stack and the portalled anchored
  surface; the harness supplies happy-dom's missing layout (a minimal box
  stub + an anchor-box stylesheet) so placement resolves collision-free. The
  surface part resolves through the portal. Layer parenthood and stack order
  derive from real layer containment (`hostElement` roots + the layers'
  `contains`), so nested overlays register correctly whatever the framework
  effect order and wherever the surfaces were portalled; registered-peer and
  reversed-portal-order tests pin the behavior (`test/headless-dom/
  dismiss-layers.test.ts`).
- **Native execution** — the renderer composition owns trigger, surface,
  shared `floating_overlay` placement (authored offset as the gap), token
  roles, accessibility metadata, and the dismiss/layer intent. The
  node-backend gained a generic overlay host: a layer registry rebuilt at
  the host's render-frame boundary (a real page converts many components
  per frame — the registry is frame-scoped, not conversion-scoped),
  rendered bounds recorded at paint, parent chains from tree order, a
  paint-time focus-request queue for machine focus effects, and
  window-level Escape/outside dispatch attached through one reusable
  `attach_overlay_host` used by the production preview root and the
  conformance mount host alike. The preview specimen renders through the
  shared composition (with per-instance ids and the machine running
  dismissal and focus effects); the local GPUI floating-overlay copy is
  gone, and the specimen's trigger is the composition's own interactive
  button — no second focusable wrapper.
- **Primitive rows** — `overlay.intent`, `semantic.expanded`,
  `overlay.dismiss`, and `overlay.layer` gained render-neutral, web, and
  GPUI probes and join the gated owned rows (21/21 owned passing).
- **Failure proof** — 7 planted defects (inert Escape, inert outside
  dismissal, wrong initial-focus target, missing focus restore, reversed
  nested-layer dismissal, absent overlay/layer evidence, wrong placement
  offset) each fail the expected runtime/case/step/field.
- **Vector disposition** — the four Popover machine vectors are deleted
  from `machines.json` (claims covered by the mounted corpus; the machine
  stays, exercised by both web runtimes and the Rust mirror through the
  corpus); the vector conformance tests' popover branches went with them.
- **Cost** — Popover pilot increment 1,821 LOC (453 authored authority, 86
  generated source, 1,282 harness/runtime deltas) and 36,548 bytes of
  Popover fixture JSON. Mechanism total 14,007 LOC.

## Before / after runtime

| Board | Before | After |
| --- | --- | --- |
| Active cohort | 39 cases × 3 runtimes (Button, RangeSlider, Tabs) | 61 cases × 3 runtimes (… + 22 Popover) |
| Primitive owned rows | 17 passing | 21 passing (overlay rows join the gate) |
| GPUI headless tests | 10 | 17 (7 planted popover defects) |

`effigy conformance:complete` (authority checks + web + full GPUI execution +
renderer-neutral Rust + compare + primitive report): green, all headless.

## Defects the corpus caught (fixed, not waived)

- **Nested overlay registration order (web)**: the inner popover's dismiss
  layer registered before the outer's (Svelte effects and React child-first
  effects), so Escape dismissed the outer first. The dismiss stack now
  derives parenthood and order from real layer containment (the layers'
  `contains` over the host roots), so the outer inserts below the inner
  whatever the effect order and wherever the portalled surfaces landed.
  Innermost-first dismissal holds on both runtimes, with registered-peer and
  reversed-portal-order tests pinning it.
- **Controlled open while disabled (web)**: a controlled `open: true` host
  with `disabled: true` rendered the surface despite the machine's guard.
  Both shells now gate the visible state with `!disabled`.
- **Node-backend**: nested overlay surfaces (a popover inside a popover)
  panicked gpui's deferred drawing pass (`defer_draw` during deferred
  drawing); overlay nodes inside an overlay now draw within the enclosing
  deferred element.
- **Nested state persistence (GPUI adapter)**: the nested instance's open
  state was rebuilt fresh each frame, so its machine transitions never
  survived; the state now lives on the host.

## Notes

- The web harness supplies happy-dom's missing layout for the placement
  claims (a box-model stub plus an anchor-box stylesheet) — the same posture
  as the browser-default keydown → click simulation. The GPUI side proves
  the same relative geometry with real rendered bounds; the authored
  assertions are `topGap == offset`, alignment deltas, and width match with
  named tolerances, so the anchor box never leaks a runtime-specific
  constant into the corpus.
- The `host` fixture extension (opaque per-component composition data) is
  documented in the corpus module; later overlay profiles (Menu, Modal) will
  use the same seam for nested-layer cases.
- `packages/jetstream/preview` compat still calls the old surface-only
  `pr::popover` signature; the Jetstream lane is program-deferred and the
  crate is not in any active gate. `message_center` was updated to the
  `popover_surface` path.
- The old `image.regional` requirement on `overlay.intent` was replaced by
  executed `node.field`/`parts.overlay` evidence — regional images were not
  headless-executable and the card's validation is entirely headless.
