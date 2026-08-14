# g14.004 — Tabs Collection And Navigation Proof

Post-completion correction: commit `8ac863b4` restored the curated Tabs
specimens. The corpus still owns executable fixtures and exhaustive evidence;
it no longer replaces the catalogue `Examples` view.

Status: complete — accepted in PR #14
Depends on: `g14.003`

## Outcome

Prove identified repeated anatomy, selection, roving focus, orientation, and
keyboard navigation with the same component pipeline.

## Scope

- Move Tabs portable interface and executable cases into shared authority.
  Preserve its curated catalogue specimen.
- Model stable item/trigger/panel identities without runtime index leakage.
- Cover controlled selection, disabled items, orientation/direction, Home/End,
  arrow navigation, activation mode, focus, and event order.
- Compare repeated part relationships, selected state, accessibility links,
  token roles, and geometry across all active runtimes.

## Acceptance

- Reordering fixture items preserves semantic identity.
- Every required navigation route executes in all active runtimes.
- A planted focus, selection, ID, or panel relationship error fails.
- No Tabs-specific runner branch or second item model.
- Replaced fixtures/vectors/declarations are removed and costed.

## Stop Conditions

- Repeated anatomy cannot be expressed without embedding renderer structure.
- A runtime uses a different selection contract to make the fixture render.

## Validation

Run Tabs cases and narrow suites, `effigy ci:conformance` (headless),
`docs:check`, and `git diff --check`. Do not run any `*-windowed` selector on
the operator desktop; isolated CI owns that legacy proof until g14.023 lands.
