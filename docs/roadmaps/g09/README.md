# g09 Architecture Unification

Status: complete
Updated: 2026-03-21

## Context

Flint currently has duplicated Rust crate infrastructure across its GPUI and
Jetstream targets. Component specs, token bindings, and shared types are
maintained in parallel — `flint-primitives` (contracts) and `flint-gpui-primitives`
(GPUI) contain diverged copies of the same specs. This makes every contract
change a two-crate update, creates inconsistencies, and forces developers to
import from three separate crates to use a single component.

Jetstream already uses the contracts crates directly (`flint-primitives`,
`flint-tokens`), proving the shared approach works. GPUI needs to follow suit.

This generation eliminates all duplicated crates, establishes a single spec
surface for both targets, and simplifies the developer experience to a single
import per component.

## Starting State

### Duplicated crates (to be eliminated)

| Contracts (shared) | GPUI (duplicate) | Divergence |
|---------------------|-------------------|------------|
| `flint-tokens` | `flint-gpui-tokens` | Same generated source, GPUI omits `typed` module |
| `flint-primitives` (65 modules) | `flint-gpui-primitives` (77 modules) | GPUI has 12 extra modules, some specs add hardcoded px helpers |
| `flint-composites` | `flint-gpui-composites` | Different module sets — 7 in GPUI only, 21 in contracts only |
| `flint-workstation` | `flint-gpui-workstation` | Already migrated to composites in g08, workstation should be deleted |

### Current developer experience (GPUI)

```rust
// Three crate imports for one component
use flint_gpui_primitives::{ButtonSpec, ButtonVariant, ControlSize};
use flint_gpui_components::FlintButton;
use flint_gpui::GpuiThemeProvider;

// Construct spec, then wrap in renderer
let spec = ButtonSpec::new()
    .with_variant(ButtonVariant::Primary)
    .with_label("Save");
let button = FlintButton::new(spec, &theme);
```

### Current developer experience (Jetstream)

```rust
// Two crate imports — already simpler
use flint_primitives::{ButtonSpec, ButtonVariant};
use flint_jetstream_components::js_button;

let spec = ButtonSpec::new()
    .with_variant(ButtonVariant::Primary)
    .with_label("Save");
let el = js_button(&spec, &theme);
```

## Exit State

### Single crate surface

```
contracts/
  tokens/      → flint-tokens         (ONE token crate)
  primitives/  → flint-primitives     (ONE spec crate for foundation components)
  composites/  → flint-composites     (ONE spec crate for composite components)
  adapter/     → flint-adapter        (ThemeProvider trait, shared Color type)

gpui/
  adapter/     → flint-gpui           (GpuiThemeProvider, color conversion)
  components/  → flint-gpui-components (Button, Checkbox, etc. — no Flint prefix)

jetstream/
  adapter/     → flint-jetstream      (JetstreamThemeProvider, color conversion)
  components/  → flint-jetstream-components (js_button, js_checkbox, etc.)
```

### Deleted crates

- `flint-gpui-tokens` — absorbed into `flint-tokens`
- `flint-gpui-primitives` — absorbed into `flint-primitives`
- `flint-gpui-composites` — absorbed into `flint-composites`
- `flint-gpui-workstation` — deleted (already migrated in g08)
- `flint-workstation` (contracts) — deleted if no longer referenced

### Target developer experience (GPUI)

```rust
use flint_gpui_components::{Button, ButtonVariant, ControlSize};

Button::new()
    .variant(ButtonVariant::Primary)
    .label("Save")
    .on_click(|e, w, cx| { ... })
```

Single crate import. No spec construction. No Flint prefix (the crate name
is the namespace). Props set directly on the component struct via builder
methods. The spec exists internally but developers don't need to see it.

### Target developer experience (Jetstream)

```rust
use flint_jetstream_components::{Button, ButtonVariant};

Button::new()
    .variant(ButtonVariant::Primary)
    .label("Save")
    .build(&theme)  // returns JsEl
```

Same pattern, same types, different output.

## Non-Goals

- No new component implementations (that's g10)
- No contract changes (specs unify to current contract definitions)
- No Jetstream runtime changes (event system is a separate Jetstream effort)
- No Svelte changes

## Milestone Status

| ID  | Milestone | Depends On | Class | Status |
|-----|-----------|------------|-------|--------|
| 001 | Unify token crates | — | Foundation | Complete |
| 002 | Merge GPUI-only specs into contracts primitives | 001 | Foundation | Complete |
| 003 | Merge composite specs | 002 | Foundation | Complete |
| 004 | Delete duplicate GPUI crates and update imports | 003 | Migration | Complete |
| 005 | Simplify component API (drop Flint prefix, re-export types) | 004 | API | Complete |
| 006 | Delete workstation crates | 004 | Cleanup | Complete |
| 007 | Verify both targets compile and preview apps run | 005, 006 | Hardening | Complete |
| 008 | Generation closeout | 007 | Closure | Complete |

## Dependency Shape

```text
001 Unify Tokens
  -> 002 Merge Primitives
      -> 003 Merge Composites
          -> 004 Delete GPUI Duplicate Crates
              -> 005 Simplify Component API
              -> 006 Delete Workstation Crates
                  -> 007 Verify Both Targets
                      -> 008 Closeout
```

## Milestone Details

### 001 — Unify Token Crates

Both `flint-tokens` and `flint-gpui-tokens` point at the same generated file
(`tokens/artifacts/rust/mod.rs`). The only difference is `flint-tokens` exports
a `typed` module that `flint-gpui-tokens` omits.

**Actions:**
- Add `typed` module export to `flint-tokens` if not already present
- Update all GPUI crates to depend on `flint-tokens` instead of `flint-gpui-tokens`
- Delete `packages/gpui/tokens/`
- Verify: `cargo check` for all GPUI crates

### 002 — Merge GPUI-Only Specs into Contracts Primitives

12 spec modules exist in `flint-gpui-primitives` but not `flint-primitives`:
`alert_dialog`, `breadcrumbs`, `bulk_action_bar`, `card`, `combobox`,
`detail_row`, `icon`, `list_card`, `nav_card`, `order_by`, `pagination`,
`table`.

Some GPUI specs also diverge from their contract counterparts (e.g. ButtonSpec
adds `chevron`, `height_offset_px()`, `min_width_px()`).

**Actions:**
- Copy 12 missing modules into `flint-primitives`, updating token imports
  from `flint_gpui_tokens` to `flint_tokens`
- For diverged specs: merge GPUI additions into the contracts version. Move
  hardcoded pixel helpers (like `height_offset_px()`) out of the spec and
  into the GPUI component renderer where they belong
- Merge any extra types/enums from GPUI's `types.rs` into contracts `types.rs`
- Verify: `cargo check` for `flint-primitives` and `flint-jetstream-components`
  (Jetstream must not break)

### 003 — Merge Composite Specs

7 modules in GPUI composites not in contracts: `action_discovery_panel`,
`app_header`, `command_palette`, `dock_region`, `metric_tile`,
`shell_status_bar`, `split_view`.

Shared types in GPUI composites `types.rs` also need merging.

**Actions:**
- Copy 7 missing modules into `flint-composites`
- Merge GPUI `types.rs` additions into contracts `types.rs`
- Verify: `cargo check` for `flint-composites`

### 004 — Delete Duplicate GPUI Crates and Update Imports

With specs unified, the GPUI-specific spec crates are redundant.

**Actions:**
- Update `flint-gpui` (adapter) Cargo.toml: depend on `flint-primitives` +
  `flint-composites` instead of `flint-gpui-primitives` + `flint-gpui-composites`
- Update `flint-gpui-components` Cargo.toml: same
- Update `flint-gpui-preview` Cargo.toml: same
- Find-and-replace all `use flint_gpui_primitives::` → `use flint_primitives::`
- Find-and-replace all `use flint_gpui_composites::` → `use flint_composites::`
- Find-and-replace all `use flint_gpui_tokens::` → `use flint_tokens::`
- Delete `packages/gpui/primitives/`
- Delete `packages/gpui/composites/`
- Verify: `cargo check` for all GPUI crates

### 005 — Simplify Component API

Drop the `Flint` prefix from component structs and re-export common types
so developers only need one import.

**Actions:**
- Rename `FlintButton` → `Button`, `FlintCheckbox` → `Checkbox`, etc. across
  all ~100 component files
- Re-export commonly used types from `flint-gpui-components`:
  `ButtonVariant`, `ControlSize`, `ButtonTone`, `IconSize`, etc.
- Update all specimen files in preview to use new names
- Consider: should the component struct own its props directly (no separate
  spec construction), or take a spec? Decide and implement consistently.
- Verify: `cargo check -p flint-gpui-preview`

### 006 — Delete Workstation Crates

Both `flint-workstation` (contracts) and `flint-gpui-workstation` should be
deleted. Their specs were migrated to composites in g08.

**Actions:**
- Verify no remaining imports of `flint_workstation` or `flint_gpui_workstation`
- Delete `packages/contracts/workstation/`
- Delete `packages/gpui/workstation/`
- Verify: full `cargo check`

### 007 — Verify Both Targets

**Actions:**
- `cargo check` for all GPUI crates
- `cargo check` for all Jetstream crates
- `cargo test` for primitives and adapter crates
- Run GPUI preview app — visually confirm components render
- Verify Jetstream preview compiles (runtime may not be available)

### 008 — Generation Closeout

**Actions:**
- Verify all milestones complete
- Count: how many crates eliminated, how many import paths simplified
- Confirm g10 (Jetstream Production Quality) can begin from unified baseline
- Close generation

## Risk Register

| Risk | Mitigation |
|------|------------|
| Spec divergence hides GPUI-specific features | Audit every diverged spec; move rendering logic to component, keep specs pure contract |
| Jetstream breaks when primitives gains new modules | New modules are additive — Jetstream doesn't import what it doesn't use |
| GPUI `typed` token module causes issues in Jetstream | Already exported by `flint-tokens` — Jetstream just doesn't use it |
| Renaming 100 Flint* structs is error-prone | Mechanical find-and-replace, verify with cargo check |
| Preview apps break during migration | Each milestone ends with cargo check; no milestone ships broken |
