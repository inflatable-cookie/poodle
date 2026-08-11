# Treatment System Retirement

Date: 2026-08-11
Status: complete

## Changed

- removed the GPUI preview's Treatments section, navigation, router branch,
  static guide, and CLI selector
- retired architecture 005 to a short tombstone and kept architecture 007 as
  the sole active appearance-override contract
- deleted the Treatment Tokens component contract and removed its index entries
- migrated active component contracts, guides, vision, and promoted specs to
  component Recipe hooks with direct semantic-token fallbacks
- removed the stale Jetstream `AppearanceTreatment` parity delta
- added `effigy drift:recipes` to reject retired system identifiers outside
  historical records and the architecture tombstone

## Current State

Active code, contracts, guides, previews, specs, and current evidence expose
Recipes as the only downstream appearance override surface. Completed roadmaps,
logs, and historical parity audits retain their point-in-time wording.

## Validation

- `effigy drift:recipes` — pass; 2,420 active files checked
- `effigy gpui:build` — pass
- `git diff --check` — pass
- `effigy docs:lint` — blocked by unrelated in-progress repository drift:
  missing current-contract index entries, missing preview coverage, stale shared
  demo audit counts, and the existing Popover `triggerIsInteractive` spec delta

## Unresolved

No Treatment-system removal remains. The unrelated docs-lint findings stay with
their owning in-progress component work.
