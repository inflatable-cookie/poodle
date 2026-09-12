# Release evidence

Status: current evidence
Updated: 2026-09-12

This directory holds release evidence still consumed by repository checks or by
an active release gate. It is not a roadmap or an executable planning surface.

- [`web-package-roster.md`](web-package-roster.md) is the frozen web package
  denominator consumed by `test/package-install/roster.ts`. Its historical
  terminology is preserved from g15.
- [`g18-027-v040-public-surface-delta.md`](g18-027-v040-public-surface-delta.md)
  is the accepted `v0.3.0` → post-g18.026 public delta that freezes `0.4.0`. It
  is the migration source for the g18.006 release notes and is re-derived by
  `bun scripts/audit-public-surface-delta.ts --base v0.3.0 --head <candidate>`.

