# g10.021 GPUI Accessibility Baseline

Status: queued
Owner: Poodle core
Depends on: g10.012
Updated: 2026-04-17

## Purpose

`g10.012` classified GPUI's accessibility limit: `gpui 0.2.2` does not expose
ARIA-like attributes on the fluent `div()` path Poodle uses for primitives.
Full `role` / `aria-*` parity with Svelte is an upstream tracking item, not a
local fix.

This milestone scopes what is achievable now — to find the floor before the
upstream ceiling is hit — and produces a clear list of what requires upstream
GPUI changes vs what can be done today.

---

## Investigation questions

1. **What GPUI 0.2.2 actually exposes today**
   - Does `ElementId` (used for `.id()`) drive anything in the accessibility
     tree, or is it purely a GPUI-internal handle?
   - Are there any accessibility-adjacent APIs (custom `Element` trait, event
     dispatch, focus tracking) that could carry semantic meaning?
   - Check `gpui/examples/` and `gpui/src/` in the vendored/crates.io source for
     any `accessibility`, `atspi`, `axkit`, or `role` symbols

2. **Low-cost wins that do not require upstream**
   - `aria-hidden` equivalent: decorative icons and separators that carry no
     meaning could be marked; does GPUI expose any path to suppress elements
     from the accessibility tree?
   - `aria-label` equivalent on interactive elements (buttons, inputs): could a
     custom `Element` wrapper carry this?
   - `aria-disabled`: currently rendered visually via opacity; could a wrapper
     carry semantic disabled state?

3. **Custom Element approach**
   - Evaluate whether a thin `AccessibleElement` wrapper (implementing the GPUI
     `Element` trait directly) could carry role/label/state data to the
     platform accessibility layer on macOS (NSAccessibility) even without GPUI
     fluent API support

4. **Focus behaviour**
   - Poodle GPUI already applies visible focus rings via `.focus(|style| …)`.
     Does GPUI's focus system integrate with VoiceOver on macOS? If so, document
     which components already work and which don't.

---

## Deliverables

- Written findings: what works today, what requires upstream, what requires a
  custom Element bridge
- If a viable low-cost path exists: implement it for one component (e.g.
  `Button`) as proof and document the pattern
- Update `g10.012 §2 True limits today` with revised accuracy based on findings

---

## Execution checklist

- [x] Grep GPUI 0.2.2 source for accessibility/role/aria symbols — zero found
- [x] Check whether `.id()` touches any platform accessibility tree — no
- [x] Investigate custom `Element` trait approach for role/label metadata — no hooks exist
- [x] Write findings to this file — see §Findings below
- [x] Update g10.012 §2 with revised limits — §2 is already accurate; confirmed, not updated
- [ ] VoiceOver smoke test — deferred; moot given no platform bridge exists (see findings)
- [ ] Button accessibility proof — not viable without upstream GPUI changes (see findings)

---

## Findings

Audited `/Users/example/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/gpui-0.2.2/src`
on 2026-04-17. GPUI is on the latest published crates.io release (0.2.2, Oct 22 2025).

### What GPUI 0.2.2 actually exposes

**Zero accessibility symbols.** Grepping the full source for `accessibility`,
`aria`, `role`, `atspi`, `axkit`, `NSAccessibility`, `accessible`, `AXElement`,
`ax_`, `voiceover` returns nothing relevant in any platform module, trait
definition, or example.

| API surface | Finding |
|-------------|---------|
| `ElementId` / `.id()` | Internal element-state key only; stored in `Interactivity.element_id`; never passed to any platform layer |
| `Element` trait | `id()`, `source_location()`, `request_layout()`, `prepaint()`, `paint()` — no accessibility hook |
| `FocusHandle` / `Focusable` | Internal focus management (tab order, `is_focused()`); no connection to VoiceOver, NVDA, or AT-SPI |
| macOS platform (`src/platform/mac/`) | 19 files — no NSAccessibility, no AX* protocol |
| Windows platform (`src/platform/windows/`) | 21 files — no UI Automation, no MSAA |
| Linux platform (`src/platform/linux/`) | 13 files — no AT-SPI2 |
| Examples | `tab_stop.rs` covers keyboard focus only; no screen-reader examples |

### Low-cost wins assessment

**None available.** There is no path to suppress elements from the
accessibility tree (`aria-hidden` equivalent), carry semantic labels on
interactive elements, or report disabled state semantically — because the
platform bridge does not exist at all. Implementing any of these would require
building the platform bridge first.

### Custom `Element` trait approach

Not viable as a workaround. The `Element` trait's three lifecycle methods
(`request_layout`, `prepaint`, `paint`) produce layout IDs and draw calls only.
There is no hook that feeds data to an accessibility tree node. A custom
`Element` wrapping an NSAccessibility bridge would require bypassing GPUI
entirely to call AppKit APIs directly — significant risk and maintenance burden,
and it would only work on macOS.

### Focus and VoiceOver

GPUI's `.track_focus()` / `tab_index()` manage an internal focus tree. That
tree is not wired to `NSAccessibilityFocusedUIElement` or any equivalent
platform notification. A focused GPUI button is invisible to VoiceOver.

A VoiceOver smoke test on the running preview app would confirm silence, but
adds no new information. Deferred.

### What this means for Poodle GPUI

- **Keyboard navigation** is achievable (and already partially implemented via
  focus rings) — this is visual and input-handling only, not screen reader.
- **Screen reader support** requires upstream GPUI work to add an accessibility
  tree API and wire it to each platform.
- **g10.012 §2 is accurate and needs no change.** The note there already
  states this correctly.

### Upstream signal

No open GPUI issue or PR for accessibility was found at the time of audit.
This is worth raising with the Zed team if GPUI accessibility becomes a
product requirement.

## Outcome

Investigation complete. No actionable local work exists. Recheck when GPUI
publishes an accessibility API.
