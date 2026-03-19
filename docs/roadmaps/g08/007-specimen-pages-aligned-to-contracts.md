# g08.007 Specimen Pages Aligned To Contract Definitions

Status: planned
Owner: Pug Core
Depends on: g08.006

## Contract Check

Before updating specimens, verify that the specimen definitions in each
contract are still current. The Svelte side may have added, removed, or
renamed specimens since the definitions were last written.

## Goals

Update every GPUI specimen page to render exactly the examples specified in
contract specimen definitions, ensuring visual parity with the Svelte preview.

## Execution Checklist

- [ ] For each specimen page, compare to the contract's Specimen Definitions
      section
- [ ] Update section headers, labels, and props to match contract
- [ ] Verify each specimen uses real Pug components (no mockups)
- [ ] Remove any specimens not in the contract
- [ ] Add any specimens the contract requires but are missing
- [ ] Verify rendering after component fixes from 003–006

## Note on Prior Work

78 GPUI specimen files were updated in a prior session to match contract
definitions. This milestone verifies those changes still hold after the
component quality fixes and any contract changes that have landed since.

## Acceptance Criteria

- [ ] Every GPUI specimen page matches its contract specimen definitions
- [ ] Every specimen uses real Pug components with full token resolution
- [ ] Visual output matches Svelte reference for each page
