# g09.002 Merge GPUI-Only Specs into Contracts Primitives

Status: planned
Owner: Pug Core
Depends on: g09.001

## Context

`pug-gpui-primitives` has 77 modules; `pug-primitives` has 65. The 12 extra
modules in GPUI need to move into the shared crate. Additionally, some specs
that exist in both crates have diverged — GPUI versions added hardcoded pixel
helpers that belong in the renderer, not the spec.

## Missing Modules (12)

These exist in `pug-gpui-primitives` but not `pug-primitives`:

1. `alert_dialog`
2. `breadcrumbs`
3. `bulk_action_bar`
4. `card`
5. `combobox`
6. `detail_row`
7. `icon`
8. `list_card`
9. `nav_card`
10. `order_by`
11. `pagination`
12. `table`

## Diverged Specs

For specs that exist in both crates, the GPUI version may have additions.
Known divergences:

- **ButtonSpec**: GPUI adds `chevron: bool`, `height_offset_px()`,
  `min_width_px()`, `padding_x_offset_px()`, `font_size_px()`,
  `icon_wrapper_size_px()`. The `chevron` field should be added to the
  shared spec. The `*_px()` methods are rendering concerns — move them
  to the GPUI component renderer.
- **Other specs**: Audit each for similar patterns.

## Actions

- [ ] Copy 12 missing module files into `packages/contracts/primitives/src/`
- [ ] Update token imports: `pug_gpui_tokens` → `pug_tokens`
- [ ] Register modules in `packages/contracts/primitives/src/lib.rs`
- [ ] For each diverged spec:
  - Add genuinely missing fields/methods to the shared version
  - Do NOT copy hardcoded pixel helpers — those go in the renderer
- [ ] Merge any extra types/enums from GPUI `types.rs` into contracts `types.rs`
- [ ] `cargo check -p pug-primitives`
- [ ] `cargo check -p pug-jetstream-components` (must not break Jetstream)

## Acceptance Criteria

- [ ] `pug-primitives` has all 77 modules
- [ ] No hardcoded pixel values in any spec's methods
- [ ] Token methods return token path strings, not pixel values
- [ ] Jetstream crates still compile
