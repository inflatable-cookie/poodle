# g16.024 — Drag-And-Drop Tree Nested Intent And Auto-Scroll

Status: complete — pending review in PR #107
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

- [x] Paired mounted web tests cover nested arbitration, all three positions,
      rejection, drop-time revalidation, virtualization, and target removal.
- [x] Chromium headless proves native touch hold-versus-scroll and nested
      auto-scroll. WebKit headless proves touch-shaped hold/tolerance plus real
      mouse/keyboard geometry, nested auto-scroll, and cleanup, with its lack
      of native touch injection stated explicitly.
- [x] Tree's existing public callbacks and non-drag interactions are preserved.
- [x] Bespoke Tree drag state is removed; shared substrate tests remain green.
- [x] No native or ledger claim changes.

## Writable Scope

- Tree Svelte/React implementations, focused core Tree helpers, tests,
  contracts, and curated specimens;
- the web substrate's geometry/auto-scroll modules and focused tests;
- headless browser fixtures/selectors needed for this proof;
- the bounded `requestKeyboardDrop` controller command, paired framework
  exposure, logical Tree targets, and focused lifecycle tests required to keep
  Alt+Up/Down on the shared session;
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

## Review Oracle

- **Invariant:** one active Tree drag has one deepest eligible semantic target,
  one before/inside/after intent, and at most one nearest scroll owner; every
  terminal path stops scrolling and revalidates the live target before commit.
- **Smallest adversarial counterexample:** drag one row across a nested parent
  boundary while the inner and outer containers can both scroll. Move through
  before, inside, and after zones, disable or remove the inner target before
  release, then cancel a second drag while edge scrolling is active.
- **Expected failure/stop:** ancestor and descendant both become active, intent
  flickers at a zone boundary, both containers scroll, a removed target commits,
  or any timer/frame continues after leave, cancellation, drop, or unmount.
- **Required proof:** paired Tree mounted tests for arbitration, all positions,
  revalidation, removal, virtualization, non-drag behavior, and Alt+Up/Down
  through `requestKeyboardDrop`; controller command tests for live eligibility,
  logical-target authority, async removal/disable, terminal callbacks,
  announcements, and focus return; deterministic geometry and auto-scroll unit
  tests; native touch hold-versus-scroll in headless Chromium; touch-shaped
  hold/tolerance plus real geometry/auto-scroll/cleanup in headless WebKit.

## Continuation

After merge, promote `g16.025` for the shared Rust and GPUI runtime.
