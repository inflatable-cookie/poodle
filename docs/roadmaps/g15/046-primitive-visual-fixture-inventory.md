# g15.046 — Primitive Visual Fixture Inventory

Status: **planned — blocked on completion of `g15.011`; `g15.045` complete**
Parent: `012-visual-conformance-lane.md`
Depends on: completed human-centred catalogue audit and headless capture in
every active runtime
Unblocks: `g15.047`
Governing refs: `release-baseline-roster.md`,
`../../contracts/001-working-rules.md`, `specimen-plan-outline.md`,
`012-visual-conformance-lane.md`

## Goal

Freeze a small first-batch inventory of renderer-neutral **fixture identities**
for primitive visual comparison. The inventory names observable inputs; every
runtime adapter still renders its real component.

## Scope Envelope

- Start with Button and a small operator-approved set of foundational
  primitives whose geometry/tokens materially affect composites.
- Name only contract-backed values: variant/appearance, state, exact public
  size and density domains, theme, content, and viewport.
- Keep interactions in focused tests. A fixture may name a resulting visual
  state, but it does not encode an action script or behavior machine.
- Define stable crop/geometry landmarks and token roles to report beside pixels.
- Record per-renderer exclusions only where the contract permits a real
  platform difference.

## Acceptance Envelope

- [ ] The batch is small enough for a human to review every baseline.
- [ ] No fixture contains framework code, node trees, generated props, event
      scripts, or a universal component schema.
- [ ] Svelte, React, and GPUI adapters can map every identity to real component
      construction without component-specific registries outside the batch.
- [ ] The inventory is diagnostic and cannot mark a component complete.

## Stop Conditions

- The inventory starts describing component APIs generally instead of naming
  bounded visual cases.
- A fixture needs executable behavior or normalized renderer output.
- The batch expands beyond the named primitives before the first comparison is
  reviewed.
