# g15.055 — Consumer Adoption Inventory

Date: 2026-08-23
Verdict: **17 authoritative consumers; foundation-first rollout**

## Result

An exact manifest, lockfile, source, and Rust dependency search across
`~/Dev/projects` found 17 authoritative repositories that consume Poodle.
The estate mixes registry `0.1.0` ranges, exact `0.1.0` example pins,
committed sibling-`file:` overrides, Longhorn Rust tag `v0.1.0`, and
Jetstream's intentional paired local Rust paths.

The committed `file:` overrides are material: changing only a declared range
would still test Poodle source from the adjacent checkout rather than the npm
release. The rollout therefore removes active committed Poodle overrides and
pins applications to exact `0.2.1`.

## Ordering

Longhorn, Underlay, and Soundcheck Library are independent foundations and can
upgrade in parallel. Jetstream follows Longhorn. Product applications follow
the shared foundation packages they consume, avoiding duplicated peer and
lockfile remediation.

## Repository State

All foundation repositories were clean on `main` and aligned with
`origin/main` at inventory time. The other consumer roots were also clean
except Bovine Accelerator Desktop: its main checkout was seven commits behind
and contained substantial unrelated active changes. Its later adoption must
use a separate worker worktree from current `origin/main` and preserve that
checkout untouched.

## Exclusions

Poodle's own workspace, temporary `acowtancy-consolidation.*` imports and
worktrees, and Finch's archived Electron application are not active consumers.
Loophole Legacy remains in the final wave pending an explicit retirement
decision.
