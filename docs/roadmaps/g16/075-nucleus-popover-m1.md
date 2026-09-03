# g16.075 — Nucleus Popover M1 Receipt

Status: complete
Type: Nucleus NP-2 mounted receipt child
Opened: 2026-09-03
Depends on: completed `g16.062`, completed `g16.068`, completed `g16.074`
Governing refs: `nucleus-gpui-parity-programme.md`,
`062-nucleus-parity-receipt-foundation.md`,
`nucleus-parity-manifest.json`, `parity-evidence-ledger.md`,
`../../contracts/components/popover.md`, `../../contracts/components/surface.md`,
`../../architecture/002-anchored-overlays.md`
Log: `../../logs/2026-09/20260903-g16-075-nucleus-popover-receipt.md`
PR: `https://github.com/inflatable-cookie/poodle/pull/181`
Handoff: `../../handoffs/20260903-081000-g16-075-nucleus-popover-receipt.md`

## Goal

Produce one validated `M1` receipt for the Nucleus `Popover` row through the
production Rust Popover adapter, renderer, Node, GPUI backend, and test-platform
paths. Extend the retained nested-overlay regression into an executed
controlled composition without turning M1 into accessibility or pixel proof.

## Fixed Boundary

- Keep the manifest's exact mounted test name
  `a_nested_popover_paints_without_nesting_deferred_draws`. Strengthen that
  test rather than adding a second receipt fixture.
- Construct the mounted composition through
  `node_compat::Popover::from_spec(...).into_element()`. A renderer-only Node
  fixture is not adapter evidence. Use generic labels and caller-scoped outer
  and inner instance ids.
- Prove exact trigger and surface relationships, expanded/controls state,
  layer identity, dialog role/name, placement and authored offset, trigger and
  surface bounds, minimum/maximum width, elevated fill, subtle border alpha,
  radius, overlay elevation plus inset highlight, and panel padding.
- Drive trigger activation, Escape, and outside pointer dismissal through
  `HeadlessDriver`. Host callbacks own state and rebuild the element. Disabled
  trigger input stays inert. Accepted close removes only the intended surface.
- Preserve nested-layer ownership: opening an inner Popover must not nest a
  deferred draw; Escape closes only the inner layer first; a pointer inside
  the outer surface but outside the inner surface closes only the inner layer;
  outer dismissal then restores the outer trigger. Two concurrent sibling
  instances with duplicate authored content must keep runtime ids, callbacks,
  bounds, and layer state separate.
- Prove the three initial-focus strategies only at the mounted focus-handle
  boundary: `content`, first focusable descendant, and `none`. Do not claim an
  accessibility tree. Closing after a focus handoff restores the matching
  trigger.
- A focused renderer/backend/GPUI compatibility repair is allowed only after a
  committed mounted counterexample. Use the existing Popover machine,
  floating-overlay renderer, dismiss stack, focus queue, and backend. Stop for
  a new public API, a second overlay machine, or app-owned focus policy.
- Emit the receipt only after every claimed assertion. Refresh the manifest,
  every existing receipt, generated ledger, this card, and one execution log
  from the exact committed runtime source. No other row advances.

## Acceptance

- Popover has one valid `nucleus.navigation.popover` M1 receipt naming the
  retained mounted test. The denominator stays 29 and existing 11 receipts
  remain valid.
- Replacing the production adapter or floating-overlay renderer with a raw
  Node; flattening nested deferred overlays; sharing instance identity;
  coupling inner and outer dismissal; bypassing mounted input; ignoring
  disabled state; dropping controlled rebuild; misrouting focus entry or
  restoration; or emitting before the terminal assertion fails the proof.
- Trigger/surface relationships and token metadata are exact. Mounted geometry
  proves positive bounds, anchored separation on the declared placement axis,
  containment, and the authored offset without claiming pixels.
- M1 does not infer A1 accessibility-tree semantics, browser portal behavior,
  collision fallback parity, V1 pixels, Nucleus M2 adoption, or Jetstream.

## Review Oracle

| Invariant | Smallest counterexample | Required proof |
| --- | --- | --- |
| Production adapter owns execution | mount `poodle_render::popover` directly | adapter-path assertion or mounted lifecycle fails |
| Nested draw remains valid | defer the inner overlay independently | mounted paint panics before receipt |
| Layer identity is scoped | reuse one layer/runtime id for outer and inner | inner dismissal closes or corrupts outer state |
| Sibling identity is isolated | compose duplicate-valued sibling instances without scopes | focus, bounds, or callbacks cross streams |
| Input is mounted | call activation/dismiss handlers directly | observation or exact callback trace is absent |
| Controlled ownership is real | record callback without rebuilding supplied open state | surface presence disagrees with host state |
| Dismissal order is exact | let Escape/outside close both nested layers | inner-first trace or retained outer surface fails |
| Disabled trigger is inert | bind activation while disabled | callback or surface appears |
| Focus strategy is exact | focus the surface for first-focusable or move focus for none | focus-handle witness fails |
| Focus restoration is scoped | restore the wrong trigger after nested close | focused runtime id crosses instances |
| Geometry and tokens are exact | alter placement offset, width, border alpha, padding, or elevation | metadata or mounted bounds assertion fails |
| Receipt is terminal | fail a final nested/sibling assertion | no Popover receipt is emitted |
| Evidence identity is exact | retain the g16.074 source SHA | receipt validation fails after source movement |
| Levels stay separate | label the receipt A1 or V1 | schema validation fails |

## Writable Scope

The retained Popover mounted regression; focused Popover spec, machine,
renderer, backend, and GPUI adapter tests; a bounded native repair only when a
committed mounted counterexample requires it; receipt/manifest/ledger refresh;
this card; one execution log; and new papercuts. Do not edit Nucleus, web
behavior, public APIs, accessibility authority, visual-lab code, Jetstream,
workflows, versions, releases, or other component rows.

## Validation

Run focused Popover spec/machine/render/backend tests, the named mounted
fixture, `effigy regressions:native`, `effigy check:parity-evidence-ledger`,
`effigy ci:rust`, `effigy ci:native`, `effigy docs:check`, and
`git diff --check origin/main...HEAD`. Do not run windowed or native-visual
selectors.

## Stop Conditions

Stop for orchestrator review if the proof needs a public API, another overlay
machine, browser-only geometry/focus selectors, Nucleus data, broad A1 or V1
claims, collision-engine redesign, or app-owned focus control. Record the exact
gap instead of weakening the receipt.

## Continuation

After merge, compile the Select M1 receipt child from the refreshed receipt
identity. Later Nucleus receipt cards remain serial.
