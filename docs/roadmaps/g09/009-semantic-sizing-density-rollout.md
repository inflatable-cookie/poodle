# g09.009 — Semantic Sizing and Density Rollout

Status: complete
Completed: 2026-03-30

## Summary

Rolled out the semantic presentation model (`size`, `sizeRole`, `density`)
across all three runtimes (Svelte, GPUI, Jetstream), all contracts, and all
component documentation.

## Deliverables

### System Contracts

- **size-and-density.md** — global contract defining how size and density
  partition visual responsibility. Size controls intrinsic dimensions and
  typography; density controls container padding and sibling gaps.
- **treatment-tokens.md** — documents the 36-token intermediate CSS layer
  used by 22 components for theme-level visual branding.

### Rust Type System

- `SemanticControlSizeRole` enum (Chrome, Control, Prominent)
- `ControlDensity` enum (Compact, Default, Comfortable)
- `Default` impl for `ControlSize` (defaults to Md)
- 52 primitive specs + 22 composite specs updated with `size`, `size_role`,
  `density` fields and builder methods
- 5 new spec structs: TotpInput, PasswordRequirements, SidebarNav, ToastHost,
  EditableList

### Adapter Infrastructure

- GPUI adapter: `with_density()`, `with_control_size()` override methods
- Jetstream adapter: `space_overrides` HashMap + `with_density()`,
  `with_control_size()` methods with priority-based resolution
- GPUI preview app: density/size toggles wired to theme provider rebuild

### Presentation Modules

- `packages/gpui/components/src/presentation.rs` — 12 helpers, 13 tests
- `packages/jetstream/components/src/presentation.rs` — identical module

Both provide: `resolve_semantic_size()`, `control_height_rem()`,
`size_height_offset_rem()`, `size_padding_x_offset_rem()`, `size_font_rem()`,
`size_min_width_rem()`, `resolve_supporting_visual_size()`,
`control_space_x_rem()`, `panel_space_x_rem()`, `panel_space_y_rem()`,
`rem_to_px()`.

### Svelte (Reference Implementation)

- 75 interactive components: `size`, `sizeRole`, `density` props wired
- CSS `[data-size]` variants (xs through xl) for all 75 components
- CSS `[data-density]` variants (compact/default/comfortable) for all 75
- `data-size` and `data-density` attributes emitted on component roots
- UiPresentationProvider context integration
- 8 size-vs-density violations audited and fixed
- 3 accessibility fixes: Combobox `aria-activedescendant`, Menubar
  hover-to-switch, NavigationMenu roving tabindex
- Component-docs entries for all 73 interactive components
- Specimen "Sizes" and "Densities" sections for ~49 components

### GPUI

- 78 components wired with presentation helpers (52 primitives + 22
  composites + 4 display fixes)
- 5 new component implementations (TotpInput, PasswordRequirements,
  SidebarNav, ToastHost, EditableList)
- 5 spec helper methods added (Meter, StatusIndicator, Eyebrow, Tooltip,
  Skeleton)
- ARIA documentation added to 6 components
- Hardcoded px values replaced in 5 display components

### Jetstream

- 128 components total (86 upgraded + 42 new)
- Presentation module with 13 tests
- 79 existing components: hardcoded px values replaced with token resolution
- 18 new primitive implementations
- 24 new composite implementations
- 13 new adapter registrations with RenderComponent trait
- 66 component tests + 177 adapter tests = 243 tests passing

### Contract Documentation

- All 35 seed contracts upgraded to detailed (zero seeds remaining)
- Deep audit of 70 components against Svelte implementations
- OrderBy and Pagination contracts rewritten from scratch
- 16 systemic discrepancies fixed across contracts
- FileUpload contract realigned to implementation
- lg/xl heading sizes, date picker fonts, missing props all corrected

## Test Results

- Svelte: `effigy health` green
- Rust primitives: 32 tests pass
- Rust composites: compile clean
- GPUI adapter: compile clean
- Jetstream components: 66 tests pass
- Jetstream adapter: 177 tests pass

## Acceptance Criteria Met

- [x] Every interactive component in all three runtimes has size/sizeRole/density
- [x] Global contract defines size vs density responsibility separation
- [x] No hardcoded px values in GPUI or Jetstream component code
- [x] Presentation helpers produce identical values across runtimes
- [x] Zero seed contracts remain
- [x] Contract audit found and fixed all major discrepancies
- [x] All components that existed in Svelte have GPUI + Jetstream counterparts
