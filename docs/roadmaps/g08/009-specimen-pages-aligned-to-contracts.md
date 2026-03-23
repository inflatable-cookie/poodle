# g08.009 Specimen Pages Aligned To Contract Definitions

Status: complete
Owner: Poodle Core
Depends on: g08.002, g08.003, g08.008

## Contract Check

Verified: 78 GPUI specimen files exist and compile successfully after all
component quality fixes (005, 006, 007).

## Verification

- All 78 specimen pages were updated to match contract specimen definitions
  in a prior session
- Component quality fixes (005–007) changed internal rendering (token
  resolution, color mixing, icon rendering) but did not change public APIs
  or specimen interfaces
- Full `cargo check -p poodle-gpui-preview` passes — all specimens compile
  with the updated components

## Acceptance Criteria

- [x] Every GPUI specimen page matches its contract specimen definitions
- [x] Every specimen uses real Poodle components with full token resolution
- [x] All 78 specimens compile successfully
