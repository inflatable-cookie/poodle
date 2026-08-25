# g16.020 / g16.022 — Longhorn Product Adoption

Date: 2026-08-25
Status: complete
PRs: [Figmatic #17](https://github.com/inflatable-cookie/figmatic/pull/17),
[Loophole #8](https://github.com/inflatable-cookie/loophole/pull/8)
Merge commits: Figmatic `a6286e88a51935e2a897b1a1dc2b1a88f63be39c`,
Loophole `92e0157792ef8d38bcce9a4d957aba490204f41d`

## Result

Figmatic Studio and Loophole Desktop now resolve exact public Poodle core and
Svelte 0.2.2 packages. Both retained their local Longhorn integration while
removing committed sibling Poodle sources and Poodle-only overrides. Each graph
contains one Poodle Svelte identity and the Longhorn adapter peer converges on
it.

## Validation

Figmatic passed Studio check, build, native-componentization tests, repository
QA, and diff checks. Loophole passed renderer build, workspace tests, repository
QA, and diff checks. Its single renderer-test failure reproduced unchanged on
`main` and was kept separate from the adoption result.

## Continuation

Finch `g16.019` remains changes-requested for stale Longhorn peer metadata.
Bovine Accelerator Desktop `g16.021` remains the other active Longhorn product
lane. Underlay Reference `g16.025` remains changes-requested independently.
