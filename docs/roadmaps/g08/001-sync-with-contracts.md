# g08.001 Sync With Contracts: Verify Names, Props, And Token Methods

Status: planned
Owner: Pug Core
Depends on: —

## Why This Comes First

The Svelte reference implementation is being actively refactored — component
names, prop types, composite boundaries, and contracts are changing. Before
fixing any GPUI component quality issues, we must ensure our implementations
target the correct contracts. Fixing a component that's been renamed or whose
props have changed is wasted work.

## Execution Checklist

### Verify Primitive Contracts

- [ ] For each component in `docs/contracts/foundation/`, confirm a matching
      GPUI component exists in `packages/gpui/components/src/`
- [ ] Check for renamed or removed contracts — delete/rename GPUI components
      to match
- [ ] Verify each spec struct in `packages/contracts/primitives/src/` matches
      its contract (prop names, types, defaults)
- [ ] Verify GPUI spec structs in `packages/gpui/primitives/src/` match the
      contracts crate — note generation drift (`g04.006` vs `g06.002`)

### Verify Composite Contracts

- [ ] For each component in `docs/contracts/composites/`, confirm a matching
      GPUI component exists
- [ ] Check for components moved between primitive/composite/workstation
      categories during the Svelte consolidation
- [ ] Verify spec structs match

### Check Token Targets

- [ ] For a representative sample (10-15 components), verify that token method
      names on the spec (e.g., `fill_token()`, `radius_token()`) return token
      strings that match what the current contract specifies
- [ ] Flag any spec methods returning stale or renamed tokens

### Document Gaps

- [ ] List any contracts with no GPUI implementation
- [ ] List any GPUI components with no matching contract (orphans)
- [ ] List spec struct divergences that need fixing

## Acceptance Criteria

- [ ] Every GPUI component maps to a current contract
- [ ] No orphaned components targeting deleted/renamed contracts
- [ ] Spec struct divergences documented and queued for fix in 002+
- [ ] Clear picture of which contracts are stable vs still changing
