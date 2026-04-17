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

- [ ] Grep GPUI 0.2.2 source for accessibility/role/aria symbols
- [ ] Check whether `.id()` touches any platform accessibility tree
- [ ] Investigate custom `Element` trait approach for role/label metadata
- [ ] Test VoiceOver focus behaviour on one component in the GPUI preview
- [ ] Write findings to this file (update §Findings below)
- [ ] Update g10.012 §2 with revised limits
- [ ] If viable path found: implement Button accessibility proof

---

## Findings

_To be filled in during execution._

## Next task

Begin with GPUI source grep and VoiceOver smoke test on the running preview app.
