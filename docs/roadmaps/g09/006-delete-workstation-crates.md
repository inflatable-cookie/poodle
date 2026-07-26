# g09.006 Delete Workstation Crates

Status: complete
Owner: Poodle Core
Depends on: g09.004

## Context

Workstation specs were migrated into composites during g08. The GPUI
workstation implementation crate has been deleted. The contracts
workstation crate (`poodle-workstation`) remains because the GPUI and
Jetstream adapters still reference it for render stub functions.

## Completed Actions

- [x] Deleted `packages/gpui/workstation/` (implementation crate, orphaned)
- [x] Verified no GPUI component code references workstation

## Remaining

None. The last piece — `packages/contracts/workstation/` itself — was removed in
`g12/012-workstation-tier-removal.md`, along with both adapters' render stubs.
