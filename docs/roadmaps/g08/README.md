# g08 GPUI Production Quality

Status: active
Updated: 2026-03-19

## Context

Prior generations built 98 GPUI component files with a working preview app and
79 specimen pages. However, a thorough audit found that **every component is
at Partial quality** — colors are mostly token-resolved, but dimensions are
universally hardcoded, focus rings are absent, ARIA attributes are minimal, and
several components have functional bugs.

Concurrently, the Svelte reference implementation is being actively refined —
component names, contracts, and composite boundaries are a moving target. Each
milestone in this generation must verify current contract state before
implementing, not assume contracts are stable from the outset.

This generation brings every GPUI component to production quality: full token
resolution, contract compliance, and visual parity with Svelte.

## Starting State

- 98 GPUI component files in `packages/gpui/components/src/` — all Partial
- 79 specimen pages in `packages/gpui/preview/src/specimens/`
- Colors mostly token-resolved; dimensions universally hardcoded
- Zero focus rings across all components
- Minimal ARIA attributes
- `icon_button` renders icon names as text, not SVG
- `color_picker` swatch rendering broken (colors never applied)
- `range_slider` fill and thumbs not rendered (values discarded)
- Disabled opacity hardcoded as `0.48` in ~18 components
- Hover states use hardcoded `hsla()` in ~10 components
- GPUI primitives spec crate 2 generations behind contracts crate
- Contract specimen definitions added to all 122 contracts
- Svelte-side refactoring in progress (names, contracts, composites may change)

## Exit State

- Every GPUI component resolves all visual properties from semantic tokens
- Zero hardcoded pixel values in component rendering code
- All interactive components have focus rings per contract
- ARIA attributes applied per contract
- All icon slots use `PugIcon` with real SVG rendering
- Broken components (`color_picker`, `range_slider`) fully functional
- Specimen pages match contract specimen definitions
- Visual parity with Svelte systematically verified and documented
- Any intentional GPUI-specific deltas documented with rationale

## Non-Goals

- No new component families or features beyond what contracts specify
- No Jetstream work (deferred to g09)
- No downstream app adoption proof

## Milestone Status

| ID  | Milestone | Depends On | Class | Status |
|-----|-----------|------------|-------|--------|
| 001 | Sync with contracts: verify names, props, and token methods | — | Foundation | Planned |
| 002 | Cross-cutting fixes: disabled opacity, hover colors, geometry tokens | 001 | Implementation | Planned |
| 003 | High-visibility component fixes (button, icon_button, checkbox, switch, text_input, select, tabs) | 002 | Implementation | Planned |
| 004 | Input and selection component fixes (text_area, number_entry, radio_group, slider, segmented_control, pin_input) | 002 | Implementation | Planned |
| 005 | Remaining component fixes (time_field, duration_input, tri_state_switch, rating, tooltip, drawer, color_picker, range_slider) | 002 | Implementation | Planned |
| 006 | Focus rings and ARIA attributes | 003, 004, 005 | Implementation | Planned |
| 007 | Specimen pages aligned to contract definitions | 006 | Implementation | Planned |
| 008 | Visual parity verification and delta register | 007 | Hardening | Planned |
| 009 | Generation closeout | 008 | Closure | Planned |

## Dependency Shape

```text
001 Sync with Contracts
  -> 002 Cross-Cutting Fixes
      -> 003 Batch 1: High-Visibility  ─┐
      -> 004 Batch 2: Input/Selection   ├─> 006 Focus Rings + ARIA
      -> 005 Batch 3: Remaining        ─┘       -> 007 Specimens
                                                      -> 008 Parity
                                                           -> 009 Closeout
```

## Execution Lanes

### Lane A: Component Quality (parallel batches)

`002 -> { 003, 004, 005 } -> 006 -> 007`

Batches 003, 004, 005 can execute in parallel once 002 lands the shared fixes.

### Lane B: Verification and Closeout

`008 -> 009`

## Contract Verification Rule

**Every milestone must begin by checking the current state of relevant
contracts in `docs/contracts/`.** Component names, props, token targets, and
composite boundaries are being actively refactored on the Svelte side.
Assumptions from previous sessions may be stale. Specifically:

- Verify the component still exists in the contract (not renamed/removed)
- Verify prop names and types match current contract
- Verify token target names match current contract
- Verify the spec struct in `pug-primitives`/`pug-composites` matches
- If a contract has changed, update the GPUI implementation to match before
  proceeding with quality fixes

This verification is not a one-time gate — it applies at the start of each
milestone because the Svelte side is a concurrent moving target.
