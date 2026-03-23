# g07.012 — GPUI Demo-App Parity Implementation

Status: Completed
Updated: 2026-03-14

## Objective

Implement a GPUI demo app that exercises all 6 screen families from the shared
demo-app contract, proving the adapter can assemble complete screen layouts from
the shared spec layer.

## Deliverables

### Demo app module (demo_app.rs)

Public module added to `flint-gpui` crate with:

- `DemoScreen` struct tracking rendered element handles per screen
- `render_all_screens()` function assembling all 6 screens
- Per-screen render functions matching the shared contract's screen families

### Screen coverage

| Screen | ID | Components | Key specs exercised |
|--------|-----|-----------|-------------------|
| Overview Shell | `overview-shell` | 15 | StateTile, Banner, Progress, NavCardGrid, LogList |
| Form & Validation | `form-and-validation` | 14 | FormShell, TextInput, ValidationSummary, RemediationBanner |
| Browse & Filter | `browse-and-filter` | 12 | DataTable, FilterToolbar, PaginationSummary, EmptyState |
| Detail & Review | `detail-and-review` | 10 | DetailShell, DetailSection, MediaThumbnail, MediaPreview |
| Picker & Selection | `picker-and-selection` | 7 | PickerShell, RelationPicker, SelectionSummary |
| Command & Workspace | `command-and-workspace` | 11 | WorkspaceShell, CommandPalette, DockRegion, SplitView |

### Test coverage

- 8 tests verifying screen assembly and component coverage:
  - All 6 screens render without panic
  - Each screen contains expected spec types
  - Total component count across all screens exceeds 60

## Verification

```
cargo test — 137 → 145 tests passing (8 new)
cargo check — clean compilation, no warnings
```
