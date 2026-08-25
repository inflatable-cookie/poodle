# g16.025 — Underlay Reference v0.9.4 Adoption

Date: 2026-08-25
Status: complete
PR: [Underlay Reference #2](https://github.com/inflatable-cookie/underlay-reference/pull/2)
Reviewed head: `25818824412441f499ee4c50f3f1861f9b321350`
Merge: `3354803ebc484d4d611878a8e60e356ab92e206e`

## Result

The canonical Underlay reference estate now resolves released Underlay v0.9.4
instead of active sibling paths. Admin, Front, UI, and Client consume the tagged
web packages. Every active API crate uses the same Git tag. All Bun and Cargo
locks resolve peeled Underlay commit
`7004af5b3461b6c89a7faa646575ff69576c73b8`; Poodle remains exact public 0.2.2.

The reviewed implementation also removed source-bypass aliases and clarified
the README boundary between released application dependencies and optional
sibling framework-development mounts. Stale v0.9.2 PR metadata was corrected
before merge without changing the approved implementation head.

## Validation

Passed:

- `effigy acme-admin/validate`
- `effigy acme-front/validate`
- `effigy acme-ui/validate`
- `effigy acme-client/validate`
- `effigy acme-api/build`
- `effigy qa:docs`
- empty-directory frozen installs for all four Bun packages
- fresh v0.9.4 tag resolution
- `git diff --check`

The pre-existing `effigy validate` and `effigy qa` test-routing baseline from
g16.013 remains separate: Effigy auto-selects Vitest for packages whose tests
are absent or outside the configured include. No local exception was added.

## Continuation

Underlay Reference is complete. Review the bounded Finch correction and
continue Bovine Accelerator Desktop. Jetstream and Loophole Legacy remain the
final adoption wave.
