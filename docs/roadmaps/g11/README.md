# g11 GPUI Contract Compliance

Status: in-progress
Updated: 2026-03-22

## Context

The GPUI component library has 118 component implementations (matching Svelte
1:1), but many have incomplete contract compliance: missing anatomy parts,
incorrect token usage, broken states, missing accessibility attributes, and
visual deviations from the Svelte reference.

The Svelte implementation has been audited against the contracts and is now the
definitive reference. This generation brings every GPUI component into full
compliance, using the contracts as the spec and Svelte as the visual reference.

## Starting State

- 118 GPUI components (1:1 with Svelte) with Deref containment pattern (from g09)
- Contracts finalized: 85 foundation + 34 composites = 119 contracts
- Svelte reference implementation complete
- Multiple structural issues identified:
  - 7 contracts without Rust specs
  - 12 Rust specs without contract markdowns (potentially orphaned)
  - Naming mismatches between layers (callout/call_out, box/bx, etc.)
  - 1 contract with no implementation anywhere (surface_elevation)
  - Unknown number of per-component compliance gaps

## Exit State

- Every contract has: Rust spec, Svelte implementation, GPUI implementation
- All naming is consistent across layers
- Every GPUI component passes the compliance checklist
- Orphaned Rust specs resolved (deleted or given contracts)
- Visual output matches Svelte reference (with documented intentional deltas)

## Milestone Status

| ID  | Milestone | Depends On | Status |
|-----|-----------|------------|--------|
| 001 | Reorganize misplaced components (6) | — | Complete |
| 002 | Implement missing composites (17) | 001 | Complete |
| 003 | Inputs batch (11 components) | contract audit | Planned |
| 004 | Selection batch (8 components) | contract audit | Planned |
| 005 | Buttons batch (3 components) | contract audit | Planned |
| 006 | Navigation batch (8 components) | contract audit | Planned |
| 007 | Layout batch (13 components) | contract audit | Planned |
| 008 | Feedback batch (13 components) | contract audit | Planned |
| 009 | Overlay batch (8 components) | contract audit | Planned |
| 010 | Temporal batch (8 components) | contract audit | Planned |
| 011 | Composites batch (41+ components) | contract audit | Planned |
| 012 | Visual parity verification | 003–011 | Planned |
| 013 | Generation closeout | 012 | Planned |

## Known Structural Issues (distributed across batches)

### Missing Rust specs (contracts exist, Rust spec doesn't)

| Contract | Batch | Notes |
|----------|-------|-------|
| `toggle.md` | 004 | GPUI component exists with direct fields, needs ToggleSpec |
| `toggle-group.md` | 004 | GPUI component exists, needs ToggleGroupSpec |
| `spacer.md` | 007 | GPUI component is zero-field unit struct, may not need spec |
| `status-bar.md` | 008 | GPUI component exists, needs StatusBarSpec |
| `callout.md` | 008 | Rust spec exists as `call_out.rs` — rename to match contract |
| `surface-elevation.md` | 007 | No implementation anywhere — determine if standalone or sub-pattern |
| `editable-list.md` | 011 | GPUI component exists without spec struct |
| `form-dialog.md` | 011 | GPUI component exists without spec struct |
| `form-layout.md` | 011 | GPUI component exists without spec struct |
| `pagination-summary.md` | 011 | Rust spec is in composites, contract is in foundation |

### Orphaned Rust specs (no contract markdown)

| Rust spec | Batch | Action needed |
|-----------|-------|---------------|
| `badge.rs` | 008 | Verify if renamed/merged — delete if orphaned |
| `banner.rs` | 008 | Verify if renamed/merged — delete if orphaned |
| `call_out.rs` | 008 | Rename to `callout.rs` to match contract |
| `autonomous_list.rs` | 011 | Verify — delete if orphaned |
| `form_shell.rs` | 011 | Verify — may be replaced by `form-layout`/`form-dialog` |
| `inline_remediation.rs` | 011 | Verify — delete if orphaned |
| `remediation_banner.rs` | 011 | Verify — delete if orphaned |
| `shell_status_bar.rs` | 011 | Verify — may be replaced by `status-bar` in foundation |
| `state_tile.rs` | 011 | Verify — delete if orphaned |
| `validation_summary.rs` | 011 | Verify — delete if orphaned |

### Naming mismatches

| Contract name | Rust spec name | GPUI name | Action |
|---------------|---------------|-----------|--------|
| `box` | `box.rs` | `bx.rs` | Expected (Rust keyword) — document |
| `callout` | `call_out.rs` | `callout.rs` | Rename Rust spec to `callout.rs` |
| `pagination-summary` | composites `pagination_summary.rs` | primitives `pagination_summary.rs` | Move Rust spec to primitives |

## Dependency Shape

```text
001 Reorganize ✓
  -> 002 Missing Composites ✓
      -> [contract audit complete ✓]
          -> 003 Inputs        ─┐
          -> 004 Selection      │
          -> 005 Buttons        │
          -> 006 Navigation     │
          -> 007 Layout         ├─> 012 Visual Parity -> 013 Closeout
          -> 008 Feedback       │
          -> 009 Overlay        │
          -> 010 Temporal       │
          -> 011 Composites    ─┘
```

## Per-Component Compliance Checklist

For each component in batches 003–011:

1. Read the contract (`docs/contracts/foundation/<component>.md` or `composites/`)
2. Read the Svelte implementation (visual reference)
3. Read the GPUI implementation
4. Check:
   - [ ] Anatomy: all parts present with correct nesting
   - [ ] Props: all contract props supported in spec
   - [ ] Tokens: every dimension/color/radius resolves from tokens
   - [ ] States: disabled, loading, hover, active, focus all handled
   - [ ] Accessibility: ARIA roles, attributes, keyboard behavior
   - [ ] Visual: matches Svelte reference output
5. Fix all gaps
6. Verify specimen renders correctly

## Non-Goals

- No new components beyond what Svelte has
- No contract changes (contracts are the spec)
- No Jetstream work (that's g10)
