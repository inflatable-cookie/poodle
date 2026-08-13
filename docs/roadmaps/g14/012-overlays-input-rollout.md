# g14.012 — Overlays And Input Rollout

Status: blocked pending `g14.008` adopt
Depends on: `g14.011`

## Outcome

Migrate overlays, pickers, editors, uploads, and text-entry components.

## Acceptance

- Layering, dismissal, focus transfer, placement, editing, IME/selection, and
  accessibility results execute in all required runtimes.
- Runtime-owned mechanisms have equivalent observations; missing required
  capability remains red.
- Obsolete machine-interface/scene/capability experiments for these families
  are removed as coverage lands.

## Stop Conditions

- A tolerance or declared absence is used to satisfy completion.
- DOM or backend editor objects enter portable cases.
