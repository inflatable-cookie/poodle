# g09 Architecture Unification

Status: planned
Updated: 2026-03-19

## Context

Pug currently has duplicated Rust crate infrastructure across its GPUI and
Jetstream targets. Component specs, token bindings, and shared types are
maintained in parallel — `pug-primitives` (contracts) and `pug-gpui-primitives`
(GPUI) contain diverged copies of the same specs. This makes every contract
change a two-crate update, creates inconsistencies, and forces developers to
import from three separate crates to use a single component.

Jetstream already uses the contracts crates directly (`pug-primitives`,
`pug-tokens`), proving the shared approach works. GPUI needs to follow suit.

This generation eliminates all duplicated crates, establishes a single spec
surface for both targets, and simplifies the developer experience to a single
import per component.

## Starting State

### Duplicated crates (to be eliminated)

| Contracts (shared) | GPUI (duplicate) | Divergence |
|---------------------|-------------------|------------|
| `pug-tokens` | `pug-gpui-tokens` | Same generated source, GPUI omits `typed` module |
| `pug-primitives` (65 modules) | `pug-gpui-primitives` (77 modules) | GPUI has 12 extra modules, some specs add hardcoded px helpers |
| `pug-composites` | `pug-gpui-composites` | Different module sets — 7 in GPUI only, 21 in contracts only |
| `pug-workstation` | `pug-gpui-workstation` | Already migrated to composites in g08, workstation should be deleted |

### Current developer experience (GPUI)

```rust
// Three crate imports for one component
use pug_gpui_primitives::{ButtonSpec, ButtonVariant, ControlSize};
use pug_gpui_components::PugButton;
use pug_gpui::GpuiThemeProvider;

// Construct spec, then wrap in renderer
let spec = ButtonSpec::new()
    .with_variant(ButtonVariant::Primary)
    .with_label("Save");
let button = PugButton::new(spec, &theme);
```

### Current developer experience (Jetstream)

```rust
// Two crate imports — already simpler
use pug_primitives::{ButtonSpec, ButtonVariant};
use pug_jetstream_components::js_button;

let spec = ButtonSpec::new()
    .with_variant(ButtonVariant::Primary)
    .with_label("Save");
let el = js_button(&spec, &theme);
```

## Exit State

### Single crate surface

```
contracts/
  tokens/      → pug-tokens         (ONE token crate)
  primitives/  → pug-primitives     (ONE spec crate for foundation components)
  composites/  → pug-composites     (ONE spec crate for composite components)
  adapter/     → pug-adapter        (ThemeProvider trait, shared Color type)

gpui/
  adapter/     → pug-gpui           (GpuiThemeProvider, color conversion)
  components/  → pug-gpui-components (Button, Checkbox, etc. — no Pug prefix)

jetstream/
  adapter/     → pug-jetstream      (JetstreamThemeProvider, color conversion)
  components/  → pug-jetstream-components (js_button, js_checkbox, etc.)
```

### Deleted crates

- `pug-gpui-tokens` — absorbed into `pug-tokens`
- `pug-gpui-primitives` — absorbed into `pug-primitives`
- `pug-gpui-composites` — absorbed into `pug-composites`
- `pug-gpui-workstation` — deleted (already migrated in g08)
- `pug-workstation` (contracts) — deleted if no longer referenced

### Target developer experience (GPUI)

```rust
use pug_gpui_components::{Button, ButtonVariant, ControlSize};

Button::new()
    .variant(ButtonVariant::Primary)
    .label("Save")
    .on_click(|e, w, cx| { ... })
```

Single crate import. No spec construction. No Pug prefix (the crate name
is the namespace). Props set directly on the component struct via builder
methods. The spec exists internally but developers don't need to see it.

### Target developer experience (Jetstream)

```rust
use pug_jetstream_components::{Button, ButtonVariant};

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
| 001 | Unify token crates | — | Foundation | Planned |
| 002 | Merge GPUI-only specs into contracts primitives | 001 | Foundation | Planned |
| 003 | Merge composite specs | 002 | Foundation | Planned |
| 004 | Delete duplicate GPUI crates and update imports | 003 | Migration | Planned |
| 005 | Simplify component API (drop Pug prefix, re-export types) | 004 | API | Planned |
| 006 | Delete workstation crates | 004 | Cleanup | Planned |
| 007 | Verify both targets compile and preview apps run | 005, 006 | Hardening | Planned |
| 008 | Generation closeout | 007 | Closure | Planned |

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

Both `pug-tokens` and `pug-gpui-tokens` point at the same generated file
(`tokens/artifacts/rust/mod.rs`). The only difference is `pug-tokens` exports
a `typed` module that `pug-gpui-tokens` omits.

**Actions:**
- Add `typed` module export to `pug-tokens` if not already present
- Update all GPUI crates to depend on `pug-tokens` instead of `pug-gpui-tokens`
- Delete `packages/gpui/tokens/`
- Verify: `cargo check` for all GPUI crates

### 002 — Merge GPUI-Only Specs into Contracts Primitives

12 spec modules exist in `pug-gpui-primitives` but not `pug-primitives`:
`alert_dialog`, `breadcrumbs`, `bulk_action_bar`, `card`, `combobox`,
`detail_row`, `icon`, `list_card`, `nav_card`, `order_by`, `pagination`,
`table`.

Some GPUI specs also diverge from their contract counterparts (e.g. ButtonSpec
adds `chevron`, `height_offset_px()`, `min_width_px()`).

**Actions:**
- Copy 12 missing modules into `pug-primitives`, updating token imports
  from `pug_gpui_tokens` to `pug_tokens`
- For diverged specs: merge GPUI additions into the contracts version. Move
  hardcoded pixel helpers (like `height_offset_px()`) out of the spec and
  into the GPUI component renderer where they belong
- Merge any extra types/enums from GPUI's `types.rs` into contracts `types.rs`
- Verify: `cargo check` for `pug-primitives` and `pug-jetstream-components`
  (Jetstream must not break)

### 003 — Merge Composite Specs

7 modules in GPUI composites not in contracts: `action_discovery_panel`,
`app_header`, `command_palette`, `dock_region`, `metric_tile`,
`shell_status_bar`, `split_view`.

Shared types in GPUI composites `types.rs` also need merging.

**Actions:**
- Copy 7 missing modules into `pug-composites`
- Merge GPUI `types.rs` additions into contracts `types.rs`
- Verify: `cargo check` for `pug-composites`

### 004 — Delete Duplicate GPUI Crates and Update Imports

With specs unified, the GPUI-specific spec crates are redundant.

**Actions:**
- Update `pug-gpui` (adapter) Cargo.toml: depend on `pug-primitives` +
  `pug-composites` instead of `pug-gpui-primitives` + `pug-gpui-composites`
- Update `pug-gpui-components` Cargo.toml: same
- Update `pug-gpui-preview` Cargo.toml: same
- Find-and-replace all `use pug_gpui_primitives::` → `use pug_primitives::`
- Find-and-replace all `use pug_gpui_composites::` → `use pug_composites::`
- Find-and-replace all `use pug_gpui_tokens::` → `use pug_tokens::`
- Delete `packages/gpui/primitives/`
- Delete `packages/gpui/composites/`
- Verify: `cargo check` for all GPUI crates

### 005 — Simplify Component API

Drop the `Pug` prefix from component structs and re-export common types
so developers only need one import.

**Actions:**
- Rename `PugButton` → `Button`, `PugCheckbox` → `Checkbox`, etc. across
  all ~100 component files
- Re-export commonly used types from `pug-gpui-components`:
  `ButtonVariant`, `ControlSize`, `ButtonTone`, `IconSize`, etc.
- Update all specimen files in preview to use new names
- Consider: should the component struct own its props directly (no separate
  spec construction), or take a spec? Decide and implement consistently.
- Verify: `cargo check -p pug-gpui-preview`

### 006 — Delete Workstation Crates

Both `pug-workstation` (contracts) and `pug-gpui-workstation` should be
deleted. Their specs were migrated to composites in g08.

**Actions:**
- Verify no remaining imports of `pug_workstation` or `pug_gpui_workstation`
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
| GPUI `typed` token module causes issues in Jetstream | Already exported by `pug-tokens` — Jetstream just doesn't use it |
| Renaming 100 Pug* structs is error-prone | Mechanical find-and-replace, verify with cargo check |
| Preview apps break during migration | Each milestone ends with cargo check; no milestone ships broken |
