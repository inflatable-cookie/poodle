# g15.047 — Primitive Visual Comparison

Status: **planned** — `g15.046` is accepted; compile the exact writable,
validation, and human-review envelope before dispatch
Parent: `012-visual-conformance-lane.md`
Depends on: accepted first-batch fixture inventory
Unblocks: `g15.012` closeout, then release certification
Governing refs: `../../roadmaps/g14/conformance-estate.md`,
`../../contracts/001-working-rules.md`, `012-visual-conformance-lane.md`

## Goal

Render the exact first-batch fixture identities through Svelte, React, and
GPUI, then compare geometry, semantic token roles, and pixels with explicit
renderer-aware tolerances. Produce evidence a human can inspect; do not create
a new component authority.

## Scope Envelope

- Reuse the existing headless web snapshot tooling and the adopted GPUI
  offscreen capture path.
- Compare Svelte↔React tightly because they share CSS, then compare web↔GPUI
  with separately stated raster/antialiasing thresholds.
- Report geometry and token-role differences separately from pixel differences
  so a blurred screenshot cannot hide a structural drift.
- Require human sign-off on every initial baseline and every accepted
  renderer-specific tolerance.
- Keep the Longhorn-backed lab optional: it may orchestrate or display results,
  but the comparator and evidence must run without it.

## Acceptance Envelope

- [ ] Every accepted fixture produces Svelte, React, and GPUI captures at fixed
      theme, viewport, scale, size, and density.
- [ ] Svelte↔React and web↔GPUI thresholds are explicit and fail on a planted
      geometry/token/pixel drift.
- [ ] Output names the fixture and runtime; missing captures fail closed.
- [ ] Baselines are human-reviewed diagnostic evidence only.
- [ ] The source-cost and registry count remain bounded and are recorded in the
      closeout log.

## Stop Conditions

- The comparator needs a normalized semantic output or generated component
  adapter layer resembling the rejected g14 mechanism.
- Tolerances are widened until meaningful component drift passes.
- A runtime borrows another runtime's capture or completion status.
