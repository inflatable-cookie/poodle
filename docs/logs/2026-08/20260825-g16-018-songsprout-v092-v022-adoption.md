# g15.072 — Songsprout Underlay 0.9.2 and Poodle 0.2.2 Adoption

Date: 2026-08-25
Status: complete
PR: [Songsprout #1](https://github.com/inflatable-cookie/songsprout/pull/1)
Merge: `22f8ae78077bdf8c3bb0841edc5066e90994a2d6`

## Result

Songsprout's active application graph no longer resolves Underlay or Poodle
from sibling source trees. Bloom, Greenhouse, and Stem use Underlay tag
`v0.9.2` at `ddba26400f480638829917cf72eecc62be4b978d`. Bloom and Greenhouse
use exact registry Poodle 0.2.2. Nursery's 27 Underlay packages resolve the
same Git revision.

Both public-config generators now use Underlay's published config-stack
subpath. Committed Poodle overrides and active web/Rust sibling dependencies
are gone. Explicit sibling mounts remain only for workspace QA.

## Validation

Independent review passed:

- `effigy bloom/validate`
- `effigy greenhouse/validate`
- `git diff --check`

Review found Bloom and Greenhouse initially retained a second local
Underlay/Poodle 0.1 graph through stale Stem lock metadata. The final repair
changes only the Stem metadata and nested Underlay identity, adds the tagged
graph's required marked resolution, and removes 140 packages inherited from
the former local Underlay development graph. No surviving unrelated package
entry changed. Nursery Cargo churn is exactly 27 local-to-tagged Underlay
packages.

Root `effigy validate` and `effigy qa` retain the reproduced pre-existing
reorder-conflict rollout baseline. No application exception was added.

## Continuation

Songsprout and Contact Patch are complete. Continue the coupled wave through
Acowtancy, Compli Me, and Composer. Composer still needs its unrelated Cargo
lock upgrades removed before merge.
