# g09.006 Delete Workstation Crates

Status: complete
Owner: Pug Core
Depends on: g09.004

## Context

Workstation specs were migrated into composites during g08. Both
`pug-workstation` (contracts) and `pug-gpui-workstation` (GPUI) should
be deleted.

## Actions

- [ ] Grep for any remaining `pug_workstation` or `pug_gpui_workstation`
      imports — fix or remove
- [ ] Remove workstation dependencies from any Cargo.toml files
- [ ] Delete `packages/contracts/workstation/`
- [ ] Delete `packages/gpui/workstation/`
- [ ] `cargo check` for all crates

## Acceptance Criteria

- [ ] Zero references to workstation crates anywhere
- [ ] Deleted directories do not exist
- [ ] All crates compile
