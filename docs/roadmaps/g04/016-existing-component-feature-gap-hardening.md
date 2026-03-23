# g04.016 Existing Component Feature Gap Hardening

Status: planned
Owner: Flint Core
Updated: 2026-03-14
Depends on: g04.002 through g04.012
Primary repos: `flint`

## Goals

- [ ] close all remaining feature gaps identified in the Underlay parity audit
  for existing Flint components
- [ ] ensure feature-extended components maintain backward compatibility

## Execution Checklist

### Button Enhancements
- [ ] add split-button variant prop to Button (shares impl with SplitButton)

### Card Enhancements
- [ ] add `selected` and `selectable` props to Card for radio/checkbox contexts
- [ ] add specialized card layout variants (media-top, horizontal, compact)

### Dialog Enhancements
- [ ] add AlertDialog variant mode to Dialog (or ensure AlertDialog primitive
  covers the need)

### Field Enhancements
- [ ] add `span` prop for CSS grid column control
- [ ] add `gridArea` prop for named grid area placement

### Skeleton Enhancements
- [ ] verify preset coverage from g04.010 covers all Underlay DataSkeleton
  patterns

### StateTile Enhancements
- [ ] verify trend/sparkline from g04.012 covers Underlay StateTile patterns

### General
- [ ] audit all amended contracts for backward compatibility
- [ ] update all affected specimens to demonstrate new features
- [ ] run full build and preview verification

## Acceptance Criteria

- [ ] all feature gaps from the Underlay audit are resolved or explicitly
  deferred with justification
- [ ] no existing component API is broken by the extensions
- [ ] all amended components have updated specimens showing new features
- [ ] full build passes

## Next Task

Open `g04.017` and ensure preview specimen coverage and documentation are
complete.
