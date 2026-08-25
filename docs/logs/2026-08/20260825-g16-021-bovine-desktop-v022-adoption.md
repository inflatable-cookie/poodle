# g16.021 — Bovine Accelerator Desktop Poodle 0.2.2 Adoption

Date: 2026-08-25
Status: complete
PR: [Bovine Accelerator Desktop #25](https://github.com/acowtancy/bovine-accelerator-desktop/pull/25)
Reviewed head: `fc779cf64d6cb6926730083f6f9b65d14720dfd8`
Merge: `3e071692afa6b790c0820f0a43c88753bd926ad1`

## Result

Bovine Accelerator Desktop now consumes exact public Poodle core and Svelte
0.2.2. Poodle overrides are removed while local Longhorn dependencies remain.
Icon generation runs through the published `poodle-icons` binary. Froyo and
the Longhorn adapter converge on the same Poodle Svelte runtime, with no nested
0.1.x identity left in the development lock.

The Acowtancy-side review found the initial stale Froyo snapshot, required its
lock and receipt refresh, then approved and merged the corrected graph. This
Poodle closeout reconciles that already-merged evidence rather than duplicating
the consumer work.

## Validation

Passed on the reviewed PR head:

- `effigy prepare:longhorn-private-candidate`
- `effigy check:dependencies:release`
- `effigy check:dependencies:release:source-independent`
- `effigy check:dependencies:development`
- `effigy check:frontend`
- `effigy test:desktop`
- `git diff --check`

Current Bovine `main` still has exact registry Poodle 0.2.2, published
integrities, no Poodle 0.1.x lock entry, and a passing development dependency
proof. A later PR 26 invalidated the frozen private-candidate receipt against
the newer source set; that separate post-merge currentness issue is recorded in
Poodle `PAPERCUTS.md`.

## Continuation

All four Longhorn-shaped product lanes are complete. Prepare independent
Jetstream and Loophole Legacy handoffs for the final adoption wave.
