# g16.013 — Underlay Reference Poodle 0.2.2 Adoption

Date: 2026-08-25
Status: complete
PR: [Underlay Reference #1](https://github.com/inflatable-cookie/underlay-reference/pull/1)
Merge: `f5ea7d72eee278e8838ba16f8f43eb2b662406d0`

## Result

The canonical Underlay reference estate now consumes public Poodle 0.2.2.
Admin, Front, and UI use exact core/Svelte registry pins. Four regenerated Bun
locks contain the published integrity values. Local Underlay development links
remain; committed Poodle overrides and older Poodle identities are gone.

Compatibility work stayed bounded to current Underlay source: Nightfire empty
drafts, `BlockDescriptor::new`, reorder-test imports, rollout-check location,
and existing Rust format/clippy drift.

## Validation

Passed:

- `effigy qa:docs`
- `effigy acme-admin/check`
- `effigy acme-front/check`
- `effigy acme-ui/check`
- `effigy acme-client/check`
- `git diff --check`

`effigy validate` and `effigy qa` expose two pre-existing routing failures.
Front's tests are outside its configured Vitest include. UI has no tests but is
auto-routed to a transitive Vitest binary. Review rejected local
`passWithNoTests` exceptions; the baseline is recorded in the consumer's
`PAPERCUTS.md`.

## Continuation

The three first-wave migration shapes are complete: direct Longhorn app,
multi-layer Longhorn app, and canonical Underlay estate. Compile one card per
remaining repository. Keep product repos independent; leave Jetstream and
Loophole Legacy to the final wave.
