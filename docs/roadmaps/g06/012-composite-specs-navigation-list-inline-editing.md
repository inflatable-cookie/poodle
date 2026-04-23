# g06.012 — Composite Specs: Navigation, List Interaction, and Inline Editing

Status: Completed
Updated: 2026-03-14

> Historical note: this milestone records the pre-`g10.004` split component
> spec crates. The live merged crate is now `poodle-specs`.

## Objective

Add 13 new composite spec structs for navigation, list interaction, and inline
editing components.

## New Specs

| Spec | Category |
|------|----------|
| `AutonomousListSpec` | List interaction |
| `BreadcrumbsSpec` | Navigation |
| `CardRadioGroupSpec` | Selection |
| `ConfirmActionSpec` | Dialog |
| `DetailSectionSpec` | Layout |
| `InlineEditableFieldSpec` | Inline editing |
| `ListCardSpec` | Navigation |
| `NavCardSpec` | Navigation |
| `NavCardGridSpec` | Navigation |
| `OrderBySpec` | List interaction |
| `PageHeaderSpec` | Navigation |
| `ReorderableListSpec` | List interaction |
| `SlugFieldSpec` | Inline editing |

## New Supporting Type

`BreadcrumbItem` added to composites `types.rs` — navigation breadcrumb entry
with id, label, and optional href.

## Running Total

Composite specs: 28 (after 011) + 13 (new) = **41**

## Final Spec Count

| Crate | Specs |
|-------|-------|
| `poodle-primitives` | 64 |
| `poodle-composites` | 41 |
| `poodle-workstation` | 13 |
| **Total** | **118** |

Note: The target was 124 to match the full Svelte surface. The delta of 6
represents shared/composite components whose Svelte implementations decompose
into multiple primitives at the spec level (e.g., AlertDialog → DialogSpec
with DialogKind::AlertDialog, FormDialog → FormShellSpec + DialogSpec).

## Verification

- [x] All 13 specs compile and are exported from `poodle-composites`
- [x] All 10 existing tests continue to pass
- [x] `BreadcrumbItem` added to types.rs and re-exported
