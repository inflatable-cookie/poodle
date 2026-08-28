# g16.024 — Drag-And-Drop Tree Nested Intent And Auto-Scroll

Status: planned — depends on the simple web migration
Depends on: `023-drag-drop-simple-reorder-migrations.md`
Governing refs: architecture 011, spec 069, and
`../../contracts/components/tree.md`

## Goal

Use Tree to prove deterministic nested targets, before/inside/after intent,
rejection, nearest-scroll-container ownership, and edge auto-scroll in Svelte
and React. Preserve Tree selection, expansion, rename, keyboard navigation,
virtualization, and its authored move callback.

## Required Behavior

- Measured row geometry becomes semantic before/inside/after intent; the core
  resolver chooses one deepest eligible target.
- Eligibility runs during hover and again at drop against current host state.
- Auto-scroll uses one frame loop, chooses the nearest eligible container that
  can scroll in the requested direction, accelerates near the edge, and stops
  on leave, cancellation, drop, unmount, or direction exhaustion.
- Keyboard movement uses the same intent and commit path as pointer/touch.
- Expansion, range selection, rename controls, and virtual-window changes may
  invalidate geometry but cannot create duplicate targets or stale commits.

## Acceptance Criteria

- [ ] Paired mounted web tests cover nested arbitration, all three positions,
      rejection, drop-time revalidation, virtualization, and target removal.
- [ ] Touch hold versus scroll and nested auto-scroll pass in Chromium and
      WebKit headlessly.
- [ ] Tree's existing public callbacks and non-drag interactions are preserved.
- [ ] Bespoke Tree drag state is removed; shared substrate tests remain green.
- [ ] No native or ledger claim changes.

## Writable Scope

- Tree Svelte/React implementations, focused core Tree helpers, tests,
  contracts, and curated specimens;
- the web substrate's geometry/auto-scroll modules and focused tests;
- headless browser fixtures/selectors needed for this proof;
- this card, one log, g16 closeout, and `PAPERCUTS.md`.

Do not edit other components, Rust/GPUI, host/file transports, package versions,
workflows, releases, or siblings.

## Validation

Run focused Tree and substrate tests, Chromium/WebKit headless probes, web and
docs boards, unchanged ledger checks, one final headless `effigy qa`, and diff
check. Never use a windowed/native visual or Jetstream selector.

## Stop Conditions

- Tree's move payload/eligibility contract is ambiguous or must break.
- Auto-scroll needs a component-owned timer or bypasses the common target
  resolver.
- Virtualization cannot preserve stable ids without a broader Tree rewrite.
- Scope expands into native, DockRegion, external files, or another component.

## Continuation

After merge, promote `g16.025` for the shared Rust and GPUI runtime.
