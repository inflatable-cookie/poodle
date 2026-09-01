# g16.044 — Nested Menu Pointer Intent Research

Status: research-complete — PR #131; conditional compose-and-extend direction,
promotion still gated
Opened: 2026-09-01
Depends on: current Menu, ContextMenu, and Menubar cascading-submenu contracts;
independent of `g16.034` and `g16.036`
Governing refs: `../../contracts/001-working-rules.md`,
`../../contracts/components/menu.md`,
`../../contracts/components/context-menu.md`,
`../../contracts/components/menubar.md`
Intake: DesEngs candidate 8, merged in PR #126
Source lead: [Web Interface Guidelines](https://interfaces.rauno.me/)

## Goal

Research a shared submenu pointer-intent rule that lets a pointer travel
diagonally into an open flyout without accidental sibling activation. Preserve
keyboard behavior and current submenu semantics. Runtime-owned hit testing may
differ; active-cohort semantic parity remains a question to answer, not a
pre-approved web-only waiver.

This card authorizes research only. It does not add a prediction cone, dwell
controller, delay, pointer-history API, or native exception.

## Questions

- Which geometry and timing rule reduces accidental close without creating a
  sticky wrong flyout?
- What happens when direction reverses, the submenu moves, a sibling is
  intentionally targeted, geometry is clipped, or the pointer leaves all
  surfaces?
- Can Menu, ContextMenu, and Menubar share one rule despite different anchors
  and dismissal owners?
- How do touch, pen, keyboard, assistive technology, and reduced motion relate
  to a pointer-only mechanism?
- Which GPUI menus are Poodle-rendered versus OS-owned, and what semantic
  result must each prove?

## Required Evidence

- Inspect Rauno's guideline and durable primary implementations of established
  submenu-aim techniques; record licensing and mutable-source limits.
- Trace current web and GPUI surface geometry, recursive submenu ownership,
  pointer events, focus, dismissal, and sibling-hover behavior.
- Prototype only in disposable research code if measurement is needed; do not
  import a consumer dwell controller.
- Define measurable accidental-close and sticky-wrong-flyout cases across
  direction, velocity, density, viewport edges, and nested depth.

## Deliverable And Promotion Gate

Write `docs/research/value-tracks/nested-menu-pointer-intent.md` with an
architecture/shared-helper/component-local/reject recommendation, measured
algorithm comparison, and runtime boundary. Promotion requires operator
acceptance of timing, geometry ownership, and active-cohort evidence.

## Writable Scope

The dossier only, plus `PAPERCUTS.md` for new execution friction. Disposable
research artifacts must remain untracked. Do not edit contracts, source,
packages, roadmaps, triage, or consumers.

## Validation

Run `effigy docs:lint` and `git diff --check`.
